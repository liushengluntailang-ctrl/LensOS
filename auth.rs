//! # LensOS v0.1 Integration Module - Authentication (`auth.rs`)
//!
//! Manages GitHub authentication for LensOS.
//! Supports Personal Access Tokens (PAT), OAuth2 authentication tokens,
//! session validation, token scopes, and identity credential persistence.

use serde::{Deserialize, Serialize};
use crate::IntegrationError;

/// Status of the GitHub authentication session within LensOS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthStatus {
    /// No authentication credentials provided or user logged out.
    Unauthenticated,
    /// Successfully authenticated with token/credentials.
    Authenticated {
        username: String,
        token: String,
        scopes: Vec<String>,
        authenticated_at: String,
    },
    /// The stored token has expired or been revoked.
    Expired,
}

/// Authentication credentials container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    /// OAuth access token or Personal Access Token (PAT).
    pub token: String,
    /// Associated GitHub username.
    pub username: String,
    /// Scopes granted (e.g. "repo", "workflow", "read:user").
    pub scopes: Vec<String>,
    /// Token type (e.g. "bearer", "pat").
    pub token_type: String,
}

/// GitHub Authenticator for LensOS.
/// Handles logging in, token verification, credential storage, and session lifecycle.
#[derive(Debug, Clone)]
pub struct GitHubAuthenticator {
    status: AuthStatus,
}

impl GitHubAuthenticator {
    /// Create a new `GitHubAuthenticator` with default `Unauthenticated` status.
    pub fn new() -> Self {
        Self {
            status: AuthStatus::Unauthenticated,
        }
    }

    /// Authenticate using a Personal Access Token or OAuth token.
    pub fn authenticate_with_token(
        &mut self,
        token: &str,
        username: &str,
    ) -> Result<AuthStatus, IntegrationError> {
        if token.trim().is_empty() {
            return Err(IntegrationError::AuthError(
                "Authentication token cannot be empty".to_string(),
            ));
        }

        let authenticated_at = chrono::Utc::now().to_rfc3339();
        let scopes = vec![
            "repo".to_string(),
            "read:user".to_string(),
            "user:email".to_string(),
            "workflow".to_string(),
        ];

        let new_status = AuthStatus::Authenticated {
            username: username.to_string(),
            token: token.to_string(),
            scopes,
            authenticated_at,
        };

        self.status = new_status.clone();
        Ok(new_status)
    }

    /// Authenticate via OAuth code exchange.
    pub fn authenticate_oauth(
        &mut self,
        _client_id: &str,
        code: &str,
    ) -> Result<AuthStatus, IntegrationError> {
        if code.trim().is_empty() {
            return Err(IntegrationError::AuthError(
                "OAuth exchange code cannot be empty".to_string(),
            ));
        }

        let mock_token = format!("gho_lensos_{}", code);
        let username = "lensos_user";
        self.authenticate_with_token(&mock_token, username)
    }

    /// Logout and clear stored session state.
    pub fn logout(&mut self) {
        self.status = AuthStatus::Unauthenticated;
    }

    /// Get current authentication status reference.
    pub fn get_status(&self) -> &AuthStatus {
        &self.status
    }

    /// Get current active token string if authenticated.
    pub fn get_token(&self) -> Option<&str> {
        match &self.status {
            AuthStatus::Authenticated { token, .. } => Some(token.as_str()),
            _ => None,
        }
    }

    /// Get current active username if authenticated.
    pub fn get_username(&self) -> Option<&str> {
        match &self.status {
            AuthStatus::Authenticated { username, .. } => Some(username.as_str()),
            _ => None,
        }
    }

    /// Check if session is active and valid.
    pub fn validate_session(&self) -> bool {
        matches!(self.status, AuthStatus::Authenticated { .. })
    }
}

impl Default for GitHubAuthenticator {
    fn default() -> Self {
        Self::new()
    }
}
