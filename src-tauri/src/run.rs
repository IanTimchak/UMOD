// src/app.rs or wherever your run() lives

use crate::ui;
use crate::state::AppState;

pub fn run() -> tauri::Result<()> {
    let app = tauri::Builder::default()
        .manage(AppState::new()) //shared state
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
