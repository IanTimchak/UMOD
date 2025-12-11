use crate::app;
use crate::state::AppState;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

mod rc_service;
pub mod region_selection; //winit - softbuffer screencapture

pub struct AppMediator {
    // later: pub note_service: NoteService,
    // pub ocr_service: OcrService,
    // pub rc_service: RcService,
    // pub dictionary_service: DictionaryService,
}

impl AppMediator {
    pub fn new() -> Self {
        Self {}
    }

    /// User asked to open dictionary UI (from tray, home overlay, etc.)
    pub fn open_dictionary_ui(&self) {
        // later: maybe preload dictionary, etc.
    }

    pub fn send_file_path(path: String) {
        println!("{}", path);
    }

    pub fn start_region_capture(app: &AppHandle) {
        let state = app.state::<AppState>();
        state.enter_selecting_region();

        let win = WebviewWindowBuilder::new(
            app,
            "region-overlay",
            WebviewUrl::App("overlays/region_selection.html".into()),
        )
        .transparent(true)
        .decorations(false)
        .background_color(tauri::webview::Color(0, 0, 0, 0)) //remove white flash
        .visible(false)
        .resizable(false)
        .always_on_top(false)
        .fullscreen(true)
        .build()
        .expect("failed to build window");

        win.manage(region_selection::RSController::new());
    }
}
