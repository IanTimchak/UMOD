// infrastructure/region_capture_adapter.rs
use std::sync::mpsc::Sender;

pub struct RegionCaptureAdapter {
    tx: Sender<OverlayCommand>, // from the winit thread
}

impl RegionCaptureAdapter {
    pub fn new(tx: Sender<OverlayCommand>) -> Self {
        Self { tx }
    }

    pub fn show_capture_overlay(&self) {
        let _ = self.tx.send(OverlayCommand::ShowOverlay);
    }

    pub fn hide_capture_overlay(&self) {
        let _ = self.tx.send(OverlayCommand::CloseOverlay);
    }
}
