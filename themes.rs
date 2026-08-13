//! Theme installation and visual appearance manager for LensOS v0.1.
//!
//! Themes define system-wide desktop aesthetics, color palettes, glassmorphism blur levels,
//! typography fonts, window decoration styles, icon assets, and wallpapers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::installer::InstallerError;

/// Color palette definition for a LensOS desktop theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary_accent: String,
    pub secondary_accent: String,
    pub background_canvas: String,
    pub surface_card: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border_subtle: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary_accent: "#0066FF".to_string(),
            secondary_accent: "#7000FF".to_string(),
            background_canvas: "#F8F9FA".to_string(),
            surface_card: "#FFFFFF".to_string(),
            text_primary: "#111827".to_string(),
            text_secondary: "#6B7280".to_string(),
            border_subtle: "#E5E7EB".to_string(),
        }
    }
}

/// Representation of a LensOS desktop theme package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensTheme {
    /// Unique theme ID (e.g., "theme.lensos.glass_dark").
    pub id: String,
    /// Display name.
    pub name: String,
    /// Version.
    pub version: String,
    /// Author or designer.
    pub author: String,
    /// Is dark mode theme.
    pub is_dark_variant: bool,
    /// Glassmorphism backdrop blur strength (0 to 30 px).
    pub backdrop_blur_px: u32,
    /// Corner radius for window frames and UI cards.
    pub window_corner_radius_px: u32,
    /// Color palette definition.
    pub palette: ColorPalette,
    /// Path or URL to default desktop wallpaper.
    pub wallpaper_path: Option<String>,
    /// Associated font family bindings (e.g. "Plus Jakarta Sans").
    pub font_family: String,
    /// Custom CSS/Styling variables map.
    pub custom_tokens: HashMap<String, String>,
}

impl LensTheme {
    pub fn new_default_light() -> Self {
        Self {
            id: "theme.lensos.default_light".to_string(),
            name: "LensOS Light Glass".to_string(),
            version: "0.1.0".to_string(),
            author: "LensOS Design System".to_string(),
            is_dark_variant: false,
            backdrop_blur_px: 16,
            window_corner_radius_px: 12,
            palette: ColorPalette::default(),
            wallpaper_path: Some("wallpapers/lens_default.png".to_string()),
            font_family: "Plus Jakarta Sans".to_string(),
            custom_tokens: HashMap::new(),
        }
    }

    pub fn new_default_dark() -> Self {
        let mut palette = ColorPalette::default();
        palette.background_canvas = "#0D0F12".to_string();
        palette.surface_card = "#161B22".to_string();
        palette.text_primary = "#F0F6FC".to_string();
        palette.text_secondary = "#8B949E".to_string();
        palette.border_subtle = "#30363D".to_string();

        Self {
            id: "theme.lensos.default_dark".to_string(),
            name: "LensOS Dark Obsidian".to_string(),
            version: "0.1.0".to_string(),
            author: "LensOS Design System".to_string(),
            is_dark_variant: true,
            backdrop_blur_px: 20,
            window_corner_radius_px: 12,
            palette,
            wallpaper_path: Some("wallpapers/obsidian_dark.png".to_string()),
            font_family: "Plus Jakarta Sans".to_string(),
            custom_tokens: HashMap::new(),
        }
    }
}

/// Information summary for installed themes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensThemeInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub is_active: bool,
    pub is_dark: bool,
}

/// Manager for installing, applying, and removing LensOS themes.
#[derive(Debug, Clone)]
pub struct ThemeManager {
    installed_themes: HashMap<String, LensTheme>,
    active_theme_id: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let light = LensTheme::new_default_light();
        let dark = LensTheme::new_default_dark();

        let mut installed = HashMap::new();
        let active_id = light.id.clone();
        installed.insert(light.id.clone(), light);
        installed.insert(dark.id.clone(), dark);

        Self {
            installed_themes: installed,
            active_theme_id: active_id,
        }
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Installs a new theme package.
    pub fn install_theme(&mut self, theme: LensTheme) -> Result<String, InstallerError> {
        if theme.id.trim().is_empty() {
            return Err(InstallerError::InvalidPackage(
                "Theme ID cannot be empty".to_string(),
            ));
        }

        let theme_id = theme.id.clone();
        self.installed_themes.insert(theme_id.clone(), theme);
        Ok(theme_id)
    }

    /// Applies an installed theme as active system desktop theme.
    pub fn apply_theme(&mut self, theme_id: &str) -> Result<(), InstallerError> {
        if self.installed_themes.contains_key(theme_id) {
            self.active_theme_id = theme_id.to_string();
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(format!(
                "Theme not found: {}",
                theme_id
            )))
        }
    }

    /// Uninstalls a theme. Cannot uninstall active theme.
    pub fn remove_theme(&mut self, theme_id: &str) -> Result<(), InstallerError> {
        if theme_id == self.active_theme_id {
            return Err(InstallerError::OperationFailed(
                "Cannot remove currently active system theme".to_string(),
            ));
        }

        if self.installed_themes.remove(theme_id).is_some() {
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(theme_id.to_string()))
        }
    }

    /// Retrieves active theme.
    pub fn get_active_theme(&self) -> Option<&LensTheme> {
        self.installed_themes.get(&self.active_theme_id)
    }

    /// Lists all installed themes.
    pub fn list_installed(&self) -> Vec<LensThemeInfo> {
        self.installed_themes
            .values()
            .map(|t| LensThemeInfo {
                id: t.id.clone(),
                name: t.name.clone(),
                version: t.version.clone(),
                is_active: t.id == self.active_theme_id,
                is_dark: t.is_dark_variant,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_management() {
        let mut manager = ThemeManager::new();

        assert_eq!(manager.list_installed().len(), 2);

        let custom_theme = LensTheme {
            id: "theme.custom.neon".to_string(),
            name: "Neon Cyber".to_string(),
            version: "1.0.0".to_string(),
            author: "Cyberpunk".to_string(),
            is_dark_variant: true,
            backdrop_blur_px: 25,
            window_corner_radius_px: 8,
            palette: ColorPalette::default(),
            wallpaper_path: None,
            font_family: "JetBrains Mono".to_string(),
            custom_tokens: HashMap::new(),
        };

        assert!(manager.install_theme(custom_theme).is_ok());
        assert!(manager.apply_theme("theme.custom.neon").is_ok());
        assert_eq!(
            manager.get_active_theme().unwrap().name,
            "Neon Cyber"
        );
    }
}
