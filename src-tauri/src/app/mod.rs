pub mod region_selection;

pub struct AppMediator {
    // later: pub note_service: NoteService,
    // pub ocr_service: OcrService,
    // pub rc_service: RcService,
    // pub dictionary_service: DictionaryService,
}

impl AppMediator {
    pub fn new() -> Self {
        Self { }
    }

    /// User asked to open dictionary UI (from tray, home overlay, etc.)
    pub fn open_dictionary_ui(&self) {
        // later: maybe preload dictionary, etc.
    }

    /// User initiated region capture
    pub fn start_region_capture(&self) {
        // later: call rc_service -> infrastructure region capture adapter
    }
}
