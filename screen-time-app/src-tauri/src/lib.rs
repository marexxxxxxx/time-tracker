use chrono::{Utc, Duration, Timelike};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, Row};
use std::str::FromStr;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_sql::{Builder as SqlBuilder, Migration, MigrationKind};
use tokio::sync::Mutex;

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

// AppState to hold the sqlx pool
struct AppState {
    pool: Mutex<Option<sqlx::SqlitePool>>,
}

#[tauri::command]
async fn get_activities(state: State<'_, AppState>) -> Result<Vec<Activity>, String> {
    let pool_guard = state.pool.lock().await;
    if let Some(pool) = pool_guard.as_ref() {
        let rows = sqlx::query("SELECT id, app_name, title, start_time, end_time, duration, category, productivity_score FROM activities")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut activities = Vec::new();
        for row in rows {
            activities.push(Activity {
                id: row.get(0),
                app_name: row.get(1),
                title: row.get(2),
                start_time: row.get(3),
                end_time: row.get(4),
                duration: row.get(5),
                category: row.get(6),
                productivity_score: row.get(7),
            });
        }
        Ok(activities)
    } else {
        Err("Database pool not initialized yet".to_string())
    }
}

async fn seed_database(_app_handle: &AppHandle, pool: &sqlx::SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Make sure tables exist even if migration plugin had a hiccup
    sqlx::query("
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
    ").execute(pool).await?;

    // Check if table is empty
    let count: i64 = sqlx::query("SELECT COUNT(*) FROM activities")
        .fetch_one(pool)
        .await?
        .get(0);

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

        for (app_name, title, start_time, end_time, duration, category, score) in activities {
            sqlx::query(
                "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(app_name)
            .bind(title)
            .bind(start_time.to_rfc3339())
            .bind(end_time.to_rfc3339())
            .bind(duration)
            .bind(category)
            .bind(score)
            .execute(pool)
            .await?;
        }
        println!("Mock data inserted.");
    } else {
        println!("Database already has {} activities.", count);
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "
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

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT
                );
            ",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .manage(AppState {
            pool: Mutex::new(None),
        })
        .plugin(
            SqlBuilder::default()
                .add_migrations("sqlite:screentime.db", migrations)
                .build()
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_handle_for_db = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                // Initialize db connection and pool
                if let Ok(app_dir) = app_handle_for_db.path().app_data_dir() {
                    if !app_dir.exists() {
                        let _ = std::fs::create_dir_all(&app_dir);
                    }
                    let db_path = app_dir.join("screentime.db");
                    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

                    if let Ok(connect_options) = SqliteConnectOptions::from_str(&db_url) {
                        let options = connect_options.create_if_missing(true);
                        if let Ok(pool) = SqlitePoolOptions::new().max_connections(5).connect_with(options).await {

                            // Seed
                            if let Err(e) = seed_database(&app_handle_for_db, &pool).await {
                                eprintln!("Failed to seed database: {}", e);
                            }

                            // Start Window Tracking
                            tracker::start_window_tracking(pool.clone());

                            // Save pool to state
                            let state: State<AppState> = app_handle_for_db.state();
                            *state.pool.lock().await = Some(pool);
                        }
                    }
                }
            });

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
