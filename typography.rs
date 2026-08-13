//! LensOS UI Typography System
//!
//! Handles font weights, typographic hierarchy, text metrics calculation,
//! alignment, line wrapping, and legibility constraints for LensOS.

use crate::colors::Color;

/// Font weight values matching standard CSS/OpenType specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin = 100,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
}

impl FontWeight {
    pub fn value(&self) -> u16 {
        *self as u16
    }
}

/// Font style classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
}

/// Horizontal text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Behavior when text exceeds bounding box width limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTruncation {
    None,
    Ellipsis,
    Clip,
    Fade,
}

/// Complete styling specification for rendering text fragments.
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub truncation: TextTruncation,
}

impl TypographyStyle {
    pub fn new(font_family: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            font_family: font_family.into(),
            font_size,
            font_weight: FontWeight::Regular,
            font_style: FontStyle::Normal,
            line_height: font_size * 1.4,
            letter_spacing: 0.0,
            color,
            alignment: TextAlignment::Left,
            truncation: TextTruncation::Ellipsis,
        }
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn with_line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

/// Standardized LensOS Typographic Hierarchy Scale.
#[derive(Debug, Clone, PartialEq)]
pub struct TypographyScale {
    pub display_large: TypographyStyle,
    pub display_medium: TypographyStyle,
    pub heading_1: TypographyStyle,
    pub heading_2: TypographyStyle,
    pub heading_3: TypographyStyle,
    pub body_large: TypographyStyle,
    pub body_medium: TypographyStyle,
    pub body_small: TypographyStyle,
    pub caption: TypographyStyle,
    pub code_mono: TypographyStyle,
}

impl TypographyScale {
    pub fn default_scale(text_primary: Color, text_secondary: Color) -> Self {
        let sans = "LensSans, Inter, system-ui, sans-serif";
        let mono = "LensMono, JetBrains Mono, monospace";

        Self {
            display_large: TypographyStyle::new(sans, 36.0, text_primary)
                .with_weight(FontWeight::Bold)
                .with_line_height(44.0),
            display_medium: TypographyStyle::new(sans, 28.0, text_primary)
                .with_weight(FontWeight::SemiBold)
                .with_line_height(36.0),
            heading_1: TypographyStyle::new(sans, 22.0, text_primary)
                .with_weight(FontWeight::SemiBold)
                .with_line_height(28.0),
            heading_2: TypographyStyle::new(sans, 18.0, text_primary)
                .with_weight(FontWeight::Medium)
                .with_line_height(24.0),
            heading_3: TypographyStyle::new(sans, 15.0, text_primary)
                .with_weight(FontWeight::Medium)
                .with_line_height(20.0),
            body_large: TypographyStyle::new(sans, 16.0, text_primary)
                .with_weight(FontWeight::Regular)
                .with_line_height(24.0),
            body_medium: TypographyStyle::new(sans, 14.0, text_primary)
                .with_weight(FontWeight::Regular)
                .with_line_height(20.0),
            body_small: TypographyStyle::new(sans, 12.0, text_secondary)
                .with_weight(FontWeight::Regular)
                .with_line_height(16.0),
            caption: TypographyStyle::new(sans, 11.0, text_secondary)
                .with_weight(FontWeight::Medium)
                .with_line_height(14.0),
            code_mono: TypographyStyle::new(mono, 13.0, text_primary)
                .with_weight(FontWeight::Regular)
                .with_line_height(18.0),
        }
    }
}

/// Helper calculations for text layout measurements and line wrapping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBounds {
    pub width: f32,
    pub height: f32,
    pub line_count: usize,
}

/// Text layout engine for calculating multi-line text wrapped boxes.
#[derive(Debug)]
pub struct TextLayoutEngine;

impl TextLayoutEngine {
    /// Estimates dimensions of a given string snippet based on average character width ratio.
    pub fn measure_text(text: &str, style: &TypographyStyle, max_width: Option<f32>) -> TextBounds {
        let avg_char_width = style.font_size * 0.55;
        let total_chars = text.chars().count();
        let total_unwrapped_width = total_chars as f32 * avg_char_width;

        if let Some(limit) = max_width {
            if limit > 0.0 && total_unwrapped_width > limit {
                let chars_per_line = ((limit / avg_char_width) as usize).max(1);
                let line_count = (total_chars + chars_per_line - 1) / chars_per_line;
                return TextBounds {
                    width: limit,
                    height: line_count as f32 * style.line_height,
                    line_count,
                };
            }
        }

        TextBounds {
            width: total_unwrapped_width,
            height: style.line_height,
            line_count: 1,
        }
    }
}
