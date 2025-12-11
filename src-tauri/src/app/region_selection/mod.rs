// src-tauri/src/app/region_selection/mod.rs

pub mod fsm;
pub mod controller;

use std::sync::Mutex;
use fsm::RegionSelectionState;
use crate::infra::screenshot::ScreenshotService;

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
    rs_cursor,
    rs_mousedown,
    rs_mouseup,
    rs_key_enter,
    rs_key_escape,
    rs_get_state,
    rs_set_window_size,
    rs_do_capture,
    rs_ready,
};
