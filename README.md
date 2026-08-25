# 📊 ScreenTime Dashboard

A modern, minimalist Linux desktop application to analyze screen time, track productivity, and manage app limits.
Built with **Tauri v2**, **Svelte 5**, **Tailwind CSS**, and **SQLite**.

---

## 🎥 Demo

<video controls autoplay loop src="demo.webm" width="100%"></video>

---

## 🚀 Features

*   **Overview Dashboard:** Visual summary of daily screen time with real Chart.js bar charts, category donut charts, and productivity scores — all connected to the backend.
*   **Productivity Tracker:** Detailed insights into "Deep Work" sessions with stacked bar charts, tracking productive vs. leisure time per day.
*   **App Blocker:** Full-featured app blocking with add/remove/toggle, enforcement via window manager rules (Hyprland, Sway, X11), and automatic blocking on each tracking poll.
*   **Dark Mode:** Toggle between light and dark themes with localStorage persistence.
*   **Automatic Window Tracking:** Live background tracking of active windows and scoring them into categories.
*   **Idle Detection:** Automatically detect if you are away from the keyboard and pause the timer.

### 🖥 Window Tracking & Idle Detection Setups

Depending on your desktop environment or window manager (especially common on Arch Linux), the background daemon uses different tools to track active windows and idle time:

*   **X11 (General):** Uses `xdotool` for window tracking and `xprintidle` (or `libxss`) for idle detection.
*   **Hyprland:** Uses `hyprctl activewindow` to track the current window. Idle detection typically relies on `ext-idle-notify` or GNOME DBus if integrated.
*   **Sway:** Uses `swaymsg -t get_tree` to find the focused window.
*   **GNOME / Wayland:** Uses DBus (`org.gnome.Mutter.IdleMonitor`) for both idle detection and window activity.
*   **System Tray:** Quick access to the dashboard from your Linux tray.

---

## 🛠 Prerequisites

Make sure you have the required system dependencies for Tauri on Linux.

**For Arch Linux:**
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl gtk3 libayatana-appindicator librsvg libxss xdotool
```

**For Debian / Ubuntu:**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxss-dev xdotool
```

### Tech Stack

*   **Backend:** Rust (Tauri v2, rusqlite, x11)
*   **Frontend:** Svelte 5 (runes), Tailwind CSS (M3 color tokens)
*   **Charts:** Chart.js v4 + svelte-chartjs
*   **Database:** SQLite (bundled via rusqlite)
*   **Window Managers:** Hyprland (hyprctl), Sway (swaymsg), X11 (xdotool/xprop)

---

## 🏗 Development

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/marexxxxxxx/time-tracker.git
    cd time-tracker/screen-time-app
    ```
2.  **Install JS dependencies:**
    ```bash
    npm install
    ```
3.  **Run in Development Mode:**
    ```bash
    npm run tauri dev
    ```

---

## 📲 Installing as Desktop App

After building, you can install Screen Time as a desktop app with a launcher entry.

### Quick Install (Dev Mode)

Copy the launcher script and desktop entry:

```bash
# Create launcher script
mkdir -p ~/.local/bin
cat > ~/.local/bin/screen-time << 'EOF'
#!/bin/bash
cd /home/user/Projects/time-tracker/screen-time-app
if ! lsof -ti:1420 &>/dev/null; then
    npx vite dev --port 1420 &>/dev/null &
    sleep 2
fi
exec /home/user/Projects/time-tracker/screen-time-app/src-tauri/target/debug/screen-time-app
EOF
chmod +x ~/.local/bin/screen-time

# Create desktop entry (shows in app launcher / Walker)
mkdir -p ~/.local/share/icons/hicolor/128x128/apps
cp screen-time-app/src-tauri/icons/128x128.png ~/.local/share/icons/hicolor/128x128/apps/screen-time.png

cat > ~/.local/share/applications/screen-time.desktop << 'EOF'
[Desktop Entry]
Type=Application
Exec=/home/user/.local/bin/screen-time
Icon=screen-time
Terminal=false
Categories=Utility;Monitor;
Name=Screen Time
GenericName=Screen Time Tracker
Comment=Track and manage your screen time
StartupNotify=true
EOF
```

Then run `screen-time` from terminal or search "Screen Time" in Walker (SUPER+D).

### Production Install

Build the release binary first, then install the desktop entry pointing to it:

```bash
cd screen-time-app
npm run tauri build

# The .deb can be installed directly:
sudo dpkg -i src-tauri/target/release/bundle/deb/screen-time-app_*.deb
```

---

## 📦 Building & Packaging

This project is configured to output both a `.deb` package and an `.AppImage`.

To build the release version:
```bash
npm run tauri build
```
The output binaries will be placed in `src-tauri/target/release/bundle/`.

### Arch Linux (AUR) Installation

A `PKGBUILD` is provided in the repository root for users of Arch-based distributions. You can install it using `makepkg`:

```bash
cd screen-time-app
makepkg -si
```

### Flatpak Installation

You can build and install the application locally as a Flatpak.

1. **Install Prerequisites:**
   Ensure you have `flatpak` and `flatpak-builder` installed, and the Flathub repository added.

   **For Arch Linux:**
   ```bash
   sudo pacman -S flatpak flatpak-builder
   flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
   ```

   **For Debian / Ubuntu:**
   ```bash
   sudo apt install flatpak flatpak-builder
   flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
   ```

2. **Build and Install:**
   Navigate to the flatpak directory and run `flatpak-builder`:
   ```bash
   cd screen-time-app/flatpak
   flatpak-builder build-dir com.marexxxxxxx.screen-time-app.yml --force-clean --user --install
   ```

---

## 📄 License

This project is licensed under the MIT License.
