// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod run;      // where your run() lives
mod ui;       // top-level ui module
mod state;    // your AppState
mod app;
//mod infra;
//mod domain;

fn main() {
    //testwindowviews_lib::run().expect("error while running tauri application")
    run::run().expect("error while running tauri application")
}
