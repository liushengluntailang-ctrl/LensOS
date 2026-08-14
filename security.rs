use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiskEncryptionStatus {
    EncryptedLuks,
    EncryptedLensVault,
    Unencrypted,
    EncryptingProgress(u8),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiometricType {
    Fingerprint,
    FaceID,
    SecurityKeyYubi,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppPermission {
    pub app_id: String,
    pub app_name: String,
    pub camera: bool,
    pub microphone: bool,
    pub location: bool,
    pub storage: bool,
    pub network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecuritySettings {
    pub firewall_enabled: bool,
    pub stealth_mode: bool,
    pub block_incoming_pings: bool,
    pub disk_encryption: DiskEncryptionStatus,
    pub screen_lock_timeout_mins: u32,
    pub require_password_after_sleep: bool,
    pub biometric_type: BiometricType,
    pub app_permissions: Vec<AppPermission>,
    pub sandbox_isolation_strict: bool,
    pub kernel_module_signing_required: bool,
    pub automatic_security_patches: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        let sample_app_perms = vec![
            AppPermission {
                app_id: "com.lensos.browser".to_string(),
                app_name: "Lens Browser".to_string(),
                camera: true,
                microphone: true,
                location: true,
                storage: true,
                network: true,
            },
            AppPermission {
                app_id: "com.lensos.terminal".to_string(),
                app_name: "Lens Glass Terminal".to_string(),
                camera: false,
                microphone: false,
                location: false,
                storage: true,
                network: true,
            },
        ];

        Self {
            firewall_enabled: true,
            stealth_mode: true,
            block_incoming_pings: true,
            disk_encryption: DiskEncryptionStatus::EncryptedLensVault,
            screen_lock_timeout_mins: 10,
            require_password_after_sleep: true,
            biometric_type: BiometricType::Fingerprint,
            app_permissions: sample_app_perms,
            sandbox_isolation_strict: true,
            kernel_module_signing_required: true,
            automatic_security_patches: true,
        }
    }
}

impl SecuritySettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_firewall(&mut self, enabled: bool) {
        self.firewall_enabled = enabled;
    }

    pub fn update_app_permission(&mut self, permission: AppPermission) {
        if let Some(pos) = self
            .app_permissions
            .iter()
            .position(|p| p.app_id == permission.app_id)
        {
            self.app_permissions[pos] = permission;
        } else {
            self.app_permissions.push(permission);
        }
    }

    pub fn calculate_security_score(&self) -> u32 {
        let mut score = 0u32;
        if self.firewall_enabled {
            score += 20;
        }
        if self.stealth_mode {
            score += 10;
        }
        if matches!(
            self.disk_encryption,
            DiskEncryptionStatus::EncryptedLuks | DiskEncryptionStatus::EncryptedLensVault
        ) {
            score += 30;
        }
        if self.require_password_after_sleep {
            score += 10;
        }
        if self.sandbox_isolation_strict {
            score += 15;
        }
        if self.kernel_module_signing_required {
            score += 15;
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_defaults() {
        let sec = SecuritySettings::default();
        assert!(sec.firewall_enabled);
        assert_eq!(sec.calculate_security_score(), 100);
    }

    #[test]
    fn test_update_permissions() {
        let mut sec = SecuritySettings::default();
        let updated = AppPermission {
            app_id: "com.lensos.browser".to_string(),
            app_name: "Lens Browser".to_string(),
            camera: false,
            microphone: false,
            location: false,
            storage: true,
            network: true,
        };

        sec.update_app_permission(updated);
        assert!(!sec.app_permissions[0].camera);
    }
}
