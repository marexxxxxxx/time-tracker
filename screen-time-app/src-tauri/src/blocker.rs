use std::process::Command;
use rusqlite::Connection;
use crate::is_app_blocked;

const BROWSER_CLASSES: &[&str] = &["chromium", "firefox", "google-chrome", "brave-browser", "vivaldi", "opera", "microsoft-edge"];

pub fn is_browser(app_name: &str) -> bool {
    let lower = app_name.to_lowercase();
    BROWSER_CLASSES.iter().any(|b| lower.contains(b))
}

pub fn enforce_blocked_apps(conn: &Connection, app_names: &[String]) {
    for app_name in app_names {
        if !is_app_blocked(conn, app_name) {
            continue;
        }
        add_wm_rule(app_name);
    }
}

pub fn remove_blocked_rules(_conn: &Connection, app_names: &[String]) {
    for app_name in app_names {
        remove_wm_rule(app_name);
    }
}

pub fn clear_all_blocked_rules(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT app_name FROM blocked_apps WHERE is_blocked = 1")
        .unwrap();
    let apps: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    for app in apps {
        remove_wm_rule(&app);
    }
}

fn add_wm_rule(app_name: &str) {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    match session_type.as_str() {
        "wayland" => add_wayland_rule(app_name),
        _ => add_x11_rule(app_name),
    }
}

fn remove_wm_rule(app_name: &str) {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    match session_type.as_str() {
        "wayland" => remove_wayland_rule(app_name),
        _ => remove_x11_rule(app_name),
    }
}

fn add_wayland_rule(app_name: &str) {
    // Hyprland: close matching windows and prevent new ones
    if Command::new("hyprctl").arg("clients").output().is_ok() {
        let _ = Command::new("hyprctl")
            .args(&[
                "dispatch",
                "killactive",
                &format!("class:{}", app_name),
            ])
            .output();
        return;
    }

    // Sway: kill matching windows
    if let Ok(output) = Command::new("swaymsg")
        .arg("-t")
        .arg("get_tree")
        .output()
    {
        if output.status.success() {
            let close_cmd = format!("[app_id=\"{}\"] kill", app_name);
            let _ = Command::new("swaymsg").arg(&close_cmd).output();
        }
    }
}

fn remove_wayland_rule(app_name: &str) {
    // Hyprland: remove any matching rule by killing remaining instances
    if Command::new("hyprctl").arg("clients").output().is_ok() {
        let _ = Command::new("hyprctl")
            .args(&[
                "dispatch",
                "killactive",
                &format!("class:{}", app_name),
            ])
            .output();
    }

    // Sway: no persistent rules to remove beyond killing active windows
    if let Ok(output) = Command::new("swaymsg")
        .arg("-t")
        .arg("get_tree")
        .output()
    {
        if output.status.success() {
            let close_cmd = format!("[app_id=\"{}\"] kill", app_name);
            let _ = Command::new("swaymsg").arg(&close_cmd).output();
        }
    }
}

fn add_x11_rule(app_name: &str) {
    // xdotool: find and close windows matching the app name
    if let Ok(output) = Command::new("xdotool")
        .arg("search")
        .arg("--name")
        .arg(app_name)
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

fn remove_x11_rule(app_name: &str) {
    // X11 has no persistent blocking rules; close matching windows
    add_x11_rule(app_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_browser_chromium() {
        assert!(is_browser("chromium"));
        assert!(is_browser("Chromium"));
    }

    #[test]
    fn test_is_browser_firefox() {
        assert!(is_browser("firefox"));
        assert!(is_browser("Firefox"));
    }

    #[test]
    fn test_is_browser_chrome() {
        assert!(is_browser("google-chrome"));
    }

    #[test]
    fn test_is_browser_edge() {
        assert!(is_browser("microsoft-edge"));
    }

    #[test]
    fn test_is_browser_brave() {
        assert!(is_browser("brave-browser"));
    }

    #[test]
    fn test_is_not_browser() {
        assert!(!is_browser("Alacritty"));
        assert!(!is_browser("VS Code"));
        assert!(!is_browser("Slack"));
        assert!(!is_browser("Nautilus"));
        assert!(!is_browser("Unknown"));
    }
}
