// src-tauri/src/state.rs
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AppState {
    next_id: AtomicUsize,
}
impl AppState {
    pub fn new() -> Self { Self { next_id: AtomicUsize::new(0) } }
    pub fn next_window_id(&self) -> usize {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}