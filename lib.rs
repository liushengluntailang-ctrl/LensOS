//! # LensOS Browser Core Engine (`apps/browser`)
//!
//! **LensOS v0.1 - Modular Browser Architecture**
//!
//! Lens Browser is the primary web exploration engine for LensOS, designed from first principles
//! with a minimalist layout, sophisticated dark theme, frosted glass aesthetic parameters,
//! fast navigation, and native bridges for future LensOS desktop, file manager, and Lens AI integrations.
//!
//! ## Architectural Principles
//! - **Modular Feature Isolation**: Each major browser domain (tabs, history, bookmarks, security, etc.)
//!   is implemented in an independent Rust module with dedicated state controllers and trait abstractions.
//! - **Low-Latency State Engine**: Pure Rust data structures with zero unnecessary heap allocations or unsafe code.
//! - **Frosted Glass & Dark Theme Design**: Embedded visual configurations providing theme density, blur depth,
//!   and glass opacity parameters for the LensOS desktop visual compositor.
//! - **Lens AI & OS Integration Hooks**: Context summaries, tab semantic indexing, and IPC hooks designed
//!   to expose active web state to Lens AI assistants and LensOS filesystem downloads.

pub mod bookmarks;
pub mod browser;
pub mod downloads;
pub mod history;
pub mod navigation;
pub mod search;
pub mod security;
pub mod settings;
pub mod tabs;

// Re-export primary entry struct
pub use browser::LensBrowser;

/// Convenient prelude for importing core Lens Browser structures and traits.
pub mod prelude {
    pub use crate::bookmarks::{Bookmark, BookmarkFolder, BookmarkManager};
    pub use crate::browser::{BrowserEvent, LensBrowser};
    pub use crate::downloads::{DownloadItem, DownloadManager, DownloadState};
    pub use crate::history::{HistoryItem, HistoryManager, HistoryQuery};
    pub use crate::navigation::{AddressBar, NavigationAction, NavigationController, NavigationState};
    pub use crate::search::{SearchEngine, SearchManager, SearchSuggestion};
    pub use crate::security::{PermissionState, PermissionType, SecurityLevel, SecurityManager};
    pub use crate::settings::{BrowserSettings, FrostedGlassTheme, ThemeConfig};
    pub use crate::tabs::{Tab, TabId, TabManager};
}

/// Result type used throughout the Lens Browser architecture.
pub type BrowserResult<T> = Result<T, BrowserError>;

/// Core error types for Lens Browser operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserError {
    /// Invalid URL string or malformed scheme.
    InvalidUrl(String),
    /// The requested tab was not found.
    TabNotFound(tabs::TabId),
    /// No active tab currently selected.
    NoActiveTab,
    /// Permission denied for requested resource or API.
    PermissionDenied(String),
    /// Navigation operation failed.
    NavigationFailed(String),
    /// Search engine or query configuration error.
    SearchError(String),
    /// Download task failed or interrupted.
    DownloadError(String),
    /// Storage, history, or settings load/save error.
    StorageError(String),
}

impl std::fmt::Display for BrowserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrowserError::InvalidUrl(url) => write!(f, "Invalid URL format: {}", url),
            BrowserError::TabNotFound(id) => write!(f, "Tab with ID {} not found", id.0),
            BrowserError::NoActiveTab => write!(f, "No tab is currently active"),
            BrowserError::PermissionDenied(reason) => write!(f, "Permission denied: {}", reason),
            BrowserError::NavigationFailed(msg) => write!(f, "Navigation failed: {}", msg),
            BrowserError::SearchError(msg) => write!(f, "Search error: {}", msg),
            BrowserError::DownloadError(msg) => write!(f, "Download error: {}", msg),
            BrowserError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for BrowserError {}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::*;

    #[test]
    fn test_lens_browser_initialization() {
        let mut browser = LensBrowser::new(1700000000);
        assert!(!browser.is_running);
        assert_eq!(browser.tab_manager.len(), 0);

        browser.initialize(1700000001).unwrap();
        assert!(browser.is_running);
        assert_eq!(browser.tab_manager.len(), 1);

        let active_tab = browser.tab_manager.active_tab().unwrap();
        assert_eq!(active_tab.url, "lens://newtab");
    }

    #[test]
    fn test_multi_tab_lifecycle() {
        let mut browser = LensBrowser::new(1700000000);
        browser.initialize(1700000001).unwrap();

        let tab2_id = browser.open_new_tab(Some("https://lensos.org"), 1700000002);
        assert_eq!(browser.tab_manager.len(), 2);
        assert_eq!(browser.tab_manager.active_tab().unwrap().id, tab2_id);

        browser.tab_manager.set_tab_pinned(tab2_id, true).unwrap();
        assert!(browser.tab_manager.get_tab(tab2_id).unwrap().is_pinned);

        let closed = browser.close_active_tab().unwrap();
        assert!(closed.is_some());
        assert_eq!(browser.tab_manager.len(), 1);
    }

    #[test]
    fn test_navigation_and_history() {
        let mut browser = LensBrowser::new(1700000000);
        browser.initialize(1700000001).unwrap();

        let nav_url = browser
            .navigate_active_tab(
                NavigationAction::NavigateTo("https://rust-lang.org".to_string()),
                1700000005,
            )
            .unwrap();

        assert_eq!(nav_url, "https://rust-lang.org");
        assert_eq!(browser.history_manager.len(), 1);

        let top_sites = browser.history_manager.top_sites(5);
        assert_eq!(top_sites[0].url, "https://rust-lang.org");
    }

    #[test]
    fn test_bookmarks_and_search() {
        let mut browser = LensBrowser::new(1700000000);
        browser.initialize(1700000001).unwrap();

        let bm_id = browser
            .bookmark_manager
            .add_bookmark(
                "LensOS Portal",
                "https://lensos.org",
                browser.bookmark_manager.bookmark_bar_folder_id,
                1700000010,
            )
            .unwrap();

        assert!(bm_id > 0);

        let suggestions = browser.search_manager.get_suggestions(
            "lens",
            &browser.history_manager,
            &browser.bookmark_manager,
        );
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_downloads_and_security() {
        let mut browser = LensBrowser::new(1700000000);
        browser.initialize(1700000001).unwrap();

        let dl_id = browser.download_file("https://lensos.org/iso/lensos-v0.1.iso", "lensos-v0.1.iso", 1700000020);
        assert_eq!(dl_id, 1);

        browser
            .download_manager
            .update_progress(dl_id, 500000, Some(1000000), 100000)
            .unwrap();

        let active_dls = browser.download_manager.active_downloads();
        assert_eq!(active_dls.len(), 1);

        let sec_level = browser.security_manager.evaluate_security_level("https://lensos.org");
        assert_eq!(sec_level, SecurityLevel::Secure);
    }

    #[test]
    fn test_frosted_glass_theme_and_ai_context() {
        let mut browser = LensBrowser::new(1700000000);
        browser.initialize(1700000001).unwrap();

        let theme = browser.get_ui_theme_state();
        assert_eq!(theme.blur_radius_px, 24);
        assert_eq!(theme.glass_opacity, 0.65);

        let ai_summary = browser.get_lens_ai_context_summary();
        assert!(ai_summary.contains("LensOS Browser Context"));
    }
}

