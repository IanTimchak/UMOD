// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod infra;
mod run; // where your run() lives
mod state; // your AppState
mod ui; // top-level ui module
mod shared;

fn main() {
    println!("Initializing OCR, please wait...");
    infra::init_ocr(true).expect("Failed to initialize Manga OCR");

    run::run().expect("error while running tauri application")
}
