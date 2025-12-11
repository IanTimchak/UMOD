// src/ui/windows.rs

use tauri::{webview::Color, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::state::AppState;

const PREFIX: &str = "dictionary-window-";

/// Create a new dictionary window with a unique label.
/// This is basically the same logic you had in the tray click.
pub fn open_dictionary_window(app: &AppHandle) {
    // get next window id from shared state
    let next_id = app.state::<AppState>().next_window_id();
    let label = format!("{}{}", PREFIX, next_id);

    // build the window just like before
    let _ = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .background_color(Color::from([0x2f, 0x2f, 0x2f]))
        .inner_size(700.0, 600.0)
        .position(100.0, 80.0)
        .always_on_top(true)
        .decorations(false)
        .shadow(true)
        .build();
}

/// Close/destroy all dictionary windows (those with our prefix).
/// Your original code did this — just extracted here.
pub fn close_all_dictionary_windows(app: &AppHandle) {
    for (label, win) in app.webview_windows() {
        if label.starts_with(PREFIX) {
            // destroy to bypass CloseRequested flow
            let _ = win.destroy();
        }
    }
}
