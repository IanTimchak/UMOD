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

    /// Close/destroy all dictionary windows (those with our prefix).
    /// Your original code did this — just extracted here.
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

//
// ----------------------------------------------------------------
//   Helper: Register controller in AppMediator
// ----------------------------------------------------------------
//

// Nothing else needed — AppMediator calls:
//
// win.manage(OCROverlayController::new());
//
// And JS calls the commands normally.
