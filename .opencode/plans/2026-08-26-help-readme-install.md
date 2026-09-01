# Help Page, README & Install Script — Implementation Plan

## Goal

Fix the Help page GitHub link, rewrite README professionally, add a multi-mode install script.

## Tasks

### Task 1: Fix Help Page GitHub Link

**File:** `screen-time-app/src/routes/help/+page.svelte:37`

Change `href="https://github.com"` to `href="https://github.com/marexxxxxxx/time-tracker/issues"`.

---

### Task 2: Rewrite README.md

**File:** `README.md`

Structure:
1. **Header** — Project name, one-liner description, tech stack badges (Tauri v2, Svelte 5, Rust, SQLite)
2. **Demo** — Embed existing `demo.webm` with autoplay loop
3. **Features** — Clean bullet list: Dashboard, Productivity Tracker, App Blocker, Dark Mode, Window Tracking, Idle Detection, Data Privacy
4. **Installation** — Section with all methods:
   - `.deb` package (Debian/Ubuntu)
   - AppImage
   - Flatpak
   - AUR (Arch)
   - From source (clone + build)
   - `install.sh` script (reference the new script)
5. **Development** — Clone, install, dev mode
6. **Install Script** — Document `./install.sh --mode build|dev|uninstall`
7. **Contributing** — Brief guidelines
8. **License** — MIT

---

### Task 3: Create `install.sh`

**File:** `install.sh` (repo root)

Three modes:
- `--mode build` (default): Detect distro, install system deps, build release, install binary + desktop entry
- `--mode dev`: Detect distro, install system deps, npm install, create dev desktop entry
- `--mode uninstall`: Remove binary, desktop entry, icon

Features:
- Distro detection (Arch/Debian/Ubuntu/Fedora)
- Colored output, error handling
- `--help` flag
- Checks for Node.js, Rust/cargo, npm before proceeding
- Creates `~/.local/bin/screen-time` launcher + `~/.local/share/applications/screen-time.desktop` + icon
