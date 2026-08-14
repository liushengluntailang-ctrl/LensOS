//! LensOS File Explorer Module (`src/explorer.rs`)
//!
//! Manages navigation history stacks (back/forward/up), path traversal, view layout modes,
//! sorting configurations, and active folder state management.

use crate::folder::FolderInfo;
use std::path::PathBuf;

/// Visual representation modes supported by the LensOS file view container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Grid,
    Compact,
    Details,
}

/// Available sorting attributes for file collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Name,
    Size,
    DateModified,
    FileType,
}

/// Sort direction choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Core navigation and view state engine for LensOS Files application.
#[derive(Debug, Clone)]
pub struct FileExplorer {
    pub current_path: PathBuf,
    pub history_back: Vec<PathBuf>,
    pub history_forward: Vec<PathBuf>,
    pub view_mode: ViewMode,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub show_hidden: bool,
    pub current_folder: Option<FolderInfo>,
}

impl FileExplorer {
    /// Creates a new `FileExplorer` initialized at `initial_path`.
    pub fn new(initial_path: PathBuf) -> Self {
        let mut explorer = Self {
            current_path: initial_path.clone(),
            history_back: Vec::new(),
            history_forward: Vec::new(),
            view_mode: ViewMode::Grid,
            sort_by: SortBy::Name,
            sort_order: SortOrder::Ascending,
            show_hidden: false,
            current_folder: None,
        };

        let _ = explorer.refresh();
        explorer
    }

    /// Navigates to a new target directory, pushing the current location onto `history_back`.
    pub fn navigate(&mut self, path: PathBuf) -> Result<FolderInfo, String> {
        if self.current_folder.is_some() && self.current_path != path {
            self.history_back.push(self.current_path.clone());
            self.history_forward.clear();
        }

        self.load_directory(path)
    }

    /// Traverses backward in navigation history.
    pub fn back(&mut self) -> Result<FolderInfo, String> {
        if let Some(previous_path) = self.history_back.pop() {
            self.history_forward.push(self.current_path.clone());
            self.load_directory(previous_path)
        } else {
            Err("No back history available".to_string())
        }
    }

    /// Traverses forward in navigation history.
    pub fn forward(&mut self) -> Result<FolderInfo, String> {
        if let Some(next_path) = self.history_forward.pop() {
            self.history_back.push(self.current_path.clone());
            self.load_directory(next_path)
        } else {
            Err("No forward history available".to_string())
        }
    }

    /// Navigates up to the parent directory.
    pub fn up(&mut self) -> Result<FolderInfo, String> {
        if let Some(parent) = self.current_path.parent().map(|p| p.to_path_buf()) {
            self.navigate(parent)
        } else {
            Err("Already at root directory".to_string())
        }
    }

    /// Reloads the contents of the current directory from disk.
    pub fn refresh(&mut self) -> Result<FolderInfo, String> {
        let path = self.current_path.clone();
        self.load_directory(path)
    }

    /// Internal helper to load directory contents and apply sorting and view filters.
    fn load_directory(&mut self, path: PathBuf) -> Result<FolderInfo, String> {
        let mut folder = FolderInfo::from_path(&path, self.show_hidden)?;
        folder.sort_items(self.sort_by, self.sort_order);

        self.current_path = folder.path.clone();
        self.current_folder = Some(folder.clone());

        Ok(folder)
    }

    /// Returns true if back navigation is available.
    pub fn can_go_back(&self) -> bool {
        !self.history_back.is_empty()
    }

    /// Returns true if forward navigation is available.
    pub fn can_go_forward(&self) -> bool {
        !self.history_forward.is_empty()
    }

    /// Updates the view layout mode (List, Grid, Compact, Details).
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    /// Updates the sorting criteria and order.
    pub fn set_sort(&mut self, sort_by: SortBy, order: SortOrder) -> Result<FolderInfo, String> {
        self.sort_by = sort_by;
        self.sort_order = order;
        self.refresh()
    }

    /// Toggles visibility of hidden dot-files (`.hidden`) and refreshes directory view.
    pub fn toggle_show_hidden(&mut self) -> Result<FolderInfo, String> {
        self.show_hidden = !self.show_hidden;
        self.refresh()
    }
}
