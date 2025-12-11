use tauri::{AppHandle, Emitter, Manager, State};
use winit::event::{ElementState, MouseButton};

use super::fsm::SelectionPhase;
use super::RSController;
use crate::app::AppMediator;
use crate::state::AppState;

#[tauri::command]
pub fn rs_cursor(app: AppHandle, rs: State<'_, RSController>, x: f64, y: f64) {
    {
        let mut fsm = rs.fsm.lock().unwrap();
        fsm.update_cursor(x, y);
    }
    let _ = app.emit("rs-update", ());
}

#[tauri::command]
pub fn rs_mousedown(app: AppHandle, rs: State<'_, RSController>, button: String) {
    if button == "left" {
        {
            let mut fsm = rs.fsm.lock().unwrap();
            fsm.handle_mouse(MouseButton::Left, ElementState::Pressed);
        }
        let _ = app.emit("rs-update", ());
    }
}

#[tauri::command]
pub fn rs_mouseup(app: AppHandle, rs: State<'_, RSController>, button: String) {
    if button == "left" {
        {
            let mut fsm = rs.fsm.lock().unwrap();
            fsm.handle_mouse(MouseButton::Left, ElementState::Released);
        }
        let _ = app.emit("rs-update", ());
    }
}

/// ENTER key triggers capture
#[tauri::command]
pub async fn rs_key_enter(app: AppHandle, rs: State<'_, RSController>) -> Result<(), String> {
    let mut fsm = rs.fsm.lock().unwrap();
    println!("rs-key-enter");

    if let SelectionPhase::Confirmed = fsm.phase {
        fsm.phase = SelectionPhase::Capturing;
        drop(fsm);

        // Emit Tauri event (Tauri 2.x)
        app.emit("rs-begin-capture", ())
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Ok(())
    }
}

/// ESC resets or closes
#[tauri::command]
pub async fn rs_key_escape(app: AppHandle, rs: State<'_, RSController>) -> Result<(), String> {
    let mut fsm = rs.fsm.lock().unwrap();
    println!("rs-key-escape");

    if matches!(fsm.phase, SelectionPhase::Idle) {
        if let Some(win) = app.webview_windows().get("region-overlay") {
            win.close().map_err(|e| e.to_string())?;
        }

        // -------- allow keybind again --------
        app.state::<AppState>().exit_selecting_region();

        return Ok(());
    }

    fsm.reset();
    Ok(())
}

#[tauri::command]
pub fn rs_set_window_size(rs: State<'_, RSController>, width: u32, height: u32) {
    use winit::dpi::PhysicalSize;
    let mut fsm = rs.fsm.lock().unwrap();
    fsm.window_size = PhysicalSize::new(width, height);
}

#[derive(serde::Serialize)]
pub struct Bounds {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[derive(serde::Serialize)]
pub struct RSStateResponse {
    phase: String,
    bounds: Option<Bounds>,
}

#[tauri::command]
pub fn rs_get_state(rs: State<'_, RSController>) -> RSStateResponse {
    let fsm = rs.fsm.lock().unwrap();

    let phase = match fsm.phase {
        SelectionPhase::Idle => "Idle",
        SelectionPhase::Drawing => "Drawing",
        SelectionPhase::Confirmed => "Confirmed",
        SelectionPhase::Moving { .. } => "Moving",
        SelectionPhase::Capturing => "Capturing",
    }
    .to_string();

    let bounds = fsm
        .selection_bounds()
        .map(|(x, y, w, h)| Bounds { x, y, w, h });

    RSStateResponse { phase, bounds }
}

#[tauri::command]
pub async fn rs_do_capture(app: AppHandle, rs: State<'_, RSController>) -> Result<(), String> {
    let mut fsm = rs.fsm.lock().unwrap();
    let path_opt = { fsm.capture(&rs.screenshot)? };

    let Some(path) = path_opt else {
        return Err("No bounds to capture".into());
    };

    // -------- close overlay window --------
    if let Some(win) = app.webview_windows().get("region-overlay") {
        win.close().map_err(|e| e.to_string())?;
    }

    // -------- send result back to main app --------
    // (Maybe AppMediator listens for this)
    AppMediator::send_file_path(path.clone());

    // -------- reset fsm --------
    fsm.reset();

    // -------- allow keybind again --------
    app.state::<AppState>().exit_selecting_region();

    Ok(())
}

#[tauri::command]
pub fn rs_ready(app: AppHandle) {
    if let Some(win) = app.webview_windows().get("region-overlay") {
        let _ = win.show();
    }
}
