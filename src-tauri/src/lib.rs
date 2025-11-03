//custom modules
mod state;

//importing
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, //v2
    Manager,
    WebviewUrl,
    WebviewWindowBuilder, //v2
    webview::Color,
};
use state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const PREFIX: &str = "dictionary-window-";
fn close_all_dictionary_windows(app: &tauri::AppHandle) {
    for (label, win) in app.webview_windows() {
        if label.starts_with(PREFIX) {
            // Use destroy() to actually tear down without CloseRequested flow
            let _ = win.destroy();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> { //must return Result in order to delay the app build.
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // (optional) tray menu
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let close_item = MenuItem::with_id(app, "close", "Close All", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&close_item, &quit_item])?;
            app.manage(AppState::new());

            TrayIconBuilder::new()
                .menu(&tray_menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Generate a window")
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle().clone();
                        let label = format!("dictionary-window-{}", app.state::<AppState>().next_window_id());
                        if true /*app.get_webview_window(label).is_none()*/ {
                            let _ = WebviewWindowBuilder::new(
                                &app,
                                label,
                                WebviewUrl::App("index.html".into()),
                            )
                            .background_color(Color::from([0x2f, 0x2f, 0x2f]))
                            .inner_size(700.0, 600.0)
                            .position(100.0, 80.0)
                            .always_on_top(true)
                            .decorations(false)
                            .always_on_top(true)
                            .shadow(true)
                            .build();
                        }
                    }
                })
                .on_menu_event(|app, ev| match ev.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "close" => {
                        close_all_dictionary_windows(app);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())?;

    app.run(|_app_handle, _event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = _event {
            // prevent app from closing
            api.prevent_exit();
        }
    });

    Ok(())
}
