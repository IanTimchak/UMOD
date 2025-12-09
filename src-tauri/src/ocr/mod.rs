use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PyBytes;
use std::path::Path;
use std::sync::OnceLock;

// Global singleton for the OCR instance
static MANGA_OCR_INSTANCE: OnceLock<Py<PyAny>> = OnceLock::new();

/// Initialize Manga OCR once at startup
pub fn initialize_manga_ocr() -> PyResult<()> {
    Python::attach(|py| {
        let manga_ocr_module = py.import("manga_ocr")?;
        let mocr = manga_ocr_module.getattr("MangaOcr")?.call0()?;
        
        MANGA_OCR_INSTANCE.set(mocr.into()).map_err(|_| 
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("OCR already initialized")
        )?;
        
        Ok(())
    })
}

/// Fast manga OCR using pre-initialized instance
pub fn manga_ocr(image_bytes: &[u8]) -> PyResult<String> {
    Python::attach(|py| {
        if MANGA_OCR_INSTANCE.get().is_none() {
            initialize_manga_ocr()?;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manga_ocr() {
        let image_file = std::fs::read("src/ocr/00.jpg").expect("Failed to read test image");
        let text = manga_ocr(&image_file).expect("Manga OCR failed");
        println!("Extracted Text: {}", text);
        assert!(!text.is_empty(), "No text extracted");
        assert!(text == "素直にあやまるしか", "Unexpected OCR result");
    }

    #[test]
    fn test_manga_ocr_2() {
        let image_file = std::fs::read("src/ocr/01.png").expect("Failed to read test image");
        let text = manga_ocr(&image_file).expect("Manga OCR failed");
        println!("Extracted Text: {}", text);
        assert!(!text.is_empty(), "No text extracted");
        assert!(text == "人気のワンパンレシピ特集", "Unexpected OCR result");
    }
}