use std::rc::Rc;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, Fullscreen},
};

pub struct OverlayWindow {
    window: Rc<Window>,
}

impl OverlayWindow {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, String> {
        let primary = event_loop.primary_monitor().ok_or("No monitor found")?;

        let attrs = WindowAttributes::default()
            .with_fullscreen(Some(Fullscreen::Borderless(Some(primary))))
            .with_transparent(true)
            .with_decorations(false)
            .with_title("Region Selection Overlay");

        let window = event_loop
            .create_window(attrs)
            .map_err(|e| format!("window create failed: {e:?}"))?;

        Ok(Self {
            window: Rc::new(window),
        })
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn inner_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Expose an Rc clone for renderer/context
    pub fn handle(&self) -> Rc<Window> {
        self.window.clone()
    }
}
