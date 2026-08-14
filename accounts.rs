use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountRole {
    Administrator,
    StandardUser,
    Guest,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccount {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub avatar_path: Option<String>,
    pub role: AccountRole,
    pub is_active: bool,
    pub biometric_enabled: bool,
    pub created_at_timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountSettings {
    pub current_user: UserAccount,
    pub other_accounts: Vec<UserAccount>,
    pub auto_login: bool,
    pub sync_across_devices: bool,
    pub sync_settings: bool,
    pub sync_wallpapers: bool,
    pub sync_passwords: bool,
}

impl Default for AccountSettings {
    fn default() -> Self {
        let admin = UserAccount {
            id: "usr_admin_01".to_string(),
            username: "lens_admin".to_string(),
            display_name: "LensOS Developer".to_string(),
            email: "admin@lensos.org".to_string(),
            avatar_path: Some("/system/avatars/default_glass.png".to_string()),
            role: AccountRole::Administrator,
            is_active: true,
            biometric_enabled: true,
            created_at_timestamp: 1700000000,
        };

        let guest = UserAccount {
            id: "usr_guest_02".to_string(),
            username: "guest".to_string(),
            display_name: "Guest User".to_string(),
            email: "guest@local".to_string(),
            avatar_path: None,
            role: AccountRole::Guest,
            is_active: false,
            biometric_enabled: false,
            created_at_timestamp: 1700005000,
        };

        Self {
            current_user: admin,
            other_accounts: vec![guest],
            auto_login: false,
            sync_across_devices: true,
            sync_settings: true,
            sync_wallpapers: true,
            sync_passwords: false,
        }
    }
}

impl AccountSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_profile(&mut self, display_name: String, email: String, avatar: Option<String>) {
        self.current_user.display_name = display_name;
        self.current_user.email = email;
        self.current_user.avatar_path = avatar;
    }

    pub fn add_account(&mut self, account: UserAccount) -> Result<(), String> {
        if account.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if self.other_accounts.iter().any(|a| a.username == account.username)
            || self.current_user.username == account.username
        {
            return Err("Account with this username already exists".to_string());
        }
        self.other_accounts.push(account);
        Ok(())
    }

    pub fn remove_account(&mut self, user_id: &str) -> Result<(), String> {
        if self.current_user.id == user_id {
            return Err("Cannot remove currently logged-in account".to_string());
        }
        if let Some(pos) = self.other_accounts.iter().position(|a| a.id == user_id) {
            self.other_accounts.remove(pos);
            Ok(())
        } else {
            Err(format!("Account with ID '{}' not found", user_id))
        }
    }

    pub fn toggle_biometrics(&mut self, enabled: bool) {
        self.current_user.biometric_enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_defaults() {
        let acc = AccountSettings::default();
        assert_eq!(acc.current_user.username, "lens_admin");
        assert_eq!(acc.current_user.role, AccountRole::Administrator);
        assert_eq!(acc.other_accounts.len(), 1);
    }

    #[test]
    fn test_update_profile() {
        let mut acc = AccountSettings::default();
        acc.update_profile(
            "Alice Glass".to_string(),
            "alice@lensos.org".to_string(),
            None,
        );
        assert_eq!(acc.current_user.display_name, "Alice Glass");
        assert_eq!(acc.current_user.email, "alice@lensos.org");
    }

    #[test]
    fn test_add_remove_account() {
        let mut acc = AccountSettings::default();
        let new_user = UserAccount {
            id: "usr_bob_03".to_string(),
            username: "bob".to_string(),
            display_name: "Bob Builder".to_string(),
            email: "bob@local".to_string(),
            avatar_path: None,
            role: AccountRole::StandardUser,
            is_active: false,
            biometric_enabled: false,
            created_at_timestamp: 1700010000,
        };

        assert!(acc.add_account(new_user).is_ok());
        assert_eq!(acc.other_accounts.len(), 2);

        assert!(acc.remove_account("usr_bob_03").is_ok());
        assert_eq!(acc.other_accounts.len(), 1);
    }
}
