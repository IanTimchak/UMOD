// src/app.rs or wherever your run() lives

use crate::state::AppState;
use crate::ui;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() -> tauri::Result<()> {
    let app = tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_opener::init()) //shared state
        .setup(|app| {
            //init services
            ui::tray::init_tray(app)?; // initialize tray from ui module
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
