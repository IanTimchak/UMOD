use crate::infra::dictionary::DictionaryAdapter;
use crate::infra::dictionary::{LookupError, LookupResult};
use crate::infra::manga_ocr;
use crate::state::AppState;
use crate::ui::reactive_overlay::OCROverlayController;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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

    pub fn send_file_path(app: &AppHandle, path: String) {
        let image_file = std::fs::read(path).expect("Failed to read test image");
        let text = manga_ocr(&image_file).expect("Manga OCR failed");
        println!("Extracted Text: {}", text);

        //let _result = Self::coordinate_lookup(text.as_str());
        //println!("{:#?}", _result);

        Self::open_ocr_overlay(app, text.as_str());
    }

    pub fn coordinate_lookup(text: &str) -> Result<LookupResult, LookupError> {
        let adapter = DictionaryAdapter::new();
        let result = adapter.lookup(text)?;

        Ok(result)
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

    /// Opens the OCR overlay window and injects the text
    pub fn open_ocr_overlay(app: &AppHandle, text: &str) {
        let js_safe_text = text.replace('`', "\\`");

        let win = WebviewWindowBuilder::new(
            app,
            "reactive-overlay",
            WebviewUrl::App("overlays/reactive_overlay.html".into()),
        )
        .transparent(true)
        .decorations(false)
        .background_color(tauri::webview::Color(0, 0, 0, 75))
        .visible(true)
        .resizable(false)
        .fullscreen(false)
        .always_on_top(true)
        .inner_size(594.0, 153.0)
        .position(0.0, 0.0)
        .initialization_script(&format!(r#"window.__OCR_TEXT = `{}`;"#, js_safe_text))
        .build()
        .expect("Failed to create OCR overlay");

        win.manage(OCROverlayController::new());
    }

    pub fn open_dictionary_lookup_window(app: &AppHandle, lookup: &LookupResult) {
        // Serialize LookupResult -> JSON
        let json = serde_json::to_string(lookup).expect("Failed to serialize LookupResult");

        // Safer than backticks: embed as JSON literal
        let init = format!(r#"
            window.__LOOKUP_RESULT = {json};
        "#);

        let _win = WebviewWindowBuilder::new(
            app,
            "dictionary-lookup",
            WebviewUrl::App("dictionary/lookup.html".into()),
        )
        .transparent(false)
        .decorations(true)
        .always_on_top(true)
        .resizable(false)
        .fullscreen(false)
        .inner_size(420.0, 520.0)
        .title("UMOD Lookup")
        .initialization_script(&init)
        .build()
        .expect("Failed to create dictionary lookup window");
    }

    /// One-shot: lookup and open UI window
    pub fn lookup_and_open(app: &AppHandle, text: &str) -> Result<(), LookupError> {
        let result = Self::coordinate_lookup(text)?;
        Self::open_dictionary_lookup_window(app, &result);
        Ok(())
    }
}
