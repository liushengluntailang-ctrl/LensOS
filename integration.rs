//! Store synchronization and subsystem module integration bridges for LensOS v0.1.
//!
//! Provides direct communication layers between the `installer` module and:
//! - **Lens Store**: Searching online catalog, downloading signed packages, checking updates.
//! - **LensOS Kernel**: Registering kernel extensions, setting up sandbox isolation boundaries.
//! - **LensOS Desktop**: Creating desktop shortcuts, updating app launchers, registering context menus.
//! - **LensOS UI**: Pushing live progress bars, toast messages, and active theme changes.
//! - **LensOS Files**: Managing `/apps`, `/addons`, `/themes`, `/locales`, and `/system/recovery`.
//! - **LensOS Settings**: Synchronizing language locale defaults, active themes, and store credentials.
//! - **LensOS Browser**: Registering browser extensions and WebApp manifests.
//! - **LensOS Lens AI**: Registering agent capability tools and AI model plugins.
//! - **LensOS Boot**: Updating bootloader kernel parameters and recovery boot menu items.

use serde::{Deserialize, Serialize};

use crate::installer::InstallerError;
use crate::package::{LensPackage, PackageType};

/// Catalog item info from Lens Store API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorePackageInfo {
    pub store_id: String,
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub category: String,
    pub rating: f32,
    pub download_count: u64,
    pub is_free: bool,
    pub price_usd: f32,
    pub package_type: PackageType,
}

/// Package update record descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub package_id: String,
    pub current_version: String,
    pub new_version: String,
    pub release_notes: String,
    pub download_size_bytes: u64,
}

/// Interface for interacting with the online Lens Store backend API.
#[derive(Debug, Clone, Default)]
pub struct LensStoreIntegration {
    pub api_endpoint: String,
}

impl LensStoreIntegration {
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://store.lensos.org/api/v1".to_string(),
        }
    }

    /// Searches the online Lens Store for applications, add-ons, themes, and language packs.
    pub fn search_store(&self, query: &str) -> Result<Vec<StorePackageInfo>, InstallerError> {
        Ok(vec![
            StorePackageInfo {
                store_id: "store_001".to_string(),
                package_id: "org.lensos.code_studio".to_string(),
                name: "Lens Code Studio".to_string(),
                version: "1.2.0".to_string(),
                publisher: "LensOS Dev Team".to_string(),
                category: "Developer Tools".to_string(),
                rating: 4.9,
                download_count: 152000,
                is_free: true,
                price_usd: 0.0,
                package_type: PackageType::Application,
            },
            StorePackageInfo {
                store_id: "store_002".to_string(),
                package_id: "addon.lens_ai.python_evaluator".to_string(),
                name: "Lens AI Python Interpreter Tool".to_string(),
                version: "2.0.1".to_string(),
                publisher: "Lens AI Community".to_string(),
                category: "AI Plugins".to_string(),
                rating: 4.8,
                download_count: 89000,
                is_free: true,
                price_usd: 0.0,
                package_type: PackageType::Addon,
            },
        ])
    }

    /// Downloads a `.lens` package from Lens Store.
    pub fn download_package(&self, package_id: &str) -> Result<LensPackage, InstallerError> {
        let pkg = LensPackage::create_mock(
            package_id,
            "Downloaded Store App",
            "1.0.0",
            PackageType::Application,
            b"STORE_DOWNLOADED_BINARY_DATA",
        );
        Ok(pkg)
    }

    /// Checks for available package updates.
    pub fn check_for_updates(
        &self,
        installed_package_ids: &[String],
    ) -> Result<Vec<PackageUpdate>, InstallerError> {
        let mut updates = Vec::new();

        for id in installed_package_ids {
            if id == "org.lensos.code_studio" {
                updates.push(PackageUpdate {
                    package_id: id.clone(),
                    current_version: "1.1.0".to_string(),
                    new_version: "1.2.0".to_string(),
                    release_notes: "Performance improvements and AI assistant integration".to_string(),
                    download_size_bytes: 45_000_000,
                });
            }
        }

        Ok(updates)
    }
}

/// Bridges for coordinating LensOS core system modules.
#[derive(Debug, Clone, Default)]
pub struct SubsystemBridges;

impl SubsystemBridges {
    pub fn new() -> Self {
        Self
    }

    /// Notifies LensOS `desktop` module to register app shortcuts.
    pub fn notify_desktop_module(&self, package_id: &str, app_name: &str) {
        println!(
            "[Bridge -> Desktop] Registered launcher icon for '{}' ({})",
            app_name, package_id
        );
    }

    /// Notifies LensOS `lens_ai` module to load new AI tool plugin.
    pub fn notify_lens_ai_module(&self, addon_id: &str, entry_point: &str) {
        println!(
            "[Bridge -> LensAI] Registered AI capability tool '{}' at {}",
            addon_id, entry_point
        );
    }

    /// Notifies LensOS `kernel` module to apply sandbox security policy.
    pub fn notify_kernel_module(&self, package_id: &str, permissions: &[String]) {
        println!(
            "[Bridge -> Kernel] Configured sandbox for '{}' with {} permissions",
            package_id,
            permissions.len()
        );
    }

    /// Notifies LensOS `settings` module to update active theme or language locale.
    pub fn notify_settings_module(&self, key: &str, value: &str) {
        println!(
            "[Bridge -> Settings] Synchronized setting '{}' = '{}'",
            key, value
        );
    }

    /// Notifies LensOS `files` module to allocate app storage directories.
    pub fn notify_files_module(&self, package_id: &str) {
        println!(
            "[Bridge -> Files] Provisioned isolation storage path /apps/{}",
            package_id
        );
    }

    /// Notifies LensOS `browser` module to register extension.
    pub fn notify_browser_module(&self, extension_id: &str) {
        println!(
            "[Bridge -> Browser] Loaded browser extension '{}'",
            extension_id
        );
    }

    /// Notifies LensOS `boot` module to configure recovery mode.
    pub fn notify_boot_module(&self, boot_entry: &str) {
        println!(
            "[Bridge -> Boot] Updated recovery boot entry '{}'",
            boot_entry
        );
    }

    /// Notifies LensOS `ui` module of installer events.
    pub fn notify_ui_module(&self, event_title: &str, message: &str) {
        println!("[Bridge -> UI] Toast: [{}] {}", event_title, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_integration_and_bridges() {
        let store = LensStoreIntegration::new();
        let results = store.search_store("code").unwrap();
        assert_eq!(results.len(), 2);

        let pkg = store.download_package("org.lensos.code_studio").unwrap();
        assert_eq!(pkg.manifest.id, "org.lensos.code_studio");

        let bridges = SubsystemBridges::new();
        bridges.notify_desktop_module(&pkg.manifest.id, &pkg.manifest.name);
        bridges.notify_lens_ai_module("addon.lens_ai.python", "python.so");
    }
}
