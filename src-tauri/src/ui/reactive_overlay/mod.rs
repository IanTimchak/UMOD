use crate::app::AppMediator;
use crate::state::AppState;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};
#[derive(Default)]
pub struct OCROverlayState {
    pub text: String,
}

pub struct OCROverlayController {
    pub data: Arc<Mutex<OCROverlayState>>,
}

impl OCROverlayController {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(OCROverlayState::default())),
        }
    }

    pub fn close_overlay(app: &AppHandle) {
        for (label, win) in app.webview_windows() {
            if label.starts_with("reactive-overlay") {
                // destroy to bypass CloseRequested flow
                let _ = win.destroy();
            }
        }
    }
}

//
// ----------------------------------------------------------------
//   Tauri Commands Exposed to JS
// ----------------------------------------------------------------
//

#[tauri::command]
pub fn ocr_show(app: AppHandle, ctrl: State<'_, OCROverlayController>, text: String) {
    {
        let mut d = ctrl.data.lock().unwrap();
        d.text = text.clone();
    }

    // reopen or create window if closed
    if let Some(win) = app.webview_windows().get("ocr-overlay") {
        // Reinject updated text
        let js = format!(r#"window.__OCR_TEXT = `{}`;"#, text.replace('`', "\\`"));
        let _ = win.eval(&js);

        win.show().ok();
        win.set_focus().ok();
        return;
    }

    // Otherwise, AppMediator should have created this.
    println!("WARNING: ocr_show called before window created");
}

#[tauri::command]
pub fn ocr_close(app: AppHandle) {
    if let Some(win) = app.webview_windows().get("ocr-overlay") {
        let _ = win.close();
    }
}

#[tauri::command]
pub fn ocr_get_text(ctrl: State<'_, OCROverlayController>) -> String {
    let d = ctrl.data.lock().unwrap();
    d.text.clone()
}

#[tauri::command]
pub fn lookup_selected_text(app: tauri::AppHandle, text: String) {
    let state = app.state::<AppState>();

    // Decide what to do (and update state) under lock,
    // but do NOT close windows while holding the lock.
    let should_close_existing = {
        let mut current = state.current_lookup.lock().unwrap();

        // Same lookup -> do nothing (or focus existing window)
        if current.as_deref() == Some(text.as_str()) {
            // Optional: just focus existing window
            if let Some(win) = app.get_webview_window("dictionary-lookup") {
                let _ = win.set_focus();
                let _ = win.show();
            }
            return;
        }

        // New lookup -> set current now (prevents duplicate lookups immediately)
        *current = Some(text.clone());
        true
    };

    if should_close_existing {
        if let Some(win) = app.get_webview_window("dictionary-lookup") {
            let _ = win.close();
        }
    }

    // fire-and-forget
    std::thread::spawn(move || {
        let _ = AppMediator::lookup_and_open(&app, &text);
    });
}
