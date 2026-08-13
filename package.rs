//! Package specification and binary representation for LensOS `.lens` packages.
//!
//! `.lens` packages are the universal software archive format for LensOS v0.1.
//! They contain a signed JSON manifest, binary payload (executables/assets),
//! metadata for desktop/UI integration, dependency declarations, and sandbox permissions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of packages supported by the LensOS Installer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageType {
    /// Full standalone application executable and UI resources.
    Application,
    /// Standalone or app-bound add-on, plugin, or extension.
    Addon,
    /// Desktop styling, visual attributes, glassmorphism parameters, and wallpapers.
    Theme,
    /// Localization files, locale maps, IME tools, and font families.
    LanguagePack,
    /// Core system service or background daemon module.
    SystemComponent,
    /// Hardware interface driver or kernel extension module.
    Driver,
}

/// Lifecycle state of a package within LensOS storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageState {
    NotInstalled,
    PendingInstall,
    Installing,
    Installed,
    UpdateAvailable,
    Corrupted,
    Disabled,
}

/// Manifest structure embedded within every `.lens` package header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Unique identifier in reverse-domain notation (e.g. "org.lensos.editor").
    pub id: String,
    /// Human-readable package display name.
    pub name: String,
    /// Semantic version string (e.g. "1.2.0").
    pub version: String,
    /// Publisher or developer organization name.
    pub publisher: String,
    /// Detailed package description.
    pub description: String,
    /// Type of package payload.
    pub package_type: PackageType,
    /// Minimum LensOS kernel version required (e.g. "0.1.0").
    pub min_os_version: String,
    /// CPU Architecture target (e.g., "x86_64", "aarch64", "universal").
    pub architecture: String,
    /// List of package IDs required as pre-requisites.
    pub dependencies: Vec<String>,
    /// Target LensOS modules this package interacts with (e.g., "desktop", "ui", "lens_ai").
    pub target_subsystems: Vec<String>,
    /// List of required system security permissions (e.g., "network", "microphone", "filesystem:read_user").
    pub permissions: Vec<String>,
    /// Relative path within the package payload to the main entry binary or asset file.
    pub main_entry: String,
    /// Relative path to icon asset.
    pub icon_path: Option<String>,
    /// Environment variables declared by the package.
    pub env_variables: HashMap<String, String>,
}

impl PackageManifest {
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            publisher: "LensOS Software Community".to_string(),
            description: "Default package manifest".to_string(),
            package_type: PackageType::Application,
            min_os_version: "0.1.0".to_string(),
            architecture: "x86_64".to_string(),
            dependencies: Vec::new(),
            target_subsystems: vec!["desktop".to_string(), "ui".to_string()],
            permissions: Vec::new(),
            main_entry: "bin/main".to_string(),
            icon_path: Some("assets/icon.png".to_string()),
            env_variables: HashMap::new(),
        }
    }

    /// Validates basic syntactic sanity of the manifest fields.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("Package ID cannot be empty".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("Package name cannot be empty".to_string());
        }
        if self.version.trim().is_empty() {
            return Err("Package version cannot be empty".to_string());
        }
        if self.main_entry.trim().is_empty() {
            return Err("Package main entry point cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Represents a loaded `.lens` archive binary package in memory or disk descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensPackage {
    /// Embedded manifest.
    pub manifest: PackageManifest,
    /// SHA-256 payload checksum hex string.
    pub payload_checksum: String,
    /// Cryptographic publisher signature.
    pub signature: Vec<u8>,
    /// Raw payload byte content size in bytes.
    pub payload_size_bytes: u64,
    /// Raw payload byte stream (compressed archive or in-memory blob).
    pub raw_payload: Vec<u8>,
    /// Installation metadata timestamp.
    pub created_at_timestamp: u64,
}

impl LensPackage {
    /// Creates a mock `.lens` package for testing or store simulation.
    pub fn create_mock(
        id: &str,
        name: &str,
        version: &str,
        pkg_type: PackageType,
        payload_bytes: &[u8],
    ) -> Self {
        let mut manifest = PackageManifest::new(id, name, version);
        manifest.package_type = pkg_type;

        Self {
            manifest,
            payload_checksum: format!("sha256:{:x}", payload_bytes.len() * 314159),
            signature: vec![0xDE, 0xAD, 0xBE, 0xEF],
            payload_size_bytes: payload_bytes.len() as u64,
            raw_payload: payload_bytes.to_vec(),
            created_at_timestamp: 1776000000,
        }
    }

    /// Verifies if the package payload size matches its declared payload header.
    pub fn check_payload_integrity(&self) -> bool {
        self.raw_payload.len() as u64 == self.payload_size_bytes
    }

    /// Checks if package requires a specific subsystem (e.g. "lens_ai", "browser").
    pub fn targets_subsystem(&self, subsystem: &str) -> bool {
        self.manifest
            .target_subsystems
            .iter()
            .any(|s| s.eq_ignore_ascii_case(subsystem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_creation_and_integrity() {
        let pkg = LensPackage::create_mock(
            "org.lensos.calculator",
            "Lens Calculator",
            "1.0.0",
            PackageType::Application,
            b"LENS_OS_BINARY_CONTENT",
        );

        assert!(pkg.check_payload_integrity());
        assert_eq!(pkg.manifest.id, "org.lensos.calculator");
        assert!(pkg.targets_subsystem("desktop"));
    }
}
