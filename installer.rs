//! Core `InstallerManager` coordinator for LensOS v0.1.
//!
//! Orchestrates `.lens` package installation, standalone add-on management,
//! theme styling installation, language pack localization, bootable USB flashing,
//! system snapshot recovery, integrity repairs, and Lens Store synchronization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::addons::{Addon, AddonInfo, AddonManager, AddonStatus};
use crate::integration::{LensStoreIntegration, StorePackageInfo, SubsystemBridges};
use crate::language_packs::{LanguagePack, LanguagePackInfo, LanguagePackManager};
use crate::package::{LensPackage, PackageState, PackageType};
use crate::progress::{ProgressTracker, TaskStatus};
use crate::recovery::{RecoveryEngine, RecoveryReport, RecoverySnapshotInfo};
use crate::repair::{RepairEngine, RepairFlags, RepairReport, SystemIntegrityReport};
use crate::themes::{LensTheme, LensThemeInfo, ThemeManager};
use crate::usb::{UsbDriveInfo, UsbMediaWriter, UsbOperationResult};
use crate::verification::{PackageVerificationEngine, VerificationReport};

/// System errors emitted during installer operations.
#[derive(Debug, Error)]
pub enum InstallerError {
    #[error("Package verification failed: {0}")]
    VerificationFailed(String),

    #[error("Invalid or malformed package: {0}")]
    InvalidPackage(String),

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Dependency unsatisfied for package '{0}': missing '{1}'")]
    MissingDependency(String, String),

    #[error("USB installation media error: {0}")]
    UsbError(String),

    #[error("Recovery error: {0}")]
    RecoveryError(String),

    #[error("Repair operation error: {0}")]
    RepairError(String),

    #[error("Lens Store communication error: {0}")]
    StoreError(String),

    #[error("Installer operation failed: {0}")]
    OperationFailed(String),

    #[error("Task cancelled by user")]
    Cancelled,
}

/// Configuration settings for the LensOS Installer engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerConfig {
    pub apps_install_dir: String,
    pub addons_dir: String,
    pub themes_dir: String,
    pub locales_dir: String,
    pub recovery_dir: String,
    pub auto_verify_signatures: bool,
    pub enforce_sandbox_permissions: bool,
    pub max_parallel_downloads: usize,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            apps_install_dir: "/apps".to_string(),
            addons_dir: "/addons".to_string(),
            themes_dir: "/themes".to_string(),
            locales_dir: "/locales".to_string(),
            recovery_dir: "/system/recovery".to_string(),
            auto_verify_signatures: true,
            enforce_sandbox_permissions: true,
            max_parallel_downloads: 4,
        }
    }
}

/// Metadata record for an installed software package in LensOS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackageInfo {
    pub package_id: String,
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
    pub install_path: String,
    pub installed_at_timestamp: u64,
    pub state: PackageState,
}

/// Core installer manager orchestrating LensOS software installation, system recovery,
/// repair, and Store synchronization.
#[derive(Debug)]
pub struct InstallerManager {
    config: InstallerConfig,
    installed_packages: HashMap<String, InstalledPackageInfo>,
    active_tasks: HashMap<String, ProgressTracker>,
    addon_manager: AddonManager,
    theme_manager: ThemeManager,
    language_manager: LanguagePackManager,
    usb_writer: UsbMediaWriter,
    recovery_engine: RecoveryEngine,
    repair_engine: RepairEngine,
    verification_engine: PackageVerificationEngine,
    store_integration: LensStoreIntegration,
    subsystem_bridges: SubsystemBridges,
    is_busy: bool,
}

impl InstallerManager {
    /// Initializes a new `InstallerManager` with default or custom configuration.
    pub fn new(config: InstallerConfig) -> Self {
        Self {
            config,
            installed_packages: HashMap::new(),
            active_tasks: HashMap::new(),
            addon_manager: AddonManager::new(),
            theme_manager: ThemeManager::new(),
            language_manager: LanguagePackManager::new(),
            usb_writer: UsbMediaWriter::new(),
            recovery_engine: RecoveryEngine::new(),
            repair_engine: RepairEngine::new(),
            verification_engine: PackageVerificationEngine::new(),
            store_integration: LensStoreIntegration::new(),
            subsystem_bridges: SubsystemBridges::new(),
            is_busy: false,
        }
    }

    /// Checks if installer is currently executing an asynchronous workflow.
    pub fn is_busy(&self) -> bool {
        self.is_busy
    }

    // =========================================================================
    // 1. APPLICATION & PACKAGE INSTALLATION (.lens packages)
    // =========================================================================

    /// Verifies and installs a `.lens` package into LensOS.
    pub fn install_package(
        &mut self,
        package: &LensPackage,
    ) -> Result<InstalledPackageInfo, InstallerError> {
        let task_id = format!("install_{}", package.manifest.id);
        let mut progress = ProgressTracker::new(&task_id, format!("Installing {}", package.manifest.name));

        // Step 1: Verification
        progress.set_stage("verify", "Verifying package signature, checksum, and sandbox policy...");
        progress.update(10.0);

        let v_report = self.verification_engine.verify_package(package);
        if !v_report.is_verified && self.config.auto_verify_signatures {
            progress.fail("Package verification failed security policy");
            return Err(InstallerError::VerificationFailed(format!(
                "Signature or checksum invalid for {}",
                package.manifest.id
            )));
        }

        // Step 2: Payload Extraction & File Allocation
        progress.set_stage("extract", "Extracting binary payload and resources to /apps...");
        progress.update(45.0);

        let install_path = format!("{}/{}", self.config.apps_install_dir, package.manifest.id);

        // Step 3: Subsystem Integration Notifications
        progress.set_stage("link", "Registering desktop shortcuts, kernel sandbox, and UI assets...");
        progress.update(80.0);

        self.subsystem_bridges.notify_files_module(&package.manifest.id);
        self.subsystem_bridges.notify_kernel_module(&package.manifest.id, &package.manifest.permissions);
        self.subsystem_bridges.notify_desktop_module(&package.manifest.id, &package.manifest.name);

        if package.targets_subsystem("lens_ai") {
            self.subsystem_bridges.notify_lens_ai_module(&package.manifest.id, &package.manifest.main_entry);
        }

        if package.targets_subsystem("browser") {
            self.subsystem_bridges.notify_browser_module(&package.manifest.id);
        }

        // Complete Progress
        progress.complete("Package installed successfully");
        self.active_tasks.insert(task_id, progress);

        let info = InstalledPackageInfo {
            package_id: package.manifest.id.clone(),
            name: package.manifest.name.clone(),
            version: package.manifest.version.clone(),
            package_type: package.manifest.package_type.clone(),
            install_path,
            installed_at_timestamp: 1776002000,
            state: PackageState::Installed,
        };

        self.installed_packages.insert(package.manifest.id.clone(), info.clone());
        self.subsystem_bridges.notify_ui_module("Package Installed", &format!("{} is ready to use", info.name));

        Ok(info)
    }

    /// Uninstalls an installed application or package.
    pub fn uninstall_package(&mut self, package_id: &str) -> Result<(), InstallerError> {
        if let Some(info) = self.installed_packages.remove(package_id) {
            self.subsystem_bridges.notify_ui_module("Package Removed", &format!("Uninstalled {}", info.name));
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(package_id.to_string()))
        }
    }

    /// Lists all installed `.lens` packages.
    pub fn list_installed_packages(&self) -> Vec<InstalledPackageInfo> {
        self.installed_packages.values().cloned().collect()
    }

    // =========================================================================
    // 2. ADD-ON MANAGEMENT (Standalone and App-bound)
    // =========================================================================

    /// Installs an add-on (can be standalone or tied to an application).
    pub fn install_addon(&mut self, addon: Addon) -> Result<AddonStatus, InstallerError> {
        if addon.is_standalone() {
            println!(
                "[InstallerManager] Installing STANDALONE add-on: {} ({})",
                addon.name, addon.id
            );
        } else {
            println!(
                "[InstallerManager] Installing APP-BOUND add-on for parent app '{:?}': {}",
                addon.parent_app_id, addon.name
            );
        }

        if let Some(ref parent) = addon.parent_app_id {
            if !self.installed_packages.contains_key(parent) {
                return Err(InstallerError::MissingDependency(
                    addon.id.clone(),
                    parent.clone(),
                ));
            }
        }

        let status = self.addon_manager.install_addon(addon.clone())?;

        // Notify Lens AI subsystem if this is an AI capability plugin
        if addon.category == crate::addons::AddonCategory::AiCapabilityPlugin {
            self.subsystem_bridges.notify_lens_ai_module(&addon.id, &addon.entry_point);
        }

        self.subsystem_bridges.notify_ui_module("Add-on Installed", &format!("Installed add-on {}", addon.name));
        Ok(status)
    }

    /// Uninstalls an add-on by ID.
    pub fn uninstall_addon(&mut self, addon_id: &str) -> Result<(), InstallerError> {
        self.addon_manager.uninstall_addon(addon_id)
    }

    /// Lists installed add-ons.
    pub fn list_addons(&self) -> Vec<AddonInfo> {
        self.addon_manager.list_all()
    }

    /// Lists standalone add-ons installed separately from applications.
    pub fn list_standalone_addons(&self) -> Vec<AddonInfo> {
        self.addon_manager.list_standalone()
    }

    // =========================================================================
    // 3. THEME INSTALLATION & MANAGMENT
    // =========================================================================

    /// Installs a new LensOS theme package.
    pub fn install_theme(&mut self, theme: LensTheme) -> Result<String, InstallerError> {
        let name = theme.name.clone();
        let id = self.theme_manager.install_theme(theme)?;
        self.subsystem_bridges.notify_ui_module("Theme Installed", &format!("Installed theme {}", name));
        Ok(id)
    }

    /// Applies an installed theme as the active system desktop theme.
    pub fn apply_theme(&mut self, theme_id: &str) -> Result<(), InstallerError> {
        self.theme_manager.apply_theme(theme_id)?;
        self.subsystem_bridges.notify_settings_module("desktop.active_theme", theme_id);
        self.subsystem_bridges.notify_ui_module("Theme Applied", &format!("Switched theme to {}", theme_id));
        Ok(())
    }

    /// Lists installed themes.
    pub fn list_themes(&self) -> Vec<LensThemeInfo> {
        self.theme_manager.list_installed()
    }

    // =========================================================================
    // 4. LANGUAGE PACK LOCALIZATION
    // =========================================================================

    /// Installs a localization language pack.
    pub fn install_language_pack(&mut self, pack: LanguagePack) -> Result<String, InstallerError> {
        let locale_name = pack.locale.name_english.clone();
        let code = self.language_manager.install_pack(pack)?;
        self.subsystem_bridges.notify_ui_module("Language Pack Installed", &format!("Added {}", locale_name));
        Ok(code)
    }

    /// Activates system language locale.
    pub fn activate_language(&mut self, locale_code: &str) -> Result<(), InstallerError> {
        self.language_manager.activate_language(locale_code)?;
        self.subsystem_bridges.notify_settings_module("system.locale", locale_code);
        Ok(())
    }

    /// Lists installed language packs.
    pub fn list_language_packs(&self) -> Vec<LanguagePackInfo> {
        self.language_manager.list_installed()
    }

    // =========================================================================
    // 5. USB INSTALLATION MEDIA CREATION
    // =========================================================================

    /// Detects connected removable storage drives.
    pub fn detect_usb_drives(&self) -> Result<Vec<UsbDriveInfo>, InstallerError> {
        self.usb_writer.detect_drives()
    }

    /// Writes LensOS bootable installation ISO image to a USB drive.
    pub fn create_bootable_usb(
        &mut self,
        drive_path: &str,
        iso_image_path: &str,
    ) -> Result<UsbOperationResult, InstallerError> {
        let task_id = format!("usb_{}", drive_path.replace('/', "_"));
        let mut progress = ProgressTracker::new(&task_id, "Creating Bootable USB");

        let result = self.usb_writer.create_bootable_media(drive_path, iso_image_path, &mut progress)?;
        self.active_tasks.insert(task_id, progress);
        Ok(result)
    }

    // =========================================================================
    // 6. SYSTEM RECOVERY & RESTORE
    // =========================================================================

    /// Creates a system recovery snapshot.
    pub fn create_recovery_snapshot(&mut self, description: &str) -> Result<String, InstallerError> {
        let installed_map = self
            .installed_packages
            .iter()
            .map(|(k, v)| (k.clone(), v.version.clone()))
            .collect();

        let active_addons = self
            .addon_manager
            .list_all()
            .into_iter()
            .map(|a| a.id)
            .collect();

        let snapshot = self.recovery_engine.create_snapshot(description, installed_map, active_addons)?;
        self.subsystem_bridges.notify_boot_module(&snapshot.snapshot_id);
        Ok(snapshot.snapshot_id)
    }

    /// Restores system state to a recovery snapshot.
    pub fn restore_system_recovery(&mut self, snapshot_id: &str) -> Result<RecoveryReport, InstallerError> {
        let mut progress = ProgressTracker::new(format!("recovery_{}", snapshot_id), "System Recovery");
        let report = self.recovery_engine.restore_snapshot(snapshot_id, &mut progress)?;
        Ok(report)
    }

    /// Lists available recovery snapshots.
    pub fn list_recovery_snapshots(&self) -> Vec<RecoverySnapshotInfo> {
        self.recovery_engine.list_snapshots()
    }

    // =========================================================================
    // 7. SYSTEM REPAIR
    // =========================================================================

    /// Scans LensOS core files and dependencies for integrity errors.
    pub fn scan_system_integrity(&self) -> Result<SystemIntegrityReport, InstallerError> {
        let flags = RepairFlags::default();
        self.repair_engine.scan_system_integrity(&flags)
    }

    /// Repairs corrupted packages and broken subsystem dependencies.
    pub fn run_system_repair(&mut self) -> Result<RepairReport, InstallerError> {
        let flags = RepairFlags::default();
        let scan = self.repair_engine.scan_system_integrity(&flags)?;

        let mut progress = ProgressTracker::new("repair_task", "System Integrity Repair");
        let report = self.repair_engine.execute_repair(&scan, &mut progress)?;
        self.subsystem_bridges.notify_ui_module("Repair Finished", "System files and packages repaired");
        Ok(report)
    }

    // =========================================================================
    // 8. PACKAGE VERIFICATION
    // =========================================================================

    /// Verifies package signature, checksum, security sandbox, and architecture compatibility.
    pub fn verify_package(&self, package: &LensPackage) -> VerificationReport {
        self.verification_engine.verify_package(package)
    }

    // =========================================================================
    // 9. LENS STORE INTEGRATION
    // =========================================================================

    /// Searches online Lens Store catalog.
    pub fn search_lens_store(&self, query: &str) -> Result<Vec<StorePackageInfo>, InstallerError> {
        self.store_integration.search_store(query)
    }

    /// Synchronizes installed packages with Lens Store and installs available updates.
    pub fn sync_and_update(&mut self) -> Result<usize, InstallerError> {
        let installed_ids: Vec<String> = self.installed_packages.keys().cloned().collect();
        let updates = self.store_integration.check_for_updates(&installed_ids)?;

        let count = updates.len();
        for update in updates {
            let pkg = self.store_integration.download_package(&update.package_id)?;
            self.install_package(&pkg)?;
        }

        Ok(count)
    }

    // =========================================================================
    // 10. PROGRESS & TASK TRACKING
    // =========================================================================

    /// Gets task status by task ID.
    pub fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        self.active_tasks.get(task_id).map(|t| t.state.status.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::addons::AddonCategory;

    #[test]
    fn test_installer_manager_full_workflow() {
        let config = InstallerConfig::default();
        let mut manager = InstallerManager::new(config);

        // 1. Package Installation
        let pkg = LensPackage::create_mock(
            "org.lensos.browser",
            "Lens Browser",
            "1.0.0",
            PackageType::Application,
            b"BROWSER_PAYLOAD",
        );

        let info = manager.install_package(&pkg).unwrap();
        assert_eq!(info.package_id, "org.lensos.browser");
        assert_eq!(manager.list_installed_packages().len(), 1);

        // 2. Add-on Installation (Standalone & App-bound)
        let standalone_addon = Addon::new_standalone(
            "addon.lens_ai.voice",
            "Lens AI Voice Tool",
            "1.0.0",
            AddonCategory::AiCapabilityPlugin,
            "voice.so",
        );
        assert!(manager.install_addon(standalone_addon).is_ok());

        let bound_addon = Addon::new_bound_to_app(
            "addon.browser.adblock",
            "AdBlocker Extension",
            "1.0.0",
            AddonCategory::BrowserExtension,
            "org.lensos.browser",
            "adblock.js",
        );
        assert!(manager.install_addon(bound_addon).is_ok());

        assert_eq!(manager.list_addons().len(), 2);
        assert_eq!(manager.list_standalone_addons().len(), 1);

        // 3. Recovery Snapshot
        let snap_id = manager.create_recovery_snapshot("Initial System Setup").unwrap();
        assert_eq!(snap_id, "snap_1");

        // 4. System Repair
        let repair_report = manager.run_system_repair().unwrap();
        assert!(repair_report.is_successful);
    }
}
