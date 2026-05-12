use std::process::Command;
use tauri::{AppHandle, Emitter};
use std::time::Duration;
use tokio::time::interval;

// 5 minutes in seconds
const IDLE_THRESHOLD_SECONDS: u64 = 5 * 60;

#[derive(Clone, serde::Serialize)]
struct IdleEvent {
    is_idle: bool,
    idle_time_seconds: u64,
}

pub fn start_idle_detection(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        let mut was_idle = false;

        loop {
            interval.tick().await;

            let idle_time_secs = get_idle_time_seconds();
            let is_idle = idle_time_secs >= IDLE_THRESHOLD_SECONDS;

            if is_idle != was_idle {
                let _ = app_handle.emit("idle-state-changed", IdleEvent {
                    is_idle,
                    idle_time_seconds: idle_time_secs,
                });
                was_idle = is_idle;

                if is_idle {
                    println!("User became idle after {} seconds.", idle_time_secs);
                } else {
                    println!("User is back from idle.");
                }
            }
        }
    });
}

fn get_idle_time_seconds() -> u64 {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    match session_type.as_str() {
        "wayland" => {
            // Try sway
            if let Ok(output) = Command::new("swaymsg").arg("-t").arg("get_idle").output() {
                if output.status.success() {
                    // Very simplistic check, real sway idle would use ext-idle-notify or swayidle
                    // Let's fallback to 0 for this mockup
                    return 0;
                }
            }

            // Try gnome dbus
            if let Ok(output) = Command::new("gdbus")
                .args(&["call", "-e", "-d", "org.gnome.Mutter.IdleMonitor", "-o", "/org/gnome/Mutter/IdleMonitor/Core", "-m", "org.gnome.Mutter.IdleMonitor.GetIdletime"])
                .output() {
                if output.status.success() {
                    let out = String::from_utf8_lossy(&output.stdout);
                    // Parse something like "(uint64 12345,)"
                    if let Some(ms_str) = out.split_whitespace().last() {
                        let ms_str = ms_str.trim_matches(|c| c == ',' || c == ')');
                        if let Ok(ms) = ms_str.parse::<u64>() {
                            return ms / 1000;
                        }
                    }
                }
            }

            return 0;
        },
        _ => {
            // Assume X11
            get_x11_idle_time()
        }
    }
}

// Extract X11 idle using x11 bindings for xss
fn get_x11_idle_time() -> u64 {
    #[cfg(target_os = "linux")]
    {
        use x11::xlib;
        use x11::xss;
        use std::ptr;

        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return 0;
            }

            let info_ptr = xss::XScreenSaverAllocInfo();

            if !info_ptr.is_null() {
                xss::XScreenSaverQueryInfo(display, xlib::XDefaultRootWindow(display), info_ptr);
                let idle = (*info_ptr).idle;
                xlib::XFree(info_ptr as *mut _);
                xlib::XCloseDisplay(display);

                // idle is in milliseconds
                return idle / 1000;
            }

            xlib::XCloseDisplay(display);
            0
        }
    }
    #[cfg(not(target_os = "linux"))]
    0
}
