//! # LensOS v0.1 Integration Module - Repository Cloning (`clone.rs`)
//!
//! Manages cloning remote GitHub repositories into local target directories in LensOS,
//! tracking clone progress, checkout options, and local workspace registration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{repository::Repository, IntegrationError};

/// Configurable options for cloning a repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOptions {
    /// Commit depth (e.g. `Some(1)` for shallow clone).
    pub depth: Option<u32>,
    /// Specific branch to check out upon cloning.
    pub branch: Option<String>,
    /// Target filesystem directory path in LensOS (e.g. `/home/user/workspace/repo`).
    pub target_directory: String,
    /// Whether to recursively initialize git submodules.
    pub recursive_submodules: bool,
    /// Whether to create a mirror repository clone.
    pub mirror: bool,
}

impl Default for CloneOptions {
    fn default() -> Self {
        Self {
            depth: None,
            branch: None,
            target_directory: "/workspace/repos".to_string(),
            recursive_submodules: true,
            mirror: false,
        }
    }
}

/// Real-time progress metric during repository cloning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneProgress {
    /// Current stage (e.g. "Counting objects", "Compressing objects", "Receiving objects", "Checking out").
    pub stage: String,
    pub objects_fetched: u32,
    pub total_objects: u32,
    pub bytes_transferred: u64,
    pub percentage: f32,
}

/// Represents a cloned repository on the local LensOS filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClonedRepository {
    /// Original remote repository metadata.
    pub info: Repository,
    /// Local path inside LensOS (e.g. `/workspace/repos/lensos-kernel`).
    pub local_path: String,
    /// Active checked-out branch.
    pub active_branch: String,
    /// Timestamp ISO string when cloned.
    pub cloned_at: String,
    /// Uncommitted changes present locally.
    pub is_dirty: bool,
}

/// Repository cloner component responsible for managing local repository clones.
#[derive(Debug, Clone)]
pub struct RepositoryCloner {
    cloned_repos: HashMap<String, ClonedRepository>,
}

impl RepositoryCloner {
    /// Create a new `RepositoryCloner`.
    pub fn new() -> Self {
        Self {
            cloned_repos: HashMap::new(),
        }
    }

    /// Clone a remote repository into LensOS local filesystem space.
    pub fn clone_repository(
        &mut self,
        repo: &Repository,
        options: &CloneOptions,
    ) -> Result<ClonedRepository, IntegrationError> {
        let path_suffix = repo.name.as_str();
        let target_path = if options.target_directory.ends_with(path_suffix) {
            options.target_directory.clone()
        } else {
            format!("{}/{}", options.target_directory.trim_end_matches('/'), path_suffix)
        };

        let branch = options
            .branch
            .clone()
            .unwrap_or_else(|| repo.default_branch.clone());

        let cloned = ClonedRepository {
            info: repo.clone(),
            local_path: target_path.clone(),
            active_branch: branch,
            cloned_at: chrono::Utc::now().to_rfc3339(),
            is_dirty: false,
        };

        self.cloned_repos.insert(target_path, cloned.clone());
        Ok(cloned)
    }

    /// Retrieve cloned repository record by its local filesystem path.
    pub fn get_cloned(&self, local_path: &str) -> Option<&ClonedRepository> {
        self.cloned_repos.get(local_path)
    }

    /// List all local repository clones managed by LensOS.
    pub fn list_cloned(&self) -> Vec<&ClonedRepository> {
        self.cloned_repos.values().collect()
    }

    /// Remove a cloned repository record from LensOS workspace tracking.
    pub fn unregister_clone(&mut self, local_path: &str) -> Option<ClonedRepository> {
        self.cloned_repos.remove(local_path)
    }
}

impl Default for RepositoryCloner {
    fn default() -> Self {
        Self::new()
    }
}
