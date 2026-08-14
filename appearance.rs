use serde::{Deserialize, Serialize};

/// Accent colors available in LensOS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccentColor {
    LensTeal,
    DeepViolet,
    GlassCyan,
    ElectricIndigo,
    AmberSunset,
    EmeraldGreen,
    Custom { r: u8, g: u8, b: u8 },
}

impl AccentColor {
    pub fn to_hex(&self) -> String {
        match self {
            AccentColor::LensTeal => "#00B4D8".to_string(),
            AccentColor::DeepViolet => "#7209B7".to_string(),
            AccentColor::GlassCyan => "#4CC9F0".to_string(),
            AccentColor::ElectricIndigo => "#4361EE".to_string(),
            AccentColor::AmberSunset => "#F72585".to_string(),
            AccentColor::EmeraldGreen => "#10B981".to_string(),
            AccentColor::Custom { r, g, b } => format!("#{:02X}{:02X}{:02X}", r, g, b),
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            AccentColor::LensTeal => (0, 180, 216),
            AccentColor::DeepViolet => (114, 9, 183),
            AccentColor::GlassCyan => (76, 201, 240),
            AccentColor::ElectricIndigo => (67, 97, 238),
            AccentColor::AmberSunset => (247, 37, 133),
            AccentColor::EmeraldGreen => (16, 185, 129),
            AccentColor::Custom { r, g, b } => (*r, *g, *b),
        }
    }
}

/// Window backdrop blur effect type for the LensOS desktop UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowBackdropEffect {
    FrostedGlass,
    Acrylic,
    Mica,
    Vibrancy,
    Solid,
}

/// Settings for controlling LensOS appearance, typography, dark mode, and frosted glass.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub fn default_font_family() -> String {
    "Lens Sans Display".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceSettings {
    pub dark_mode: bool,
    pub auto_dark_mode_schedule: bool,
    pub accent_color: AccentColor,
    pub backdrop_effect: WindowBackdropEffect,
    pub transparency_level: f32, // 0.0 (opaque) to 1.0 (fully translucent)
    pub blur_radius_px: u32,
    pub font_family: String,
    pub font_scale: f32, // e.g. 1.0 = 100%
    pub corner_radius_px: u32,
    pub animations_enabled: bool,
    pub reduce_motion: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            auto_dark_mode_schedule: false,
            accent_color: AccentColor::LensTeal,
            backdrop_effect: WindowBackdropEffect::FrostedGlass,
            transparency_level: 0.25,
            blur_radius_px: 24,
            font_family: default_font_family(),
            font_scale: 1.0,
            corner_radius_px: 12,
            animations_enabled: true,
            reduce_motion: false,
        }
    }
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }

    pub fn set_accent_color(&mut self, color: AccentColor) {
        self.accent_color = color;
    }

    pub fn set_transparency(&mut self, level: f32) -> Result<(), String> {
        if (0.0..=1.0).contains(&level) {
            self.transparency_level = level;
            Ok(())
        } else {
            Err("Transparency level must be between 0.0 and 1.0".to_string())
        }
    }

    pub fn set_blur_radius(&mut self, blur_px: u32) {
        self.blur_radius_px = blur_px.min(100);
    }

    pub fn css_backdrop_filter_spec(&self) -> String {
        format!(
            "backdrop-filter: blur({}px) saturate(180%); background: rgba({}, {}, {}, {});",
            self.blur_radius_px,
            if self.dark_mode { 18 } else { 245 },
            if self.dark_mode { 22 } else { 245 },
            if self.dark_mode { 32 } else { 250 },
            1.0 - self.transparency_level
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance_defaults() {
        let app = AppearanceSettings::default();
        assert!(app.dark_mode);
        assert_eq!(app.accent_color, AccentColor::LensTeal);
        assert_eq!(app.blur_radius_px, 24);
    }

    #[test]
    fn test_accent_color_conversion() {
        let color = AccentColor::LensTeal;
        assert_eq!(color.to_hex(), "#00B4D8");
        assert_eq!(color.to_rgb(), (0, 180, 216));

        let custom = AccentColor::Custom { r: 255, g: 128, b: 0 };
        assert_eq!(custom.to_hex(), "#FF8000");
    }

    #[test]
    fn test_transparency_bounds() {
        let mut app = AppearanceSettings::default();
        assert!(app.set_transparency(0.5).is_ok());
        assert_eq!(app.transparency_level, 0.5);

        assert!(app.set_transparency(1.5).is_err());
    }
}
