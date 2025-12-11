// src-tauri/src/app/region_selection/mod.rs

pub mod controller;
pub mod fsm;

use crate::infra::screenshot::ScreenshotService;
use fsm::RegionSelectionState;
use std::sync::Mutex;

pub struct RSController {
    pub fsm: Mutex<RegionSelectionState>,
    screenshot: ScreenshotService,
}

impl RSController {
    pub fn new() -> Self {
        Self {
            fsm: Mutex::new(RegionSelectionState::default()),
            screenshot: ScreenshotService,
        }
    }
}

// Re-export commands
pub use controller::{
    rs_cursor, rs_do_capture, rs_get_state, rs_key_enter, rs_key_escape, rs_mousedown, rs_mouseup,
    rs_ready, rs_set_window_size,
};
