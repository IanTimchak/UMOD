
use tauri::AppHandle;
use std::thread;
use std::sync::mpsc;

//use crate::app::region_selection::RegionSelection;

/// Launch region selection overlay on a blocking Winit event loop
pub fn spawn_region_selection(app: &AppHandle) {
    // thread::spawn(move || {
    //     let (tx, rx) = mpsc::channel();

    //     // Start the overlay (blocking call)
    //     RegionSelection::run_overlay(tx);

    //     // When overlay exits, we receive screenshot result
    //     if let Ok(Some(path)) = rx.recv() {
    //         println!("Overlay completed. Screenshot: {path}");

    //         // TODO: open the next overlay window (OCR preview, etc.)
            
    //     }
    // });
}