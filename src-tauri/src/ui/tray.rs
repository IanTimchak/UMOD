// src/ui/tray.rs

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App,
};

use crate::ui::windows;

/// Initialize the tray icon + menu and wire up actions
/// Call this from your Tauri `.setup(...)` in app.rs
pub fn init_tray(app: &App) -> tauri::Result<()> {
    // menu items
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let close_item = MenuItem::with_id(app, "close", "Close All", true, None::<&str>)?;
    let tray_menu = Menu::with_items(app, &[&close_item, &quit_item])?;

    // build tray
    TrayIconBuilder::new()
        .menu(&tray_menu)
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Generate a window")
        .show_menu_on_left_click(false)
        // left-click on tray → open a new dictionary window
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app_handle = tray.app_handle().clone();
                // delegate to the window module
                windows::open_dictionary_window(&app_handle);
            }
        })
        // menu items
        .on_menu_event(|app_handle, ev| match ev.id.as_ref() {
            "quit" => {
                // same as before
                std::process::exit(0);
            }
            "close" => {
                windows::close_all_dictionary_windows(app_handle);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
