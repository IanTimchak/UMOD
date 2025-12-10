// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod run;      // where your run() lives
mod ui;       // top-level ui module
mod state;    // your AppState
mod app;
mod infra;

use app::region_selection::RegionSelection;
use winit::event_loop::EventLoop;

//mod infra;
//mod domain;

fn main() {
    infra::init_ocr(true).expect("Failed to initialize Manga OCR");


    // // REGION SELECTION SPAWN PROTOCOL:
    // let event_loop = EventLoop::new().unwrap();

    // if let Err(e) = event_loop.run_app(&mut RegionSelection::new()) {
    //     eprintln!("Runtime error: {e}");
    // }


    run::run().expect("error while running tauri application")
}
