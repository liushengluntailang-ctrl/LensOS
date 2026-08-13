//! LensOS Theme Management Engine
//!
//! Controls global design tokens, spacing scales, corner radii, elevation depth,
//! theme switching, and WCAG accessibility validation.

use crate::colors::{Color, ColorPalette};
use crate::glass::GlassMaterial;
use crate::typography::TypographyScale;

/// Distinct visual theme modes supported natively by LensOS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeMode {
    SophisticatedDark,
    DarkObsidian,
    MidnightNeon,
    CyberGlass,
    Custom(String),
}

/// Standardized corner radius tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadiusScale {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for CornerRadiusScale {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            full: 9999.0,
        }
    }
}

/// Standardized layout spacing tokens (8pt spatial grid).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingScale {
    pub xxs: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xxs: 2.0,
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

/// Glass Z-axis elevation levels specifying drop shadow intensities.
#[derive(Debug, Clone, PartialEq)]
pub struct ElevationScale {
    pub level_0_flat: GlassMaterial,
    pub level_1_card: GlassMaterial,
    pub level_2_window: GlassMaterial,
    pub level_3_popover: GlassMaterial,
    pub level_4_modal: GlassMaterial,
}

impl Default for ElevationScale {
    fn default() -> Self {
        Self {
            level_0_flat: GlassMaterial::frosted_crystal(),
            level_1_card: GlassMaterial::frosted_crystal(),
            level_2_window: GlassMaterial::deep_acrylic(),
            level_3_popover: GlassMaterial::luminous_popover(),
            level_4_modal: {
                let mut mat = GlassMaterial::deep_acrylic();
                mat.shadow_blur = 64.0;
                mat.shadow_offset_y = 24.0;
                mat.shadow_color = Color::rgba(0.0, 0.0, 0.0, 0.80);
                mat
            },
        }
    }
}

/// Master Theme holding all active OS design tokens.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub mode: ThemeMode,
    pub palette: ColorPalette,
    pub typography: TypographyScale,
    pub radii: CornerRadiusScale,
    pub spacing: SpacingScale,
    pub elevation: ElevationScale,
}

impl Theme {
    /// Creates the flagship LensOS Sophisticated Dark theme instance.
    pub fn sophisticated_dark() -> Self {
        let palette = ColorPalette::sophisticated_dark();
        let typography = TypographyScale::default_scale(palette.text_primary, palette.text_secondary);

        Self {
            name: "LensOS Sophisticated Dark".to_string(),
            mode: ThemeMode::SophisticatedDark,
            palette,
            typography,
            radii: CornerRadiusScale::default(),
            spacing: SpacingScale::default(),
            elevation: ElevationScale::default(),
        }
    }

    /// Creates the flagship LensOS Dark Obsidian theme instance.
    pub fn dark_obsidian() -> Self {
        Self::sophisticated_dark()
    }

    /// Creates the Cyber Glass theme instance.
    pub fn cyber_glass() -> Self {
        let palette = ColorPalette::cyber_glass();
        let typography = TypographyScale::default_scale(palette.text_primary, palette.text_secondary);

        Self {
            name: "Cyber Glass".to_string(),
            mode: ThemeMode::CyberGlass,
            palette,
            typography,
            radii: CornerRadiusScale::default(),
            spacing: SpacingScale::default(),
            elevation: ElevationScale::default(),
        }
    }

    /// Validates that text colors meet minimum WCAG AA contrast ratio (4.5:1).
    pub fn validate_accessibility(&self) -> bool {
        let text = self.palette.text_primary;
        let bg = self.palette.surface_background;
        text.wcag_contrast_ratio(&bg) >= 4.5
    }
}

/// Global Theme Manager coordinating active runtime themes and dynamic switching.
#[derive(Debug)]
pub struct ThemeManager {
    active_theme: Theme,
    custom_themes: Vec<Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            active_theme: Theme::dark_obsidian(),
            custom_themes: Vec::new(),
        }
    }

    pub fn active_theme(&self) -> &Theme {
        &self.active_theme
    }

    pub fn active_theme_mut(&mut self) -> &mut Theme {
        &mut self.active_theme
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.active_theme = match mode {
            ThemeMode::SophisticatedDark | ThemeMode::DarkObsidian | ThemeMode::MidnightNeon => {
                Theme::sophisticated_dark()
            }
            ThemeMode::CyberGlass => Theme::cyber_glass(),
            ThemeMode::Custom(ref name) => self
                .custom_themes
                .iter()
                .find(|t| &t.name == name)
                .cloned()
                .unwrap_or_else(Theme::sophisticated_dark),
        };
    }

    pub fn register_custom_theme(&mut self, theme: Theme) {
        self.custom_themes.push(theme);
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
