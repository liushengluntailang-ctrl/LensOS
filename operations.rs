//! LensOS File Operations Module (`src/operations.rs`)
//!
//! Handles disk operations: folder creation, renaming, deletion, file copying,
//! and moving items with comprehensive error reporting and operation history tracking.

use std::fs;
use std::path::{Path, PathBuf};

/// Enumeration of filesystem mutating operations executable within LensOS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOperation {
    CreateFolder { parent: PathBuf, name: String },
    Rename { target: PathBuf, new_name: String },
    Delete { target: PathBuf, recursive: bool },
    Copy { src: PathBuf, dest: PathBuf },
    Move { src: PathBuf, dest: PathBuf },
}

/// Standardized output result structure for all file operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub affected_path: Option<PathBuf>,
    pub operation_type: String,
}

impl OperationResult {
    pub fn ok(operation_type: &str, message: impl Into<String>, path: Option<PathBuf>) -> Self {
        Self {
            success: true,
            message: message.into(),
            affected_path: path,
            operation_type: operation_type.to_string(),
        }
    }

    pub fn err(operation_type: &str, message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            affected_path: None,
            operation_type: operation_type.to_string(),
        }
    }
}

/// Disk operation executor maintaining an audit history of file manipulations.
#[derive(Debug, Clone, Default)]
pub struct FileOperationHandler {
    pub history: Vec<OperationResult>,
}

impl FileOperationHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new directory inside `parent` directory with `name`.
    pub fn create_folder(&mut self, parent: &Path, name: &str) -> OperationResult {
        let target_path = parent.join(name);
        if target_path.exists() {
            let res = OperationResult::err("create_folder", format!("Folder {:?} already exists", target_path));
            self.history.push(res.clone());
            return res;
        }

        match fs::create_dir_all(&target_path) {
            Ok(_) => {
                let res = OperationResult::ok(
                    "create_folder",
                    format!("Created folder '{}'", name),
                    Some(target_path),
                );
                self.history.push(res.clone());
                res
            }
            Err(e) => {
                let res = OperationResult::err("create_folder", format!("Failed to create folder: {}", e));
                self.history.push(res.clone());
                res
            }
        }
    }

    /// Renames a target file or folder to `new_name` in its current parent directory.
    pub fn rename(&mut self, target: &Path, new_name: &str) -> OperationResult {
        if !target.exists() {
            let res = OperationResult::err("rename", format!("Target path {:?} does not exist", target));
            self.history.push(res.clone());
            return res;
        }

        let parent = match target.parent() {
            Some(p) => p,
            None => {
                let res = OperationResult::err("rename", "Cannot rename root directory");
                self.history.push(res.clone());
                return res;
            }
        };

        let destination = parent.join(new_name);
        if destination.exists() {
            let res = OperationResult::err("rename", format!("An item named '{}' already exists", new_name));
            self.history.push(res.clone());
            return res;
        }

        match fs::rename(target, &destination) {
            Ok(_) => {
                let res = OperationResult::ok(
                    "rename",
                    format!("Renamed item to '{}'", new_name),
                    Some(destination),
                );
                self.history.push(res.clone());
                res
            }
            Err(e) => {
                let res = OperationResult::err("rename", format!("Failed to rename: {}", e));
                self.history.push(res.clone());
                res
            }
        }
    }

    /// Deletes a file or directory at `target`. If `recursive` is true, deletes non-empty directories.
    pub fn delete(&mut self, target: &Path, recursive: bool) -> OperationResult {
        if !target.exists() {
            let res = OperationResult::err("delete", format!("Target {:?} does not exist", target));
            self.history.push(res.clone());
            return res;
        }

        let is_dir = target.is_dir();
        let result = if is_dir {
            if recursive {
                fs::remove_dir_all(target)
            } else {
                fs::remove_dir(target)
            }
        } else {
            fs::remove_file(target)
        };

        match result {
            Ok(_) => {
                let res = OperationResult::ok("delete", "Item successfully deleted", Some(target.to_path_buf()));
                self.history.push(res.clone());
                res
            }
            Err(e) => {
                let res = OperationResult::err("delete", format!("Failed to delete item: {}", e));
                self.history.push(res.clone());
                res
            }
        }
    }

    /// Copies a file or entire directory structure from `src` to `dest`.
    pub fn copy(&mut self, src: &Path, dest: &Path) -> OperationResult {
        if !src.exists() {
            let res = OperationResult::err("copy", format!("Source path {:?} does not exist", src));
            self.history.push(res.clone());
            return res;
        }

        let final_dest = if dest.is_dir() {
            if let Some(file_name) = src.file_name() {
                dest.join(file_name)
            } else {
                dest.to_path_buf()
            }
        } else {
            dest.to_path_buf()
        };

        let result = if src.is_dir() {
            copy_dir_all(src, &final_dest)
        } else {
            fs::copy(src, &final_dest).map(|_| ())
        };

        match result {
            Ok(_) => {
                let res = OperationResult::ok("copy", "Successfully copied item", Some(final_dest));
                self.history.push(res.clone());
                res
            }
            Err(e) => {
                let res = OperationResult::err("copy", format!("Failed to copy item: {}", e));
                self.history.push(res.clone());
                res
            }
        }
    }

    /// Moves a file or directory from `src` to `dest`.
    pub fn move_item(&mut self, src: &Path, dest: &Path) -> OperationResult {
        if !src.exists() {
            let res = OperationResult::err("move", format!("Source path {:?} does not exist", src));
            self.history.push(res.clone());
            return res;
        }

        let final_dest = if dest.is_dir() {
            if let Some(file_name) = src.file_name() {
                dest.join(file_name)
            } else {
                dest.to_path_buf()
            }
        } else {
            dest.to_path_buf()
        };

        // Try fast atomic rename first
        match fs::rename(src, &final_dest) {
            Ok(_) => {
                let res = OperationResult::ok("move", "Successfully moved item", Some(final_dest));
                self.history.push(res.clone());
                res
            }
            Err(_) => {
                // Fallback to copy then delete across filesystems
                let copy_res = self.copy(src, &final_dest);
                if copy_res.success {
                    let _ = self.delete(src, true);
                    let res = OperationResult::ok("move", "Successfully moved item across drives", Some(final_dest));
                    self.history.push(res.clone());
                    res
                } else {
                    copy_res
                }
            }
        }
    }
}

/// Helper function to recursively copy a directory and all subcontents.
fn copy_dir_all(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
