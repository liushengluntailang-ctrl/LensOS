//! # Download Manager Module (`downloads.rs`)
//!
//! Handles file download tasks, transfer progress tracking, speed calculation,
//! pause/resume state machines, and integration with LensOS file manager.

use crate::{BrowserError, BrowserResult};

/// Unique identifier for a download task.
pub type DownloadId = u64;

/// Represents the execution state of a file download.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadState {
    Queued,
    Downloading {
        bytes_received: u64,
        total_bytes: Option<u64>,
        speed_bps: u64,
    },
    Paused {
        bytes_received: u64,
        total_bytes: Option<u64>,
    },
    Completed {
        total_bytes: u64,
        file_path: String,
    },
    Failed(String),
    Cancelled,
}

/// Represents an individual file download item.
#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub id: DownloadId,
    pub source_url: String,
    pub file_name: String,
    pub destination_path: String,
    pub mime_type: Option<String>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub state: DownloadState,
}

impl DownloadItem {
    pub fn new(
        id: DownloadId,
        source_url: impl Into<String>,
        file_name: impl Into<String>,
        destination_path: impl Into<String>,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            source_url: source_url.into(),
            file_name: file_name.into(),
            destination_path: destination_path.into(),
            mime_type: None,
            start_time: timestamp,
            end_time: None,
            state: DownloadState::Queued,
        }
    }

    /// Computes current completion percentage (0.0 to 100.0) if total size is known.
    pub fn progress_percentage(&self) -> Option<f32> {
        match &self.state {
            DownloadState::Downloading {
                bytes_received,
                total_bytes: Some(total),
                ..
            }
            | DownloadState::Paused {
                bytes_received,
                total_bytes: Some(total),
            } => {
                if *total == 0 {
                    Some(0.0)
                } else {
                    Some((*bytes_received as f32 / *total as f32) * 100.0)
                }
            }
            DownloadState::Completed { .. } => Some(100.0),
            _ => None,
        }
    }
}

/// Manages active and historical file download tasks.
#[derive(Debug)]
pub struct DownloadManager {
    downloads: Vec<DownloadItem>,
    next_id: u64,
    pub default_download_directory: String,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            downloads: Vec::new(),
            next_id: 1,
            default_download_directory: "/home/lens/Downloads".to_string(),
        }
    }

    /// Enqueues a new download task.
    pub fn start_download(
        &mut self,
        source_url: impl Into<String>,
        file_name: impl Into<String>,
        timestamp: u64,
    ) -> DownloadId {
        let id = self.next_id;
        self.next_id += 1;

        let name = file_name.into();
        let dest = format!("{}/{}", self.default_download_directory, name);
        let item = DownloadItem::new(id, source_url, name, dest, timestamp);

        self.downloads.push(item);
        id
    }

    /// Updates download byte progress and transfer speed.
    pub fn update_progress(
        &mut self,
        id: DownloadId,
        bytes_received: u64,
        total_bytes: Option<u64>,
        speed_bps: u64,
    ) -> BrowserResult<()> {
        let item = self
            .downloads
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| BrowserError::DownloadError(format!("Download task {} not found", id)))?;

        item.state = DownloadState::Downloading {
            bytes_received,
            total_bytes,
            speed_bps,
        };

        Ok(())
    }

    /// Pauses an active download task.
    pub fn pause_download(&mut self, id: DownloadId) -> BrowserResult<()> {
        let item = self
            .downloads
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| BrowserError::DownloadError(format!("Download task {} not found", id)))?;

        if let DownloadState::Downloading {
            bytes_received,
            total_bytes,
            ..
        } = item.state
        {
            item.state = DownloadState::Paused {
                bytes_received,
                total_bytes,
            };
            Ok(())
        } else {
            Err(BrowserError::DownloadError("Download is not active".into()))
        }
    }

    /// Resumes a paused download task.
    pub fn resume_download(&mut self, id: DownloadId) -> BrowserResult<()> {
        let item = self
            .downloads
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| BrowserError::DownloadError(format!("Download task {} not found", id)))?;

        if let DownloadState::Paused {
            bytes_received,
            total_bytes,
        } = item.state
        {
            item.state = DownloadState::Downloading {
                bytes_received,
                total_bytes,
                speed_bps: 0,
            };
            Ok(())
        } else {
            Err(BrowserError::DownloadError("Download is not paused".into()))
        }
    }

    /// Marks a download as completed.
    pub fn complete_download(&mut self, id: DownloadId, timestamp: u64) -> BrowserResult<()> {
        let item = self
            .downloads
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| BrowserError::DownloadError(format!("Download task {} not found", id)))?;

        let bytes = match item.state {
            DownloadState::Downloading { bytes_received, .. } => bytes_received,
            _ => 0,
        };

        item.end_time = Some(timestamp);
        item.state = DownloadState::Completed {
            total_bytes: bytes,
            file_path: item.destination_path.clone(),
        };

        Ok(())
    }

    /// Cancels a download task.
    pub fn cancel_download(&mut self, id: DownloadId) -> BrowserResult<()> {
        let item = self
            .downloads
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| BrowserError::DownloadError(format!("Download task {} not found", id)))?;

        item.state = DownloadState::Cancelled;
        Ok(())
    }

    /// Returns references to all active downloads.
    pub fn active_downloads(&self) -> Vec<&DownloadItem> {
        self.downloads
            .iter()
            .filter(|d| matches!(d.state, DownloadState::Downloading { .. } | DownloadState::Queued))
            .collect()
    }

    /// Clears completed or cancelled downloads from list.
    pub fn clear_finished(&mut self) -> usize {
        let initial = self.downloads.len();
        self.downloads.retain(|d| {
            matches!(
                d.state,
                DownloadState::Downloading { .. } | DownloadState::Queued | DownloadState::Paused { .. }
            )
        });
        initial - self.downloads.len()
    }

    /// Returns list of all downloads.
    pub fn all_downloads(&self) -> &[DownloadItem] {
        &self.downloads
    }
}
