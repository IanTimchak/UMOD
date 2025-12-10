pub mod overlay_window;
pub mod renderer;
pub mod state;

use crate::infra::screenshot::ScreenshotService;
use overlay_window::OverlayWindow;
use renderer::RegionSelectionRenderer;
use state::RegionSelectionState;
use state::SelectionPhase;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

pub struct RegionSelection {
    window: Option<OverlayWindow>,
    renderer: Option<RegionSelectionRenderer>,
    state: RegionSelectionState,
    modifiers: ModifiersState,
    //ui: Option<RegionSelectionUi>,
    screenshot: ScreenshotService,
}

impl RegionSelection {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            state: RegionSelectionState::default(),
            modifiers: ModifiersState::empty(),
            //ui: None,
            screenshot: ScreenshotService,
        }
    }
}

impl ApplicationHandler for RegionSelection {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the overlay window
        let window = OverlayWindow::new(event_loop).expect("Failed to create overlay window");
        self.state.window_size = window.inner_size();

        // IMPORTANT: Renderer must use Rc<Window> from OverlayWindow::handle()
        let renderer =
            RegionSelectionRenderer::new(&window).expect("Failed to initialize renderer");

        // UI
        //let ui = RegionSelectionUi::new(&window.handle());

        //self.ui = Some(ui);
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else { return };

        if window_id != window.id() {
            return;
        }

        // forward events to egui layer
        // if let Some(ui) = &mut self.ui {
        //     let repaint = ui.handle_event(&window.handle(), &event); //takes an Rc<Window>, hence handle()

        //     if repaint {
        //         window.request_redraw();
        //     }
        // }

        match event {
            WindowEvent::Resized(size) => {
                self.state.window_size = size;
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.state.update_cursor(position.x, position.y);
                window.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.state.handle_mouse(button, state);
                window.request_redraw();
            }

            WindowEvent::ModifiersChanged(new_mods) => {
                self.modifiers = new_mods.state();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                // Detect if CTRL is down
                //let ctrl_down = self.modifiers.control_key();

                // Detect key code
                // if ctrl_down {
                //     match event.physical_key {
                //         PhysicalKey::Code(KeyCode::KeyZ) => {
                //             // Ctrl+Z → reset the selection state
                //             self.state.reset();

                //             // Request redraw
                //             if let Some(window) = &self.window {
                //                 window.request_redraw();
                //             }

                //             return;
                //         }

                //         _ => {}
                //     }
                // }

                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        // if matches ::Idle, close app.
                        if matches!(&self.state.phase, SelectionPhase::Idle) {
                            event_loop.exit();
                        }

                        // Ctrl+Z → reset the selection state
                        self.state.reset();

                        // Request redraw
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }

                        return;
                    }

                    PhysicalKey::Code(KeyCode::Enter) => {
                        if let SelectionPhase::Confirmed = self.state.phase {
                            if self.state.capture_debounce {
                                return; // debounce
                            }
                            self.state.capture_debounce = true;

                            // Step 1 — switch FSM to Capturing
                            self.state.phase = SelectionPhase::Capturing;

                            // Step 2 — force redraw of transparent frame
                            if let Some(window) = &self.window {
                                window.request_redraw();
                            }
                            return;
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let (Some(renderer), Some(window)) = (&mut self.renderer, &self.window) else {
            return;
        };

        //First, draw the softbuffer workspace
        renderer.draw(&self.state);

        match self.state.phase {
            SelectionPhase::Capturing => {
                // safe to capture now—overlay is transparent
                if let Some((x, y, w, h)) = self.state.selection_bounds() {
                    match self
                        .screenshot
                        .capture_region(x as i32, y as i32, w as u32, h as u32)
                    {
                        Ok(_img) => {
                            println!("Captured screenshot {w}x{h}");
                            // TODO: pass to OCR subsystem
                        }
                        Err(err) => {
                            eprintln!("Screenshot failed: {err}");
                        }
                    }
                }

                // You can now transition to a post-capture UI phase
                // self.state.phase = SelectionPhase::WhateverNext;
                self.state.phase = SelectionPhase::Idle;
                self.state.capture_debounce = false;

                return;
            }

            // keep frame animation alive during the Confirmed state.
            SelectionPhase::Confirmed => {
                // keep marching ants animating
                window.request_redraw();
            }

            _ => {}
        }
    }
}
