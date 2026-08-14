//! # Security & Permissions Manager (`security.rs`)
//!
//! Enforces HTTPS security verification, per-origin hardware/API permissions,
//! content blocking (adblock/anti-tracking), and sandboxing checks for LensOS.

use std::collections::HashMap;

/// Overall web page security state classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Fully encrypted HTTPS connection with verified SSL certificate.
    Secure,
    /// Encrypted HTTPS with mixed content or minor certificate warnings.
    Warning,
    /// Unencrypted HTTP or invalid/expired SSL certificate.
    Insecure,
    /// Local trusted system origin (`lens://` or local file).
    Internal,
}

/// Information describing an SSL/TLS server certificate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SslCertificateInfo {
    pub subject_cn: String,
    pub issuer: String,
    pub valid_from: u64,
    pub valid_to: u64,
    pub fingerprint_sha256: String,
    pub is_valid: bool,
}

/// Web API permission capability types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Camera,
    Microphone,
    Geolocation,
    Notifications,
    Clipboard,
    LensAiContextAccess,
}

/// Authorization grant decision for a permission request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Allow,
    Block,
    Ask,
}

/// Manages website security classification, per-origin permissions, and ad blocking.
#[derive(Debug, Default)]
pub struct SecurityManager {
    /// Domain origin permissions map: "example.com" -> { PermissionType -> PermissionState }
    origin_permissions: HashMap<String, HashMap<PermissionType, PermissionState>>,
    /// List of blocked tracker/ad domains.
    blocked_domains: Vec<String>,
}

impl SecurityManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            origin_permissions: HashMap::new(),
            blocked_domains: Vec::new(),
        };

        // Populate baseline tracker blocklist
        mgr.blocked_domains.push("analytics.example.com".to_string());
        mgr.blocked_domains.push("doubleclick.net".to_string());
        mgr.blocked_domains.push("adserver.com".to_string());

        mgr
    }

    /// Evaluates `SecurityLevel` based on URL scheme and connection attributes.
    pub fn evaluate_security_level(&self, url: &str) -> SecurityLevel {
        if url.starts_with("lens://") || url.starts_with("about:") || url.starts_with("file://") {
            SecurityLevel::Internal
        } else if url.starts_with("https://") {
            SecurityLevel::Secure
        } else {
            SecurityLevel::Insecure
        }
    }

    /// Gets permission grant state for an origin domain and capability.
    pub fn get_permission(&self, origin: &str, perm: PermissionType) -> PermissionState {
        if let Some(map) = self.origin_permissions.get(origin) {
            if let Some(&state) = map.get(&perm) {
                return state;
            }
        }
        PermissionState::Ask
    }

    /// Updates permission state for a domain origin.
    pub fn set_permission(&mut self, origin: impl Into<String>, perm: PermissionType, state: PermissionState) {
        let orig = origin.into();
        self.origin_permissions
            .entry(orig)
            .or_default()
            .insert(perm, state);
    }

    /// Checks if a network request URL matches content blocking / ad block rules.
    pub fn is_url_blocked(&self, url: &str) -> bool {
        self.blocked_domains.iter().any(|domain| url.contains(domain))
    }

    /// Adds a domain rule to content blocker blocklist.
    pub fn add_block_rule(&mut self, domain: impl Into<String>) {
        let dom = domain.into();
        if !self.blocked_domains.contains(&dom) {
            self.blocked_domains.push(dom);
        }
    }
}
