//! LensOS v0.1 - User Account Management
//!
//! Handles user profile representations, roles, administrative privileges,
//! user identity validation, and user directory management across the system.

use std::collections::HashMap;

/// User role levels supported by LensOS v0.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserRole {
    /// Full administrative rights to configure system parameters and manage users
    Admin,
    /// Standard user account with personalized workspace and standard permissions
    Standard,
    /// Restricted guest account with transient storage and non-persistent session state
    Guest,
    /// Low-level system daemon account
    System,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Standard
    }
}

/// Represents a registered LensOS user account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// Unique immutable account identifier
    pub id: String,
    /// User login name (lowercase, alphanumeric)
    pub username: String,
    /// Human-friendly display name shown in UI/Desktop components
    pub display_name: String,
    /// Role assigned to the user
    pub role: UserRole,
    /// Indicates whether the account is currently enabled
    pub is_active: bool,
    /// Timestamp (seconds since epoch) when account was created
    pub created_at: u64,
    /// Path to user's home directory within the files module filesystem
    pub home_directory: String,
    /// Associated email address or biometric identifier hash
    pub email: Option<String>,
}

impl User {
    /// Creates a new user profile with standard default paths
    pub fn new(id: impl Into<String>, username: impl Into<String>, display_name: impl Into<String>, role: UserRole) -> Self {
        let u_str = username.into();
        let home = format!("/home/{}", u_str);
        Self {
            id: id.into(),
            username: u_str,
            display_name: display_name.into(),
            role,
            is_active: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            home_directory: home,
            email: None,
        }
    }

    /// Checks if the user has administrative privileges
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

/// Database/Registry holding registered users in the LensOS system.
#[derive(Debug, Default)]
pub struct UserManager {
    users: HashMap<String, User>,
}

impl UserManager {
    /// Creates an empty `UserManager` and registers default accounts
    pub fn new() -> Self {
        let mut manager = Self {
            users: HashMap::new(),
        };
        // Provision initial root/admin user for LensOS v0.1
        let root_user = User::new("usr_root_001", "root", "System Administrator", UserRole::Admin);
        let default_user = User::new("usr_lens_002", "lensuser", "LensOS User", UserRole::Standard);
        manager.add_user(root_user);
        manager.add_user(default_user);
        manager
    }

    /// Registers a new user account in the system registry
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    /// Retrieves a user profile by unique user ID
    pub fn get_user_by_id(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    /// Retrieves a user profile by username
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username.eq_ignore_ascii_case(username))
    }

    /// Lists all active registered accounts
    pub fn list_active_users(&self) -> Vec<&User> {
        self.users.values().filter(|u| u.is_active).collect()
    }

    /// Deactivates a user account
    pub fn deactivate_user(&mut self, id: &str) -> bool {
        if let Some(user) = self.users.get_mut(id) {
            user.is_active = false;
            true
        } else {
            false
        }
    }
}
