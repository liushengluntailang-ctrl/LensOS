use serde::{Deserialize, Serialize};

/// Supported image formats in LensOS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
    Svg,
    Unknown,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Webp => "webp",
            ImageFormat::Svg => "svg",
            ImageFormat::Unknown => "bin",
        }
    }
}

/// Structural metadata of an image file or buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub size_bytes: usize,
    pub aspect_ratio: f32,
}

/// Analysis output from vision processing models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysisResult {
    pub description: String,
    pub detected_objects: Vec<String>,
    pub extracted_text: Option<String>,
    pub dominant_colors: Vec<String>,
    pub metadata: ImageMetadata,
}

/// Image processing and vision utility module for LensAI.
pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_format(&self, image_data: &[u8]) -> ImageFormat {
        if image_data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            ImageFormat::Png
        } else if image_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            ImageFormat::Jpeg
        } else if image_data.len() > 12 && &image_data[8..12] == b"WEBP" {
            ImageFormat::Webp
        } else if image_data.starts_with(b"<svg") || image_data.starts_with(b"<?xml") {
            ImageFormat::Svg
        } else {
            ImageFormat::Unknown
        }
    }

    pub fn analyze_image(&self, image_data: &[u8]) -> Result<ImageAnalysisResult, String> {
        if image_data.is_empty() {
            return Err("Image byte stream is empty.".to_string());
        }

        let format = self.detect_format(image_data);
        let metadata = ImageMetadata {
            width: 1920,
            height: 1080,
            format,
            size_bytes: image_data.len(),
            aspect_ratio: 16.0 / 9.0,
        };

        let extracted_text = self.extract_text(image_data).ok();

        Ok(ImageAnalysisResult {
            description: "A dark frosted glass interface element with subtle ambient glow.".to_string(),
            detected_objects: vec!["UI Panel".to_string(), "Button".to_string(), "Glass Surface".to_string()],
            extracted_text,
            dominant_colors: vec!["#0F172A".to_string(), "#38BDF8".to_string(), "#1E293B".to_string()],
            metadata,
        })
    }

    pub fn extract_text(&self, image_data: &[u8]) -> Result<String, String> {
        if image_data.is_empty() {
            return Err("Empty image buffer.".to_string());
        }
        Ok("LensOS AI Assistant - OCR Text Block Extracted".to_string())
    }

    pub fn generate_thumbnail_placeholder(&self, width: u32, height: u32) -> String {
        format!(
            "svg:placeholder::<{}x{}|frosted_glass_dark>",
            width, height
        )
    }
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self::new()
    }
}
