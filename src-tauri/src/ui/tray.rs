// src/ui/tray.rs

use tauri::{
    App, Manager, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}
};

use crate::{app::AppMediator, state::AppState, ui::{self}};

/// Initialize the tray icon + menu and wire up actions
/// Call this from your Tauri `.setup(...)` in app.rs
pub fn init_tray(app: &App) -> tauri::Result<()> {
    // menu items
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let capture_item = MenuItem::with_id(app, "capture", "Capture Region", true, None::<&str>)?;
    let tray_menu = Menu::with_items(app, &[&capture_item, &quit_item])?;

    // build tray
    TrayIconBuilder::new()
        .menu(&tray_menu)
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Generate a window")
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|_tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {}
        })
        // menu items
        .on_menu_event(|app_handle, ev| match ev.id.as_ref() {
            "quit" => {
                // same as before
                std::process::exit(0);
            }
            "capture" => {
                // allow user to enter RS if they are not in it
                if !app_handle.state::<AppState>().is_selecting_region() {
                    ui::reactive_overlay::OCROverlayController::close_overlay(&app_handle);
                    app_handle.state::<AppState>().enter_selecting_region();
                    println!("Region Selection activated...");

                    AppMediator::start_region_capture(app_handle);
                }
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
