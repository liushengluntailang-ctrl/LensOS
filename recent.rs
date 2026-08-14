//! LensOS Recent Items Tracker Module (`src/recent.rs`)
//!
//! Tracks recently opened files and folders with frequency counts, pin toggling,
//! and persistent cap limits for LensOS fast-recovery workflows.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Represents an item recorded in the user's recent activity stream.
#[derive(Debug, Clone, PartialEq)]
pub struct RecentItem {
    pub path: PathBuf,
    pub file_name: String,
    pub accessed_at: SystemTime,
    pub access_count: u32,
    pub is_pinned: bool,
}

/// Managing component for recent access history and pinned bookmarks.
#[derive(Debug, Clone)]
pub struct RecentTracker {
    pub max_items: usize,
    pub items: Vec<RecentItem>,
}

impl Default for RecentTracker {
    fn default() -> Self {
        Self {
            max_items: 20,
            items: Vec::new(),
        }
    }
}

impl RecentTracker {
    /// Creates a new `RecentTracker` capping history entries to `max_items`.
    pub fn new(max_items: usize) -> Self {
        Self {
            max_items,
            items: Vec::new(),
        }
    }

    /// Records or updates access for a path, incrementing access count and updating timestamp.
    pub fn record_access(&mut self, path: PathBuf) {
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        if let Some(existing) = self.items.iter_mut().find(|item| item.path == path) {
            existing.accessed_at = SystemTime::now();
            existing.access_count += 1;
        } else {
            let item = RecentItem {
                path,
                file_name,
                accessed_at: SystemTime::now(),
                access_count: 1,
                is_pinned: false,
            };
            self.items.push(item);
        }

        self.sort_and_trim();
    }

    /// Pins an item so it won't be pruned when the history cap is exceeded.
    pub fn pin_item(&mut self, path: &Path) {
        if let Some(item) = self.items.iter_mut().find(|i| i.path == path) {
            item.is_pinned = true;
        } else {
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            self.items.push(RecentItem {
                path: path.to_path_buf(),
                file_name,
                accessed_at: SystemTime::now(),
                access_count: 1,
                is_pinned: true,
            });
        }
        self.sort_and_trim();
    }

    /// Unpins an item.
    pub fn unpin_item(&mut self, path: &Path) {
        if let Some(item) = self.items.iter_mut().find(|i| i.path == path) {
            item.is_pinned = false;
        }
    }

    /// Returns all current recent items (pinned items first, followed by newest timestamp).
    pub fn get_recents(&self) -> Vec<RecentItem> {
        self.items.clone()
    }

    /// Clears all unpinned items from the history list.
    pub fn clear_unpinned(&mut self) {
        self.items.retain(|i| i.is_pinned);
    }

    /// Helper method sorting items by pin status and access time, pruning extra entries.
    fn sort_and_trim(&mut self) {
        self.items.sort_by(|a, b| {
            // Pinned items first
            if a.is_pinned != b.is_pinned {
                return b.is_pinned.cmp(&a.is_pinned);
            }
            // Newer access timestamps first
            b.accessed_at.cmp(&a.accessed_at)
        });

        if self.items.len() > self.max_items {
            // Retain pinned items plus the newest unpinned items up to max_items
            let pinned_count = self.items.iter().filter(|i| i.is_pinned).count();
            let unpinned_allowed = self.max_items.saturating_sub(pinned_count);

            let mut unpinned_seen = 0;
            self.items.retain(|item| {
                if item.is_pinned {
                    true
                } else if unpinned_seen < unpinned_allowed {
                    unpinned_seen += 1;
                    true
                } else {
                    false
                }
            });
        }
    }
}
