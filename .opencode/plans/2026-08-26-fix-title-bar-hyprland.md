# Fix: Title Bar auf Wayland/Hyprland entfernen

## Problem
`decorations: false` in `tauri.conf.json` ist gesetzt, aber der Titelrand bleibt sichtbar. Die Einstellung wird beim Kompilieren des Rust-Binärcodes gelesen — ein Hot-Reload oder `npm run dev` allein reicht nicht.

## Ursache
Tauri liest `decorations` beim Erstellen des Fensters (Window Creation Time). Der vorhandene Binary wurde mit der alten Config kompiliert. Ein voller Rebuild ist nötig.

## Lösung

### Schritt 1: Clean Build
```bash
cd screen-time-app
rm -rf src-tauri/target
npm run tauri dev
```

Das erzwingt eine vollständige Neukompilierung des Rust-Binärcodes mit der neuen Config.

### Schritt 2 (Fallback): Falls Schritt 1 nicht funktioniert
Setze `decorations` programmatisch in `lib.rs` beim App-Start:

```rust
// In lib.rs, nach .setup(|app_handle| { ... })
use tauri::Manager;

// Nach dem Setup-Block:
if let Some(window) = app_handle.get_webview_window("main") {
    let _ = window.set_decorations(false);
}
```

Dies setzt die Dekoration zur Laufzeit, unabhängig von der Config.

## Dateien
- `src-tauri/tauri.conf.json` — `"decorations": false` (bereits gesetzt)
- `src-tauri/src/lib.rs` — Fallback: `set_decorations(false)` zur Laufzeit

## Verifikation
1. App komplett stoppen (`pkill -f screen-time-app`)
2. `npm run tauri dev` ausführen
3. Prüfen ob Titelrand verschwunden ist
4. Fenster per Dragging auf TopBar verschieben (data-tauri-drag-region ist bereits gesetzt)
