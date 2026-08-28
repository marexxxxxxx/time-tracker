# Design: Hintergrund-Daemon für kontinuierliche Aufzeichnung (C1)

Datum: 2026-08-28

## Problem

Der Screen-Time-Tracker zeichnet nur auf, solange der GUI-Prozess läuft
(`tracker::start_window_tracking`, lib.rs:734, wird in `setup()` gestartet).
Schließt der Nutzer das Fenster, endet der Prozess und die Aufzeichnung stoppt.
Es gibt weder TrayIcon noch Autostart noch `preventClose`. Ziel ist die
Befund C1 aus dem Review: Aufzeichnung muss unabhängig von der GUI permanent
laufen.

## Lösung

Ein eigener headless Daemon-Binary `screen-time-daemon` im selben Cargo-Paket,
der die Tracking-Kernlogik wiederverwendet und kontinuierlich in die SQLite-DB
schreibt. Die GUI wird reiner Viewer/Controller und startet keinen Tracking-Loop
mehr selbst.

## Architektur

- **Gemeinsames Kernmodul:** `tracker.rs` und `idle.rs` werden von Tauri-Typen
  (`AppHandle`, `tauri::async_runtime::spawn`, `Emitter`) entkoppelt. Die
  Tracking-/Idle-/Blocking-Logik arbeitet auf `Connection` + tokio und ist
  frei von Tauri. Der GUI-Emit (`limit-warning`, `idle-state-changed`) wird
  durch ein optionales Ereignis-Callback ersetzt, das nur die GUI nutzt.
- **Daemon-Binary:** `src-tauri/src/bin/daemon.rs`. Öffnet dieselbe DB (via
  wiederverwendetem `init_db`), läuft mit tokio, führt den Tracking-Loop aus.
- **GUI = Viewer:** `run()`/`setup()` in `lib.rs` startet den Tracking-Loop
  **nicht** mehr. Die GUI liest nur noch die DB an.
- **Autostart:** `tauri-plugin-autostart` sorgt dafür, dass der Daemon beim
  Login startet.
- **Lebenszyklus:** Single-Instance-Guard (PID-File), damit nur ein Daemon
  läuft. Die GUI startet den Daemon beim Öffnen, falls er nicht läuft, und
  beendet ihn beim Schließen **nicht**.

## DB-Zugriff / Nebenläufigkeit

- `init_db` setzt bereits `PRAGMA journal_mode=WAL` (lib.rs:494) — gut für
  gleichzeitiges Lesen/Schreiben.
- **Neu:** `PRAGMA busy_timeout` setzen (z.B. 5000ms), damit konkurrierende
  Zugriffe (GUI liest, Daemon schreibt) nicht mit „database is locked“ scheitern.
- Schreib-Verantwortung: Nur der Daemon schreibt `activities`. Die GUI schreibt
  Settings/Blocked-Apps/Limits; der Daemon liest diese Tabellen jede Poll-Runde
  erneut (bestehendes Verhalten in `is_app_blocked`/`get_app_limit_config`).

## Datei-Änderungen

1. `Cargo.toml`: `tauri-plugin-autostart = "2"` hinzufügen.
2. `tauri` feature `tray-icon` **nicht** benötigt (kein Tray bei GUI=Viewer).
3. `lib.rs`:
   - `init_db` und benötigte Kernfunktionen öffentlich machen (`pub`),
     falls der Daemon sie braucht (Daemon ist eigenes bin-Target → braucht
     `pub` API).
   - `setup()`: `start_window_tracking`-Aufruf entfernen; stattdessen Daemon
     sicherstellen (starten falls nicht läuft) und Autostart-Plugin
     registrieren.
4. `tracker.rs` / `idle.rs`: von Tauri entkoppeln; Emits durch optionales
   Callback ersetzen.
5. Neu `src-tauri/src/bin/daemon.rs`: Einstiegspunkt, öffnet DB, startet
   Tracking- und Idle-Loop, PID-File-Guard.
6. `install.sh` / Bundle: Daemon-Binary mit installieren; Autostart-Entry.

## Inhaltstest / Verifikation

- `cargo build` erfolgreich; beide Binaries erzeugt.
- Daemon läuft headless, schreibt `activities` in die DB.
- GUI startet Daemon, wenn er nicht läuft; GUI schließt → Daemon läuft weiter.
- Beim Login startet der Daemon automatisch (Autostart-Entry vorhanden).

## Nicht im Umfang (spätere Befunde)

- C3 (lokale Zeitzone statt UTC) — separate Aufgabe.
- I1–I4, M1–M4 — separate Aufgaben.
- C2 (Idle-Pause + `idle_timeout`-Setting + echte Wayland-Idle-Quelle) wird in
  diesem Zug nur soweit entkoppelt, wie die Extraktion es erfordert; die
  eigentlichen C2-Fixes sind separat.
