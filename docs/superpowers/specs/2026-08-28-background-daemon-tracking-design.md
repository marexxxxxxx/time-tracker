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
  frei von Tauri.
- **Limit-/Idle-Ereignisse via DB:** Der Daemon schreibt anstehende
  Limit-Warnungen (`limit-warning`) in eine kleine `events`-Tabelle in der DB.
  Die GUI pollt diese Tabelle (sie pollt `fetchAll` bereits alle 5s) und zeigt
  den Toast. Kein direkter Prozess-IPC nötig; konsistent mit „GUI = Viewer“.
- **Daemon-Binary:** `src-tauri/src/bin/daemon.rs`. Öffnet dieselbe DB (via
  wiederverwendetem `init_db`), läuft mit tokio, führt den Tracking-Loop aus.
- **GUI = Viewer:** `run()`/`setup()` in `lib.rs` startet den Tracking-Loop
  **nicht** mehr. Die GUI liest nur noch die DB an.
- **Autostart:** Manuell erzeugter Autostart-Eintrag
  (`~/.config/autostart/screen-time-daemon.desktop`), der **direkt auf das
  Daemon-Binary** zeigt (nicht auf die GUI). `tauri-plugin-autostart` wird
  **nicht** verwendet, weil es das aktuelle (GUI-)Binary starten würde. Die GUI
  bekommt einen Toggle in den Settings (enable/disable), der diesen Eintrag
  schreibt/entfernt.
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

1. ~~`tauri-plugin-autostart = "2"` hinzufügen.~~ → **Nicht** benötigt; Autostart
   wird als Desktop-Eintrag manuell geschrieben (siehe Abschnitt Autostart).
2. `tauri` feature `tray-icon` **nicht** benötigt (kein Tray bei GUI=Viewer).
3. `lib.rs`:
   - `init_db` und benötigte Kernfunktionen öffentlich machen (`pub`),
     falls der Daemon sie braucht (Daemon ist eigenes bin-Target → braucht
     `pub` API).
   - `setup()`: `start_window_tracking`-Aufruf entfernen; stattdessen Daemon
     sicherstellen (starten falls nicht läuft) und die Autostart-Toggle-Commands
     registrieren.
4. `tracker.rs` / `idle.rs`: von Tauri entkoppeln; Tauri-Emits durch Schreiben
   in die `events`-Tabelle ersetzen (Daemon-seitig).
5. Neu `src-tauri/src/bin/daemon.rs`: Einstiegspunkt, öffnet DB, startet
   Tracking- und Idle-Loop, PID-File-Guard.
6. `install.sh` / Bundle: Daemon-Binary mit installieren; Autostart-Entry.
7. `lib.rs`: Autostart-Toggle-Commands (`set_autostart(enabled)`) schreiben bzw.
   entfernen den `screen-time-daemon.desktop`-Eintrag.
8. `init_db` (lib.rs:490): `events`-Tabelle anlegen (id, type, payload, created_at)
   und `PRAGMA busy_timeout` setzen.
9. Frontend: GUI pollt `events`-Tabelle und zeigt Limit-Warning-Toast; die alten
   `limit-warning`-Listener (Emit) entfallen.

## Autostart-Eintrag (konkret)

`~/.config/autostart/screen-time-daemon.desktop`:
```
[Desktop Entry]
Type=Application
Name=Screen Time Daemon
Exec=/home/<USER>/.local/bin/screen-time-daemon
Terminal=false
X-GNOME-Autostart-enabled=true
```
Die GUI erstellt/entfernt ihn über einen Settings-Toggle. `/dev/null`-Umleitung
o.Ä. ist nicht nötig; der Daemon läuft headless ohne Fenster.

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
