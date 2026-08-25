use std::process::Command;
use std::time::Duration;
use chrono::Utc;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use tokio::time::interval;

#[derive(Debug, Clone)]
struct WindowInfo {
    app_name: String,
    title: String,
}

pub fn start_window_tracking(conn: Arc<Mutex<Option<Connection>>>) {
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(3));
        let mut current_window: Option<WindowInfo> = None;
        let mut current_start = Utc::now();
        let mut current_id: Option<i64> = None;

        loop {
            interval.tick().await;

            // Don't track if we're idle (you could use the idle detection logic here,
            // but for simplicity, we'll just always track if there's an active window)
            // It's usually better to share idle state, but we'll stick to a simpler independent loop for now
            // or we could check X11 idle directly.

            if let Some(active) = get_active_window() {
                // Check if this app is blocked
                if let Ok(db_guard) = conn.lock() {
                    if let Some(db) = db_guard.as_ref() {
                        if crate::is_app_blocked(db, &active.app_name) {
                            current_window = None;
                            current_id = None;
                            continue;
                        }
                    }
                }

                // Determine category and score (very basic heuristic)
                let category = categorize_app(&active.app_name, &active.title);
                let score = if category == "Coding" || category == "Design" || category == "Writing" { 1 }
                            else if category == "Entertainment" { -1 } else { 0 };

                match &current_window {
                    Some(cw) if cw.app_name == active.app_name && cw.title == active.title => {
                        // Still the same window, update the current record's duration and end_time
                        if let Some(id) = current_id {
                            let now = Utc::now();
                            let duration = (now - current_start).num_seconds();

                            if let Ok(mut db_guard) = conn.lock() {
                                if let Some(db) = db_guard.as_mut() {
                                    let _ = db.execute(
                                        "UPDATE activities SET end_time = ?1, duration = ?2 WHERE id = ?3",
                                        params![now.to_rfc3339(), duration, id]
                                    );
                                }
                            }
                        }
                    }
                    _ => {
                        // Changed window or just started
                        let now = Utc::now();
                        current_start = now;
                        current_window = Some(active.clone());

                        // Insert new record
                        let mut inserted_id = None;

                        if let Ok(mut db_guard) = conn.lock() {
                            if let Some(db) = db_guard.as_mut() {
                                if let Ok(_) = db.execute(
                                    "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                                    params![
                                        &active.app_name,
                                        &active.title,
                                        now.to_rfc3339(),
                                        now.to_rfc3339(),
                                        0,
                                        &category,
                                        score
                                    ]
                                ) {
                                    inserted_id = Some(db.last_insert_rowid());
                                }
                            }
                        }

                        current_id = inserted_id;
                    }
                }
            } else {
                // No active window (e.g. locked screen)
                current_window = None;
                current_id = None;
            }
        }
    });
}

fn categorize_app(app_name: &str, title: &str) -> String {
    let app_lower = app_name.to_lowercase();
    let title_lower = title.to_lowercase();

    if app_lower.contains("code") || app_lower.contains("idea") || app_lower.contains("terminal") || app_lower.contains("alacritty") || app_lower.contains("kitty") {
        return "Coding".to_string();
    }
    if app_lower.contains("figma") || app_lower.contains("gimp") || app_lower.contains("inkscape") {
        return "Design".to_string();
    }
    if title_lower.contains("youtube") || title_lower.contains("netflix") || app_lower.contains("spotify") || app_lower.contains("vlc") || app_lower.contains("steam") {
        return "Entertainment".to_string();
    }
    if app_lower.contains("slack") || app_lower.contains("discord") || app_lower.contains("teams") {
        return "Communication".to_string();
    }

    // Default
    "Neutral".to_string()
}

fn get_active_window() -> Option<WindowInfo> {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    match session_type.as_str() {
        "wayland" => {
            // Try sway
            if let Ok(output) = Command::new("swaymsg").arg("-t").arg("get_tree").output() {
                if output.status.success() {
                    let json_str = String::from_utf8_lossy(&output.stdout);
                    if let Ok(tree) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(focused) = find_focused_node(&tree) {
                            let app_name = focused["app_id"].as_str()
                                .or(focused["window_properties"]["class"].as_str())
                                .unwrap_or("Unknown").to_string();
                            let title = focused["name"].as_str().unwrap_or("").to_string();
                            return Some(WindowInfo { app_name, title });
                        }
                    }
                }
            }

            // Try hyprland
            if let Ok(output) = Command::new("hyprctl").arg("activewindow").arg("-j").output() {
                if output.status.success() {
                    let json_str = String::from_utf8_lossy(&output.stdout);
                    if let Ok(window) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        let app_name = window["class"].as_str().unwrap_or("Unknown").to_string();
                        let title = window["title"].as_str().unwrap_or("").to_string();
                        if !app_name.is_empty() && app_name != "Unknown" {
                            return Some(WindowInfo { app_name, title });
                        }
                    }
                }
            }

            None
        },
        _ => {
            // Assume X11
            get_x11_active_window()
        }
    }
}

fn find_focused_node(node: &serde_json::Value) -> Option<serde_json::Value> {
    if node["focused"].as_bool() == Some(true) {
        return Some(node.clone());
    }

    if let Some(nodes) = node["nodes"].as_array() {
        for n in nodes {
            if let Some(focused) = find_focused_node(n) {
                return Some(focused);
            }
        }
    }

    if let Some(floating_nodes) = node["floating_nodes"].as_array() {
        for n in floating_nodes {
            if let Some(focused) = find_focused_node(n) {
                return Some(focused);
            }
        }
    }

    None
}

fn get_x11_active_window() -> Option<WindowInfo> {
    // We use xdotool which is widely available and easy to use
    if let Ok(output) = Command::new("xdotool").arg("getactivewindow").arg("getwindowname").output() {
        if output.status.success() {
            let title = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Getting class is a bit harder with just xdotool cleanly, we can use xprop
            // xprop -id $(xdotool getactivewindow) WM_CLASS
            if let Ok(id_output) = Command::new("xdotool").arg("getactivewindow").output() {
                let id = String::from_utf8_lossy(&id_output.stdout).trim().to_string();
                if let Ok(xprop_out) = Command::new("xprop").arg("-id").arg(&id).arg("WM_CLASS").output() {
                    let xprop_str = String::from_utf8_lossy(&xprop_out.stdout);
                    // Output looks like: WM_CLASS(STRING) = "navigator", "Firefox"
                    if let Some(class_part) = xprop_str.split('=').nth(1) {
                        let classes: Vec<&str> = class_part.split(',').collect();
                        if let Some(last_class) = classes.last() {
                            let app_name = last_class.trim().trim_matches('"').to_string();
                            return Some(WindowInfo { app_name, title });
                        }
                    }
                }
            }

            return Some(WindowInfo { app_name: "Unknown".to_string(), title });
        }
    }
    None
}
