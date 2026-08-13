//! # LensOS v0.1 Integration Module - Commit Creation & Staging (`commit.rs`)
//!
//! Manages file staging, commit creation, commit metadata, diff logs, and commit history.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::IntegrationError;

/// Author / committer identity metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub timestamp: String,
}

impl Default for CommitAuthor {
    fn default() -> Self {
        Self {
            name: "LensOS Developer".to_string(),
            email: "dev@lensos.org".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Types of changes applied to a file in git staging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileChangeType {
    Added,
    Modified,
    Deleted,
    Renamed { old_path: String },
}

/// A staged file change record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: String,
    pub change_type: FileChangeType,
}

/// Structure representing a created Git commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// SHA-1 / SHA-256 commit hash string.
    pub hash: String,
    /// Committer author details.
    pub author: CommitAuthor,
    /// Commit message description.
    pub message: String,
    /// ISO-8601 creation timestamp.
    pub timestamp: String,
    /// Parent commit hashes.
    pub parents: Vec<String>,
    /// List of file changes included in this commit.
    pub changed_files: Vec<FileChange>,
}

/// Manager for staging workspace changes and generating commits.
#[derive(Debug, Clone)]
pub struct CommitManager {
    /// Maps repository local paths to their staged file changes.
    staged_changes: HashMap<String, Vec<FileChange>>,
    /// Maps repository local paths to their commit history vector.
    commit_history: HashMap<String, Vec<Commit>>,
}

impl CommitManager {
    /// Create a new `CommitManager`.
    pub fn new() -> Self {
        Self {
            staged_changes: HashMap::new(),
            commit_history: HashMap::new(),
        }
    }

    /// Stage a file change in a repository clone.
    pub fn stage_file(
        &mut self,
        repo_path: &str,
        file_path: &str,
        change_type: FileChangeType,
    ) -> Result<(), IntegrationError> {
        let staged = self.staged_changes.entry(repo_path.to_string()).or_default();
        
        // Remove existing entry for the same file if staged earlier
        staged.retain(|change| change.file_path != file_path);
        
        staged.push(FileChange {
            file_path: file_path.to_string(),
            change_type,
        });

        Ok(())
    }

    /// Get list of staged file changes for a repository.
    pub fn get_staged(&self, repo_path: &str) -> Vec<&FileChange> {
        self.staged_changes
            .get(repo_path)
            .map(|list| list.iter().collect())
            .unwrap_or_default()
    }

    /// Clear staging area for a repository.
    pub fn unstage_all(&mut self, repo_path: &str) {
        self.staged_changes.remove(repo_path);
    }

    /// Create a commit from currently staged changes or explicit file lists.
    pub fn create_commit(
        &mut self,
        repo_path: &str,
        author: CommitAuthor,
        message: &str,
    ) -> Result<Commit, IntegrationError> {
        if message.trim().is_empty() {
            return Err(IntegrationError::CommitError(
                "Commit message cannot be empty".to_string(),
            ));
        }

        let staged = self.staged_changes.remove(repo_path).unwrap_or_default();
        if staged.is_empty() {
            return Err(IntegrationError::CommitError(
                "No staged changes to commit".to_string(),
            ));
        }

        let timestamp = chrono::Utc::now().to_rfc3339();
        let hash = format!(
            "{:x}",
            md5_hash(&format!("{}{}{}", repo_path, message, timestamp))
        );

        let history = self.commit_history.entry(repo_path.to_string()).or_default();
        let parent_hash = history.last().map(|c| c.hash.clone()).unwrap_or_else(|| "0000000000000000000000000000000000000000".to_string());

        let commit = Commit {
            hash,
            author,
            message: message.to_string(),
            timestamp,
            parents: vec![parent_hash],
            changed_files: staged,
        };

        history.push(commit.clone());
        Ok(commit)
    }

    /// Retrieve commit history log for a repository.
    pub fn get_commit_history(&self, repo_path: &str, limit: usize) -> Vec<Commit> {
        self.commit_history
            .get(repo_path)
            .map(|list| list.iter().rev().take(limit).cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for CommitManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility pseudo-hash calculation for deterministic commit hash generation.
fn md5_hash(input: &str) -> u128 {
    let mut hash: u128 = 0xcbf29ce484222325;
    for byte in input.bytes() {
        hash ^= byte as u128;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
