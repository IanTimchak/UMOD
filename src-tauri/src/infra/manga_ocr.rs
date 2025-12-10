use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PyBytes;
use std::sync::{Mutex, OnceLock};

// Global singleton for the OCR instance
static MANGA_OCR_INSTANCE: OnceLock<Py<PyAny>> = OnceLock::new();
static INIT_LOCK: Mutex<()> = Mutex::new(());

/// Initialize Manga OCR once at startup
/// Set force_cpu to true to avoid CUDA tensor issues
/// Returns Ok(()) if already initialized (idempotent)
pub fn initialize_manga_ocr(force_cpu: bool) -> PyResult<()> {
    // Check if already initialized (fast path, no lock needed)
    if MANGA_OCR_INSTANCE.get().is_some() {
        return Ok(());
    }
    
    // Lock to prevent race condition
    let _guard = INIT_LOCK.lock().unwrap();
    
    // Double-check after acquiring lock
    if MANGA_OCR_INSTANCE.get().is_some() {
        return Ok(());
    }
    
    Python::attach(|py| {
        let manga_ocr_module = py.import("manga_ocr")?;
        
        let mocr = if force_cpu {
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("force_cpu", true)?;
            manga_ocr_module.getattr("MangaOcr")?.call((), Some(&kwargs))?
        } else {
            manga_ocr_module.getattr("MangaOcr")?.call0()?
        };
        
        let _ = MANGA_OCR_INSTANCE.set(mocr.into());
        
        Ok(())
    })
}

/// Fast manga OCR using pre-initialized instance.
/// 
/// Panics if not initialized.
/// 
/// ONLY USE THIS IF YOU ARE SURE initialize_manga_ocr() HAS BEEN CALLED
fn manga_ocr_fast(image_bytes: &[u8]) -> PyResult<String> {
    Python::attach(|py| {
        let mocr = MANGA_OCR_INSTANCE.get()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "OCR not initialized. Call initialize_manga_ocr() first"
            ))?;
        
        let pil = py.import("PIL.Image")?;
        let io = py.import("io")?;
        let bytes_io = io.getattr("BytesIO")?.call1((PyBytes::new(py, image_bytes),))?;
        let image = pil.getattr("open")?.call1((bytes_io,))?;
        
        let result = mocr.call1(py, (image,))?;
        result.extract(py)
    })
}

/// Convenience wrapper that auto-initializes on first use
/// Uses CPU by default to avoid CUDA issues
pub fn manga_ocr(image_bytes: &[u8]) -> PyResult<String> {
    // Initialize if needed (thread-safe due to the lock in initialize_manga_ocr)
    initialize_manga_ocr(true)?;
    manga_ocr_fast(image_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manga_ocr() {
        initialize_manga_ocr(true).expect("Failed to initialize OCR");

        let image_file = std::fs::read("tests/assets/00.jpg").expect("Failed to read test image");
        let text = manga_ocr(&image_file).expect("Manga OCR failed");
        println!("Extracted Text: {}", text);
        assert!(!text.is_empty(), "No text extracted");
        assert!(text == "素直にあやまるしか", "Unexpected OCR result");
    }

    #[test]
    fn test_manga_ocr_2() {
        initialize_manga_ocr(true).expect("Failed to initialize OCR");

        let image_file = std::fs::read("tests/assets/01.png").expect("Failed to read test image");
        let text = manga_ocr(&image_file).expect("Manga OCR failed");
        println!("Extracted Text: {}", text);
        assert!(!text.is_empty(), "No text extracted");
        assert!(text == "人気のワンパンレシピ特集", "Unexpected OCR result");
    }
}