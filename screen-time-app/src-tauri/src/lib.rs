use chrono::{Utc, Duration, Timelike};
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

mod idle;
mod tray;
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

            // Setup Tray
            if let Err(e) = tray::setup_tray(&app_handle) {
                eprintln!("Failed to setup tray: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_activities])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
