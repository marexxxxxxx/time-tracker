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
