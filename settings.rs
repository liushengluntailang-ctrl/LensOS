use crate::ai::AIModel;
use serde::{Deserialize, Serialize};

/// Configuration parameters for LensOS Frosted Glass UI effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrostedGlassConfig {
    pub blur_radius: u32,
    pub opacity: f32,
    pub border_opacity: f32,
    pub tint_color: String,
    pub shadow_intensity: f32,
}

impl Default for FrostedGlassConfig {
    fn default() -> Self {
        Self {
            blur_radius: 24,
            opacity: 0.75,
            border_opacity: 0.15,
            tint_color: "#0B0F17".to_string(),
            shadow_intensity: 0.40,
        }
    }
}

/// Theme settings adhering to the LensOS Sophisticated Dark Theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub dark_mode: bool,
    pub accent_color: String,
    pub glass_effect: FrostedGlassConfig,
    pub font_family: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_color: "#38BDF8".to_string(), // Lens Light Cyan accent
            glass_effect: FrostedGlassConfig::default(),
            font_family: "Inter, system-ui, sans-serif".to_string(),
        }
    }
}

/// Central configuration settings for the LensAI application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISettings {
    pub default_model: AIModel,
    pub temperature: f32,
    pub max_tokens: usize,
    pub system_prompt: String,
    pub auto_save_history: bool,
    pub assistant_mode_enabled: bool,
    pub theme: ThemeSettings,
    pub sync_with_lens_os: bool,
    pub auto_summarize_clipboard: bool,
}

impl Default for AISettings {
    fn default() -> Self {
        Self {
            default_model: AIModel::GeminiFlash,
            temperature: 0.7,
            max_tokens: 4096,
            system_prompt: "You are LensAI, the built-in intelligent assistant for LensOS.".to_string(),
            auto_save_history: true,
            assistant_mode_enabled: true,
            theme: ThemeSettings::default(),
            sync_with_lens_os: true,
            auto_summarize_clipboard: false,
        }
    }
}

impl AISettings {
    pub fn save_to_string(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize AISettings: {}", e))
    }

    pub fn load_from_string(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to deserialize AISettings: {}", e))
    }

    pub fn sync_to_lens_os_desktop(&self) -> Result<(), String> {
        if !self.sync_with_lens_os {
            return Ok(());
        }
        // Simulates deep synchronization with LensOS desktop settings daemon
        Ok(())
    }

    pub fn update_accent_color(&mut self, color: impl Into<String>) {
        self.theme.accent_color = color.into();
    }

    pub fn toggle_dark_mode(&mut self) {
        self.theme.dark_mode = !self.theme.dark_mode;
    }
}
