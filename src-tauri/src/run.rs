// src/app.rs or wherever your run() lives

use crate::app::AppMediator;
use crate::app::region_selection::controller::*;
use crate::state::AppState;
use crate::ui;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() -> tauri::Result<()> {
    let app = tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            rs_cursor,
            rs_mousedown,
            rs_mouseup,
            rs_key_enter,
            rs_key_escape,
            rs_get_state,
            rs_set_window_size,
            rs_do_capture,
            rs_ready,
        ]) // Interaction between Tauri and Rust
        .plugin(tauri_plugin_opener::init()) //shared state
        .setup(|app| {
            //init services
            // (for now just tray functionality)
            ui::tray::init_tray(app)?; // initialize tray from ui module

            // Register global hotkey Ctrl+Shift+R to initialize
            // region selection and reactive overlay
            use tauri_plugin_global_shortcut::{
                Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
            };
            let ctrl_shift_r =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyR);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        if shortcut == &ctrl_shift_r {
                            match event.state() {
                                ShortcutState::Pressed => {

                                    // println!("Ctrl-Shift-R Pressed!");
                                }
                                ShortcutState::Released => {
                                    // allow user to enter RS if they are not in it
                                    if !app.state::<AppState>().is_selecting_region() {
                                        app.state::<AppState>().enter_selecting_region();
                                        println!("Region Selection activated...");

                                        AppMediator::start_region_capture(app);
                                    }
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(ctrl_shift_r)?;

            Ok(())
        })
        .build(tauri::generate_context!())?;

    app.run(|_app_handle, _event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = _event {
            api.prevent_exit();
        }
    });

    Ok(())
}
