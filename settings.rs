use crate::accounts::AccountSettings;
use crate::ai::AiSettings;
use crate::appearance::AppearanceSettings;
use crate::network::NetworkSettings;
use crate::security::SecuritySettings;
use crate::system_info::SystemInfo;
use crate::theme::ThemeManager;
use crate::updates::UpdateSettings;
use crate::wallpaper::WallpaperSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsTab {
    Appearance,
    Wallpaper,
    Theme,
    Accounts,
    Security,
    Network,
    Ai,
    Updates,
    SystemInfo,
}

impl SettingsTab {
    pub fn display_name(&self) -> &'static str {
        match self {
            SettingsTab::Appearance => "Appearance & Glass",
            SettingsTab::Wallpaper => "Wallpaper & Display",
            SettingsTab::Theme => "Theme Studio",
            SettingsTab::Accounts => "User Accounts",
            SettingsTab::Security => "Security & Privacy",
            SettingsTab::Network => "Network & VPN",
            SettingsTab::Ai => "Lens AI Engine",
            SettingsTab::Updates => "System Updates",
            SettingsTab::SystemInfo => "About LensOS",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            SettingsTab::Appearance => "sparkles",
            SettingsTab::Wallpaper => "image",
            SettingsTab::Theme => "palette",
            SettingsTab::Accounts => "users",
            SettingsTab::Security => "shield-check",
            SettingsTab::Network => "wifi",
            SettingsTab::Ai => "cpu",
            SettingsTab::Updates => "refresh-cw",
            SettingsTab::SystemInfo => "info",
        }
    }
}

/// Consolidated settings configuration for the entire LensOS environment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SettingsConfig {
    pub appearance: AppearanceSettings,
    pub wallpaper: WallpaperSettings,
    pub theme_manager: ThemeManager,
    pub accounts: AccountSettings,
    pub security: SecuritySettings,
    pub network: NetworkSettings,
    pub ai: AiSettings,
    pub updates: UpdateSettings,
    pub system_info: SystemInfo,
}

/// IPC Message types dispatched to LensOS Desktop compositor and Linux Kernel integration layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KernelIpcMessage {
    ApplyAppearanceConfig { dark_mode: bool, blur_radius_px: u32 },
    SetWallpaperPath { path: String },
    ApplyThemeId { theme_id: String },
    SyncAccountProfile { username: String },
    UpdateSecurityPolicy { firewall_enabled: bool },
    ToggleWifiState { enabled: bool },
    ReconfigureAiEngine { provider_name: String },
    TriggerSystemUpdateCheck,
}

/// Main LensOS Settings Application manager.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SettingsApp {
    pub config: SettingsConfig,
    pub active_tab: SettingsTab,
    pub search_query: String,
    pub unsaved_changes: bool,
    pub status_message: Option<String>,
}

impl Default for SettingsApp {
    fn default() -> Self {
        Self {
            config: SettingsConfig::default(),
            active_tab: SettingsTab::Appearance,
            search_query: String::new(),
            unsaved_changes: false,
            status_message: None,
        }
    }
}

impl SettingsApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse settings JSON: {}", e))
    }

    pub fn export_to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))
    }

    pub fn select_tab(&mut self, tab: SettingsTab) {
        self.active_tab = tab;
        self.status_message = Some(format!("Viewing {}", tab.display_name()));
    }

    pub fn search(&mut self, query: &str) -> Vec<SettingsTab> {
        self.search_query = query.trim().to_lowercase();
        if self.search_query.is_empty() {
            return vec![
                SettingsTab::Appearance,
                SettingsTab::Wallpaper,
                SettingsTab::Theme,
                SettingsTab::Accounts,
                SettingsTab::Security,
                SettingsTab::Network,
                SettingsTab::Ai,
                SettingsTab::Updates,
                SettingsTab::SystemInfo,
            ];
        }

        let q = &self.search_query;
        let mut results = Vec::new();

        if "appearance dark mode glass blur font opacity".contains(q) {
            results.push(SettingsTab::Appearance);
        }
        if "wallpaper background desktop image dynamic fill".contains(q) {
            results.push(SettingsTab::Wallpaper);
        }
        if "theme color accent cyan purple dark light palette".contains(q) {
            results.push(SettingsTab::Theme);
        }
        if "user account admin password profile avatar login guest".contains(q) {
            results.push(SettingsTab::Accounts);
        }
        if "security firewall luks encryption password biometric fingerprint permissions".contains(q) {
            results.push(SettingsTab::Security);
        }
        if "network wifi ethernet vpn dns ip address hotspot".contains(q) {
            results.push(SettingsTab::Network);
        }
        if "ai gemini assistant model prompt wake word privacy tokens".contains(q) {
            results.push(SettingsTab::Ai);
        }
        if "update channel beta stable patch download check restart".contains(q) {
            results.push(SettingsTab::Updates);
        }
        if "system info os version cpu ram disk uptime kernel device".contains(q) {
            results.push(SettingsTab::SystemInfo);
        }

        results
    }

    pub fn mark_changed(&mut self) {
        self.unsaved_changes = true;
        self.status_message = Some("Unsaved settings changes".to_string());
    }

    pub fn reset_to_defaults(&mut self) {
        self.config = SettingsConfig::default();
        self.unsaved_changes = false;
        self.status_message = Some("Reset all settings to LensOS factory defaults".to_string());
    }

    pub fn apply_changes(&mut self) -> Vec<KernelIpcMessage> {
        self.unsaved_changes = false;
        self.status_message = Some("Settings applied to LensOS Desktop & Kernel".to_string());

        vec![
            KernelIpcMessage::ApplyAppearanceConfig {
                dark_mode: self.config.appearance.dark_mode,
                blur_radius_px: self.config.appearance.blur_radius_px,
            },
            KernelIpcMessage::SetWallpaperPath {
                path: self.config.wallpaper.current_path.clone(),
            },
            KernelIpcMessage::ApplyThemeId {
                theme_id: self.config.theme_manager.active_theme_id.clone(),
            },
            KernelIpcMessage::SyncAccountProfile {
                username: self.config.accounts.current_user.username.clone(),
            },
            KernelIpcMessage::UpdateSecurityPolicy {
                firewall_enabled: self.config.security.firewall_enabled,
            },
            KernelIpcMessage::ToggleWifiState {
                enabled: self.config.network.wifi_enabled,
            },
            KernelIpcMessage::ReconfigureAiEngine {
                provider_name: format!("{:?}", self.config.ai.model_provider),
            },
        ]
    }

    /// Renders a structured Glass UI specification layout representation.
    pub fn render_glass_ui_spec(&self) -> String {
        format!(
            "LensOS Frosted Glass UI Layout [Tab: {}] - Theme: {} - DarkMode: {} - Accent: {}",
            self.active_tab.display_name(),
            self.config.theme_manager.get_active_theme().name,
            self.config.appearance.dark_mode,
            self.config.appearance.accent_color.to_hex()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_app_new() {
        let app = SettingsApp::new();
        assert_eq!(app.active_tab, SettingsTab::Appearance);
        assert!(!app.unsaved_changes);
    }

    #[test]
    fn test_search() {
        let mut app = SettingsApp::new();
        let res = app.search("wifi");
        assert!(res.contains(&SettingsTab::Network));

        let res_ai = app.search("gemini");
        assert!(res_ai.contains(&SettingsTab::Ai));
    }

    #[test]
    fn test_json_roundtrip() {
        let app = SettingsApp::new();
        let json = app.export_to_json().unwrap();
        let loaded = SettingsApp::load_from_json(&json).unwrap();
        assert_eq!(app.config.appearance.dark_mode, loaded.config.appearance.dark_mode);
    }

    #[test]
    fn test_apply_changes_ipc() {
        let mut app = SettingsApp::new();
        app.mark_changed();
        let ipc_messages = app.apply_changes();
        assert!(!app.unsaved_changes);
        assert!(!ipc_messages.is_empty());
    }
}
