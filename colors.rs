//! LensOS UI Color System
//!
//! Provides RGBA color representations, color manipulation algorithms,
//! contrast validation, and sophisticated dark theme color palettes.

/// RGBA color representation using floating-point values `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Opaque black
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    /// Opaque white
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    /// Fully transparent
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

    /// LensOS Signature Obsidian Deep Background (`#0A0D14`)
    pub const OBSIDIAN: Color = Color::rgb(10.0 / 255.0, 13.0 / 255.0, 20.0 / 255.0);
    /// LensOS Frosted Cyan (`#00F0FF`)
    pub const CYAN_NEON: Color = Color::rgb(0.0, 240.0 / 255.0, 1.0);
    /// LensOS Midnight Violet (`#7000FF`)
    pub const VIOLET_NEON: Color = Color::rgb(112.0 / 255.0, 0.0, 1.0);
    /// LensOS Glass Highlight (`#FFFFFF1F`)
    pub const GLASS_WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 0.12);

    /// Constructs an opaque color from `0.0..=1.0` RGB channels.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Constructs a color from `0.0..=1.0` RGBA channels.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs a color from 8-bit integer RGBA values (`0..=255`).
    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Parses a hexadecimal color code string (e.g. `"#0A0D14"` or `"00F0FF80"`).
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgba_u8(r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba_u8(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Returns a new `Color` with modified alpha channel transparency.
    pub fn with_alpha(self, alpha: f32) -> Self {
        Self {
            a: alpha.clamp(0.0, 1.0),
            ..self
        }
    }

    /// Linearly interpolates between `self` and `target` color by factor `t` (`0.0..=1.0`).
    pub fn lerp(self, target: Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (target.r - self.r) * t,
            g: self.g + (target.g - self.g) * t,
            b: self.b + (target.b - self.b) * t,
            a: self.a + (target.a - self.a) * t,
        }
    }

    /// Darkens the color by multiplying RGB channels by `(1.0 - factor)`.
    pub fn darken(self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: (self.r * (1.0 - factor)).clamp(0.0, 1.0),
            g: (self.g * (1.0 - factor)).clamp(0.0, 1.0),
            b: (self.b * (1.0 - factor)).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    /// Lightens the color by interpolating RGB channels towards white (`1.0`).
    pub fn lighten(self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: (self.r + (1.0 - self.r) * factor).clamp(0.0, 1.0),
            g: (self.g + (1.0 - self.g) * factor).clamp(0.0, 1.0),
            b: (self.b + (1.0 - self.b) * factor).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    /// Blends `self` (foreground) over a `background` color using standard Porter-Duff alpha compositing.
    pub fn blend_over(self, background: Color) -> Self {
        let out_a = self.a + background.a * (1.0 - self.a);
        if out_a <= 0.0 {
            return Color::TRANSPARENT;
        }
        let out_r = (self.r * self.a + background.r * background.a * (1.0 - self.a)) / out_a;
        let out_g = (self.g * self.a + background.g * background.a * (1.0 - self.a)) / out_a;
        let out_b = (self.b * self.a + background.b * background.a * (1.0 - self.a)) / out_a;

        Self {
            r: out_r.clamp(0.0, 1.0),
            g: out_g.clamp(0.0, 1.0),
            b: out_b.clamp(0.0, 1.0),
            a: out_a.clamp(0.0, 1.0),
        }
    }

    /// Calculates relative luminance according to W3C WCAG 2.1 specifications.
    pub fn relative_luminance(&self) -> f32 {
        fn adjust(c: f32) -> f32 {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * adjust(self.r) + 0.7152 * adjust(self.g) + 0.0722 * adjust(self.b)
    }

    /// Calculates WCAG 2.1 contrast ratio relative to another color (`1.0..=21.0`).
    pub fn wcag_contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.relative_luminance();
        let l2 = other.relative_luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }
}

/// Comprehensive LensOS Color Palette definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPalette {
    pub surface_background: Color,
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_tertiary: Color,
    pub glass_panel: Color,
    pub glass_card: Color,
    pub glass_popover: Color,

    pub accent_primary: Color,
    pub accent_hover: Color,
    pub accent_active: Color,
    pub accent_glow: Color,
    pub gradient_start: Color,
    pub gradient_end: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_disabled: Color,
    pub text_accent: Color,

    pub border_subtle: Color,
    pub border_glass: Color,
    pub border_highlight: Color,
    pub border_focus: Color,

    pub status_success: Color,
    pub status_warning: Color,
    pub status_error: Color,
    pub status_info: Color,

    pub syntax_keyword: Color,
    pub syntax_func: Color,
    pub syntax_string: Color,
    pub syntax_comment: Color,
    pub syntax_type: Color,
}

impl ColorPalette {
    /// Generates the Sophisticated Dark Palette matching the LensOS design spec.
    pub fn sophisticated_dark() -> Self {
        Self {
            surface_background: Color::rgba_u8(5, 5, 5, 255),
            surface_primary: Color::rgba_u8(20, 20, 25, 255),
            surface_secondary: Color::rgba_u8(30, 30, 38, 255),
            surface_tertiary: Color::rgba_u8(40, 40, 52, 255),
            glass_panel: Color::rgba_u8(20, 20, 25, 178),
            glass_card: Color::rgba_u8(25, 25, 32, 190),
            glass_popover: Color::rgba_u8(30, 30, 40, 230),

            accent_primary: Color::rgba_u8(97, 175, 239, 255),
            accent_hover: Color::rgba_u8(120, 190, 250, 255),
            accent_active: Color::rgba_u8(80, 160, 230, 255),
            accent_glow: Color::rgba_u8(96, 165, 250, 76),
            gradient_start: Color::rgba_u8(30, 58, 138, 50),
            gradient_end: Color::rgba_u8(88, 28, 135, 50),

            text_primary: Color::rgba_u8(224, 224, 224, 255),
            text_secondary: Color::rgba_u8(160, 160, 170, 255),
            text_muted: Color::rgba_u8(100, 100, 115, 255),
            text_disabled: Color::rgba_u8(60, 60, 70, 255),
            text_accent: Color::rgba_u8(97, 175, 239, 255),

            border_subtle: Color::rgba_u8(255, 255, 255, 20),
            border_glass: Color::rgba_u8(255, 255, 255, 20),
            border_highlight: Color::rgba_u8(255, 255, 255, 40),
            border_focus: Color::rgba_u8(97, 175, 239, 200),

            status_success: Color::rgba_u8(39, 201, 63, 255),
            status_warning: Color::rgba_u8(255, 189, 46, 255),
            status_error: Color::rgba_u8(255, 95, 86, 255),
            status_info: Color::rgba_u8(97, 175, 239, 255),

            syntax_keyword: Color::rgba_u8(198, 120, 221, 255),
            syntax_func: Color::rgba_u8(97, 175, 239, 255),
            syntax_string: Color::rgba_u8(152, 195, 121, 255),
            syntax_comment: Color::rgba_u8(92, 99, 112, 255),
            syntax_type: Color::rgba_u8(229, 192, 123, 255),
        }
    }

    /// Generates the default LensOS Dark Palette.
    pub fn lensos_dark() -> Self {
        Self::sophisticated_dark()
    }

    /// Cyber Glass variant with vibrant neon accents and deeper contrast.
    pub fn cyber_glass() -> Self {
        let mut palette = Self::sophisticated_dark();
        palette.surface_background = Color::rgba_u8(5, 5, 5, 255);
        palette.accent_primary = Color::rgba_u8(198, 120, 221, 255);
        palette.accent_glow = Color::rgba_u8(198, 120, 221, 90);
        palette
    }
}
