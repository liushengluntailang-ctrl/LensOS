//! LensOS Main Files Application Orchestrator (`src/files.rs`)
//!
//! Provides the primary [`FilesApp`] struct unifying navigation, file operations, sidebar quick-access,
//! search, recent access, and the LensOS dark frosted glass UI theme configuration.

use crate::explorer::{FileExplorer, ViewMode};
use crate::folder::FolderInfo;
use crate::operations::FileOperationHandler;
use crate::recent::{RecentItem, RecentTracker};
use crate::search::{SearchEngine, SearchFilter, SearchResult};
use crate::sidebar::{Sidebar, SidebarItem, SidebarLocation};
use std::env;
use std::path::PathBuf;

/// Visual styling configuration adhering to the LensOS dark frosted glass design system.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeConfig {
    /// Canvas dark void color (e.g. `#0D0F12`)
    pub background_color: String,
    /// Surface container frosted color (e.g. `rgba(22, 25, 32, 0.75)`)
    pub surface_color: String,
    /// Hairline border stroke color (e.g. `rgba(255, 255, 255, 0.08)`)
    pub border_color: String,
    /// Primary accent color (e.g. `#6366F1`)
    pub accent_color: String,
    /// Text primary color (e.g. `#F3F4F6`)
    pub text_primary: String,
    /// Text secondary muted color (e.g. `#9CA3AF`)
    pub text_secondary: String,
    /// Frosted glass backdrop blur in pixels
    pub blur_radius_px: u32,
    /// Surface corner radius in pixels
    pub corner_radius_px: u32,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            background_color: "#0D0F12".to_string(),
            surface_color: "rgba(22, 25, 32, 0.75)".to_string(),
            border_color: "rgba(255, 255, 255, 0.08)".to_string(),
            accent_color: "#6366F1".to_string(),
            text_primary: "#F3F4F6".to_string(),
            text_secondary: "#9CA3AF".to_string(),
            blur_radius_px: 20,
            corner_radius_px: 12,
        }
    }
}

/// Unified UI and state snapshot payload for LensOS compositor rendering.
#[derive(Debug, Clone)]
pub struct FilesAppRenderState {
    pub current_path: PathBuf,
    pub view_mode: ViewMode,
    pub active_folder: Option<FolderInfo>,
    pub sidebar_items: Vec<SidebarItem>,
    pub recent_items: Vec<RecentItem>,
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub theme: ThemeConfig,
}

/// Main entry point and orchestrator for the LensOS Files application.
#[derive(Debug, Clone)]
pub struct FilesApp {
    pub home_dir: PathBuf,
    pub explorer: FileExplorer,
    pub sidebar: Sidebar,
    pub search_engine: SearchEngine,
    pub operations: FileOperationHandler,
    pub recents: RecentTracker,
    pub theme: ThemeConfig,
}

impl FilesApp {
    /// Creates a new `FilesApp` centered at the specified `home_dir`.
    pub fn new(home_dir: impl Into<PathBuf>) -> Self {
        let home_path = home_dir.into();
        let canonical_home = std::fs::canonicalize(&home_path).unwrap_or(home_path);

        let explorer = FileExplorer::new(canonical_home.clone());
        let sidebar = Sidebar::new(&canonical_home);
        let search_engine = SearchEngine::new();
        let operations = FileOperationHandler::new();
        let mut recents = RecentTracker::new(25);
        let theme = ThemeConfig::default();

        // Record initial home access in recents
        recents.record_access(canonical_home.clone());

        Self {
            home_dir: canonical_home,
            explorer,
            sidebar,
            search_engine,
            operations,
            recents,
            theme,
        }
    }

    /// Creates a `FilesApp` instance using system environment home directory or fallback.
    pub fn new_with_defaults() -> Self {
        let home_dir = env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));

        Self::new(home_dir)
    }

    /// Opens a folder at the given path, updating navigation history and recent access tracker.
    pub fn open_folder(&mut self, path: PathBuf) -> Result<FolderInfo, String> {
        let folder = self.explorer.navigate(path)?;
        self.recents.record_access(folder.path.clone());
        Ok(folder)
    }

    /// Navigates to one of the sidebar locations (Home, Desktop, Documents, Downloads, Pictures).
    pub fn navigate_sidebar(&mut self, location: &SidebarLocation) -> Result<FolderInfo, String> {
        if let Some(path) = self.sidebar.select_by_location(location) {
            self.open_folder(path)
        } else {
            Err("Sidebar location not found".to_string())
        }
    }

    /// Searches for files matching query within the active directory and subdirectories.
    pub fn search(&mut self, query: &str) -> Vec<SearchResult> {
        let current_path = self.explorer.current_path.clone();
        let filter = SearchFilter {
            include_hidden: self.explorer.show_hidden,
            ..Default::default()
        };

        self.search_engine.search(&current_path, query, &filter)
    }

    /// Returns recently accessed files and pinned items.
    pub fn get_recent_files(&self) -> Vec<RecentItem> {
        self.recents.get_recents()
    }

    /// Creates a new subfolder in the current directory.
    pub fn create_folder(&mut self, folder_name: &str) -> Result<PathBuf, String> {
        let current_path = self.explorer.current_path.clone();
        let res = self.operations.create_folder(&current_path, folder_name);

        if res.success {
            let _ = self.explorer.refresh();
            let new_path = res.affected_path.unwrap_or_else(|| current_path.join(folder_name));
            self.recents.record_access(new_path.clone());
            Ok(new_path)
        } else {
            Err(res.message)
        }
    }

    /// Renames a file or folder at `target_path`.
    pub fn rename(&mut self, target_path: PathBuf, new_name: &str) -> Result<PathBuf, String> {
        let res = self.operations.rename(&target_path, new_name);

        if res.success {
            let _ = self.explorer.refresh();
            let updated_path = res.affected_path.unwrap_or_else(|| {
                target_path
                    .parent()
                    .map(|p| p.join(new_name))
                    .unwrap_or_else(|| PathBuf::from(new_name))
            });
            self.recents.record_access(updated_path.clone());
            Ok(updated_path)
        } else {
            Err(res.message)
        }
    }

    /// Deletes a file or directory at `target_path`.
    pub fn delete(&mut self, target_path: PathBuf) -> Result<(), String> {
        let res = self.operations.delete(&target_path, true);

        if res.success {
            let _ = self.explorer.refresh();
            Ok(())
        } else {
            Err(res.message)
        }
    }

    /// Copies an item from `src` to `dest`.
    pub fn copy(&mut self, src: PathBuf, dest: PathBuf) -> Result<PathBuf, String> {
        let res = self.operations.copy(&src, &dest);

        if res.success {
            let _ = self.explorer.refresh();
            let copied_path = res.affected_path.unwrap_or(dest);
            self.recents.record_access(copied_path.clone());
            Ok(copied_path)
        } else {
            Err(res.message)
        }
    }

    /// Moves an item from `src` to `dest`.
    pub fn move_item(&mut self, src: PathBuf, dest: PathBuf) -> Result<PathBuf, String> {
        let res = self.operations.move_item(&src, &dest);

        if res.success {
            let _ = self.explorer.refresh();
            let moved_path = res.affected_path.unwrap_or(dest);
            self.recents.record_access(moved_path.clone());
            Ok(moved_path)
        } else {
            Err(res.message)
        }
    }

    /// Generates a state payload for the LensOS desktop compositor renderer.
    pub fn render_state(&self) -> FilesAppRenderState {
        FilesAppRenderState {
            current_path: self.explorer.current_path.clone(),
            view_mode: self.explorer.view_mode,
            active_folder: self.explorer.current_folder.clone(),
            sidebar_items: self.sidebar.items.clone(),
            recent_items: self.get_recent_files(),
            search_query: self.search_engine.last_query.clone(),
            search_results: self.search_engine.results.clone(),
            can_go_back: self.explorer.can_go_back(),
            can_go_forward: self.explorer.can_go_forward(),
            theme: self.theme.clone(),
        }
    }
}
