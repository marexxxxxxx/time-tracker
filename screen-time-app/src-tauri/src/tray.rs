use tauri::{AppHandle, Manager, menu::{Menu, MenuItem}, tray::TrayIconBuilder};
use tauri::image::Image;

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    // Load icons
    let icon_bytes = include_bytes!("../icons/32x32.png").to_vec();
    let icon = Image::from_bytes(&icon_bytes)?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .icon(icon)
        .build(app)?;

    Ok(())
}

// Function to update tray icon (called when idle state changes)
pub fn set_tray_active(app: &AppHandle, is_active: bool) {
    if let Some(tray) = app.tray_by_id("main") {
        let icon_bytes = if is_active {
            include_bytes!("../icons/32x32.png").to_vec()
        } else {
            include_bytes!("../icons/32x32_inactive.png").to_vec()
        };

        if let Ok(icon) = Image::from_bytes(&icon_bytes) {
            let _ = tray.set_icon(Some(icon));
        }
    }
}
