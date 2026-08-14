//! LensOS Folder Container Module (`src/folder.rs`)
//!
//! Encapsulates directory content representation, aggregated stats (total files/folders/sizes),
//! child item listing, sorting operations, and parent hierarchy navigation for LensOS.

use crate::explorer::{SortBy, SortOrder};
use crate::file::{FileInfo, FileType};
use std::fs;
use std::path::{Path, PathBuf};

/// Detailed representation of a directory and its immediate child contents.
#[derive(Debug, Clone, PartialEq)]
pub struct FolderInfo {
    pub path: PathBuf,
    pub name: String,
    pub items: Vec<FileInfo>,
    pub total_files: usize,
    pub total_folders: usize,
    pub total_size_bytes: u64,
    pub is_empty: bool,
    pub is_root: bool,
    pub parent_path: Option<PathBuf>,
}

impl FolderInfo {
    /// Reads and constructs a `FolderInfo` from the filesystem for a given directory path.
    pub fn from_path(path: impl AsRef<Path>, show_hidden: bool) -> Result<Self, String> {
        let path_buf = path.as_ref().to_path_buf();
        let canonical_path = fs::canonicalize(&path_buf)
            .unwrap_or_else(|_| path_buf.clone());

        let name = canonical_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let parent_path = canonical_path.parent().map(|p| p.to_path_buf());
        let is_root = parent_path.is_none() || canonical_path.parent() == Some(&canonical_path);

        let read_dir = fs::read_dir(&canonical_path)
            .map_err(|e| format!("Failed to read directory {:?}: {}", canonical_path, e))?;

        let mut items = Vec::new();
        let mut total_files = 0;
        let mut total_folders = 0;
        let mut total_size_bytes = 0;

        for entry in read_dir.flatten() {
            if let Ok(file_info) = FileInfo::from_path(entry.path()) {
                if !show_hidden && file_info.is_hidden {
                    continue;
                }

                if file_info.file_type == FileType::Directory {
                    total_folders += 1;
                } else {
                    total_files += 1;
                    total_size_bytes += file_info.size_bytes;
                }

                items.push(file_info);
            }
        }

        let is_empty = items.is_empty();

        Ok(Self {
            path: canonical_path,
            name,
            items,
            total_files,
            total_folders,
            total_size_bytes,
            is_empty,
            is_root,
            parent_path,
        })
    }

    /// Sorts the items inside the folder according to the requested criteria and ordering.
    pub fn sort_items(&mut self, sort_by: SortBy, sort_order: SortOrder) {
        self.items.sort_by(|a, b| {
            // Always keep directories at top unless specifically handled
            let type_cmp = match (a.file_type, b.file_type) {
                (FileType::Directory, FileType::Directory) => std::cmp::Ordering::Equal,
                (FileType::Directory, _) => std::cmp::Ordering::Less,
                (_, FileType::Directory) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            };

            if type_cmp != std::cmp::Ordering::Equal {
                return type_cmp;
            }

            let primary_cmp = match sort_by {
                SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortBy::Size => a.size_bytes.cmp(&b.size_bytes),
                SortBy::DateModified => a.modified_time.cmp(&b.modified_time),
                SortBy::FileType => a.extension.cmp(&b.extension),
            };

            match sort_order {
                SortOrder::Ascending => primary_cmp,
                SortOrder::Descending => primary_cmp.reverse(),
            }
        });
    }

    /// Filters and returns child items matching a specific file extension.
    pub fn filter_by_extension(&self, extension: &str) -> Vec<&FileInfo> {
        let ext_lower = extension.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                item.extension
                    .as_ref()
                    .map(|e| e.to_lowercase() == ext_lower)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Searches for a direct child item by exact name.
    pub fn find_item(&self, name: &str) -> Option<&FileInfo> {
        self.items.iter().find(|item| item.name == name)
    }

    /// Returns human-readable aggregated folder status summary (e.g. "12 items (8 files, 4 folders)").
    pub fn status_summary(&self) -> String {
        format!(
            "{} items ({} files, {} folders)",
            self.items.len(),
            self.total_files,
            self.total_folders
        )
    }
}
