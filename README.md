<p align="center">
  <img src="screen-time-app/src-tauri/icons/128x128.png" width="96" alt="Screen Time Tracker icon">
</p>

<h1 align="center">Screen Time Tracker</h1>

<p align="center">
  A privacy-first Linux desktop app for tracking screen time, blocking distracting apps, and understanding your digital habits.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri%20v2-blue?logo=tauri" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Svelte%205-orange?logo=svelte" alt="Svelte 5">
  <img src="https://img.shields.io/badge/Rust-2021-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/SQLite-003B57?logo=sqlite" alt="SQLite">
  <img src="https://img.shields.io/badge/License-MIT-green" alt="MIT License">
</p>

---

## Demo

<video controls loop muted width="100%">
  <source src="demo.webm" type="video/webm">
  Your browser does not support the video tag.
</video>

---

## Features

- **Overview Dashboard** -- Daily screen time with bar charts, category breakdowns, and productivity scores
- **Productivity Tracker** -- Deep work sessions with stacked charts tracking productive vs. leisure time
- **App Blocker** -- Block apps or set daily/weekly time limits with automatic enforcement
- **Dark Mode** -- System-aware light/dark theme with M3 design tokens
- **Window Tracking** -- Live background monitoring of active windows, categorized automatically
- **Idle Detection** -- Pauses tracking when you step away from your keyboard
- **Data Privacy** -- Everything stays local. SQLite database, no cloud, no accounts

---

## Installation

### .deb Package (Debian / Ubuntu / Pop!_OS)

Download the latest `.deb` from [Releases](https://github.com/marexxxxxxx/time-tracker/releases) and install:

```bash
sudo dpkg -i screen-time-app_*.deb
sudo apt install -f   # resolve dependencies if needed
```

### AppImage (Any Linux)

Download the `.AppImage`, make it executable, and run:

```bash
chmod +x screen-time-app_*.AppImage
./screen-time-app_*.AppImage
```

### Flatpak

Build and install locally:

```bash
cd screen-time-app/flatpak
flatpak-builder build-dir com.marexxxxxxx.screen-time-app.yml --force-clean --user --install
```

### AUR (Arch Linux)

```bash
# Using an AUR helper like yay
yay -S screen-time-app

# Or manually
cd screen-time-app
makepkg -si
```

### Install Script

The included script handles dependency installation, building, and desktop integration:

```bash
# Build and install system-wide
./install.sh --mode build

# Set up development environment
./install.sh --mode dev

# Remove installed files
./install.sh --mode uninstall
```

Run `./install.sh --help` for details.

---

## Development

**Prerequisites:** [Node.js](https://nodejs.org/) (v18+), [Rust](https://rustup.rs/), npm

**System dependencies** (Tauri on Linux):

```bash
# Arch Linux
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl gtk3 libayatana-appindicator librsvg libxss xdotool

# Debian / Ubuntu
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxss-dev xdotool
```

**Run in dev mode:**

```bash
git clone https://github.com/marexxxxxxx/time-tracker.git
cd time-tracker/screen-time-app
npm install
npm run tauri dev
```

**Build for production:**

```bash
npm run tauri build
```

Output binaries are in `src-tauri/target/release/bundle/` (.deb, .AppImage).

---

## Window Manager Support

| Environment | Window Tracking | Idle Detection |
|------------|----------------|----------------|
| **Hyprland** | `hyprctl activewindow` | ext-idle-notify / GNOME DBus |
| **Sway** | `swaymsg -t get_tree` | -- |
| **GNOME (Wayland)** | DBus | `org.gnome.Mutter.IdleMonitor` |
| **X11** | `xdotool` / `xprop` | `libxss` (XScreenSaver) |

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop framework | Tauri v2 |
| Frontend | Svelte 5 (runes), Tailwind CSS (M3 tokens) |
| Charts | Chart.js v4 |
| Database | SQLite (rusqlite, bundled) |
| Backend | Rust (edition 2021) |

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'feat: add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

Please open an [issue](https://github.com/marexxxxxxx/time-tracker/issues) for bugs or feature requests.

---

## License

[MIT](LICENSE)
