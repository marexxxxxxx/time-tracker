# 📊 ScreenTime Dashboard

A modern, minimalist Linux desktop application to analyze screen time, track productivity, and manage app limits.
Built with **Tauri v2**, **Svelte 5**, **Tailwind CSS**, and **SQLite**.

## 🚀 Features

*   **Overview Dashboard:** Visual summary of daily usage with faux bar charts and category breakdowns.
*   **Productivity Tracker:** Detailed insights into "Deep Work" sessions, tracking productive vs. leisure time.
*   **App Blocker:** Manage focus schedules (mock UI).
*   **Automatic Window Tracking:** Live background tracking of active windows (via `xdotool`, `hyprctl`, `swaymsg`) and scoring them into categories.
*   **Idle Detection:** Automatically detect if you are away from the keyboard and pause the timer (via `xss` for X11, DBus for GNOME/Wayland).
*   **System Tray:** Quick access to the dashboard from your Linux tray.

---

## 🛠 Prerequisites

Make sure you have the required dependencies for Tauri on Linux:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxss-dev xdotool
```

---

## 🏗 Development

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/DEIN-BENUTZERNAME/screen-time-app.git
    cd screen-time-app
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

---

## 📄 License

This project is licensed under the MIT License.
