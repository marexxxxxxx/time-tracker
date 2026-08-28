#!/usr/bin/env bash
set -euo pipefail

# Screen Time Tracker — Install Script
# Modes: build (default), dev, uninstall

APP_NAME="screen-time-app"
DISPLAY_NAME="Screen Time"
REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
APP_DIR="$REPO_DIR/screen-time-app"
BIN_NAME="screen-time-app"
DAEMON_NAME="screen-time-daemon"
LOCAL_BIN="$HOME/.local/bin"
LOCAL_BIN_APP="$LOCAL_BIN/screen-time"
LOCAL_BIN_DAEMON="$LOCAL_BIN/screen-time-daemon"
LOCAL_SHARE="$HOME/.local/share"
ICON_SRC="$APP_DIR/src-tauri/icons/128x128.png"
ICON_DEST="$LOCAL_SHARE/icons/hicolor/128x128/apps/screen-time.png"
DESKTOP_FILE="$LOCAL_SHARE/applications/screen-time.desktop"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
ok()    { echo -e "${GREEN}[ OK ]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
err()   { echo -e "${RED}[ERR ]${NC} $*" >&2; }
die()   { err "$*"; exit 1; }

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Options:
  --mode MODE    Install mode: build, dev, uninstall (default: build)
  --help         Show this help message

Modes:
  build    Install system dependencies, build release, install binary + desktop entry
  dev      Install system dependencies, set up dev environment with desktop entry
  uninstall Remove installed binary, desktop entry, and icon
EOF
}

# --- Distro Detection ---

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            arch|manjaro|endeavouros) echo "arch" ;;
            debian|ubuntu|pop|linuxmint|zorin) echo "debian" ;;
            fedora) echo "fedora" ;;
            *) echo "unknown" ;;
        esac
    else
        echo "unknown"
    fi
}

# --- Dependency Installation ---

install_deps_arch() {
    info "Installing dependencies for Arch Linux..."
    sudo pacman -S --needed --noconfirm \
        webkit2gtk-4.1 base-devel curl wget file openssl gtk3 \
        libayatana-appindicator librsvg libxss xdotool
}

install_deps_debian() {
    info "Installing dependencies for Debian/Ubuntu..."
    sudo apt update -qq
    sudo apt install -y -qq \
        libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev \
        libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxss-dev xdotool
}

install_deps_fedora() {
    info "Installing dependencies for Fedora..."
    sudo dnf install -y \
        webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
        librsvg2-devel libXScrnSaver-devel xdotool openssl-devel \
        curl wget file
}

install_deps() {
    local distro
    distro=$(detect_distro)
    case "$distro" in
        arch)   install_deps_arch ;;
        debian) install_deps_debian ;;
        fedora) install_deps_fedora ;;
        *)      warn "Unknown distro ($distro). Attempting generic install..."
                warn "You may need to install Tauri dependencies manually."
                warn "See: https://v2.tauri.app/start/prerequisites/"
                return 0 ;;
    esac
}

# --- Prerequisite Checks ---

check_prereqs() {
    local missing=0
    command -v node >/dev/null 2>&1 || { err "Node.js not found. Install from https://nodejs.org/"; missing=1; }
    command -v npm  >/dev/null 2>&1 || { err "npm not found."; missing=1; }
    command -v rustc >/dev/null 2>&1 || { err "Rust not found. Install from https://rustup.rs/"; missing=1; }
    command -v cargo >/dev/null 2>&1 || { err "cargo not found."; missing=1; }
    [ "$missing" -eq 1 ] && die "Missing prerequisites. Install them and retry."
    ok "All prerequisites found"
}

# --- Desktop Entry ---

create_desktop_entry() {
    local exec_cmd="$1"
    mkdir -p "$LOCAL_BIN" "$LOCAL_SHARE/icons/hicolor/128x128/apps" "$LOCAL_SHARE/applications"
    [ -f "$ICON_SRC" ] && cp "$ICON_SRC" "$ICON_DEST"
    cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Type=Application
Exec=$exec_cmd
Icon=screen-time
Terminal=false
Categories=Utility;Monitor;
Name=$DISPLAY_NAME
GenericName=Screen Time Tracker
Comment=Track and manage your screen time
StartupNotify=true
EOF
    chmod +x "$DESKTOP_FILE"
    ok "Desktop entry created at $DESKTOP_FILE"
}

remove_desktop_entry() {
    rm -f "$DESKTOP_FILE" "$ICON_DEST" "$LOCAL_BIN_APP" "$LOCAL_BIN_DAEMON"
    rm -f "$HOME/.config/autostart/screen-time-daemon.desktop"
    ok "Desktop entry, icon, daemon, and autostart entry removed"
}

create_autostart_entry() {
    local entry_dir="$HOME/.config/autostart"
    mkdir -p "$entry_dir"
    cat > "$entry_dir/screen-time-daemon.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Screen Time Daemon
Exec=$LOCAL_BIN_DAEMON
Terminal=false
X-GNOME-Autostart-enabled=true
EOF
    ok "Autostart entry created at $entry_dir/screen-time-daemon.desktop"
}

# --- Build Mode ---

do_build() {
    info "=== Build Mode ==="

    check_prereqs
    install_deps

    info "Installing npm dependencies..."
    cd "$APP_DIR"
    npm install --no-fund --no-audit

    info "Building release..."
    npm run tauri build

    # Find the built binary
    local binary="$APP_DIR/src-tauri/target/release/$BIN_NAME"
    [ -f "$binary" ] || die "Build failed: binary not found at $binary"

    # Install binary
    mkdir -p "$LOCAL_BIN"
    cp "$binary" "$LOCAL_BIN_APP"
    chmod +x "$LOCAL_BIN_APP"
    ok "Binary installed to $LOCAL_BIN_APP"

    # Install daemon binary
    local daemon_bin="$APP_DIR/src-tauri/target/release/$DAEMON_NAME"
    local daemon_installed=0
    if [ -f "$daemon_bin" ]; then
        cp "$daemon_bin" "$LOCAL_BIN_DAEMON"
        chmod +x "$LOCAL_BIN_DAEMON"
        daemon_installed=1
        ok "Daemon installed to $LOCAL_BIN_DAEMON"
    else
        warn "Daemon binary not found at $daemon_bin; skipping daemon install"
    fi

    # Install .deb if available
    local deb
    deb=$(find "$APP_DIR/src-tauri/target/release/bundle/deb" -name "*.deb" 2>/dev/null | head -1)
    if [ -n "$deb" ]; then
        info "Installing .deb package..."
        sudo dpkg -i "$deb" || sudo apt install -f -y
        ok ".deb installed"
    fi

    # Desktop entry
    create_desktop_entry "$LOCAL_BIN_APP"

    # Autostart entry for background daemon
    if [ "$daemon_installed" -eq 1 ]; then
        create_autostart_entry
    fi

    echo ""
    ok "Installation complete!"
    info "Run '$DISPLAY_NAME' from your application launcher or terminal."
}

# --- Dev Mode ---

do_dev() {
    info "=== Dev Mode ==="

    check_prereqs
    install_deps

    info "Installing npm dependencies..."
    cd "$APP_DIR"
    npm install --no-fund --no-audit

    # Create dev launcher
    mkdir -p "$LOCAL_BIN"
    cat > "$LOCAL_BIN_APP" <<LAUNCHER
#!/bin/bash
cd "$APP_DIR"
if ! lsof -ti:1420 &>/dev/null; then
    npx vite dev --port 1420 &>/dev/null &
    sleep 2
fi
exec cargo tauri dev
LAUNCHER
    chmod +x "$LOCAL_BIN_APP"
    ok "Dev launcher created at $LOCAL_BIN_APP"

    # Desktop entry
    create_desktop_entry "$LOCAL_BIN_APP"

    echo ""
    ok "Dev environment ready!"
    info "Run 'screen-time' from your launcher, or cd into screen-time-app and run 'npm run tauri dev'."
}

# --- Uninstall Mode ---

do_uninstall() {
    info "=== Uninstall ==="

    remove_desktop_entry

    # Remove binary
    if [ -f "$LOCAL_BIN_APP" ]; then
        rm -f "$LOCAL_BIN_APP"
        ok "Binary removed from $LOCAL_BIN_APP"
    fi

    # Remove build artifacts (optional, ask)
    if [ -d "$APP_DIR/src-tauri/target" ]; then
        info "Build artifacts still exist at src-tauri/target/"
        read -rp "Remove build artifacts? This saves ~1GB+ [y/N] " confirm
        if [[ "$confirm" =~ ^[Yy]$ ]]; then
            rm -rf "$APP_DIR/src-tauri/target"
            ok "Build artifacts removed"
        fi
    fi

    echo ""
    ok "Uninstall complete."
}

# --- Main ---

MODE="build"

while [ $# -gt 0 ]; do
    case "$1" in
        --mode)
            shift
            MODE="${1:-build}"
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            die "Unknown option: $1. Use --help for usage."
            ;;
    esac
done

case "$MODE" in
    build)    do_build ;;
    dev)      do_dev ;;
    uninstall) do_uninstall ;;
    *)        die "Invalid mode: $MODE. Use build, dev, or uninstall." ;;
esac
