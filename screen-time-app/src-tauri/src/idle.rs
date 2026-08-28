use std::process::Command;
use std::time::Duration;
use tokio::time::interval;

// 5 minutes in seconds
const IDLE_THRESHOLD_SECONDS: u64 = 5 * 60;

pub fn start_idle_detection<F>(on_idle_change: F)
where
    F: Fn(bool, u64) + Send + 'static,
{
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(10));
        let mut was_idle = false;

        loop {
            interval.tick().await;

            let idle_time_secs = get_idle_time_seconds();

            if let Some((is_idle, secs)) = evaluate_idle_change(idle_time_secs, was_idle) {
                on_idle_change(is_idle, secs);

                if is_idle {
                    println!("User became idle after {} seconds.", secs);
                } else {
                    println!("User is back from idle.");
                }
                was_idle = is_idle;
            }
        }
    });
}

// Determines whether the idle state changed from `was_idle` given a fresh idle
// measurement. Returns `Some((new_state, idle_time_secs))` only when the state
// changes, and `None` when it stays the same.
fn evaluate_idle_change(idle_time_secs: u64, was_idle: bool) -> Option<(bool, u64)> {
    let is_idle = idle_time_secs >= IDLE_THRESHOLD_SECONDS;
    if is_idle != was_idle {
        Some((is_idle, idle_time_secs))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_change_from_active_to_idle() {
        // Below threshold (active) -> no change reported.
        assert_eq!(evaluate_idle_change(IDLE_THRESHOLD_SECONDS - 1, false), None);
        // Reaches threshold (idle) -> change reported.
        assert_eq!(
            evaluate_idle_change(IDLE_THRESHOLD_SECONDS, false),
            Some((true, IDLE_THRESHOLD_SECONDS))
        );
        // Remains idle -> no new change reported.
        assert_eq!(
            evaluate_idle_change(IDLE_THRESHOLD_SECONDS + 60, true),
            None
        );
    }

    #[test]
    fn reports_change_from_idle_back_to_active() {
        // Was idle, now active -> change reported.
        assert_eq!(evaluate_idle_change(0, true), Some((false, 0)));
        // Remains active -> no new change reported.
        assert_eq!(evaluate_idle_change(10, false), None);
    }
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
