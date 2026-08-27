use std::process::Command;
use std::time::Duration;
use chrono::Utc;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, AppHandle};
use tokio::time::interval;

const BLOCKED_TITLE_MARKER: &str = "Blocked — ScreenTime";
const WARNING_THRESHOLD_SECS: i64 = 5 * 60; // Warn 5 minutes before limit

#[derive(Debug, Clone)]
struct WindowInfo {
    app_name: String,
    original_class: String,
    title: String,
}

pub fn start_window_tracking(conn: Arc<Mutex<Option<Connection>>>, app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(3));
        let mut current_window: Option<WindowInfo> = None;
        let mut current_start = Utc::now();
        let mut current_id: Option<i64> = None;
        let mut warned_apps: std::collections::HashSet<String> = std::collections::HashSet::new();

        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

        loop {
            interval.tick().await;

            if let Some(active) = get_active_window() {
                // Check if this app is blocked
                if let Ok(db_guard) = conn.lock() {
                    if let Some(db) = db_guard.as_ref() {
                        if crate::is_app_blocked(db, &active.app_name) {
                            // Don't redirect if already on blocked page
                            if !active.title.contains(BLOCKED_TITLE_MARKER) {
                                let active_clone = active.clone();
                                let st = session_type.clone();
                                tokio::task::spawn_blocking(move || {
                                    handle_blocked_app(&active_clone, &st);
                                });
                            }
                            current_window = None;
                            current_id = None;
                            continue;
                        }
                    }
                }

                // Check time limits before recording
                if let Ok(db_guard) = conn.lock() {
                    if let Some(db) = db_guard.as_ref() {
                        if let Some(config) = crate::get_app_limit_config(db, &active.app_name) {
                            if config.limit_enabled {
                                let daily_secs = crate::get_app_usage_today(db, &active.app_name);
                                let daily_limit_secs = config.daily_limit_minutes * 60;
                                let weekly_secs = crate::get_app_usage_this_week(db, &active.app_name);
                                let weekly_limit_secs = config.weekly_limit_minutes * 60;

                                let daily_over = config.daily_limit_minutes > 0 && daily_secs >= daily_limit_secs;
                                let weekly_over = config.weekly_limit_minutes > 0 && weekly_secs >= weekly_limit_secs;

                                if daily_over || weekly_over {
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

                                // Warning threshold check (5 min before limit)
                                let daily_warn = config.daily_limit_minutes > 0
                                    && daily_secs >= daily_limit_secs - WARNING_THRESHOLD_SECS
                                    && daily_secs < daily_limit_secs;
                                let weekly_warn = config.weekly_limit_minutes > 0
                                    && weekly_secs >= weekly_limit_secs - WARNING_THRESHOLD_SECS
                                    && weekly_secs < weekly_limit_secs;

                                if (daily_warn || weekly_warn) && !warned_apps.contains(&active.app_name) {
                                    warned_apps.insert(active.app_name.clone());
                                    let remaining = if daily_warn {
                                        (daily_limit_secs - daily_secs) / 60
                                    } else {
                                        (weekly_limit_secs - weekly_secs) / 60
                                    };
                                    let limit_type = if daily_warn { "daily" } else { "weekly" };
                                    let _ = app_handle.emit("limit-warning", serde_json::json!({
                                        "app_name": active.app_name,
                                        "limit_type": limit_type,
                                        "remaining_minutes": remaining,
                                    }));
                                }
                            }
                        }
                    }
                }

                // Determine category and score
                let category = categorize_app(&active.app_name, &active.title);
                let score = if category == "Coding" || category == "Design" || category == "Writing" { 1 }
                            else if category == "Entertainment" { -1 } else { 0 };

                match &current_window {
                    Some(cw) if cw.app_name == active.app_name && cw.title == active.title => {
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
                        let now = Utc::now();

                        // Finalize the old activity's duration before switching
                        if let Some(id) = current_id {
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

                        current_start = now;
                        current_window = Some(active.clone());

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
                current_window = None;
                current_id = None;
            }
        }
    });
}

fn handle_blocked_app(active: &WindowInfo, session_type: &str) {
    let is_browser = crate::blocker::is_browser(&active.original_class);

    if is_browser {
        let user = std::env::var("USER").unwrap_or_default();
        let html_path = format!(
            "/home/{}/.local/share/marexxxxxxx.screen-time-app/blocked.html",
            user
        );
        if !std::path::Path::new(&html_path).exists() {
            return;
        }
        let encoded_name = url_encode(&active.app_name);
        let url = format!("file://{}?app={}", html_path, encoded_name);

        // Focus the browser window first
        let _ = Command::new("hyprctl")
            .args(&["dispatch", "focuswindow", &format!("class:{}", active.original_class)])
            .output();
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Copy URL to clipboard
        let _ = Command::new("wl-copy").arg(&url).output();
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Ctrl+L → focus address bar, Ctrl+V → paste URL, Enter → navigate
        let _ = Command::new("wtype").args(&["-M", "ctrl", "-k", "l"]).output();
        std::thread::sleep(std::time::Duration::from_millis(150));
        let _ = Command::new("wtype").args(&["-M", "ctrl", "-k", "v"]).output();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = Command::new("wtype").args(&["-k", "Return"]).output();
    } else {
        // Non-browser: close the window
        match session_type {
            "wayland" => {
                let _ = Command::new("hyprctl")
                    .args(&["dispatch", "killactive", &format!("class:{}", active.original_class)])
                    .output();
            }
            _ => {
                if let Ok(output) = Command::new("xdotool")
                    .arg("search").arg("--name").arg(&active.app_name)
                    .output()
                {
                    if output.status.success() {
                        let ids = String::from_utf8_lossy(&output.stdout);
                        for id in ids.lines() {
                            let id = id.trim();
                            if !id.is_empty() {
                                let _ = Command::new("xdotool").args(&["windowclose", id]).output();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn url_encode(s: &str) -> String {
    let mut encoded = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push_str("%20"),
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }
    encoded
}

const BROWSER_CLASSES: &[&str] = &["chromium", "firefox", "google-chrome", "brave-browser", "vivaldi", "opera", "microsoft-edge"];

pub const BROWSER_SUFFIXES: &[&str] = &[
    " — Mozilla Firefox", " - Mozilla Firefox",
    " — Chromium", " - Chromium",
    " — Google Chrome", " - Google Chrome",
    " — Brave", " - Brave",
    " — Vivaldi", " - Vivaldi",
    " — Opera", " - Opera",
    " — Microsoft Edge", " - Microsoft Edge",
];

fn extract_browser_site(app_name: &str, title: &str) -> Option<String> {
    let lower = app_name.to_lowercase();
    if !BROWSER_CLASSES.iter().any(|b| lower.contains(b)) {
        return None;
    }
    let mut cleaned = title.to_string();
    for suffix in BROWSER_SUFFIXES {
        cleaned = cleaned.replace(suffix, "");
    }
    let cleaned = cleaned.trim().to_string();
    if cleaned.is_empty() { return None; }
    // Extract just the site name: last segment after " — " or " - "
    if let Some(pos) = cleaned.rfind(" — ") {
        let site = cleaned[pos + " — ".len()..].trim().to_string();
        if !site.is_empty() { return Some(site); }
    }
    if let Some(pos) = cleaned.rfind(" - ") {
        let site = cleaned[pos + " - ".len()..].trim().to_string();
        if !site.is_empty() { return Some(site); }
    }
    // Fallback: " | " separator — site name is before the pipe
    if let Some(pos) = cleaned.find(" | ") {
        let site = cleaned[..pos].trim().to_string();
        if !site.is_empty() { return Some(site); }
    }
    Some(cleaned)
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
    if app_lower.contains("youtube") || app_lower.contains("netflix") || app_lower.contains("twitch")
        || app_lower.contains("spotify") || app_lower.contains("vlc") || app_lower.contains("steam")
        || title_lower.contains("youtube") || title_lower.contains("netflix") || title_lower.contains("twitch") {
        return "Entertainment".to_string();
    }
    if app_lower.contains("slack") || app_lower.contains("discord") || app_lower.contains("teams") {
        return "Communication".to_string();
    }
    if BROWSER_CLASSES.iter().any(|b| app_lower.contains(b)) {
        return "Neutral".to_string();
    }

    "Neutral".to_string()
}

fn get_active_window() -> Option<WindowInfo> {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    let mut info = match session_type.as_str() {
        "wayland" => get_wayland_active_window(),
        _ => get_x11_active_window(),
    }?;

    // For browsers, use the website name instead of "chromium"/"firefox" etc.
    if let Some(site) = extract_browser_site(&info.app_name, &info.title) {
        info.app_name = site;
    }

    Some(info)
}

fn get_wayland_active_window() -> Option<WindowInfo> {
    if let Ok(output) = Command::new("swaymsg").arg("-t").arg("get_tree").output() {
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(tree) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(focused) = find_focused_node(&tree) {
                    let app_name = focused["app_id"].as_str()
                        .or(focused["window_properties"]["class"].as_str())
                        .unwrap_or("Unknown").to_string();
                    let title = focused["name"].as_str().unwrap_or("").to_string();
                    return Some(WindowInfo { original_class: app_name.clone(), app_name, title });
                }
            }
        }
    }

    if let Ok(output) = Command::new("hyprctl").arg("activewindow").arg("-j").output() {
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(window) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let app_name = window["class"].as_str().unwrap_or("Unknown").to_string();
                let title = window["title"].as_str().unwrap_or("").to_string();
                if !app_name.is_empty() && app_name != "Unknown" {
                    return Some(WindowInfo { original_class: app_name.clone(), app_name, title });
                }
            }
        }
    }

    None
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
    if let Ok(output) = Command::new("xdotool").arg("getactivewindow").arg("getwindowname").output() {
        if output.status.success() {
            let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(id_output) = Command::new("xdotool").arg("getactivewindow").output() {
                let id = String::from_utf8_lossy(&id_output.stdout).trim().to_string();
                if let Ok(xprop_out) = Command::new("xprop").arg("-id").arg(&id).arg("WM_CLASS").output() {
                    let xprop_str = String::from_utf8_lossy(&xprop_out.stdout);
                    if let Some(class_part) = xprop_str.split('=').nth(1) {
                        let classes: Vec<&str> = class_part.split(',').collect();
                        if let Some(last_class) = classes.last() {
                            let app_name = last_class.trim().trim_matches('"').to_string();
                            return Some(WindowInfo { original_class: app_name.clone(), app_name, title });
                        }
                    }
                }
            }
            return Some(WindowInfo { app_name: "Unknown".to_string(), original_class: "Unknown".to_string(), title });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_app_coding() {
        assert_eq!(categorize_app("Visual Studio Code", "main.rs"), "Coding");
        assert_eq!(categorize_app("Alacritty", "user@host: ~/project"), "Coding");
        assert_eq!(categorize_app("IntelliJ IDEA", "App.kt"), "Coding");
        assert_eq!(categorize_app("kitty", "nvim file.txt"), "Coding");
        assert_eq!(categorize_app("Terminal", "bash"), "Coding");
    }

    #[test]
    fn test_categorize_app_design() {
        assert_eq!(categorize_app("Figma", "My Design File"), "Design");
        assert_eq!(categorize_app("GIMP", "photo.xcf"), "Design");
        assert_eq!(categorize_app("Inkscape", "logo.svg"), "Design");
    }

    #[test]
    fn test_categorize_app_entertainment() {
        assert_eq!(categorize_app("YouTube", "Cat Videos"), "Entertainment");
        assert_eq!(categorize_app("Spotify", "Focus Playlist"), "Entertainment");
        assert_eq!(categorize_app("Netflix", "The Office"), "Entertainment");
        assert_eq!(categorize_app("VLC", "movie.mkv"), "Entertainment");
        assert_eq!(categorize_app("Steam", "Half-Life 3"), "Entertainment");
        assert_eq!(categorize_app("firefox", "Twitch - Mozilla Firefox"), "Entertainment");
        assert_eq!(categorize_app("chromium", "YouTube - Chromium"), "Entertainment");
    }

    #[test]
    fn test_categorize_app_communication() {
        assert_eq!(categorize_app("Slack", "team-channel"), "Communication");
        assert_eq!(categorize_app("Discord", "voice-chat"), "Communication");
        assert_eq!(categorize_app("Microsoft Teams", "Meeting"), "Communication");
    }

    #[test]
    fn test_categorize_app_neutral() {
        assert_eq!(categorize_app("chromium", "GitHub"), "Neutral");
        assert_eq!(categorize_app("firefox", "Stack Overflow"), "Neutral");
        assert_eq!(categorize_app("Nautilus", "Home"), "Neutral");
        assert_eq!(categorize_app("Unknown App", "Some Title"), "Neutral");
    }

    #[test]
    fn test_extract_browser_site_chromium() {
        let result = extract_browser_site("chromium", "GitHub - Chromium");
        assert_eq!(result, Some("GitHub".to_string()));
    }

    #[test]
    fn test_extract_browser_site_firefox_with_em_dash() {
        let result = extract_browser_site("firefox", "Stack Overflow — Mozilla Firefox");
        assert_eq!(result, Some("Stack Overflow".to_string()));
    }

    #[test]
    fn test_extract_browser_site_with_pipe() {
        let result = extract_browser_site("google-chrome", "My Page | Google Docs — Google Chrome");
        assert_eq!(result, Some("My Page".to_string()));
    }

    #[test]
    fn test_extract_browser_site_not_browser() {
        let result = extract_browser_site("Alacritty", "user@host: ~/dir");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_browser_site_empty_after_strip() {
        let result = extract_browser_site("chromium", " — Chromium");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_browser_site_fallback_cleaned() {
        let result = extract_browser_site("chromium", "Some Title Here");
        assert_eq!(result, Some("Some Title Here".to_string()));
    }

    #[test]
    fn test_url_encode_simple() {
        assert_eq!(url_encode("hello"), "hello");
        assert_eq!(url_encode("hello world"), "hello%20world");
    }

    #[test]
    fn test_url_encode_special_chars() {
        assert_eq!(url_encode("a&b=c"), "a%26b%3Dc");
        assert_eq!(url_encode("100%"), "100%25");
        assert_eq!(url_encode("path/to/file"), "path%2Fto%2Ffile");
    }

    #[test]
    fn test_url_encode_empty() {
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn test_url_encode_safe_chars() {
        assert_eq!(url_encode("ABCxyz012-_.~"), "ABCxyz012-_.~");
    }

    #[test]
    fn test_find_focused_node_found() {
        let tree = serde_json::json!({
            "nodes": [
                { "focused": false, "name": "Window 1" },
                { "focused": true, "name": "Focused Window", "app_id": "firefox" }
            ]
        });
        let result = find_focused_node(&tree);
        assert!(result.is_some());
        assert_eq!(result.unwrap()["name"].as_str().unwrap(), "Focused Window");
    }

    #[test]
    fn test_find_focused_node_nested() {
        let tree = serde_json::json!({
            "nodes": [
                { "focused": false, "nodes": [
                    { "focused": true, "name": "Deep Window" }
                ]}
            ]
        });
        let result = find_focused_node(&tree);
        assert!(result.is_some());
        assert_eq!(result.unwrap()["name"].as_str().unwrap(), "Deep Window");
    }

    #[test]
    fn test_find_focused_node_floating() {
        let tree = serde_json::json!({
            "nodes": [],
            "floating_nodes": [
                { "focused": true, "name": "Floating Window" }
            ]
        });
        let result = find_focused_node(&tree);
        assert!(result.is_some());
        assert_eq!(result.unwrap()["name"].as_str().unwrap(), "Floating Window");
    }

    #[test]
    fn test_find_focused_node_none_focused() {
        let tree = serde_json::json!({
            "nodes": [
                { "focused": false, "name": "Window 1" }
            ]
        });
        assert!(find_focused_node(&tree).is_none());
    }
}
