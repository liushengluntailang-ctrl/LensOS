//! # Bookmark Manager Module (`bookmarks.rs`)
//!
//! Provides hierarchical bookmark tree management, folder organization, tag indexing,
//! search filtering, and JSON export/import primitives.

use crate::{BrowserError, BrowserResult};
use std::collections::HashMap;

/// Unique identifier for a bookmark.
pub type BookmarkId = u64;

/// Unique identifier for a bookmark folder.
pub type FolderId = u64;

/// Individual bookmark item metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub title: String,
    pub url: String,
    pub folder_id: FolderId,
    pub tags: Vec<String>,
    pub date_added: u64,
    pub favicon_url: Option<String>,
    pub notes: Option<String>,
}

impl Bookmark {
    pub fn new(
        id: BookmarkId,
        title: impl Into<String>,
        url: impl Into<String>,
        folder_id: FolderId,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            url: url.into(),
            folder_id,
            tags: Vec::new(),
            date_added: timestamp,
            favicon_url: None,
            notes: None,
        }
    }
}

/// Represents a folder in the bookmark tree hierarchy.
#[derive(Debug, Clone)]
pub struct BookmarkFolder {
    pub id: FolderId,
    pub name: String,
    pub parent_id: Option<FolderId>,
    pub child_folder_ids: Vec<FolderId>,
    pub bookmark_ids: Vec<BookmarkId>,
}

impl BookmarkFolder {
    pub fn new(id: FolderId, name: impl Into<String>, parent_id: Option<FolderId>) -> Self {
        Self {
            id,
            name: name.into(),
            parent_id,
            child_folder_ids: Vec::new(),
            bookmark_ids: Vec::new(),
        }
    }
}

/// Core bookmark storage engine and indexer.
#[derive(Debug)]
pub struct BookmarkManager {
    bookmarks: HashMap<BookmarkId, Bookmark>,
    folders: HashMap<FolderId, BookmarkFolder>,
    pub root_folder_id: FolderId,
    pub bookmark_bar_folder_id: FolderId,
    next_bookmark_id: u64,
    next_folder_id: u64,
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BookmarkManager {
    /// Creates a new `BookmarkManager` with default Root and Bookmark Bar folders.
    pub fn new() -> Self {
        let root_id = 1;
        let bar_id = 2;

        let mut folders = HashMap::new();
        let mut root = BookmarkFolder::new(root_id, "Root", None);
        root.child_folder_ids.push(bar_id);

        let bar = BookmarkFolder::new(bar_id, "Bookmarks Bar", Some(root_id));

        folders.insert(root_id, root);
        folders.insert(bar_id, bar);

        Self {
            bookmarks: HashMap::new(),
            folders,
            root_folder_id: root_id,
            bookmark_bar_folder_id: bar_id,
            next_bookmark_id: 100,
            next_folder_id: 10,
        }
    }

    /// Creates a new subfolder under `parent_id`.
    pub fn create_folder(&mut self, name: impl Into<String>, parent_id: FolderId) -> BrowserResult<FolderId> {
        if !self.folders.contains_key(&parent_id) {
            return Err(BrowserError::StorageError(format!("Parent folder ID {} not found", parent_id)));
        }

        let new_id = self.next_folder_id;
        self.next_folder_id += 1;

        let folder = BookmarkFolder::new(new_id, name, Some(parent_id));
        self.folders.insert(new_id, folder);

        if let Some(parent) = self.folders.get_mut(&parent_id) {
            parent.child_folder_ids.push(new_id);
        }

        Ok(new_id)
    }

    /// Adds a new bookmark to the specified folder.
    pub fn add_bookmark(
        &mut self,
        title: impl Into<String>,
        url: impl Into<String>,
        folder_id: FolderId,
        timestamp: u64,
    ) -> BrowserResult<BookmarkId> {
        if !self.folders.contains_key(&folder_id) {
            return Err(BrowserError::StorageError(format!("Folder ID {} not found", folder_id)));
        }

        let id = self.next_bookmark_id;
        self.next_bookmark_id += 1;

        let bookmark = Bookmark::new(id, title, url, folder_id, timestamp);
        self.bookmarks.insert(id, bookmark);

        if let Some(folder) = self.folders.get_mut(&folder_id) {
            folder.bookmark_ids.push(id);
        }

        Ok(id)
    }

    /// Removes a bookmark by ID.
    pub fn remove_bookmark(&mut self, id: BookmarkId) -> Option<Bookmark> {
        if let Some(bookmark) = self.bookmarks.remove(&id) {
            if let Some(folder) = self.folders.get_mut(&bookmark.folder_id) {
                folder.bookmark_ids.retain(|&b_id| b_id != id);
            }
            Some(bookmark)
        } else {
            None
        }
    }

    /// Checks whether a URL is already bookmarked and returns its ID.
    pub fn find_by_url(&self, url: &str) -> Option<&Bookmark> {
        self.bookmarks.values().find(|b| b.url == url)
    }

    /// Toggles bookmark state for a given URL in the Bookmark Bar.
    pub fn toggle_bookmark(&mut self, title: &str, url: &str, timestamp: u64) -> bool {
        if let Some(existing) = self.find_by_url(url) {
            let id = existing.id;
            self.remove_bookmark(id);
            false
        } else {
            let _ = self.add_bookmark(title, url, self.bookmark_bar_folder_id, timestamp);
            true
        }
    }

    /// Searches bookmarks by title, URL, or tags.
    pub fn search(&self, keyword: &str) -> Vec<&Bookmark> {
        let kw = keyword.to_lowercase();
        self.bookmarks
            .values()
            .filter(|b| {
                b.title.to_lowercase().contains(&kw)
                    || b.url.to_lowercase().contains(&kw)
                    || b.tags.iter().any(|t| t.to_lowercase().contains(&kw))
            })
            .collect()
    }

    /// Returns all bookmarks within a given folder ID.
    pub fn get_bookmarks_in_folder(&self, folder_id: FolderId) -> Vec<&Bookmark> {
        if let Some(folder) = self.folders.get(&folder_id) {
            folder
                .bookmark_ids
                .iter()
                .filter_map(|id| self.bookmarks.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Total count of bookmarks saved.
    pub fn total_bookmarks(&self) -> usize {
        self.bookmarks.len()
    }
}
