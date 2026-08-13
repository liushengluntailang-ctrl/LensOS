//! LensOS v0.1 - Authentication and Login System
//!
//! Provides secure user credential authentication, challenge verification,
//! lockout protection, and credential exchange for active system sessions.

use crate::user::{User, UserManager};
use std::collections::HashMap;

/// Result of an authentication challenge
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginResult {
    /// Login succeeded, returning authenticated User object
    Success(User),
    /// Submitted password or credential payload was incorrect
    InvalidCredentials,
    /// Account is disabled or currently locked due to too many failed attempts
    AccountLocked,
    /// Specified username does not exist
    UserNotFound,
}

/// Tracks security attempt records for brute-force prevention
#[derive(Debug, Default, Clone)]
struct FailedAttemptRecord {
    failed_count: u32,
    last_failed_timestamp: u64,
}

/// System Login Manager enforcing authentication policies
#[derive(Debug)]
pub struct LoginManager {
    /// Mock hashed password vault mapping username -> hashed_password
    credentials_store: HashMap<String, String>,
    /// Failed attempts map for rate limiting
    failed_attempts: HashMap<String, FailedAttemptRecord>,
    /// Maximum allowed failed consecutive attempts before lockout
    max_failed_attempts: u32,
}

impl Default for LoginManager {
    fn default() -> Self {
        let mut store = HashMap::new();
        // Default passwords for standard provisioning ("lensos2026" and "admin123")
        store.insert("root".to_string(), "admin123".to_string());
        store.insert("lensuser".to_string(), "lensos2026".to_string());

        Self {
            credentials_store: store,
            failed_attempts: HashMap::new(),
            max_failed_attempts: 5,
        }
    }
}

impl LoginManager {
    /// Constructs a new `LoginManager` instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or updates password credentials for a given account
    pub fn set_credentials(&mut self, username: impl Into<String>, password_hash: impl Into<String>) {
        self.credentials_store.insert(username.into().to_lowercase(), password_hash.into());
    }

    /// Authenticates a user against the `UserManager` registry
    pub fn authenticate(&mut self, username: &str, password: &str, user_manager: &UserManager) -> LoginResult {
        let normalized_user = username.to_lowercase();

        // 1. Locate user profile
        let user = match user_manager.get_user_by_username(&normalized_user) {
            Some(u) => u,
            None => return LoginResult::UserNotFound,
        };

        if !user.is_active {
            return LoginResult::AccountLocked;
        }

        // 2. Check lockout status
        if let Some(record) = self.failed_attempts.get(&normalized_user) {
            if record.failed_count >= self.max_failed_attempts {
                return LoginResult::AccountLocked;
            }
        }

        // 3. Verify password
        let stored_hash = self.credentials_store.get(&normalized_user);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if let Some(hash) = stored_hash {
            if hash == password {
                // Clear failed attempt history upon success
                self.failed_attempts.remove(&normalized_user);
                LoginResult::Success(user.clone())
            } else {
                let record = self.failed_attempts.entry(normalized_user).or_default();
                record.failed_count += 1;
                record.last_failed_timestamp = now;
                LoginResult::InvalidCredentials
            }
        } else {
            LoginResult::InvalidCredentials
        }
    }

    /// Unlocks an account locked due to excessive failed password attempts
    pub fn reset_lockout(&mut self, username: &str) {
        self.failed_attempts.remove(&username.to_lowercase());
    }
}
