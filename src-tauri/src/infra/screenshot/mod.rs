use anyhow::Result;
use screenshots::image::{ImageBuffer, ImageFormat, Rgba};
use screenshots::Screen;

const SCREENSHOT_PATH: &str = "region_capture.png";

pub struct ScreenshotService;

impl ScreenshotService {
    pub fn capture_region(&self, x: i32, y: i32, w: u32, h: u32) -> Result<String> {
        // Choose primary display (or later: display that contains the region)
        let screens = Screen::all()?;
        let screen = &screens[0];

        // Capture specified region
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = screen.capture_area(x, y, w, h)?;
        img.save_with_format(SCREENSHOT_PATH, ImageFormat::Png)
            .unwrap();

        // img is already an ImageBuffer, return it directly
        Ok(String::from(SCREENSHOT_PATH))
    }
}
