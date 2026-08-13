//! # LensOS v0.1 Integration Module - GitHub Client Facade (`github.rs`)
//!
//! Provides a unified GitHub API client facade combining authentication, rate-limiting,
//! repository browsing, API endpoint routing, and network status tracking.

use serde::{Deserialize, Serialize};
use crate::{
    auth::GitHubAuthenticator,
    repository::{Repository, RepositoryBrowser, RepositorySearchQuery},
    IntegrationError,
};

/// Rate limit tracking telemetry for GitHub API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Maximum request allocation per hour.
    pub limit: u32,
    /// Remaining request quota.
    pub remaining: u32,
    /// Unix timestamp when quota resets.
    pub reset_timestamp: u64,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            limit: 5000,
            remaining: 4982,
            reset_timestamp: 1780000000,
        }
    }
}

/// GitHub API Client Facade for LensOS.
#[derive(Debug, Clone)]
pub struct GitHubClient {
    pub authenticator: GitHubAuthenticator,
    pub browser: RepositoryBrowser,
    pub rate_limit: RateLimit,
}

impl GitHubClient {
    /// Create a new `GitHubClient`.
    pub fn new(authenticator: GitHubAuthenticator, browser: RepositoryBrowser) -> Self {
        Self {
            authenticator,
            browser,
            rate_limit: RateLimit::default(),
        }
    }

    /// Search repositories on GitHub.
    pub fn search(&mut self, query: &RepositorySearchQuery) -> Vec<Repository> {
        self.decrement_rate_limit();
        self.browser.search(query)
    }

    /// Fetch repository details by owner and name.
    pub fn fetch_repository(&mut self, owner: &str, name: &str) -> Result<Repository, IntegrationError> {
        self.decrement_rate_limit();
        self.browser.get_repository(owner, name).ok_or_else(|| {
            IntegrationError::RepositoryNotFound(format!("{}/{}", owner, name))
        })
    }

    /// Get current rate limit telemetry status.
    pub fn get_rate_limit(&self) -> &RateLimit {
        &self.rate_limit
    }

    /// Internal rate limit decrement simulator.
    fn decrement_rate_limit(&mut self) {
        if self.rate_limit.remaining > 0 {
            self.rate_limit.remaining -= 1;
        }
    }
}
