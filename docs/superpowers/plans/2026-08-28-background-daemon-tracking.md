# Background Daemon Tracking (C1) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move continuous screen-time recording out of the GUI process into a separate headless daemon binary so tracking runs regardless of whether the GUI window is open.

**Architecture:** A new `screen-time-daemon` binary target in the existing `screen-time-app` Cargo package reuses the shared tracking/idle core. `tracker.rs`/`idle.rs` are decoupled from Tauri types (callbacks instead of `emit`). The GUI becomes a pure viewer: `setup()` no longer starts the tracking loop, instead it ensures the daemon is running and exposes `poll_events`/`set_autostart` commands. Limit/idle events travel from daemon to GUI through a new `events` SQLite table.

**Tech Stack:** Rust 2021 (tokio, rusqlite, chrono), Tauri v2 (GUI only), Svelte 5 / TypeScript frontend, SQLite (WAL).

**Spec:** `docs/superpowers/specs/2026-08-28-background-daemon-tracking-design.md`

## Global Constraints

- Same Cargo package: daemon is a bin target at `src-tauri/src/bin/daemon.rs`, reusing the lib crate `screen_time_app_lib`.
- `tracker.rs` and `idle.rs` must contain **no** Tauri types (`AppHandle`, `Emitter`, `tauri::async_runtime`). They use `tokio` directly.
- All day boundaries / timestamps remain in `Utc` — timezone is out of scope (C3 separate).
- C2 (idle pause + `idle_timeout` + real Wayland idle) is explicitly out of scope; idle.rs is only decoupled enough to run in the daemon and write an `idle-state` event row.
- Only the daemon writes to `activities`. The GUI writes settings/blocked_apps/limits; the daemon re-reads them each poll round (existing behavior via `is_app_blocked`/`get_app_limit_config`).
- App identifier for data dir: `marexxxxxxx.screen-time-app`.
- DB path is shared and computed by `lib::db_path()` (both GUI and daemon), matching Tauri's `app_data_dir` resolution on Linux: `$XDG_DATA_HOME/<identifier>` or `$HOME/.local/share/<identifier>`.
- Autostart is a manually written XDG autostart entry pointing **directly at the daemon binary** (`~/.local/bin/screen-time-daemon`). `tauri-plugin-autostart` is NOT used.
- Existing test commands: backend `cargo test` (in `screen-time-app/src-tauri`), frontend `npm run test` (in `screen-time-app`).

---

### Task 1: Backend foundation — shared paths, `events` table, public surface

**Files:**
- Modify: `screen-time-app/src-tauri/src/lib.rs`
- Test: `screen-time-app/src-tauri/src/lib.rs` (in `mod tests`)

**Interfaces:**
- Consumes: nothing new (refactors existing `lib.rs`).
- Produces:
  - `pub fn app_data_dir() -> std::path::PathBuf`
  - `pub fn db_path() -> std::path::PathBuf`
  - `pub fn init_db(db_path: &std::path::Path) -> Result<Connection, Box<dyn std::error::Error>>` (now `pub`)
  - `pub fn write_event(conn: &Connection, event_type: &str, payload: &str) -> rusqlite::Result<usize>`
  - `pub mod tracker;` and `pub mod idle;` (modules made `pub`)

- [ ] **Step 1: Write the failing backend test**

Add to `mod tests` in `lib.rs` a test that `write_event` inserts a row and that `events` rows can be read back:

```rust
#[test]
fn test_write_event_roundtrip() {
    let conn = setup_db();
    // setup_db creates activities/settings/blocked_apps but not events — the
    // events table is created by init_db, so create it here to match.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (id INTEGER PRIMARY KEY AUTOINCREMENT, type TEXT, payload TEXT, created_at TEXT DEFAULT (datetime('now')))",
        [],
    ).unwrap();
    write_event(&conn, "limit-warning", r#"{"app_name":"YouTube","limit_type":"daily","remaining_minutes":5}"#).unwrap();
    let (ty, payload): (String, String) = conn.query_row(
        "SELECT type, payload FROM events WHERE type = 'limit-warning'",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).unwrap();
    assert_eq!(ty, "limit-warning");
    assert!(payload.contains("\"app_name\":\"YouTube\""));
}
```

Also a test that `db_path()` ends with the `screentime.db` filename:

```rust
#[test]
fn test_db_path_basename() {
    let p = db_path();
    assert_eq!(p.file_name().unwrap().to_str().unwrap(), "screentime.db");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test test_write_event_roundtrip test_db_path_basename`
Expected: FAIL — `write_event` and `db_path` are not defined.

- [ ] **Step 3: Implement path helpers, expose modules, add events table**

In `lib.rs`, add after the `mod` declarations:

```rust
pub mod blocker;
pub mod idle;
pub mod tracker;

// Shared data-directory resolution (mirrors Tauri app_data_dir on Linux).
pub fn app_data_dir() -> std::path::PathBuf {
    let identifier = "marexxxxxxx.screen-time-app";
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        std::path::PathBuf::from(xdg).join(identifier)
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(identifier)
    } else {
        std::path::PathBuf::from(identifier)
    }
}

pub fn db_path() -> std::path::PathBuf {
    app_data_dir().join("screentime.db")
}
```

Note: keep the original `mod blocker; mod idle; mod tracker;` first lines as the single declaration — that is, change the existing three lines to `pub mod ...` and ADD the helper functions after them (do not duplicate the module declarations).

- [ ] **Step 4: Make `init_db` public, add `busy_timeout` and `events` table**

Change `fn init_db(` to `pub fn init_db(` at line 490. Inside `init_db`, right after the WAL pragma, add the busy timeout pragma, and after the `blocked_apps`/migrations add the events table:

```rust
    // Enable WAL mode for better concurrency
    conn.pragma_update(None, "journal_mode", "WAL")?;

    // Wait up to 5s instead of failing with "database is locked" when the GUI
    // and daemon access the DB concurrently.
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
```

and after the limit-column migration block:

```rust
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
        [],
    )?;
```

- [ ] **Step 5: Add `write_event` helper**

Add near the other helper functions in `lib.rs`:

```rust
pub fn write_event(conn: &Connection, event_type: &str, payload: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO events (type, payload) VALUES (?1, ?2)",
        params![event_type, payload],
    )
}
```

- [ ] **Step 6: Update GUI `setup()` to use `db_path()`**

In `run()`/`setup()`, replace the tauri-path-based DB resolution. The current block (lines 713-740) resolves `app_data_dir` via `app_handle.path()`. Replace it so it uses the shared helper (identical resolved path) and creates the dir:

```rust
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Initialize db connection using the same path the daemon uses
            let app_dir = crate::app_data_dir();
            if !app_dir.exists() {
                let _ = std::fs::create_dir_all(&app_dir);
            }
            let db_path = crate::db_path();

            match init_db(&db_path) {
                Ok(conn) => {
                    // Seed
                    if let Err(e) = seed_database(&conn) {
                        eprintln!("Failed to seed database: {}", e);
                    }

                    let shared_conn = Arc::new(Mutex::new(Some(conn)));

                    // Save conn to state first
                    let state: State<AppState> = app_handle.state();
                    *state.conn.lock().unwrap() = shared_conn.lock().unwrap().take();
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                }
            }

            Ok(())
        })
```

This temporarily removes the `tracker::start_window_tracking` and `idle::start_idle_detection` calls (they are re-added — daemon-side — in later tasks). Keep `app_handle` (used by later task for daemon spawn).

- [ ] **Step 7: Run the full backend test suite**

Run: `cargo test`
Expected: PASS (existing 70+ tests still pass; new tests pass).

- [ ] **Step 8: Commit**

```bash
git add screen-time-app/src-tauri/src/lib.rs
git commit -m "feat: add shared DB path, events table, and public tracking surface"
```

---

### Task 2: Decouple and refactor `tracker.rs`

**Files:**
- Modify: `screen-time-app/src-tauri/src/tracker.rs`
- Test: `screen-time-app/src-tauri/src/tracker.rs` (in `mod tests`)

**Interfaces:**
- Consumes: nothing new.
- Produces:
  - `pub enum LimitDecision { Ok, Block(String), Warn { limit_type: String, remaining_minutes: i64 } }`
  - `pub fn evaluate_limits(daily_secs: i64, weekly_secs: i64, daily_limit_min: i64, weekly_limit_min: i64) -> LimitDecision`
  - `pub fn start_window_tracking<W>(conn: Arc<Mutex<Option<Connection>>>, on_limit_warning: W) where W: Fn(String, String, i64) + Send + 'static`
  - (unchanged) `WindowInfo`, `categorize_app`, `get_active_window`, etc. remain internal.

- [ ] **Step 1: Write the failing unit tests for `evaluate_limits`**

Add to `mod tests` in `tracker.rs`:

```rust
    #[test]
    fn test_evaluate_limits_ok() {
        use super::LimitDecision;
        assert!(matches!(
            evaluate_limits(0, 0, 60, 300),
            LimitDecision::Ok
        ));
    }

    #[test]
    fn test_evaluate_limits_block_daily() {
        use super::LimitDecision;
        assert!(matches!(
            evaluate_limits(60 * 60, 0, 60, 300),
            LimitDecision::Block(t) if t == "daily"
        ));
    }

    #[test]
    fn test_evaluate_limits_block_weekly() {
        use super::LimitDecision;
        assert!(matches!(
            evaluate_limits(0, 301 * 60, 60, 300),
            LimitDecision::Block(t) if t == "weekly"
        ));
    }

    #[test]
    fn test_evaluate_limits_warn_daily() {
        use super::LimitDecision;
        // 55 min used of 60 daily = warn (5 min remaining)
        assert!(matches!(
            evaluate_limits(55 * 60, 0, 60, 300),
            LimitDecision::Warn { limit_type, remaining_minutes } if limit_type == "daily" && remaining_minutes == 5
        ));
    }

    #[test]
    fn test_evaluate_limits_warn_weekly() {
        use super::LimitDecision;
        // 295 min used of 300 weekly = warn (5 min remaining)
        assert!(matches!(
            evaluate_limits(0, 295 * 60, 60, 300),
            LimitDecision::Warn { limit_type, remaining_minutes } if limit_type == "weekly" && remaining_minutes == 5
        ));
    }

    #[test]
    fn test_evaluate_limits_not_warn_when_not_close() {
        use super::LimitDecision;
        assert!(matches!(
            evaluate_limits(30 * 60, 0, 60, 300),
            LimitDecision::Ok
        ));
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_evaluate_limits`
Expected: FAIL — `evaluate_limits` and `LimitDecision` not defined.

- [ ] **Step 3: Add `LimitDecision` and `evaluate_limits`, and change the signature**

In `tracker.rs`, add near the top after the constants:

```rust
pub enum LimitDecision {
    Ok,
    Block(String),
    Warn { limit_type: String, remaining_minutes: i64 },
}

pub fn evaluate_limits(
    daily_secs: i64,
    weekly_secs: i64,
    daily_limit_min: i64,
    weekly_limit_min: i64,
) -> LimitDecision {
    if daily_limit_min > 0 && daily_secs >= daily_limit_min * 60 {
        return LimitDecision::Block("daily".to_string());
    }
    if weekly_limit_min > 0 && weekly_secs >= weekly_limit_min * 60 {
        return LimitDecision::Block("weekly".to_string());
    }
    let daily_warn = daily_limit_min > 0
        && daily_secs >= daily_limit_min * 60 - WARNING_THRESHOLD_SECS
        && daily_secs < daily_limit_min * 60;
    let weekly_warn = weekly_limit_min > 0
        && weekly_secs >= weekly_limit_min * 60 - WARNING_THRESHOLD_SECS
        && weekly_secs < weekly_limit_min * 60;
    if daily_warn {
        return LimitDecision::Warn {
            limit_type: "daily".to_string(),
            remaining_minutes: (daily_limit_min * 60 - daily_secs) / 60,
        };
    }
    if weekly_warn {
        return LimitDecision::Warn {
            limit_type: "weekly".to_string(),
            remaining_minutes: (weekly_limit_min * 60 - weekly_secs) / 60,
        };
    }
    LimitDecision::Ok
}
```

- [ ] **Step 4: Decouple `start_window_tracking` from Tauri**

Change the signature at line 19:

```rust
pub fn start_window_tracking<W>(
    conn: Arc<Mutex<Option<Connection>>>,
    on_limit_warning: W,
) where
    W: Fn(String, String, i64) + Send + 'static,
{
    tokio::spawn(async move {
```

Then inside the async block, at the top, capture the callback mutably:

```rust
        let mut on_limit_warning = on_limit_warning;
```

Replace the entire limit-check block (currently lines 52-105) with code that uses `evaluate_limits` and the callback:

```rust
                // Check time limits before recording
                if let Ok(db_guard) = conn.lock() {
                    if let Some(db) = db_guard.as_ref() {
                        if let Some(config) = crate::get_app_limit_config(db, &active.app_name) {
                            if config.limit_enabled {
                                let daily_secs = crate::get_app_usage_today(db, &active.app_name);
                                let weekly_secs = crate::get_app_usage_this_week(db, &active.app_name);

                                match evaluate_limits(
                                    daily_secs,
                                    weekly_secs,
                                    config.daily_limit_minutes,
                                    config.weekly_limit_minutes,
                                ) {
                                    LimitDecision::Block(_) => {
                                        // Limit exceeded — block the app
                                        if !active.title.contains(BLOCKED_TITLE_MARKER) {
                                            let active_clone = active.clone();
                                            let st = session_type.clone();
                                            tokio::task::spawn_blocking(move || {
                                                handle_blocked_app(&active_clone, &st);
                                            });
                                        }
                                        current_window = None;
                                        current_id = None;
                                        warned_apps.remove(&active.app_name);
                                        continue;
                                    }
                                    LimitDecision::Warn { limit_type, remaining_minutes } => {
                                        if !warned_apps.contains(&active.app_name) {
                                            warned_apps.insert(active.app_name.clone());
                                            on_limit_warning(
                                                active.app_name.clone(),
                                                limit_type,
                                                remaining_minutes,
                                            );
                                        }
                                    }
                                    LimitDecision::Ok => {}
                                }
                            }
                        }
                    }
                }
```

Remove the now-unused `WARNING_THRESHOLD_SECS`-based inline computation (the constant `WARNING_THRESHOLD_SECS` is still used inside `evaluate_limits`, so keep the constant).

- [ ] **Step 5: Remove Tauri imports from `tracker.rs`**

At the top of `tracker.rs`, change:

```rust
use tauri::{Emitter, AppHandle};
```
to be removed entirely (delete that line). `tokio` is already imported.

- [ ] **Step 6: Run the full backend test suite**

Run: `cargo test`
Expected: PASS — all tracker tests (categorize/extract/url/normalize/find_focused and the new `evaluate_limits` tests) pass; no compile errors from removing Tauri imports.

- [ ] **Step 7: Commit**

```bash
git add screen-time-app/src-tauri/src/tracker.rs
git commit -m "refactor: decouple tracker from Tauri, extract evaluate_limits"
```

---

### Task 3: Decouple `idle.rs`

**Files:**
- Modify: `screen-time-app/src-tauri/src/idle.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `pub fn start_idle_detection<F>(on_idle_change: F) where F: Fn(bool, u64) + Send + 'static` (Tauri-free).

- [ ] **Step 1: Change the signature and remove Tauri usage**

Replace the top of `idle.rs`:

```rust
use std::process::Command;
use std::time::Duration;
use tokio::time::interval;

// 5 minutes in seconds
const IDLE_THRESHOLD_SECONDS: u64 = 5 * 60;

pub fn start_idle_detection<F>(on_idle_change: F)
where
    F: Fn(bool, u64) + Send + 'static,
{
    tokio::spawn(async move {
        let mut on_idle_change = on_idle_change;
        let mut interval = interval(Duration::from_secs(10));
        let mut was_idle = false;

        loop {
            interval.tick().await;

            let idle_time_secs = get_idle_time_seconds();
            let is_idle = idle_time_secs >= IDLE_THRESHOLD_SECONDS;

            if is_idle != was_idle {
                on_idle_change(is_idle, idle_time_secs);

                if is_idle {
                    println!("User became idle after {} seconds.", idle_time_secs);
                } else {
                    println!("User is back from idle.");
                }
                was_idle = is_idle;
            }
        }
    });
}
```

- [ ] **Step 2: Verify it compiles and remove stale imports**

Remove `use tauri::{AppHandle, Emitter};` and the `IdleEvent` struct (now unused). Run:

Run: `cargo build`
Expected: builds with no warnings about unused imports.

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src-tauri/src/idle.rs
git commit -m "refactor: decouple idle detection from Tauri"
```

---

### Task 4: Daemon binary

**Files:**
- Create: `screen-time-app/src-tauri/src/bin/daemon.rs`

**Interfaces:**
- Consumes: `screen_time_app_lib::{db_path, app_data_dir, init_db, write_event, tracker::{start_window_tracking}, idle::start_idle_detection}`
- Produces: a headless binary `screen-time-daemon` that writes PID file, runs tracking + idle loops.

- [ ] **Step 1: Create the daemon source**

Create `screen-time-app/src-tauri/src/bin/daemon.rs` with this complete content (it creates a tokio runtime, guards against double-start via a PID file, opens the shared DB, and starts the tracking and idle loops as tokio tasks):

```rust
use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use screen_time_app_lib::{app_data_dir, db_path, init_db, write_event};

// Writes a limit-warning event row for the GUI to poll.
fn set_limit_warning(conn: &Arc<Mutex<Option<Connection>>>, app_name: String, limit_type: String, remaining_minutes: i64) {
    if let Ok(guard) = conn.lock() {
        if let Some(db) = guard.as_ref() {
            let payload = format!(
                r#"{{"app_name":{},"limit_type":{},"remaining_minutes":{}}}"#,
                serde_json::to_string(&app_name).unwrap_or_default(),
                serde_json::to_string(&limit_type).unwrap_or_default(),
                remaining_minutes
            );
            let _ = write_event(db, "limit-warning", &payload);
        }
    }
}

fn set_idle_state(conn: &Arc<Mutex<Option<Connection>>>, is_idle: bool, idle_time_seconds: u64) {
    if let Ok(guard) = conn.lock() {
        if let Some(db) = guard.as_ref() {
            let payload = format!(
                r#"{{"is_idle":{},"idle_time_seconds":{}}}"#,
                is_idle, idle_time_seconds
            );
            let _ = write_event(db, "idle-state", &payload);
        }
    }
}

fn write_pid_file() {
    let dir = app_data_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("daemon.pid"), format!("{}\n", std::process::id()));
}

fn acquire_single_instance() -> bool {
    let pid_file = app_data_dir().join("daemon.pid");
    if let Ok(content) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = content.trim().parse::<i32>() {
            if Path::new(&format!("/proc/{}", pid)).exists() {
                return false; // another daemon is already running
            }
        }
    }
    write_pid_file();
    true
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("failed to start tokio runtime");

    if !acquire_single_instance() {
        println!("screen-time-daemon already running; exiting.");
        std::process::exit(0);
    }

    let dir = app_data_dir();
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }

    let conn = match init_db(&db_path()) {
        Ok(c) => Arc::new(Mutex::new(Some(c))),
        Err(e) => {
            eprintln!("Failed to init database: {}", e);
            std::process::exit(1);
        }
    };

    let conn_for_events = Arc::clone(&conn);
    let conn_for_idle = Arc::clone(&conn);

    // Start the loops from inside the runtime so their internal tokio::spawn
    // calls have an active runtime context, then keep the process alive.
    rt.block_on(async {
        screen_time_app_lib::tracker::start_window_tracking(
            Arc::clone(&conn),
            move |app_name, limit_type, remaining| {
                set_limit_warning(&conn_for_events, app_name, limit_type, remaining);
            },
        );

        screen_time_app_lib::idle::start_idle_detection(move |is_idle, idle_time_seconds| {
            set_idle_state(&conn_for_idle, is_idle, idle_time_seconds);
        });

        std::future::pending::<()>().await;
    });
}
```

- [ ] **Step 2: Build the daemon**

Run: `cargo build --bin screen-time-daemon`
(Within `screen-time-app`.) Expected: compiles; binary produced at `src-tauri/target/debug/screen-time-daemon`.

- [ ] **Step 4: Manual smoke test of single-instance guard**

Run: `./src-tauri/target/debug/screen-time-daemon` then immediately run it again in the background in a second shell and confirm the second prints "already running" and exits. Then kill the first.

- [ ] **Step 5: Commit**

```bash
git add screen-time-app/src-tauri/src/bin/daemon.rs
git commit -m "feat: add headless screen-time-daemon binary"
```

---

### Task 5: GUI — ensure daemon runs + `poll_events` + `set_autostart`

**Files:**
- Modify: `screen-time-app/src-tauri/src/lib.rs`
- Test: `screen-time-app/src-tauri/src/lib.rs` (`mod tests`)

**Interfaces:**
- Consumes: `db_path()`/`app_data_dir()` (Task 1).
- Produces:
  - `#[tauri::command] fn poll_events(state: State<AppState>, after_id: i64) -> Result<Vec<TrackedEvent>, String>`
  - `#[tauri::command] fn set_autostart(enabled: bool) -> Result<(), String>`
  - `struct TrackedEvent { id: i64, event_type: String, payload: String }`
  - `fn ensure_daemon_running()` (internal)
  - `fn daemon_running() -> bool` (internal)

- [ ] **Step 1: Add `poll_events` command**

Add near other commands in `lib.rs`:

```rust
#[derive(Debug, Serialize)]
pub struct TrackedEvent {
    pub id: i64,
    pub event_type: String,
    pub payload: String,
}

#[tauri::command]
fn poll_events(state: State<'_, AppState>, after_id: i64) -> Result<Vec<TrackedEvent>, String> {
    let conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_ref().ok_or("Database not initialized")?;
    let mut stmt = conn
        .prepare("SELECT id, type, payload FROM events WHERE id > ?1 ORDER BY id ASC")
        .map_err(|e| e.to_string())?;
    let events: Vec<TrackedEvent> = stmt
        .query_map(params![after_id], |row| {
            Ok(TrackedEvent {
                id: row.get(0)?,
                event_type: row.get(1)?,
                payload: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .flatten()
        .collect();
    if let Some(max_id) = events.last().map(|e| e.id) {
        conn.execute("DELETE FROM events WHERE id <= ?1", params![max_id])
            .map_err(|e| e.to_string())?;
    }
    Ok(events)
}
```

- [ ] **Step 2: Add `set_autostart` command**

```rust
#[tauri::command]
fn set_autostart(enabled: bool) -> Result<(), String> {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| std::path::PathBuf::from(h).join(".config"))
                .unwrap_or_default()
        });
    let autostart_dir = config_home.join("autostart");
    let entry = autostart_dir.join("screen-time-daemon.desktop");

    if enabled {
        std::fs::create_dir_all(&autostart_dir).map_err(|e| e.to_string())?;
        let home = std::env::var("HOME").unwrap_or_default();
        let exec = format!("{}/.local/bin/screen-time-daemon", home);
        let content = format!(
            "[Desktop Entry]\nType=Application\nName=Screen Time Daemon\nExec={}\nTerminal=false\nX-GNOME-Autostart-enabled=true\n",
            exec
        );
        std::fs::write(&entry, content).map_err(|e| e.to_string())?;
    } else {
        let _ = std::fs::remove_file(&entry);
    }
    Ok(())
}
```

- [ ] **Step 3: Add daemon-ensure helpers**

```rust
fn daemon_running() -> bool {
    let pid_file = app_data_dir().join("daemon.pid");
    if let Ok(content) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = content.trim().parse::<i32>() {
            return std::path::Path::new(&format!("/proc/{}", pid)).exists();
        }
    }
    false
}

fn ensure_daemon_running() {
    if daemon_running() {
        return;
    }
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("screen-time-daemon"));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        candidates.push(std::path::PathBuf::from(home)
            .join(".local/bin/screen-time-daemon"));
    }
    for cand in candidates {
        if cand.exists() {
            let _ = std::process::Command::new(&cand).spawn();
            return;
        }
    }
    eprintln!("screen-time-daemon binary not found; tracking is not running.");
}
```

- [ ] **Step 4: Wire `ensure_daemon_running()` into `setup()` and register commands**

In `setup()`, inside the `match init_db(...)` `Ok(conn)` arm after storing the connection, add:

```rust
                    // Ensure the background daemon is running (do not kill on exit).
                    ensure_daemon_running();
```

Add `poll_events` and `set_autostart` to the `invoke_handler!` list:

```rust
        .invoke_handler(tauri::generate_handler![
            ...existing...
            poll_events,
            set_autostart,
        ])
```

- [ ] **Step 5: Build and run the full backend test suite**

Run: `cargo test && cargo build`
Expected: PASS; GUI builds with the daemon-ensure call and new commands.

- [ ] **Step 6: Commit**

```bash
git add screen-time-app/src-tauri/src/lib.rs
git commit -m "feat: GUI ensures daemon runs; add poll_events and set_autostart"
```

---

### Task 6: Frontend — poll events for limit warning toast + autostart toggle

**Files:**
- Modify: `screen-time-app/src/routes/+layout.svelte`
- Modify: `screen-time-app/src/lib/stores/idle.ts`
- Modify: `screen-time-app/src/routes/settings/+page.svelte`
- Test: `screen-time-app/src/lib/stores/*.test.ts` (ensure existing tests still pass)

**Interfaces:**
- Consumes: `invoke('poll_events', { afterId })` returning `Array<{ id, event_type, payload }>`.
- Produces: a 5s-poll that handles `limit-warning` (toast) events; settings autostart toggle calls `invoke('set_autostart', { enabled })`.

- [ ] **Step 1: Update `idle.ts` to expose a pure parser**

Replace `setupIdleListener` usage with a pure helper (keep `isIdle` store):

```ts
import { writable } from 'svelte/store';

export const isIdle = writable(false);

export interface IdleStatePayload {
    is_idle: boolean;
    idle_time_seconds: number;
}

export function parseIdleEvent(payload: string): IdleStatePayload | null {
    try {
        const parsed = JSON.parse(payload);
        if (typeof parsed.is_idle !== 'boolean') return null;
        return {
            is_idle: parsed.is_idle,
            idle_time_seconds: typeof parsed.idle_time_seconds === 'number' ? parsed.idle_time_seconds : 0,
        };
    } catch (e) {
        return null;
    }
}
```

Remove `setupIdleListener`. Update any still-referencing import in `+layout.svelte` (handled in the next step).

- [ ] **Step 2: Update `+layout.svelte` to poll events**

Replace the event-listener imports and logic. Remove `import { listen } from '@tauri-apps/api/event';` and `import { setupIdleListener } from '$lib/stores/idle';` — import `isIdle` and `parseIdleEvent` instead:

```ts
    import { isIdle, parseIdleEvent } from '$lib/stores/idle';
```

Add state for the last seen event id, and a `pollEvents()` function:

```ts
    let lastEventId = 0;

    type TrackedEvent = { id: number; event_type: string; payload: string };

    async function pollEvents() {
        try {
            const events = await invoke<TrackedEvent[]>('poll_events', { afterId: lastEventId });
            for (const ev of events) {
                if (ev.event_type === 'limit-warning') {
                    const p = JSON.parse(ev.payload);
                    showWarning(p.app_name, p.limit_type, p.remaining_minutes);
                } else if (ev.event_type === 'idle-state') {
                    const idle = parseIdleEvent(ev.payload);
                    if (idle) isIdle.set(idle.is_idle);
                }
                lastEventId = ev.id;
            }
        } catch (e) {
            console.error('Failed to poll events:', e);
        }
    }
```

`invoke` must be imported: `import { invoke } from '@tauri-apps/api/core';`.

Replace the `onMount` body so it no longer sets up Tauri event listeners:

```ts
    onMount(async () => {
        try {
            pollEvents();
            await fetchAll();
            pollInterval = setInterval(() => {
                pollEvents();
                fetchAll();
            }, 5000);
        } catch (e) {
            console.error("Failed to setup layout:", e);
        }
    });
```

Remove the `unlisten`/`unlistenWarning` variables and their `onDestroy` cleanup (keep cleaning up `pollInterval` and `warningTimeout`):

```ts
    onDestroy(() => {
        if (pollInterval) clearInterval(pollInterval);
        if (warningTimeout) clearTimeout(warningTimeout);
    });
```

- [ ] **Step 3: Add autostart toggle to settings page**

Open `src/routes/settings/+page.svelte`. Add a new toggle section under Notifications (or a new "Startup" section). Add handlers:

```ts
    async function toggleAutostart() {
        const next = !autostartEnabled;
        await invoke('set_autostart', { enabled: next });
        autostartEnabled = next;
    }
```

Declare `let autostartEnabled = $state(false);` in the script, and initialize it on mount by checking for the toggled state (for the MVP, default to `false`; the toggle calls the backend which writes/removes the entry — the script-side initial value is best-effort). Add a section:

```svelte
        <!-- Startup -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Startup</h2>
            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Start in background at login</span>
                <button
                    aria-label="Toggle background start at login"
                    class="w-12 h-6 rounded-full transition-colors relative {autostartEnabled ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={toggleAutostart}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {autostartEnabled ? 'translate-x-6' : ''}"></span>
                </button>
            </label>
        </section>
```

- [ ] **Step 4: Run frontend checks and tests**

Run: `npm run check && npm run test` (in `screen-time-app`).
Expected: svelte-check passes; existing vitest tests pass (idle.ts no longer exports `setupIdleListener`, so update `stores.test.ts` only if it imported it — it does not currently).

- [ ] **Step 5: Commit**

```bash
git add screen-time-app/src/routes/+layout.svelte screen-time-app/src/lib/stores/idle.ts screen-time-app/src/routes/settings/+page.svelte
git commit -m "feat: poll events table for limit warnings; add autostart toggle"
```

---

### Task 7: install.sh — install daemon binary and autostart entry

**Files:**
- Modify: `install.sh`

**Interfaces:**
- Consumes: the built `screen-time-daemon` binary at `src-tauri/target/release/screen-time-daemon`.
- Produces: `~/.local/bin/screen-time-daemon` and `~/.config/autostart/screen-time-daemon.desktop` in build mode.

- [ ] **Step 1: Update install.sh build mode to copy the daemon**

Add a `LOCAL_BIN_DAEMON` variable and copy the daemon binary in `do_build()` after copying the GUI binary:

```bash
BIN_NAME="screen-time-app"
DAEMON_NAME="screen-time-daemon"
LOCAL_BIN_APP="$LOCAL_BIN/screen-time"
LOCAL_BIN_DAEMON="$LOCAL_BIN/screen-time-daemon"
```

In `do_build()`, after the existing GUI binary install block:

```bash
    # Install daemon binary
    local daemon_bin="$APP_DIR/src-tauri/target/release/$DAEMON_NAME"
    if [ -f "$daemon_bin" ]; then
        cp "$daemon_bin" "$LOCAL_BIN_DAEMON"
        chmod +x "$LOCAL_BIN_DAEMON"
        ok "Daemon installed to $LOCAL_BIN_DAEMON"
    else
        warn "Daemon binary not found at $daemon_bin; skipping daemon install"
    fi
```

- [ ] **Step 2: Create the autostart entry**

Add a function and call it in `do_build()` (after desktop entry creation):

```bash
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
```

Call `create_autostart_entry` at the end of `do_build()`.

- [ ] **Step 3: Update uninstall to remove daemon and autostart**

In `remove_desktop_entry()` (or `do_uninstall()`), remove the daemon binary and autostart entry:

```bash
remove_desktop_entry() {
    rm -f "$DESKTOP_FILE" "$ICON_DEST" "$LOCAL_BIN_APP" "$LOCAL_BIN_DAEMON"
    rm -f "$HOME/.config/autostart/screen-time-daemon.desktop"
    ok "Desktop entry, icon, daemon, and autostart entry removed"
}
```

- [ ] **Step 4: Verify script syntax**

Run: `bash -n install.sh`
Expected: no syntax errors.

- [ ] **Step 5: Commit**

```bash
git add install.sh
git commit -m "feat: install daemon binary and autostart entry"
```

---

## Execution

Plan complete. Two execution options:

1. **Subagent-Driven (recommended)** — dispatch a fresh subagent per task, review between tasks.
2. **Inline Execution** — run tasks in this session with checkpoints.
