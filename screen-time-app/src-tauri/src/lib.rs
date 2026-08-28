use chrono::{Utc, Duration, Timelike};
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub app_name: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub duration: i64,
    pub category: String,
    pub productivity_score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    pub total_duration: i64,
    pub productivity_score: i64,
    pub app_usage: Vec<AppUsage>,
    pub categories: Vec<CategoryBreakdown>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUsage {
    pub app_name: String,
    pub duration: i64,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub name: String,
    pub duration: i64,
    pub percentage: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityDay {
    pub day: String,
    pub productive_duration: i64,
    pub neutral_duration: i64,
    pub leisure_duration: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepWorkSession {
    pub app_name: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub duration: i64,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedApp {
    pub id: i64,
    pub app_name: String,
    pub is_blocked: bool,
    pub daily_limit_minutes: i64,
    pub weekly_limit_minutes: i64,
    pub limit_enabled: bool,
}

// AppState to hold the rusqlite connection
struct AppState {
    conn: Arc<Mutex<Option<Connection>>>,
}

#[tauri::command]
fn get_activities(state: State<'_, AppState>) -> Result<Vec<Activity>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT id, app_name, title, start_time, end_time, duration, category, productivity_score FROM activities")
            .map_err(|e| e.to_string())?;

        let activity_iter = stmt.query_map([], |row| {
            Ok(Activity {
                id: row.get(0)?,
                app_name: normalize_display_name(&row.get::<_, String>(1)?),
                title: row.get(2)?,
                start_time: row.get(3)?,
                end_time: row.get(4)?,
                duration: row.get(5)?,
                category: row.get(6)?,
                productivity_score: row.get(7)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut activities = Vec::new();
        for activity in activity_iter {
            if let Ok(act) = activity {
                activities.push(act);
            }
        }
        Ok(activities)
    } else {
        Err("Database connection not initialized yet".to_string())
    }
}

#[tauri::command]
/// Normalize stored app names for display grouping (terminals → "Terminal")
fn normalize_display_name(app_name: &str) -> String {
    if app_name.contains('@') && app_name.contains(':') && (app_name.contains('~') || app_name.contains('/')) {
        return "Terminal".to_string();
    }
    match app_name {
        "Alacritty" | "kitty" | "foot" | "wezterm" | "org.gnome.Terminal" | "org.gnome.Console" => "Terminal".to_string(),
        "OC" => "OpenCode".to_string(),
        other => other.to_string(),
    }
}

#[tauri::command]
fn get_daily_summary(state: State<'_, AppState>) -> Result<DailySummary, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T00:00:00", today);
        let end = format!("{}T23:59:59", today);

        let mut stmt = conn.prepare(
            "SELECT app_name, duration, category, productivity_score FROM activities WHERE start_time >= ?1 AND start_time <= ?2"
        ).map_err(|e| e.to_string())?;

        let rows: Vec<(String, i64, String, i64)> = stmt.query_map(params![start, end], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        let total_duration: i64 = rows.iter().map(|r| r.1).sum();
        let productive_duration: i64 = rows.iter().filter(|r| r.3 > 0).map(|r| r.1).sum();
        let productivity_score = if total_duration > 0 {
            productive_duration * 100 / total_duration
        } else { 0 };

        let mut app_map: std::collections::HashMap<String, (i64, String)> = std::collections::HashMap::new();
        for (app, dur, cat, _) in &rows {
            let normalized = normalize_display_name(app);
            let entry = app_map.entry(normalized).or_insert((0, cat.clone()));
            entry.0 += dur;
        }
        let mut app_usage: Vec<AppUsage> = app_map.iter()
            .map(|(k, (d, c))| AppUsage { app_name: k.clone(), duration: *d, category: c.clone() })
            .collect();
        app_usage.sort_by(|a, b| b.duration.cmp(&a.duration));

        let mut cat_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for (_, dur, cat, _) in &rows {
            *cat_map.entry(cat.clone()).or_insert(0) += dur;
        }
        let mut categories: Vec<CategoryBreakdown> = cat_map.iter()
            .map(|(k, d)| CategoryBreakdown {
                name: k.clone(),
                duration: *d,
                percentage: if total_duration > 0 { d * 100 / total_duration } else { 0 },
            })
            .collect();
        categories.sort_by(|a, b| b.duration.cmp(&a.duration));

        Ok(DailySummary { total_duration, productivity_score, app_usage, categories })
    } else {
        Err("Database connection not initialized".to_string())
    }
}

#[tauri::command]
fn get_productivity_by_week(state: State<'_, AppState>) -> Result<Vec<ProductivityDay>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now();
        let week_start = today - Duration::days(6);
        let start = week_start.format("%Y-%m-%dT00:00:00").to_string();

        let mut stmt = conn.prepare(
            "SELECT start_time, duration, productivity_score FROM activities WHERE start_time >= ?1"
        ).map_err(|e| e.to_string())?;

        let rows: Vec<(String, i64, i64)> = stmt.query_map(params![start], |row| {
            Ok((row.get::<_, String>(0)?, row.get(1)?, row.get(2)?))
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let days: Vec<ProductivityDay> = (0..7).map(|i| {
            let d = week_start + Duration::days(i);
            let day_str = d.format("%Y-%m-%d").to_string();
            let day_idx = d.format("%u").to_string().parse::<usize>().unwrap() - 1;
            let day_label = day_names[day_idx];

            let day_rows: Vec<_> = rows.iter().filter(|r| r.0.starts_with(&day_str)).collect();
            let productive: i64 = day_rows.iter().filter(|r| r.2 > 0).map(|r| r.1).sum();
            let total: i64 = day_rows.iter().map(|r| r.1).sum();
            let leisure: i64 = day_rows.iter().filter(|r| r.2 < 0).map(|r| r.1).sum();
            let neutral = total - productive - leisure;

            ProductivityDay {
                day: day_label.to_string(),
                productive_duration: productive,
                neutral_duration: neutral,
                leisure_duration: leisure,
            }
        }).collect();

        Ok(days)
    } else {
        Err("Database connection not initialized".to_string())
    }
}

#[tauri::command]
fn get_deep_work_sessions(state: State<'_, AppState>) -> Result<Vec<DeepWorkSession>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare(
            "SELECT app_name, title, start_time, end_time, duration, category FROM activities WHERE productivity_score > 0 AND duration >= 1800 ORDER BY start_time DESC LIMIT 10"
        ).map_err(|e| e.to_string())?;

        let sessions: Vec<DeepWorkSession> = stmt.query_map([], |row| {
            Ok(DeepWorkSession {
                app_name: row.get(0)?,
                title: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get(3)?,
                duration: row.get(4)?,
                category: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        Ok(sessions)
    } else {
        Err("Database connection not initialized".to_string())
    }
}

#[tauri::command]
fn get_blocked_apps(state: State<'_, AppState>) -> Result<Vec<BlockedApp>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT id, app_name, is_blocked, daily_limit_minutes, weekly_limit_minutes, limit_enabled FROM blocked_apps")
            .map_err(|e| e.to_string())?;
        let apps = stmt.query_map([], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
                daily_limit_minutes: row.get(3)?,
                weekly_limit_minutes: row.get(4)?,
                limit_enabled: row.get::<_, i64>(5)? == 1,
            })
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();
        Ok(apps)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn add_blocked_app(state: State<'_, AppState>, app_name: String) -> Result<BlockedApp, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let changed = conn.execute("INSERT OR IGNORE INTO blocked_apps (app_name, is_blocked) VALUES (?1, 1)", params![app_name])
            .map_err(|e| e.to_string())?;
        if changed == 0 {
            return Err("App already blocked".to_string());
        }
        let id = conn.last_insert_rowid();
        Ok(BlockedApp { id, app_name, is_blocked: true, daily_limit_minutes: 0, weekly_limit_minutes: 0, limit_enabled: false })
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn remove_blocked_app(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM blocked_apps WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn toggle_blocked_app(state: State<'_, AppState>, id: i64) -> Result<BlockedApp, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("UPDATE blocked_apps SET is_blocked = NOT is_blocked WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        let app = conn.query_row("SELECT id, app_name, is_blocked, daily_limit_minutes, weekly_limit_minutes, limit_enabled FROM blocked_apps WHERE id = ?1", params![id], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
                daily_limit_minutes: row.get(3)?,
                weekly_limit_minutes: row.get(4)?,
                limit_enabled: row.get::<_, i64>(5)? == 1,
            })
        }).map_err(|e| e.to_string())?;
        Ok(app)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn update_app_limits(
    state: State<'_, AppState>,
    id: i64,
    daily_limit_minutes: i64,
    weekly_limit_minutes: i64,
    limit_enabled: bool,
) -> Result<BlockedApp, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute(
            "UPDATE blocked_apps SET daily_limit_minutes = ?1, weekly_limit_minutes = ?2, limit_enabled = ?3 WHERE id = ?4",
            params![daily_limit_minutes, weekly_limit_minutes, limit_enabled as i64, id],
        ).map_err(|e| e.to_string())?;
        let app = conn.query_row(
            "SELECT id, app_name, is_blocked, daily_limit_minutes, weekly_limit_minutes, limit_enabled FROM blocked_apps WHERE id = ?1",
            params![id],
            |row| {
                Ok(BlockedApp {
                    id: row.get(0)?,
                    app_name: row.get(1)?,
                    is_blocked: row.get::<_, i64>(2)? == 1,
                    daily_limit_minutes: row.get(3)?,
                    weekly_limit_minutes: row.get(4)?,
                    limit_enabled: row.get::<_, i64>(5)? == 1,
                })
            },
        ).map_err(|e| e.to_string())?;
        Ok(app)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn get_app_daily_usage(state: State<'_, AppState>, app_name: String) -> Result<i64, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T00:00:00", today);
        let end = format!("{}T23:59:59", today);
        let total: i64 = conn.query_row(
            "SELECT COALESCE(SUM(duration), 0) FROM activities WHERE app_name = ?1 AND start_time >= ?2 AND start_time <= ?3",
            params![app_name, start, end],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        Ok(total)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn get_app_weekly_usage(state: State<'_, AppState>, app_name: String) -> Result<i64, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now();
        let week_start = today - chrono::Duration::days(today.format("%u").to_string().parse::<i64>().unwrap() - 1);
        let start = week_start.format("%Y-%m-%dT00:00:00").to_string();
        let total: i64 = conn.query_row(
            "SELECT COALESCE(SUM(duration), 0) FROM activities WHERE app_name = ?1 AND start_time >= ?2",
            params![app_name, start],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        Ok(total)
    } else {
        Err("Database not initialized".to_string())
    }
}

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
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    if let Some(max_id) = events.last().map(|e| e.id) {
        conn.execute("DELETE FROM events WHERE id <= ?1", params![max_id])
            .map_err(|e| e.to_string())?;
    }
    Ok(events)
}

pub fn is_app_blocked(conn: &Connection, app_name: &str) -> bool {
    // Check exact match first
    let exact = conn.query_row(
        "SELECT COUNT(*) FROM blocked_apps WHERE LOWER(app_name) = LOWER(?1) AND is_blocked = 1",
        params![app_name],
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0;
    if exact { return true; }

    // Check if any blocked app name appears within the active app name
    // (e.g. "YouTube" in "Video Title - YouTube")
    if let Ok(mut stmt) = conn.prepare("SELECT app_name FROM blocked_apps WHERE is_blocked = 1") {
        if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
            let lower = app_name.to_lowercase();
            for row in rows.flatten() {
                if lower.contains(&row.to_lowercase()) {
                    return true;
                }
            }
        }
    }
    false
}

pub struct AppLimitConfig {
    pub daily_limit_minutes: i64,
    pub weekly_limit_minutes: i64,
    pub limit_enabled: bool,
}

pub fn get_app_limit_config(conn: &Connection, app_name: &str) -> Option<AppLimitConfig> {
    conn.query_row(
        "SELECT daily_limit_minutes, weekly_limit_minutes, limit_enabled FROM blocked_apps WHERE LOWER(app_name) = LOWER(?1)",
        params![app_name],
        |row| {
            Ok(AppLimitConfig {
                daily_limit_minutes: row.get(0)?,
                weekly_limit_minutes: row.get(1)?,
                limit_enabled: row.get::<_, i64>(2)? == 1,
            })
        },
    ).ok()
}

pub fn get_app_usage_today(conn: &Connection, app_name: &str) -> i64 {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let start = format!("{}T00:00:00", today);
    let end = format!("{}T23:59:59", today);
    conn.query_row(
        "SELECT COALESCE(SUM(duration), 0) FROM activities WHERE app_name = ?1 AND start_time >= ?2 AND start_time <= ?3",
        params![app_name, start, end],
        |row| row.get(0),
    ).unwrap_or(0)
}

pub fn get_app_usage_this_week(conn: &Connection, app_name: &str) -> i64 {
    let today = Utc::now();
    let weekday = today.format("%u").to_string().parse::<i64>().unwrap_or(1);
    let week_start = today - chrono::Duration::days(weekday - 1);
    let start = week_start.format("%Y-%m-%dT00:00:00").to_string();
    conn.query_row(
        "SELECT COALESCE(SUM(duration), 0) FROM activities WHERE app_name = ?1 AND start_time >= ?2",
        params![app_name, start],
        |row| row.get(0),
    ).unwrap_or(0)
}

pub fn write_event(conn: &Connection, event_type: &str, payload: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO events (type, payload) VALUES (?1, ?2)",
        params![event_type, payload],
    )
}

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

fn seed_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM activities", [], |row| row.get(0))?;

    if count == 0 {
        println!("Database is empty. Seeding demo data for last week...");
        let now = Utc::now();
        let last_week = now - Duration::days(7);
        let day_start = last_week.with_hour(9).unwrap().with_minute(0).unwrap().with_second(0).unwrap();

        let activities = vec![
            ("VS Code", "Backend Refactoring", day_start, day_start + Duration::hours(2) + Duration::minutes(30), (2 * 60 + 30) * 60, "Coding", 1),
            ("Safari", "Stack Overflow", day_start + Duration::hours(2) + Duration::minutes(30), day_start + Duration::hours(3), 30 * 60, "Coding", 1),
            ("YouTube", "Lofi Beats", day_start + Duration::hours(3), day_start + Duration::hours(3) + Duration::minutes(45), 45 * 60, "Entertainment", -1),
            ("Slack", "Team Standup", day_start + Duration::hours(4), day_start + Duration::hours(4) + Duration::minutes(15), 15 * 60, "Communication", 0),
            ("VS Code", "Frontend Work", day_start + Duration::hours(4) + Duration::minutes(15), day_start + Duration::hours(7) + Duration::minutes(15), 3 * 60 * 60, "Coding", 1),
            ("Spotify", "Focus Playlist", day_start + Duration::hours(4) + Duration::minutes(15), day_start + Duration::hours(7) + Duration::minutes(15), 3 * 60 * 60, "Entertainment", -1),
        ];

        let mut stmt = conn.prepare(
            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
        )?;

        let count = activities.len();
        for (app_name, title, start_time, end_time, duration, category, score) in &activities {
            stmt.execute(params![app_name, title, start_time.to_rfc3339(), end_time.to_rfc3339(), duration, category, score])?;
        }
        println!("Demo data inserted ({} activities from last week).", count);
    } else {
        println!("Database already has {} activities.", count);
    }

    Ok(())
}

pub fn init_db(db_path: &std::path::Path) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrency
    conn.pragma_update(None, "journal_mode", "WAL")?;

    // Wait up to 5s instead of failing with "database is locked" when the GUI
    // and daemon access the DB concurrently.
    conn.busy_timeout(std::time::Duration::from_secs(5))?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS activities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT,
            title TEXT,
            start_time DATETIME,
            end_time DATETIME,
            duration INTEGER,
            category TEXT,
            productivity_score INTEGER
        );
    ", [])?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );
    ", [])?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS blocked_apps (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL UNIQUE,
            is_blocked INTEGER NOT NULL DEFAULT 1,
            daily_limit_minutes INTEGER NOT NULL DEFAULT 0,
            weekly_limit_minutes INTEGER NOT NULL DEFAULT 0,
            limit_enabled INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
    ", [])?;

    // One-time migration: clean up browser app names that still contain full titles
    migrate_browser_app_names(&conn)?;

    // One-time migration: normalize terminal-prompt app names
    migrate_terminal_app_names(&conn)?;

    // Migration: add limit columns to blocked_apps if missing
    let _ = conn.execute("ALTER TABLE blocked_apps ADD COLUMN daily_limit_minutes INTEGER NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE blocked_apps ADD COLUMN weekly_limit_minutes INTEGER NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE blocked_apps ADD COLUMN limit_enabled INTEGER NOT NULL DEFAULT 0", []);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
        [],
    )?;

    Ok(conn)
}

fn migrate_browser_app_names(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::tracker::BROWSER_SUFFIXES;

    let mut stmt = conn.prepare("SELECT id, app_name, title FROM activities")?;
    let rows: Vec<(i64, String, String)> = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?.filter_map(|r| r.ok()).collect();

    let mut updated = 0;
    for (id, app_name, title) in &rows {
        // Try to extract site name from title using browser suffix stripping
        let mut cleaned = title.clone();
        for suffix in BROWSER_SUFFIXES {
            cleaned = cleaned.replace(suffix, "");
        }
        let cleaned = cleaned.trim().to_string();
        if cleaned.is_empty() { continue; }

        // Get the site name (last segment after " — " or " - ", fallback " | ")
        let site = if let Some(pos) = cleaned.rfind(" — ") {
            cleaned[pos + " — ".len()..].trim().to_string()
        } else if let Some(pos) = cleaned.rfind(" - ") {
            cleaned[pos + " - ".len()..].trim().to_string()
        } else if let Some(pos) = cleaned.find(" | ") {
            cleaned[..pos].trim().to_string()
        } else {
            cleaned.clone()
        };

        // Fix terminal-prompt app_names (e.g. "user@host: ~/dir" → "Terminal")
        if site == *app_name && app_name.contains('@') && app_name.contains('~') {
            conn.execute(
                "UPDATE activities SET app_name = 'Terminal' WHERE id = ?1",
                params![id],
            )?;
            updated += 1;
            continue;
        }

        // Update if the extracted site name differs from current app_name
        if !site.is_empty() && site != *app_name {
            conn.execute(
                "UPDATE activities SET app_name = ?1 WHERE id = ?2",
                params![site, id],
            )?;
            updated += 1;
        }
    }
    if updated > 0 {
        println!("Migrated {} browser app names to site names.", updated);
    }
    Ok(())
}

fn migrate_terminal_app_names(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Normalize terminal prompt patterns: "user@host: ~/dir" → "Terminal"
    let changed = conn.execute(
        "UPDATE activities SET app_name = 'Terminal' WHERE app_name LIKE '%@%:%' AND (app_name LIKE '%~%' OR app_name LIKE '%/%')",
        [],
    )?;
    if changed > 0 {
        println!("Migrated {} terminal-prompt app names to 'Terminal'.", changed);
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub settings: std::collections::HashMap<String, String>,
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> Result<SettingsResponse, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT key, value FROM settings")
            .map_err(|e| e.to_string())?;
        let mut settings = std::collections::HashMap::new();
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|e| e.to_string())?;
        for row in rows.flatten() {
            settings.insert(row.0, row.1);
        }
        Ok(SettingsResponse { settings })
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        ).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn export_activities_csv(state: State<'_, AppState>) -> Result<String, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT app_name, title, start_time, end_time, duration, category, productivity_score FROM activities")
            .map_err(|e| e.to_string())?;
        let mut csv = String::from("app_name,title,start_time,end_time,duration,category,productivity_score\n");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
            ))
        }).map_err(|e| e.to_string())?;
        for row in rows.flatten() {
            csv.push_str(&format!("{},{},{},{},{},{},{}\n", row.0, row.1, row.2, row.3, row.4, row.5, row.6));
        }
        Ok(csv)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn export_activities_json(state: State<'_, AppState>) -> Result<String, String> {
    let activities = get_activities(state)?;
    serde_json::to_string_pretty(&activities).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_all_data(state: State<'_, AppState>) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM activities", []).map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM blocked_apps", []).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn reset_demo_data(state: State<'_, AppState>) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM activities", []).map_err(|e| e.to_string())?;
        seed_database(conn).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            conn: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_opener::init())
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

                    // Ensure the background daemon is running (do not kill on exit).
                    ensure_daemon_running();
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_activities,
            get_daily_summary,
            get_productivity_by_week,
            get_deep_work_sessions,
            get_blocked_apps,
            add_blocked_app,
            remove_blocked_app,
            toggle_blocked_app,
            update_app_limits,
            get_app_daily_usage,
            get_app_weekly_usage,
            get_settings,
            update_setting,
            export_activities_csv,
            export_activities_json,
            clear_all_data,
            reset_demo_data,
            poll_events,
            set_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "journal_mode", "WAL").unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS activities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT, title TEXT, start_time DATETIME,
            end_time DATETIME, duration INTEGER, category TEXT,
            productivity_score INTEGER
        )", []).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY, value TEXT
        )", []).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS blocked_apps (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app_name TEXT NOT NULL UNIQUE,
            is_blocked INTEGER NOT NULL DEFAULT 1,
            daily_limit_minutes INTEGER NOT NULL DEFAULT 0,
            weekly_limit_minutes INTEGER NOT NULL DEFAULT 0,
            limit_enabled INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )", []).unwrap();

        conn
    }

    fn insert_activity(conn: &Connection, app: &str, title: &str, duration: i64, category: &str, score: i64) {
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![app, title, &now, &now, duration, category, score],
        ).unwrap();
    }

    fn insert_activity_at(conn: &Connection, app: &str, title: &str, start: &str, end: &str, duration: i64, category: &str, score: i64) {
        conn.execute(
            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![app, title, start, end, duration, category, score],
        ).unwrap();
    }

    // --- is_app_blocked tests ---

    #[test]
    fn test_is_app_blocked_exact_match() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        assert!(is_app_blocked(&conn, "YouTube"));
    }

    #[test]
    fn test_is_app_blocked_case_insensitive() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('youtube', 1)", []).unwrap();
        assert!(is_app_blocked(&conn, "YouTube"));
        assert!(is_app_blocked(&conn, "YOUTUBE"));
    }

    #[test]
    fn test_is_app_blocked_substring_match() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        assert!(is_app_blocked(&conn, "YouTube - Video Title - Mozilla Firefox"));
    }

    #[test]
    fn test_is_app_blocked_not_blocked() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        assert!(!is_app_blocked(&conn, "GitHub"));
        assert!(!is_app_blocked(&conn, "VS Code"));
    }

    #[test]
    fn test_is_app_blocked_disabled() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 0)", []).unwrap();
        assert!(!is_app_blocked(&conn, "YouTube"));
    }

    #[test]
    fn test_is_app_blocked_empty_db() {
        let conn = setup_db();
        assert!(!is_app_blocked(&conn, "Anything"));
    }

    // --- Blocked apps CRUD ---

    #[test]
    fn test_add_blocked_app() {
        let conn = setup_db();
        let changed = conn.execute("INSERT OR IGNORE INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        assert_eq!(changed, 1);
        assert!(is_app_blocked(&conn, "YouTube"));
    }

    #[test]
    fn test_add_blocked_app_duplicate() {
        let conn = setup_db();
        conn.execute("INSERT OR IGNORE INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        let changed = conn.execute("INSERT OR IGNORE INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        assert_eq!(changed, 0);
    }

    #[test]
    fn test_remove_blocked_app() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        conn.execute("DELETE FROM blocked_apps WHERE app_name = 'YouTube'", []).unwrap();
        assert!(!is_app_blocked(&conn, "YouTube"));
    }

    #[test]
    fn test_toggle_blocked_app() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();
        conn.execute("UPDATE blocked_apps SET is_blocked = NOT is_blocked WHERE app_name = 'YouTube'", []).unwrap();
        assert!(!is_app_blocked(&conn, "YouTube"));
        conn.execute("UPDATE blocked_apps SET is_blocked = NOT is_blocked WHERE app_name = 'YouTube'", []).unwrap();
        assert!(is_app_blocked(&conn, "YouTube"));
    }

    // --- App limit config ---

    #[test]
    fn test_get_app_limit_config_exists() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked, daily_limit_minutes, weekly_limit_minutes, limit_enabled) VALUES ('YouTube', 1, 60, 300, 1)", []).unwrap();
        let config = get_app_limit_config(&conn, "YouTube").unwrap();
        assert!(config.limit_enabled);
        assert_eq!(config.daily_limit_minutes, 60);
        assert_eq!(config.weekly_limit_minutes, 300);
    }

    #[test]
    fn test_get_app_limit_config_not_found() {
        let conn = setup_db();
        assert!(get_app_limit_config(&conn, "YouTube").is_none());
    }

    #[test]
    fn test_get_app_limit_config_disabled() {
        let conn = setup_db();
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked, daily_limit_minutes, weekly_limit_minutes, limit_enabled) VALUES ('YouTube', 1, 60, 300, 0)", []).unwrap();
        let config = get_app_limit_config(&conn, "YouTube").unwrap();
        assert!(!config.limit_enabled);
    }

    // --- Daily/weekly usage ---

    #[test]
    fn test_get_app_usage_today() {
        let conn = setup_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T12:00:00", today);
        let end = format!("{}T12:00:30", today);
        insert_activity_at(&conn, "VS Code", "main.rs", &start, &end, 30, "Coding", 1);
        insert_activity_at(&conn, "VS Code", "lib.rs", &start, &end, 120, "Coding", 1);

        let total = get_app_usage_today(&conn, "VS Code");
        assert_eq!(total, 150);
    }

    #[test]
    fn test_get_app_usage_today_different_app() {
        let conn = setup_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T12:00:00", today);
        let end = format!("{}T12:00:30", today);
        insert_activity_at(&conn, "VS Code", "main.rs", &start, &end, 300, "Coding", 1);

        let total = get_app_usage_today(&conn, "Spotify");
        assert_eq!(total, 0);
    }

    #[test]
    fn test_get_app_usage_this_week() {
        let conn = setup_db();
        let now = Utc::now();
        let start = now.to_rfc3339();
        conn.execute(
            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params!["VS Code", "main.rs", &start, &start, 600, "Coding", 1],
        ).unwrap();

        let total = get_app_usage_this_week(&conn, "VS Code");
        assert_eq!(total, 600);
    }

    // --- Settings ---

    #[test]
    fn test_settings_crud() {
        let conn = setup_db();
        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES ('idle_timeout', '5')", []).unwrap();
        let val: String = conn.query_row("SELECT value FROM settings WHERE key = 'idle_timeout'", [], |row| row.get(0)).unwrap();
        assert_eq!(val, "5");

        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES ('idle_timeout', '10')", []).unwrap();
        let val: String = conn.query_row("SELECT value FROM settings WHERE key = 'idle_timeout'", [], |row| row.get(0)).unwrap();
        assert_eq!(val, "10");
    }

    #[test]
    fn test_settings_multiple() {
        let conn = setup_db();
        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES ('key1', 'val1')", []).unwrap();
        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES ('key2', 'val2')", []).unwrap();

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 2);
    }

    // --- Events / shared paths ---

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

    #[test]
    fn test_db_path_basename() {
        let p = db_path();
        assert_eq!(p.file_name().unwrap().to_str().unwrap(), "screentime.db");
    }

    // --- Activity queries ---

    #[test]
    fn test_get_daily_summary_empty() {
        let conn = setup_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T00:00:00", today);
        let end = format!("{}T23:59:59", today);

        let mut stmt = conn.prepare("SELECT app_name, duration, category, productivity_score FROM activities WHERE start_time >= ?1 AND start_time <= ?2").unwrap();
        let rows: Vec<(String, i64, String, i64)> = stmt.query_map(params![start, end], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).unwrap().filter_map(|r| r.ok()).collect();

        assert!(rows.is_empty());
    }

    #[test]
    fn test_get_daily_summary_with_data() {
        let conn = setup_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T12:00:00", today);
        let end = format!("{}T12:00:30", today);

        insert_activity_at(&conn, "VS Code", "main.rs", &start, &end, 300, "Coding", 1);
        insert_activity_at(&conn, "YouTube", "Cat Video", &start, &end, 120, "Entertainment", -1);

        let mut stmt = conn.prepare("SELECT app_name, duration, category, productivity_score FROM activities WHERE start_time >= ?1 AND start_time <= ?2").unwrap();
        let rows: Vec<(String, i64, String, i64)> = stmt.query_map(params![start, end], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).unwrap().filter_map(|r| r.ok()).collect();

        let total: i64 = rows.iter().map(|r| r.1).sum();
        assert_eq!(total, 420);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_productivity_score_calculation() {
        let conn = setup_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T12:00:00", today);
        let end = format!("{}T12:00:30", today);

        insert_activity_at(&conn, "VS Code", "main.rs", &start, &end, 300, "Coding", 1);
        insert_activity_at(&conn, "YouTube", "Cat Video", &start, &end, 100, "Entertainment", -1);

        let total_duration: i64 = 400;
        let productive_duration: i64 = 300;
        let score = productive_duration * 100 / total_duration;
        assert_eq!(score, 75);
    }

    // --- Clear data ---

    #[test]
    fn test_clear_all_data() {
        let conn = setup_db();
        insert_activity(&conn, "VS Code", "main.rs", 300, "Coding", 1);
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES ('YouTube', 1)", []).unwrap();

        conn.execute("DELETE FROM activities", []).unwrap();
        conn.execute("DELETE FROM blocked_apps", []).unwrap();

        let act_count: i64 = conn.query_row("SELECT COUNT(*) FROM activities", [], |row| row.get(0)).unwrap();
        let blocked_count: i64 = conn.query_row("SELECT COUNT(*) FROM blocked_apps", [], |row| row.get(0)).unwrap();
        assert_eq!(act_count, 0);
        assert_eq!(blocked_count, 0);
    }

    // --- Export ---

    #[test]
    fn test_csv_export() {
        let conn = setup_db();
        insert_activity(&conn, "VS Code", "main.rs", 300, "Coding", 1);

        let mut stmt = conn.prepare("SELECT app_name, title, start_time, end_time, duration, category, productivity_score FROM activities").unwrap();
        let mut csv = String::from("app_name,title,start_time,end_time,duration,category,productivity_score\n");
        let rows: Vec<(String, String, String, String, i64, String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?))
        }).unwrap().filter_map(|r| r.ok()).collect();
        for row in &rows {
            csv.push_str(&format!("{},{},{},{},{},{},{}\n", row.0, row.1, row.2, row.3, row.4, row.5, row.6));
        }

        assert!(csv.contains("app_name,title"));
        assert!(csv.contains("VS Code"));
    }

    #[test]
    fn test_json_export() {
        let conn = setup_db();
        insert_activity(&conn, "VS Code", "main.rs", 300, "Coding", 1);

        let mut stmt = conn.prepare("SELECT id, app_name, title, start_time, end_time, duration, category, productivity_score FROM activities").unwrap();
        let activities: Vec<Activity> = stmt.query_map([], |row| {
            Ok(Activity {
                id: row.get(0)?,
                app_name: row.get(1)?,
                title: row.get(2)?,
                start_time: row.get(3)?,
                end_time: row.get(4)?,
                duration: row.get(5)?,
                category: row.get(6)?,
                productivity_score: row.get(7)?,
            })
        }).unwrap().filter_map(|r| r.ok()).collect();

        let json = serde_json::to_string_pretty(&activities).unwrap();
        assert!(json.contains("VS Code"));
        assert!(json.contains("main.rs"));
    }
}
