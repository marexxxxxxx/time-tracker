use chrono::{Utc, Duration, Timelike};
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

mod idle;
mod tracker;

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
                app_name: row.get(1)?,
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
            let entry = app_map.entry(app.clone()).or_insert((0, cat.clone()));
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
        let mut stmt = conn.prepare("SELECT id, app_name, is_blocked FROM blocked_apps")
            .map_err(|e| e.to_string())?;
        let apps = stmt.query_map([], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
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
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES (?1, 1)", params![app_name])
            .map_err(|e| e.to_string())?;
        let id = conn.last_insert_rowid();
        Ok(BlockedApp { id, app_name, is_blocked: true })
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
        let app = conn.query_row("SELECT id, app_name, is_blocked FROM blocked_apps WHERE id = ?1", params![id], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
            })
        }).map_err(|e| e.to_string())?;
        Ok(app)
    } else {
        Err("Database not initialized".to_string())
    }
}

pub fn is_app_blocked(conn: &Connection, app_name: &str) -> bool {
    conn.query_row(
        "SELECT COUNT(*) FROM blocked_apps WHERE app_name = ?1 AND is_blocked = 1",
        params![app_name],
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0
}

fn seed_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Check if table is empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM activities", [], |row| row.get(0))?;

    if count == 0 {
        println!("Database is empty. Seeding mock data...");
        let now = Utc::now();
        let today_start = now.with_hour(9).unwrap().with_minute(0).unwrap().with_second(0).unwrap();

        let activities = vec![
            (
                "VS Code",
                "Backend Refactoring",
                today_start,
                today_start + Duration::hours(2) + Duration::minutes(30),
                (2 * 60 + 30) * 60,
                "Coding",
                1,
            ),
            (
                "Safari",
                "Stack Overflow",
                today_start + Duration::hours(2) + Duration::minutes(30),
                today_start + Duration::hours(3),
                30 * 60,
                "Coding",
                1,
            ),
            (
                "YouTube",
                "Lofi Beats",
                today_start + Duration::hours(3),
                today_start + Duration::hours(3) + Duration::minutes(45),
                45 * 60,
                "Entertainment",
                -1,
            ),
        ];

        let mut stmt = conn.prepare(
            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
        )?;

        for (app_name, title, start_time, end_time, duration, category, score) in activities {
            stmt.execute(params![
                app_name,
                title,
                start_time.to_rfc3339(),
                end_time.to_rfc3339(),
                duration,
                category,
                score
            ])?;
        }
        println!("Mock data inserted.");
    } else {
        println!("Database already has {} activities.", count);
    }

    Ok(())
}

fn init_db(db_path: &std::path::Path) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrency
    conn.pragma_update(None, "journal_mode", "WAL")?;

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
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
    ", [])?;

    Ok(conn)
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

            // Initialize db connection
            if let Ok(app_dir) = app_handle.path().app_data_dir() {
                if !app_dir.exists() {
                    let _ = std::fs::create_dir_all(&app_dir);
                }
                let db_path = app_dir.join("screentime.db");

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

                        // Start Window Tracking by cloning the state's Arc
                        tracker::start_window_tracking(Arc::clone(&state.conn));
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize database: {}", e);
                    }
                }
            }

            // Start idle detection
            idle::start_idle_detection(app.handle().clone());

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
