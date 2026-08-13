//! # LensOS v0.1 Integration Module - Repository Browsing (`repository.rs`)
//!
//! Models GitHub repositories, search queries, metadata, access permissions,
//! and repository browsing capabilities within LensOS.

use serde::{Deserialize, Serialize};

/// Visibility levels for a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepositoryVisibility {
    Public,
    Private,
    Internal,
}

/// Access permission levels granted to the authenticated user for a repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

impl Default for RepositoryPermissions {
    fn default() -> Self {
        Self {
            admin: false,
            push: true,
            pull: true,
        }
    }
}

/// Metadata model for a GitHub repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub visibility: RepositoryVisibility,
    pub clone_url: String,
    pub ssh_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub updated_at: String,
    pub permissions: RepositoryPermissions,
}

/// Query parameters for searching repositories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySearchQuery {
    pub query: String,
    pub language: Option<String>,
    pub owner: Option<String>,
    pub include_private: bool,
    pub limit: usize,
}

impl Default for RepositorySearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            language: None,
            owner: None,
            include_private: true,
            limit: 20,
        }
    }
}

/// Repository browser component for discovering and searching GitHub repositories.
#[derive(Debug, Clone)]
pub struct RepositoryBrowser {
    cached_repositories: Vec<Repository>,
}

impl RepositoryBrowser {
    /// Create a new `RepositoryBrowser` initialized with default mock/system repositories.
    pub fn new() -> Self {
        let mut browser = Self {
            cached_repositories: Vec::new(),
        };

        // Populate initial default system repositories for demo/testing compatibility
        browser.register_repository(Repository {
            id: 101,
            name: "lensos-kernel".to_string(),
            full_name: "lensos/lensos-kernel".to_string(),
            owner: "lensos".to_string(),
            description: Some("LensOS Core Microkernel and Operating System Services".to_string()),
            default_branch: "main".to_string(),
            visibility: RepositoryVisibility::Public,
            clone_url: "https://github.com/lensos/lensos-kernel.git".to_string(),
            ssh_url: "git@github.com:lensos/lensos-kernel.git".to_string(),
            stargazers_count: 1240,
            forks_count: 88,
            updated_at: chrono::Utc::now().to_rfc3339(),
            permissions: RepositoryPermissions { admin: true, push: true, pull: true },
        });

        browser.register_repository(Repository {
            id: 102,
            name: "lensos-apps".to_string(),
            full_name: "lensos/lensos-apps".to_string(),
            owner: "lensos".to_string(),
            description: Some("Built-in System Applications for LensOS Desktop".to_string()),
            default_branch: "main".to_string(),
            visibility: RepositoryVisibility::Public,
            clone_url: "https://github.com/lensos/lensos-apps.git".to_string(),
            ssh_url: "git@github.com:lensos/lensos-apps.git".to_string(),
            stargazers_count: 620,
            forks_count: 42,
            updated_at: chrono::Utc::now().to_rfc3339(),
            permissions: RepositoryPermissions { admin: false, push: true, pull: true },
        });

        browser
    }

    /// Register or update a repository in the browser cache.
    pub fn register_repository(&mut self, repo: Repository) {
        if let Some(pos) = self.cached_repositories.iter().position(|r| r.full_name == repo.full_name) {
            self.cached_repositories[pos] = repo;
        } else {
            self.cached_repositories.push(repo);
        }
    }

    /// Search repositories matching query filter parameters.
    pub fn search(&self, query: &RepositorySearchQuery) -> Vec<Repository> {
        let q = query.query.to_lowercase();
        self.cached_repositories
            .iter()
            .filter(|repo| {
                let matches_text = q.is_empty()
                    || repo.name.to_lowercase().contains(&q)
                    || repo.full_name.to_lowercase().contains(&q)
                    || repo.description.as_deref().unwrap_or("").to_lowercase().contains(&q);

                let matches_owner = match &query.owner {
                    Some(owner) => repo.owner.eq_ignore_ascii_case(owner),
                    None => true,
                };

                let matches_visibility = query.include_private || repo.visibility == RepositoryVisibility::Public;

                matches_text && matches_owner && matches_visibility
            })
            .take(query.limit)
            .cloned()
            .collect()
    }

    /// Fetch repository details by owner and repository name.
    pub fn get_repository(&self, owner: &str, name: &str) -> Option<Repository> {
        let full_name = format!("{}/{}", owner, name).to_lowercase();
        self.cached_repositories
            .iter()
            .find(|r| r.full_name.to_lowercase() == full_name)
            .cloned()
    }

    /// List repositories belonging to a specific user or organization.
    pub fn list_user_repositories(&self, username: &str) -> Vec<Repository> {
        self.cached_repositories
            .iter()
            .filter(|r| r.owner.eq_ignore_ascii_case(username))
            .cloned()
            .collect()
    }

    /// List all repositories currently discovered by the browser.
    pub fn list_all(&self) -> Vec<Repository> {
        self.cached_repositories.clone()
    }
}

impl Default for RepositoryBrowser {
    fn default() -> Self {
        Self::new()
    }
}
