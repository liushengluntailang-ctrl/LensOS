//! # Browser Settings & Visual Theme Module (`settings.rs`)
//!
//! Encapsulates user preferences, Frosted Glass visual compositor parameters,
//! dark theme styling options, privacy toggles, and LensOS system configuration defaults.

use crate::{BrowserError, BrowserResult};

/// Theme visual mode selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    FrostedGlassDark,
    UltraDark,
    DeepMidnight,
    AdaptiveSystem,
}

/// Parameters controlling the LensOS Frosted Glass desktop rendering engine.
#[derive(Debug, Clone, PartialEq)]
pub struct FrostedGlassTheme {
    /// Backdrop gaussian blur radius in pixels (default: 24px).
    pub blur_radius_px: u32,
    /// Translucent background opacity value between 0.0 and 1.0 (default: 0.65).
    pub glass_opacity: f32,
    /// Subtle grain noise overlay intensity for premium depth feel (default: 0.04).
    pub noise_grain_intensity: f32,
    /// Primary accent color hex string (e.g., "#00E5FF" for Lens Electric Cyan).
    pub accent_color_hex: String,
    /// Surface background tint RGBA color string.
    pub background_tint_rgba: String,
}

impl Default for FrostedGlassTheme {
    fn default() -> Self {
        Self {
            blur_radius_px: 24,
            glass_opacity: 0.65,
            noise_grain_intensity: 0.04,
            accent_color_hex: "#00E5FF".to_string(),
            background_tint_rgba: "rgba(18, 20, 28, 0.75)".to_string(),
        }
    }
}

/// Full theme configuration structure.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub frosted_glass: FrostedGlassTheme,
    pub font_family: String,
    pub compact_tabs: bool,
    pub enable_animations: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: ThemeMode::FrostedGlassDark,
            frosted_glass: FrostedGlassTheme::default(),
            font_family: "Lens Sans Display".to_string(),
            compact_tabs: false,
            enable_animations: true,
        }
    }
}

/// Browser behavior upon startup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupBehavior {
    OpenNewTabPage,
    RestorePreviousSession,
    OpenSpecificPages(Vec<String>),
}

/// Privacy and tracking protection settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacySettings {
    pub do_not_track: bool,
    pub https_only_mode: bool,
    pub ad_block_enabled: bool,
    pub third_party_cookie_blocking: bool,
    pub telemetry_enabled: bool,
    pub clear_data_on_exit: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            do_not_track: true,
            https_only_mode: true,
            ad_block_enabled: true,
            third_party_cookie_blocking: true,
            telemetry_enabled: false,
            clear_data_on_exit: false,
        }
    }
}

/// Central configuration settings store for Lens Browser v0.1.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowserSettings {
    pub theme: ThemeConfig,
    pub startup_behavior: StartupBehavior,
    pub privacy: PrivacySettings,
    pub homepage_url: String,
    pub default_download_dir: String,
    pub show_bookmark_bar: bool,
    pub enable_lens_ai_assistant: bool,
    pub zoom_level_percent: u32,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            startup_behavior: StartupBehavior::OpenNewTabPage,
            privacy: PrivacySettings::default(),
            homepage_url: "lens://newtab".to_string(),
            default_download_dir: "/home/lens/Downloads".to_string(),
            show_bookmark_bar: true,
            enable_lens_ai_assistant: true,
            zoom_level_percent: 100,
        }
    }
}

impl BrowserSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets settings to factory default state.
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Adjusts page display zoom level percentage (min 25%, max 500%).
    pub fn set_zoom(&mut self, percent: u32) -> BrowserResult<()> {
        if !(25..=500).contains(&percent) {
            return Err(BrowserError::StorageError("Zoom level must be between 25% and 500%".into()));
        }
        self.zoom_level_percent = percent;
        Ok(())
    }
}
