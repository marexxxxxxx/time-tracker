use std::process::Command;
use std::time::Duration;
use chrono::{Utc, Timelike};
use sqlx::{SqlitePool, Row};
use tokio::time::interval;
use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use crate::quotes::get_random_quote;

#[derive(Debug, Clone)]
struct WindowInfo {
    app_name: String,
    title: String,
}

pub fn start_window_tracking(pool: SqlitePool, app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(3));
        let mut current_window: Option<WindowInfo> = None;
        let mut current_start = Utc::now();
        let mut current_id: Option<i64> = None;

        // Track the last time we sent a notification for a specific app
        let mut last_notifications: HashMap<String, chrono::DateTime<Utc>> = HashMap::new();

        loop {
            interval.tick().await;

            // Don't track if we're idle (you could use the idle detection logic here,
            // but for simplicity, we'll just always track if there's an active window)
            // It's usually better to share idle state, but we'll stick to a simpler independent loop for now
            // or we could check X11 idle directly.

            if let Some(active) = get_active_window() {
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

                            let _ = sqlx::query("UPDATE activities SET end_time = ?, duration = ? WHERE id = ?")
                                .bind(now.to_rfc3339())
                                .bind(duration)
                                .bind(id)
                                .execute(&pool)
                                .await;

                            // Limit checking logic
                            check_and_notify_limit(&pool, &app_handle, &active.app_name, &mut last_notifications).await;
                        }
                    }
                    _ => {
                        // Changed window or just started
                        let now = Utc::now();
                        current_start = now;
                        current_window = Some(active.clone());

                        // Insert new record
                        if let Ok(res) = sqlx::query(
                            "INSERT INTO activities (app_name, title, start_time, end_time, duration, category, productivity_score) VALUES (?, ?, ?, ?, ?, ?, ?)"
                        )
                        .bind(&active.app_name)
                        .bind(&active.title)
                        .bind(now.to_rfc3339())
                        .bind(now.to_rfc3339())
                        .bind(0)
                        .bind(&category)
                        .bind(score)
                        .execute(&pool)
                        .await {
                            current_id = Some(res.last_insert_rowid());
                        } else {
                            current_id = None;
                        }
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

async fn check_and_notify_limit(
    pool: &SqlitePool,
    app_handle: &AppHandle,
    app_name: &str,
    last_notifications: &mut HashMap<String, chrono::DateTime<Utc>>,
) {
    // 1. Check if the app has a limit configured
    let limit_result = sqlx::query("SELECT time_limit_minutes FROM app_limits WHERE app_name = ?")
        .bind(app_name)
        .fetch_optional(pool)
        .await;

    if let Ok(Some(row)) = limit_result {
        let limit_minutes: i64 = row.get(0);

        // 2. Calculate today's total usage for this app
        let now = Utc::now();
        // Simple start of day (UTC). For a real app, you might want local time start of day.
        let today_start = now.with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap();

        let usage_result = sqlx::query(
            "SELECT SUM(duration) FROM activities WHERE app_name = ? AND start_time >= ?"
        )
        .bind(app_name)
        .bind(today_start.to_rfc3339())
        .fetch_one(pool)
        .await;

        if let Ok(row) = usage_result {
            // SUM might return NULL if there are no records matching the condition, so we check using an Option first
            let total_duration_seconds_opt: Option<i64> = row.get(0);
            let total_duration_seconds = total_duration_seconds_opt.unwrap_or(0);

            let total_minutes = total_duration_seconds / 60;

            // 3. Check if usage exceeds limit
            if total_minutes >= limit_minutes {
                // 4. Check if we need to send a notification (every 5 minutes)
                let should_notify = match last_notifications.get(app_name) {
                    Some(last_time) => {
                        let diff = now.signed_duration_since(*last_time);
                        diff.num_minutes() >= 5
                    }
                    None => true, // First time exceeding
                };

                if should_notify {
                    // Fetch a random quote
                    let (quote_text, quote_author) = get_random_quote(pool).await.unwrap_or_else(|| {
                        ("Time to take a break!".to_string(), "System".to_string())
                    });

                    // Send notification
                    app_handle.notification()
                        .builder()
                        .title(format!("App Limit Reached: {}", app_name))
                        .body(format!("\"{}\" - {}", quote_text, quote_author))
                        .show()
                        .unwrap_or_else(|e| eprintln!("Failed to show notification: {}", e));

                    // Update last notification time
                    last_notifications.insert(app_name.to_string(), now);
                }
            }
        }
    }
}
