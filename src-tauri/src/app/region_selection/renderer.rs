use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::window::Window as WinitWindow;

use super::overlay_window::OverlayWindow;
use super::state::RegionSelectionState;
use super::state::SelectionPhase;

//CONSTANTS
const DASH: usize = 8;
const LINE: usize = 4;

pub struct RegionSelectionRenderer {
    window: Rc<WinitWindow>,
    context: Context<Rc<WinitWindow>>,
    surface: Surface<Rc<WinitWindow>, Rc<WinitWindow>>,
    frame: u32, //for animated borders
}

impl RegionSelectionRenderer {
    pub fn new(overlay: &OverlayWindow) -> Result<Self, String> {
        // Rc<Window> from overlay
        let window = overlay.handle();

        // SAFETY: softbuffer requires that the window outlives the context and surface.
        // We keep an Rc<Window> in this struct, so that holds.
        let context = Context::new(window.clone()).map_err(|e| format!("Context error: {e:?}"))?;

        let surface =
            Surface::new(&context, window.clone()).map_err(|e| format!("Surface error: {e:?}"))?;

        Ok(Self {
            window,
            context,
            surface,
            frame: 0,
        })
    }

    pub fn draw(&mut self, state: &RegionSelectionState) {
        self.frame = self.frame.wrapping_add(1); //loops back to 0 on overflow
        let size = state.window_size;
        let width = size.width.max(1);
        let height = size.height.max(1);

        // Keep surface size in sync with window
        if let Err(e) = self.surface.resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        ) {
            eprintln!("surface resize error: {e:?}");
            return;
        }

        let mut buffer = match self.surface.buffer_mut() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("buffer_mut error: {e:?}");
                return;
            }
        };

        // 50% opaque black (ARGB) so you can SEE the overlay
        let dim_color: u32 = 0x8000_0000;

        for pixel in buffer.iter_mut() {
            *pixel = dim_color;
        }

        if let Some((mut x, mut y, mut w, mut h)) = state.selection_bounds() {
            let w_total = width as usize;
            let h_total = height as usize;

            // Clamp left/top corner
            if x >= w_total {
                x = w_total - 1;
            }
            if y >= h_total {
                y = h_total - 1;
            }

            // Clamp width/height so we don't exceed screen
            if x + w >= w_total {
                w = w_total - 1 - x;
            }
            if y + h >= h_total {
                h = h_total - 1 - y;
            }

            // Draw according to current state phase
            match state.phase {
                SelectionPhase::Idle => {}

                SelectionPhase::Drawing => {
                    //drag border style

                    // Draw bounding box
                    Self::fill_region(&mut buffer, w_total, h_total, x, y, w, h, 0x00000000); //make transparent
                    Self::draw_border(&mut buffer, w_total, h_total, x, y, w, h, 0xFFFFFFFF);
                }

                SelectionPhase::Confirmed | SelectionPhase::Moving { .. } => {
                    //bold marching ants

                    // Draw bounding box
                    Self::draw_bold_border(&mut buffer, w_total, h_total, x, y, w, h, 0xFFFFFF00);
                    Self::draw_marching_ants(
                        &mut buffer,
                        w_total,
                        h_total,
                        x,
                        y,
                        w,
                        h,
                        self.frame,
                        0xFF000000,
                    );
                }

                SelectionPhase::Capturing => {
                    // 50% opaque black (ARGB) so you can SEE the overlay
                    let transparent: u32 = 0x0000_0000;

                    for pixel in buffer.iter_mut() {
                        *pixel = transparent;
                    }
                }
            }
        }

        if let Err(e) = buffer.present() {
            eprintln!("present error: {e:?}");
        }
    }

    fn draw_border(
        buffer: &mut [u32],
        w_total: usize,
        _h_total: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: u32,
    ) {
        let top = y;
        let bottom = y + h;
        let left = x;
        let right = x + w;

        for dx in left..=right {
            let i1 = top * w_total + dx;
            let i2 = bottom * w_total + dx;
            if i1 < buffer.len() {
                buffer[i1] = color;
            }
            if i2 < buffer.len() {
                buffer[i2] = color;
            }
        }

        for dy in top..=bottom {
            let i1 = dy * w_total + left;
            let i2 = dy * w_total + right;
            if i1 < buffer.len() {
                buffer[i1] = color;
            }
            if i2 < buffer.len() {
                buffer[i2] = color;
            }
        }
    }

    fn fill_region(
        buffer: &mut [u32],
        w_total: usize,
        _h_total: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: u32,
    ) {
        for row in y..=y + h {
            let row_start = row * w_total;
            let slice_start = row_start + x;
            let slice_end = slice_start + w;

            // Bounds safety
            if slice_end <= buffer.len() {
                buffer[slice_start..slice_end].fill(color);
            }
        }
    }

    fn draw_bold_border(
        buffer: &mut [u32],
        w_total: usize,
        h_total: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: u32,
    ) {
        Self::fill_region(buffer, w_total, h_total, x, y, w, h, 0x00000000); //make transparent

        // Thickness: 2px border drawn OUTWARDS
        for offset in 0..2 {
            let ox = x.saturating_sub(offset);
            let oy = y.saturating_sub(offset);
            let ow = w + offset * 2;
            let oh = h + offset * 2;

            Self::draw_border(buffer, w_total, h_total, ox, oy, ow, oh, color);
        }
    }

    fn draw_marching_ants(
        buffer: &mut [u32],
        w_total: usize,
        _h_total: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        frame: u32,
        color: u32,
    ) {
        let top = y;
        let bottom = y + h;
        let left = x;
        let right = x + w;

        let phase = frame as usize;

        // --- TOP & BOTTOM ---
        for dx in 0..=w {
            let px = left + dx;

            // top moves → : parameter grows with dx
            let t_top = dx;
            if (t_top + phase) % DASH < LINE {
                let idx = top * w_total + px;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }

            // bottom moves ← : parameter shrinks with dx
            let t_bottom = w - dx; // safe: dx ∈ [0, w]
            if (t_bottom + phase) % DASH < LINE {
                let idx = bottom * w_total + px;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }
        }

        // --- LEFT & RIGHT ---
        for dy in 0..=h {
            let py = top + dy;

            // right moves ↓ : parameter grows with dy
            let t_right = dy;
            if (t_right + phase) % DASH < LINE {
                let idx = py * w_total + right;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }

            // left moves ↑ : parameter shrinks with dy
            let t_left = h - dy; // safe: dy ∈ [0, h]
            if (t_left + phase) % DASH < LINE {
                let idx = py * w_total + left;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }
        }
    }
}
