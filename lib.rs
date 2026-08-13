//! # LensOS v0.1 Installer Module (`installer`)
//!
//! ## Architectural Overview
//! The `installer` crate serves as the core package management, application lifecycle,
//! add-on orchestration, system recovery, repair, and USB installation media provisioning engine
//! for **LensOS v0.1**.
//!
//! ### Integration with LensOS Subsystems
//! The installer interacts directly with key system modules:
//! - **boot**: Handles bootloader entry registration for installer & recovery modes.
//! - **kernel**: Registers system drivers, kernel modules, and sandbox security policies.
//! - **desktop**: Updates desktop app shortcuts, launcher menus, and window manager bindings.
//! - **ui**: Streams installation progress events, modal notifications, and theme changes.
//! - **system**: Manages system services, daemon hooks, and resource allocation.
//! - **files**: Manages `/apps`, `/addons`, `/themes`, and `/recovery` filesystem paths.
//! - **settings**: Synchronizes localized language packs, active desktop themes, and store preferences.
//! - **browser**: Integrates browser extensions and web application manifests.
//! - **lens_ai**: Coordinates AI model add-ons, agent tools, and smart capability plugins.
//!
//! ### Submodule Matrix
//! - [`package`]: `.lens` package definitions, binary manifests, dependencies, and payloads.
//! - [`installer`]: `InstallerManager` core orchestrator controlling installation workflows.
//! - [`addons`]: Standalone and application-tied add-on management (plugins, AI tools, widgets).
//! - [`themes`]: LensOS theme installation, styling attributes, and visual assets.
//! - [`language_packs`]: Localization, translations, IME support, and font bindings.
//! - [`usb`]: Bootable USB installer media creation, formatting, and disk flashing.
//! - [`recovery`]: Point-in-time system snapshot creation and disaster recovery restoration.
//! - [`repair`]: System file integrity scanning, dependency fixing, and component repairs.
//! - [`verification`]: Cryptographic signature validation, SHA-256 integrity, and sandbox policy checks.
//! - [`progress`]: Real-time task progress tracking, stage notifications, and UI callbacks.
//! - [`integration`]: Direct integration with Lens Store and LensOS kernel/desktop service bridges.

pub mod addons;
pub mod installer;
pub mod integration;
pub mod language_packs;
pub mod package;
pub mod progress;
pub mod recovery;
pub mod repair;
pub mod themes;
pub mod usb;
pub mod verification;

// Re-exports for convenient top-level access
pub use addons::{Addon, AddonCategory, AddonInfo, AddonManager, AddonStatus};
pub use installer::{InstallerConfig, InstallerError, InstallerManager};
pub use integration::{LensStoreIntegration, StorePackageInfo, SubsystemBridges};
pub use language_packs::{LanguagePack, LanguagePackInfo, LanguagePackManager, LocaleInfo};
pub use package::{LensPackage, PackageManifest, PackageState, PackageType};
pub use progress::{ProgressStage, ProgressState, ProgressTracker, TaskStatus};
pub use recovery::{RecoveryEngine, RecoveryReport, RecoverySnapshot, RecoverySnapshotInfo};
pub use repair::{RepairEngine, RepairFlags, RepairReport, SystemIntegrityReport};
pub use themes::{LensTheme, LensThemeInfo, ThemeManager};
pub use usb::{UsbDriveInfo, UsbMediaWriter, UsbOperationResult};
pub use verification::{
    CompatibilityResult, PackageVerificationEngine, SecurityResult, VerificationReport,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_crate_initialization() {
        let config = InstallerConfig::default();
        let manager = InstallerManager::new(config);
        assert!(!manager.is_busy());
    }
}
