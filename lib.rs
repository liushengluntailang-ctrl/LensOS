//! # LensOS Settings Crate
//!
//! `lensos-settings` is a modular, high-performance settings architecture for LensOS.
//! It features frosted glass theme management, wallpaper customization, user account management,
//! security hardening, network configuration, local & cloud AI parameters, and kernel IPC messaging.

pub mod accounts;
pub mod ai;
pub mod appearance;
pub mod network;
pub mod security;
pub mod settings;
pub mod system_info;
pub mod theme;
pub mod updates;
pub mod wallpaper;

pub use accounts::{AccountRole, AccountSettings, UserAccount};
pub use ai::{AiPrivacyMode, AiSettings, ModelProvider};
pub use appearance::{AccentColor, AppearanceSettings, WindowBackdropEffect};
pub use network::{NetworkSettings, VpnConfig, VpnProtocol, WifiNetwork};
pub use security::{AppPermission, BiometricType, DiskEncryptionStatus, SecuritySettings};
pub use settings::{KernelIpcMessage, SettingsApp, SettingsConfig, SettingsTab};
pub use system_info::SystemInfo;
pub use theme::{Theme, ThemeManager};
pub use updates::{ReleaseChannel, SystemUpdateInfo, UpdateSettings, UpdateStatus};
pub use wallpaper::{WallpaperFit, WallpaperItem, WallpaperSettings};

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_full_settings_crate_initialization() {
        let app = SettingsApp::new();
        assert_eq!(app.config.system_info.os_name, "LensOS");
        assert!(app.config.security.firewall_enabled);
        assert!(app.config.ai.assistant_enabled);
        assert_eq!(app.config.appearance.accent_color, AccentColor::LensTeal);
    }
}
