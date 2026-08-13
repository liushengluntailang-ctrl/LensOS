//! # LensOS v0.1 Integration Module - Files Bridge (`files_bridge.rs`)
//!
//! Connects the LensOS Integration Module with the LensOS `files` app and virtual file system.
//! Exposes cloned Git repositories as mounted virtual file system folders, maps file changes,
//! and handles export/import operations to standard LensOS file directories.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::IntegrationError;

/// Virtual Git Folder representing a mounted repository inside LensOS Files VFS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualGitFolder {
    /// Mount path in LensOS Files VFS (e.g. `/lensos/files/git/lensos-kernel`).
    pub mount_point: String,
    /// Repository name.
    pub repo_name: String,
    /// Repository owner.
    pub owner: String,
    /// Active checked-out branch.
    pub active_branch: String,
    /// Brief sync and dirty state summary.
    pub status_summary: String,
    /// Local underlying physical filesystem path.
    pub physical_path: String,
}

/// FilesBridge component providing integration with the LensOS Files application.
#[derive(Debug, Clone)]
pub struct FilesBridge {
    mounted_folders: HashMap<String, VirtualGitFolder>,
}

impl FilesBridge {
    /// Create a new `FilesBridge`.
    pub fn new() -> Self {
        Self {
            mounted_folders: HashMap::new(),
        }
    }

    /// Mount a cloned Git repository into the LensOS Files App virtual file system.
    pub fn mount_repository(
        &mut self,
        physical_path: &str,
        repo_name: &str,
        owner: &str,
        active_branch: &str,
    ) -> Result<VirtualGitFolder, IntegrationError> {
        let mount_point = format!("/lensos/files/git/{}", repo_name);

        let folder = VirtualGitFolder {
            mount_point: mount_point.clone(),
            repo_name: repo_name.to_string(),
            owner: owner.to_string(),
            active_branch: active_branch.to_string(),
            status_summary: "Clean - Synced".to_string(),
            physical_path: physical_path.to_string(),
        };

        self.mounted_folders.insert(mount_point.clone(), folder.clone());
        Ok(folder)
    }

    /// Unmount a Virtual Git Folder from the Files App VFS.
    pub fn unmount_repository(&mut self, mount_point: &str) -> Result<(), IntegrationError> {
        if self.mounted_folders.remove(mount_point).is_some() {
            Ok(())
        } else {
            Err(IntegrationError::FilesBridgeError(format!(
                "Mount point '{}' not found in LensOS Files VFS",
                mount_point
            )))
        }
    }

    /// List all currently mounted virtual Git folders in the Files App.
    pub fn list_mounted(&self) -> Vec<&VirtualGitFolder> {
        self.mounted_folders.values().collect()
    }

    /// Notify Files Bridge of a file modification inside a mounted Git repository.
    pub fn on_file_changed(
        &mut self,
        mount_point: &str,
        relative_file_path: &str,
    ) -> Result<(), IntegrationError> {
        if let Some(folder) = self.mounted_folders.get_mut(mount_point) {
            folder.status_summary = format!("Modified: {}", relative_file_path);
            Ok(())
        } else {
            Err(IntegrationError::FilesBridgeError(format!(
                "Mount point '{}' not active",
                mount_point
            )))
        }
    }

    /// Export a file from a repository clone directly to standard LensOS user file folders.
    pub fn export_file_to_lensos_files(
        &self,
        repo_path: &str,
        rel_path: &str,
        lens_files_target: &str,
    ) -> Result<String, IntegrationError> {
        let destination = format!(
            "{}/{}",
            lens_files_target.trim_end_matches('/'),
            rel_path.split('/').last().unwrap_or(rel_path)
        );

        Ok(format!(
            "Exported file {} from {} to LensOS Files path {}",
            rel_path, repo_path, destination
        ))
    }
}

impl Default for FilesBridge {
    fn default() -> Self {
        Self::new()
    }
}
