// presentation/window_presenter.rs
use tauri::{AppHandle, WebviewWindowBuilder, WebviewUrl, webview::Color};

pub struct WindowPresenter;

impl WindowPresenter {
    pub fn new() -> Self { Self }

    pub fn open_home_overlay(&self, app: &AppHandle) -> &'static str {
        let _ = WebviewWindowBuilder::new(
            app,
            "home-overlay",
            WebviewUrl::App("index.html".into()),
        )
        .background_color(Color::from([0x2f, 0x2f, 0x2f]))
        .inner_size(700.0, 600.0)
        .decorations(true)
        .shadow(true)
        .build();

        return "done";
    }

    pub fn open_dictionary_window(&self, app: &AppHandle, label: String) {
        let _ = WebviewWindowBuilder::new(
            app,
            label,
            WebviewUrl::App("index.html".into()),
        )
        .background_color(Color::from([0x2f, 0x2f, 0x2f]))
        .inner_size(700.0, 600.0)
        .decorations(false)
        .shadow(true)
        .build();
    }
}
