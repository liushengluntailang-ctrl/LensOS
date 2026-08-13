//! Add-on management engine for LensOS v0.1.
//!
//! Add-ons are modular extensions (AI agent tools, browser extensions, desktop widgets,
//! system hooks, and settings panels) that can be installed, enabled, disabled,
//! and managed independently from main applications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::installer::InstallerError;

/// Specific category of add-on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddonCategory {
    /// Plugin extending Lens AI model capabilities or custom tool calling agents.
    AiCapabilityPlugin,
    /// Extension designed for the LensOS web browser module.
    BrowserExtension,
    /// Desktop widget or dock panel applet for the desktop module.
    DesktopWidget,
    /// System file handler or virtual driver filter.
    FileSystemFilter,
    /// Custom configuration page for the settings module.
    SettingsPanel,
    /// Generic system or UI service extension.
    SystemPlugin,
}

/// Status of an installed add-on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddonStatus {
    InstalledActive,
    InstalledDisabled,
    UpdatePending,
    ErrorFaulted(String),
}

/// Representation of an add-on entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    /// Unique identifier (e.g. "addon.lens_ai.code_interpreter").
    pub id: String,
    /// Human readable name.
    pub name: String,
    /// Version string.
    pub version: String,
    /// Publisher/Author.
    pub publisher: String,
    /// Category of the add-on.
    pub category: AddonCategory,
    /// If bound to a parent application ID, this contains Some(parent_app_id).
    /// If None, this is a STANDALONE add-on installable separately from applications.
    pub parent_app_id: Option<String>,
    /// Permissions required by the add-on.
    pub required_permissions: Vec<String>,
    /// Entry point script or dynamic module handle path.
    pub entry_point: String,
    /// Configurable key-value metadata.
    pub config_attributes: HashMap<String, String>,
}

impl Addon {
    /// Constructs a standalone add-on (installable separately from applications).
    pub fn new_standalone(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        category: AddonCategory,
        entry_point: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            publisher: "LensOS Community Publisher".to_string(),
            category,
            parent_app_id: None,
            required_permissions: Vec::new(),
            entry_point: entry_point.into(),
            config_attributes: HashMap::new(),
        }
    }

    /// Constructs an add-on bound to a specific parent application.
    pub fn new_bound_to_app(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        category: AddonCategory,
        parent_app_id: impl Into<String>,
        entry_point: impl Into<String>,
    ) -> Self {
        let mut addon = Self::new_standalone(id, name, version, category, entry_point);
        addon.parent_app_id = Some(parent_app_id.into());
        addon
    }

    /// Checks if this add-on is standalone.
    pub fn is_standalone(&self) -> bool {
        self.parent_app_id.is_none()
    }
}

/// Summary information for an installed add-on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub category: AddonCategory,
    pub parent_app_id: Option<String>,
    pub status: AddonStatus,
    pub is_standalone: bool,
}

/// Dedicated manager for installing and orchestrating add-ons in LensOS.
#[derive(Debug, Clone, Default)]
pub struct AddonManager {
    installed_addons: HashMap<String, (Addon, AddonStatus)>,
}

impl AddonManager {
    pub fn new() -> Self {
        Self {
            installed_addons: HashMap::new(),
        }
    }

    /// Installs an add-on separately from main applications or as a bound plugin.
    pub fn install_addon(&mut self, addon: Addon) -> Result<AddonStatus, InstallerError> {
        if addon.id.trim().is_empty() {
            return Err(InstallerError::InvalidPackage(
                "Addon ID cannot be empty".to_string(),
            ));
        }

        let status = AddonStatus::InstalledActive;
        let id = addon.id.clone();
        self.installed_addons.insert(id, (addon, status.clone()));
        Ok(status)
    }

    /// Uninstalls an add-on by its ID.
    pub fn uninstall_addon(&mut self, addon_id: &str) -> Result<(), InstallerError> {
        if self.installed_addons.remove(addon_id).is_some() {
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(addon_id.to_string()))
        }
    }

    /// Enables an installed add-on.
    pub fn enable_addon(&mut self, addon_id: &str) -> Result<(), InstallerError> {
        if let Some((_, status)) = self.installed_addons.get_mut(addon_id) {
            *status = AddonStatus::InstalledActive;
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(addon_id.to_string()))
        }
    }

    /// Disables an installed add-on without deleting its files.
    pub fn disable_addon(&mut self, addon_id: &str) -> Result<(), InstallerError> {
        if let Some((_, status)) = self.installed_addons.get_mut(addon_id) {
            *status = AddonStatus::InstalledDisabled;
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(addon_id.to_string()))
        }
    }

    /// Returns all installed add-ons.
    pub fn list_all(&self) -> Vec<AddonInfo> {
        self.installed_addons
            .values()
            .map(|(addon, status)| AddonInfo {
                id: addon.id.clone(),
                name: addon.name.clone(),
                version: addon.version.clone(),
                category: addon.category.clone(),
                parent_app_id: addon.parent_app_id.clone(),
                status: status.clone(),
                is_standalone: addon.is_standalone(),
            })
            .collect()
    }

    /// Returns only standalone add-ons installed separately from applications.
    pub fn list_standalone(&self) -> Vec<AddonInfo> {
        self.list_all().into_iter().filter(|a| a.is_standalone).collect()
    }

    /// Gets an installed add-on reference.
    pub fn get_addon(&self, addon_id: &str) -> Option<&Addon> {
        self.installed_addons.get(addon_id).map(|(addon, _)| addon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standalone_addon_installation() {
        let mut manager = AddonManager::new();
        let addon = Addon::new_standalone(
            "addon.lens_ai.summarizer",
            "Lens AI Text Summarizer Tool",
            "1.0.0",
            AddonCategory::AiCapabilityPlugin,
            "lib/summarizer.so",
        );

        assert!(addon.is_standalone());

        let res = manager.install_addon(addon);
        assert!(res.is_ok());

        let standalone_list = manager.list_standalone();
        assert_eq!(standalone_list.len(), 1);
        assert_eq!(standalone_list[0].id, "addon.lens_ai.summarizer");

        // Test disable
        assert!(manager.disable_addon("addon.lens_ai.summarizer").is_ok());
        assert_eq!(
            manager.list_all()[0].status,
            AddonStatus::InstalledDisabled
        );
    }
}
