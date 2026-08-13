//! LensOS v0.1 - Application Permissions Manager
//!
//! Enforces granular capability security controls (Camera, Microphone, Storage, Network, AI Access)
//! for system applications and third-party modules running on LensOS.

use std::collections::HashMap;

/// Types of system capability permissions available in LensOS v0.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionType {
    /// Access to physical webcams and video input feeds
    Camera,
    /// Access to audio microphones
    Microphone,
    /// Permission to read/write files in storage
    FileSystemAccess,
    /// Network socket connection and outbound Internet access
    Network,
    /// Privilege to query local AI models via `lens_ai` engine
    AIModelAccess,
    /// Administrative OS parameters / power management rights
    SystemControl,
    /// Permission to post desktop toast notifications
    Notifications,
    /// Inter-process communication across LensOS modules
    IPCCommunication,
}

/// Status of a permission request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStatus {
    /// Explicitly allowed by user or system policy
    Granted,
    /// Explicitly blocked/denied
    Denied,
    /// Awaiting user consent dialog
    Prompt,
}

impl Default for PermissionStatus {
    fn default() -> Self {
        PermissionStatus::Prompt
    }
}

/// Granular permission entry
#[derive(Debug, Clone)]
pub struct AppPermission {
    pub app_id: String,
    pub permission: PermissionType,
    pub status: PermissionStatus,
    pub last_requested_timestamp: u64,
}

/// Permission Manager maintaining access security enforcement
#[derive(Debug, Default)]
pub struct PermissionManager {
    /// Key: (app_id, PermissionType) -> AppPermission
    permissions: HashMap<(String, PermissionType), AppPermission>,
}

impl PermissionManager {
    /// Creates a new `PermissionManager`
    pub fn new() -> Self {
        let mut manager = Self {
            permissions: HashMap::new(),
        };
        manager.seed_core_module_permissions();
        manager
    }

    /// Pre-grants permissions to built-in system modules (`browser`, `desktop`, `lens_ai`, `files`, `settings`, `ui`)
    fn seed_core_module_permissions(&mut self) {
        let core_modules = ["browser", "desktop", "lens_ai", "files", "settings", "ui", "boot", "kernel"];
        for module in core_modules {
            self.grant_permission(module, PermissionType::AIModelAccess);
            self.grant_permission(module, PermissionType::FileSystemAccess);
            self.grant_permission(module, PermissionType::Notifications);
            self.grant_permission(module, PermissionType::IPCCommunication);
        }
        self.grant_permission("lens_ai", PermissionType::SystemControl);
        self.grant_permission("browser", PermissionType::Network);
    }

    /// Grants a specific capability permission to an application
    pub fn grant_permission(&mut self, app_id: impl Into<String>, permission: PermissionType) {
        let id = app_id.into();
        let entry = AppPermission {
            app_id: id.clone(),
            permission,
            status: PermissionStatus::Granted,
            last_requested_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        self.permissions.insert((id, permission), entry);
    }

    /// Denies a capability permission to an application
    pub fn deny_permission(&mut self, app_id: impl Into<String>, permission: PermissionType) {
        let id = app_id.into();
        let entry = AppPermission {
            app_id: id.clone(),
            permission,
            status: PermissionStatus::Denied,
            last_requested_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        self.permissions.insert((id, permission), entry);
    }

    /// Checks whether an application is authorized for a specific capability
    pub fn check_permission(&self, app_id: &str, permission: PermissionType) -> PermissionStatus {
        if let Some(perm) = self.permissions.get(&(app_id.to_string(), permission)) {
            perm.status
        } else {
            PermissionStatus::Prompt
        }
    }

    /// Helper returning `true` if permission status is `Granted`
    pub fn is_granted(&self, app_id: &str, permission: PermissionType) -> bool {
        self.check_permission(app_id, permission) == PermissionStatus::Granted
    }

    /// Lists all permissions configured for a given app ID
    pub fn list_app_permissions(&self, app_id: &str) -> Vec<&AppPermission> {
        self.permissions
            .values()
            .filter(|p| p.app_id == app_id)
            .collect()
    }
}
