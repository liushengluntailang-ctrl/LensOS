//! # Master Browser Engine (`browser.rs`)
//!
//! Defines `LensBrowser`, the central coordinator orchestrating tabs, navigation,
//! bookmarks, history, downloads, search, security, settings, and LensOS system bridges.

use crate::bookmarks::BookmarkManager;
use crate::downloads::{DownloadId, DownloadManager};
use crate::history::HistoryManager;
use crate::navigation::{NavigationAction, NavigationController};
use crate::search::SearchManager;
use crate::security::SecurityManager;
use crate::settings::{BrowserSettings, FrostedGlassTheme};
use crate::tabs::{TabId, TabManager};
use crate::{BrowserError, BrowserResult};

/// System events emitted by `LensBrowser` for desktop integration.
#[derive(Debug, Clone)]
pub enum BrowserEvent {
    TabChanged(Option<TabId>),
    UrlNavigated { tab_id: TabId, url: String },
    DownloadStarted(DownloadId),
    SettingsUpdated,
    ShutdownTriggered,
}

/// The Lens Browser application engine for LensOS v0.1.
#[derive(Debug)]
pub struct LensBrowser {
    /// LensOS Browser version string.
    pub version: String,
    /// Whether the browser engine is initialized and active.
    pub is_running: bool,
    /// Engine startup Unix timestamp.
    pub start_timestamp: u64,

    /// Subsystem controllers
    pub tab_manager: TabManager,
    pub navigation: NavigationController,
    pub bookmark_manager: BookmarkManager,
    pub history_manager: HistoryManager,
    pub download_manager: DownloadManager,
    pub search_manager: SearchManager,
    pub settings: BrowserSettings,
    pub security_manager: SecurityManager,
}

impl Default for LensBrowser {
    fn default() -> Self {
        Self::new(1700000000)
    }
}

impl LensBrowser {
    /// Constructs a new `LensBrowser` instance.
    pub fn new(timestamp: u64) -> Self {
        Self {
            version: "0.1.0-alpha".to_string(),
            is_running: false,
            start_timestamp: timestamp,
            tab_manager: TabManager::new(),
            navigation: NavigationController::new(),
            bookmark_manager: BookmarkManager::new(),
            history_manager: HistoryManager::new(),
            download_manager: DownloadManager::new(),
            search_manager: SearchManager::new(),
            settings: BrowserSettings::new(),
            security_manager: SecurityManager::new(),
        }
    }

    /// Initializes browser subsystems and opens default home tab.
    pub fn initialize(&mut self, current_time: u64) -> BrowserResult<()> {
        self.is_running = true;
        let initial_url = self.settings.homepage_url.clone();
        self.tab_manager.create_tab(initial_url, current_time, true);
        Ok(())
    }

    /// Opens a new tab with specified URL or default home page.
    pub fn open_new_tab(&mut self, url: Option<&str>, current_time: u64) -> TabId {
        let target_url = url.unwrap_or(&self.settings.homepage_url);
        self.tab_manager.create_tab(target_url.to_string(), current_time, true)
    }

    /// Closes currently active tab.
    pub fn close_active_tab(&mut self) -> BrowserResult<Option<TabId>> {
        if let Some(tab) = self.tab_manager.active_tab() {
            let id = tab.id;
            self.tab_manager.close_tab(id)
        } else {
            Err(BrowserError::NoActiveTab)
        }
    }

    /// Executes a navigation command on the active tab (e.g. NavigateTo, GoBack, GoForward, Reload).
    pub fn navigate_active_tab(&mut self, action: NavigationAction, current_time: u64) -> BrowserResult<String> {
        let target_url_opt = self.navigation.execute_action(action, current_time)?;
        let url = target_url_opt.ok_or_else(|| BrowserError::NavigationFailed("No URL produced".into()))?;

        if let Some(tab) = self.tab_manager.active_tab_mut() {
            tab.url = url.clone();
            tab.is_loading = true;
            tab.last_accessed_timestamp = current_time;
            
            // Log to history
            self.history_manager.record_visit(&url, &tab.title, false, current_time);
        } else {
            return Err(BrowserError::NoActiveTab);
        }

        Ok(url)
    }

    /// Toggles bookmark state for current active tab URL.
    pub fn toggle_bookmark_active_tab(&mut self, current_time: u64) -> BrowserResult<bool> {
        if let Some(tab) = self.tab_manager.active_tab() {
            let is_bookmarked = self
                .bookmark_manager
                .toggle_bookmark(&tab.title, &tab.url, current_time);
            Ok(is_bookmarked)
        } else {
            Err(BrowserError::NoActiveTab)
        }
    }

    /// Triggers a new file download.
    pub fn download_file(&mut self, url: &str, file_name: &str, current_time: u64) -> DownloadId {
        self.download_manager.start_download(url, file_name, current_time)
    }

    /// Generates structured context information for Lens AI assistant integration.
    pub fn get_lens_ai_context_summary(&self) -> String {
        let active_tab_info = if let Some(tab) = self.tab_manager.active_tab() {
            format!("Active Tab: \"{}\" ({})", tab.title, tab.url)
        } else {
            "No active tab".to_string()
        };

        format!(
            "LensOS Browser Context v{}\n\
             {}\n\
             Open Tabs Count: {}\n\
             Total Saved Bookmarks: {}\n\
             Total Visits Logged: {}",
            self.version,
            active_tab_info,
            self.tab_manager.len(),
            self.bookmark_manager.total_bookmarks(),
            self.history_manager.len()
        )
    }

    /// Exposes Frosted Glass theme composition parameters for the LensOS desktop visual manager.
    pub fn get_ui_theme_state(&self) -> &FrostedGlassTheme {
        &self.settings.theme.frosted_glass
    }

    /// Gracefully shuts down browser engine and releases resources.
    pub fn shutdown(&mut self) -> BrowserResult<()> {
        self.is_running = false;
        Ok(())
    }
}
