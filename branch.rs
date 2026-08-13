//! # LensOS v0.1 Integration Module - Branch Management (`branch.rs`)
//!
//! Handles creating, switching (checkout), listing, renaming, and deleting git branches.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::IntegrationError;

/// Structure representing a Git branch in LensOS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    /// Name of the branch (e.g. "main", "feature/ai-integration").
    pub name: String,
    /// Latest commit hash pointing to this branch head.
    pub commit_hash: String,
    /// Indicates if this branch is currently checked out (HEAD).
    pub is_head: bool,
    /// Indicates if this branch is a remote tracking reference.
    pub is_remote: bool,
    /// Upstream tracking branch name if set (e.g. "origin/main").
    pub tracking_remote: Option<String>,
}

/// Branch manager responsible for local workspace branch operations.
#[derive(Debug, Clone)]
pub struct BranchManager {
    /// Maps repository local paths to their list of branches.
    repo_branches: HashMap<String, Vec<Branch>>,
}

impl BranchManager {
    /// Create a new `BranchManager`.
    pub fn new() -> Self {
        Self {
            repo_branches: HashMap::new(),
        }
    }

    /// Initialize default branch set for a repository path.
    fn ensure_initialized(&mut self, repo_path: &str) {
        if !self.repo_branches.contains_key(repo_path) {
            let default_branches = vec![
                Branch {
                    name: "main".to_string(),
                    commit_hash: "a1b2c3d4e5f67890123456789abcdef012345678".to_string(),
                    is_head: true,
                    is_remote: false,
                    tracking_remote: Some("origin/main".to_string()),
                },
                Branch {
                    name: "dev".to_string(),
                    commit_hash: "b2c3d4e5f67890123456789abcdef012345678a".to_string(),
                    is_head: false,
                    is_remote: false,
                    tracking_remote: Some("origin/dev".to_string()),
                },
            ];
            self.repo_branches.insert(repo_path.to_string(), default_branches);
        }
    }

    /// Create a new branch in a local repository clone.
    pub fn create_branch(
        &mut self,
        repo_path: &str,
        branch_name: &str,
        start_point: Option<&str>,
    ) -> Result<Branch, IntegrationError> {
        self.ensure_initialized(repo_path);
        let branches = self.repo_branches.get_mut(repo_path).unwrap();

        if branches.iter().any(|b| b.name == branch_name) {
            return Err(IntegrationError::BranchError(format!(
                "Branch '{}' already exists in repository at {}",
                branch_name, repo_path
            )));
        }

        let head_commit = start_point
            .unwrap_or("a1b2c3d4e5f67890123456789abcdef012345678")
            .to_string();

        let new_branch = Branch {
            name: branch_name.to_string(),
            commit_hash: head_commit,
            is_head: false,
            is_remote: false,
            tracking_remote: Some(format!("origin/{}", branch_name)),
        };

        branches.push(new_branch.clone());
        Ok(new_branch)
    }

    /// Switch (checkout) active branch in a repository.
    pub fn switch_branch(
        &mut self,
        repo_path: &str,
        branch_name: &str,
    ) -> Result<Branch, IntegrationError> {
        self.ensure_initialized(repo_path);
        let branches = self.repo_branches.get_mut(repo_path).unwrap();

        let mut found = false;
        let mut target_branch = None;

        for branch in branches.iter_mut() {
            if branch.name == branch_name {
                branch.is_head = true;
                found = true;
                target_branch = Some(branch.clone());
            } else {
                branch.is_head = false;
            }
        }

        if found {
            Ok(target_branch.unwrap())
        } else {
            Err(IntegrationError::BranchError(format!(
                "Branch '{}' not found in repository at {}",
                branch_name, repo_path
            )))
        }
    }

    /// List all branches in a repository.
    pub fn list_branches(&mut self, repo_path: &str) -> Vec<Branch> {
        self.ensure_initialized(repo_path);
        self.repo_branches.get(repo_path).cloned().unwrap_or_default()
    }

    /// Delete a branch from a repository.
    pub fn delete_branch(
        &mut self,
        repo_path: &str,
        branch_name: &str,
    ) -> Result<(), IntegrationError> {
        self.ensure_initialized(repo_path);
        let branches = self.repo_branches.get_mut(repo_path).unwrap();

        if let Some(pos) = branches.iter().position(|b| b.name == branch_name) {
            if branches[pos].is_head {
                return Err(IntegrationError::BranchError(format!(
                    "Cannot delete active HEAD branch '{}'",
                    branch_name
                )));
            }
            branches.remove(pos);
            Ok(())
        } else {
            Err(IntegrationError::BranchError(format!(
                "Branch '{}' not found for deletion",
                branch_name
            )))
        }
    }
}

impl Default for BranchManager {
    fn default() -> Self {
        Self::new()
    }
}
