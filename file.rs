//! LensOS File Representation Module (`src/file.rs`)
//!
//! Encapsulates individual file and directory metadata, file types, formatted file sizes,
//! system timestamps, and icon hints for the LensOS dark frosted glass UI.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Classification of filesystem entries supported by LensOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Unknown,
}

impl FileType {
    /// Returns true if this entry is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, FileType::Directory)
    }

    /// Returns true if this entry is a regular file.
    pub fn is_file(&self) -> bool {
        matches!(self, FileType::File)
    }
}

/// Abstract interface for inspecting file metadata across LensOS modules.
pub trait FileMetadata {
    fn path(&self) -> &Path;
    fn name(&self) -> &str;
    fn size_bytes(&self) -> u64;
    fn file_type(&self) -> FileType;
    fn is_hidden(&self) -> bool;
}

/// Represents detailed metadata and display properties for a single filesystem item.
#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<String>,
    pub file_type: FileType,
    pub size_bytes: u64,
    pub modified_time: SystemTime,
    pub created_time: Option<SystemTime>,
    pub is_readonly: bool,
    pub is_hidden: bool,
    pub icon_name: String,
}

impl FileInfo {
    /// Constructs a `FileInfo` object by querying filesystem metadata for the given path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_ref = path.as_ref();
        let metadata = fs::symlink_metadata(path_ref)
            .map_err(|e| format!("Failed to read metadata for {:?}: {}", path_ref, e))?;

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_file() {
            FileType::File
        } else if metadata.file_type().is_symlink() {
            FileType::Symlink
        } else {
            FileType::Unknown
        };

        let name = path_ref
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path_ref.to_string_lossy().to_string());

        let extension = path_ref
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase());

        let size_bytes = metadata.len();
        let modified_time = metadata.modified().unwrap_or_else(|_| SystemTime::now());
        let created_time = metadata.created().ok();
        let is_readonly = metadata.permissions().readonly();
        let is_hidden = name.starts_with('.');

        let icon_name = Self::determine_icon(file_type, extension.as_deref());

        Ok(Self {
            path: path_ref.to_path_buf(),
            name,
            extension,
            file_type,
            size_bytes,
            modified_time,
            created_time,
            is_readonly,
            is_hidden,
            icon_name,
        })
    }

    /// Selects an appropriate system icon key based on file type and extension.
    fn determine_icon(file_type: FileType, extension: Option<&str>) -> String {
        match file_type {
            FileType::Directory => "folder".to_string(),
            FileType::Symlink => "link".to_string(),
            FileType::Unknown => "file-question".to_string(),
            FileType::File => match extension {
                Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "svg") => "file-image".to_string(),
                Some("mp4" | "mkv" | "webm" | "mov" | "avi") => "file-video".to_string(),
                Some("mp3" | "wav" | "flac" | "ogg") => "file-audio".to_string(),
                Some("rs" | "js" | "ts" | "py" | "c" | "cpp" | "json" | "toml") => "file-code".to_string(),
                Some("pdf") => "file-text".to_string(),
                Some("zip" | "tar" | "gz" | "7z") => "archive".to_string(),
                _ => "file".to_string(),
            },
        }
    }

    /// Formats the raw size in bytes into a human-readable string (e.g. "1.2 MB", "450 KB", "12 B").
    pub fn formatted_size(&self) -> String {
        if self.file_type == FileType::Directory {
            return "--".to_string();
        }

        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size_bytes >= GB {
            format!("{:.1} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.1} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.1} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} B", self.size_bytes)
        }
    }

    /// Formats the modified timestamp as a human-readable string.
    pub fn formatted_modified_time(&self) -> String {
        match self.modified_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs();
                let days = secs / 86400;
                format!("{}d ago", days)
            }
            Err(_) => "Unknown".to_string(),
        }
    }
}

impl FileMetadata for FileInfo {
    fn path(&self) -> &Path {
        &self.path
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }

    fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}
