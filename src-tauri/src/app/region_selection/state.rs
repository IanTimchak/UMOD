use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton},
};

// CONSTANTS
const MIN_BOX_SIZE: usize = 25;

#[derive(Default)]
pub enum SelectionPhase {
    #[default]
    Idle,
    Drawing, // (previously Dragging)
    Confirmed,
    Moving {
        offset: (f64, f64),
    },
    Capturing,
}

pub struct RegionSelectionState {
    pub start: Option<(f64, f64)>,
    pub end: Option<(f64, f64)>,
    pub phase: SelectionPhase,
    pub cursor_pos: (f64, f64),

    //window
    pub window_size: PhysicalSize<u32>,

    //debounce
    pub capture_debounce: bool,
}

impl Default for RegionSelectionState {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            phase: SelectionPhase::Idle,
            cursor_pos: (0.0, 0.0),
            window_size: PhysicalSize::new(1, 1),
            capture_debounce: false,
        }
    }
}

impl RegionSelectionState {
    // ---------------------------------------------------------------
    // Cursor update
    // ---------------------------------------------------------------
    pub fn update_cursor(&mut self, mut x: f64, mut y: f64) {
        // Clamp cursor to non-negative
        x = x.max(0.0);
        y = y.max(0.0);

        self.cursor_pos = (x, y);

        match &self.phase {
            SelectionPhase::Drawing => {
                self.end = Some((x, y));
            }

            SelectionPhase::Moving { offset } => {
                if let Some((_x, _y, w, h)) = self.selection_bounds() {
                    let mut new_x = x - offset.0;
                    let mut new_y = y - offset.1;

                    let win_w = self.window_size.width as f64;
                    let win_h = self.window_size.height as f64;

                    // Clamp left / top
                    if new_x < 0.0 {
                        new_x = 0.0;
                    }
                    if new_y < 0.0 {
                        new_y = 0.0;
                    }

                    // Clamp right / bottom
                    if new_x + w as f64 > win_w {
                        new_x = (win_w - w as f64).max(0.0);
                    }
                    if new_y + h as f64 > win_h {
                        new_y = (win_h - h as f64).max(0.0);
                    }

                    // Store updated box
                    self.start = Some((new_x, new_y));
                    self.end = Some((new_x + w as f64, new_y + h as f64));
                }
            }

            _ => {}
        }
    }

    // ---------------------------------------------------------------
    // Bounds lookup
    // ---------------------------------------------------------------
    pub fn selection_bounds(&self) -> Option<(usize, usize, usize, usize)> {
        let (sx, sy) = self.start?;
        let (ex, ey) = self.end?;

        let x = sx.min(ex) as usize;
        let y = sy.min(ey) as usize;
        let w = (sx - ex).abs() as usize;
        let h = (sy - ey).abs() as usize;

        Some((x, y, w, h))
    }

    // ---------------------------------------------------------------
    // Mouse logic (the heart of the FSM)
    // ---------------------------------------------------------------
    pub fn handle_mouse(&mut self, button: MouseButton, state: ElementState) {
        if button != MouseButton::Left {
            return; // only left click used
        }

        match (&self.phase, state) {
            // -------------------------------------------------------
            // Idle → Start drawing
            // -------------------------------------------------------
            (SelectionPhase::Idle, ElementState::Pressed) => {
                self.start = Some(self.cursor_pos);
                self.end = Some(self.cursor_pos);
                self.phase = SelectionPhase::Drawing;
            }

            // Drawing → Release: confirm or cancel
            (SelectionPhase::Drawing, ElementState::Released) => {
                if let Some((_x, _y, w, h)) = self.selection_bounds() {
                    if w < MIN_BOX_SIZE || h < MIN_BOX_SIZE {
                        self.reset();
                    } else {
                        self.phase = SelectionPhase::Confirmed;
                    }
                } else {
                    self.reset();
                }
            }

            // -------------------------------------------------------
            // Confirmed → Press: either start moving or ignore
            // -------------------------------------------------------
            (SelectionPhase::Confirmed, ElementState::Pressed) => {
                let (cx, cy) = self.cursor_pos;

                if self.hit_test(cx, cy) {
                    // Begin move
                    if let Some((x, y, _w, _h)) = self.selection_bounds() {
                        let offset = (cx - x as f64, cy - y as f64);
                        self.phase = SelectionPhase::Moving { offset };
                    }
                }
                // else: click outside does nothing
            }

            // Moving → Release → back to Confirmed
            (SelectionPhase::Moving { .. }, ElementState::Released) => {
                self.phase = SelectionPhase::Confirmed;
            }

            _ => {}
        }
    }

    // ---------------------------------------------------------------
    // Hit-test
    // ---------------------------------------------------------------
    pub fn hit_test(&self, x: f64, y: f64) -> bool {
        if let Some((bx, by, bw, bh)) = self.selection_bounds() {
            x >= bx as f64 && y >= by as f64 && x <= (bx + bw) as f64 && y <= (by + bh) as f64
        } else {
            false
        }
    }

    // ---------------------------------------------------------------
    // Reset to Idle (public API)
    // ---------------------------------------------------------------
    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
        self.phase = SelectionPhase::Idle;
    }
}
