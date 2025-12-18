// src-tauri/src/state.rs
use std::sync::{Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};

pub struct AppState {
    next_id: AtomicUsize,
    is_selecting_region: AtomicBool,
    pub current_lookup: Mutex<Option<String>>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            is_selecting_region: AtomicBool::new(false),
            current_lookup: Mutex::new(None),
        }
    }
    pub fn next_window_id(&self) -> usize {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn is_selecting_region(&self) -> bool {
        self.is_selecting_region.load(Ordering::Relaxed)
    }

    pub fn enter_selecting_region(&self) {
        self.is_selecting_region.store(true, Ordering::Relaxed);
    }

    pub fn exit_selecting_region(&self) {
        self.is_selecting_region.store(false, Ordering::Relaxed);
    }
}
