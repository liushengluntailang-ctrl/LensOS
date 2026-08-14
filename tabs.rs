//! # Tab Management Module (`tabs.rs`)
//!
//! Provides multi-tab lifecycle management, tab state tracking, memory usage accounting,
//! and tab context extraction for Lens AI and LensOS desktop windowing.

use crate::{BrowserError, BrowserResult};

/// Unique identifier for a tab instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TabId(pub u64);

/// Represents the state and metadata of an individual browser tab.
#[derive(Debug, Clone)]
pub struct Tab {
    /// Unique tab ID.
    pub id: TabId,
    /// Current loaded or loading URL.
    pub url: String,
    /// Page title displayed on the tab header.
    pub title: String,
    /// Favicon icon URL or internal resource path.
    pub favicon_url: Option<String>,
    /// Whether the page is currently being fetched or rendered.
    pub is_loading: bool,
    /// Whether the tab is pinned to the left tab list.
    pub is_pinned: bool,
    /// Whether tab audio is explicitly muted.
    pub is_audio_muted: bool,
    /// Whether audio is actively playing in this tab.
    pub is_audio_playing: bool,
    /// Unix timestamp of when this tab was last interacted with.
    pub last_accessed_timestamp: u64,
    /// Estimated memory consumption in bytes.
    pub estimated_memory_bytes: usize,
    /// Whether the tab contents are unloaded (suspended) to conserve RAM.
    pub is_suspended: bool,
    /// Semantic text summary extracted by Lens AI for OS-wide context awareness.
    pub ai_context_summary: Option<String>,
}

impl Tab {
    /// Creates a new Tab instance with default initial values.
    pub fn new(id: TabId, url: String, timestamp: u64) -> Self {
        let title = if url == "lens://newtab" {
            "New Tab".to_string()
        } else {
            url.clone()
        };

        Self {
            id,
            url,
            title,
            favicon_url: None,
            is_loading: true,
            is_pinned: false,
            is_audio_muted: false,
            is_audio_playing: false,
            last_accessed_timestamp: timestamp,
            estimated_memory_bytes: 1024 * 1024 * 15, // ~15MB baseline state
            is_suspended: false,
            ai_context_summary: None,
        }
    }

    /// Sets the tab title and finishes loading state.
    pub fn update_page_info(&mut self, title: String, favicon_url: Option<String>) {
        self.title = title;
        self.favicon_url = favicon_url;
        self.is_loading = false;
    }

    /// Toggles the audio muting state.
    pub fn toggle_mute(&mut self) -> bool {
        self.is_audio_muted = !self.is_audio_muted;
        self.is_audio_muted
    }

    /// Suspends tab memory allocation while retaining URL and state.
    pub fn suspend(&mut self) {
        if !self.is_pinned && !self.is_audio_playing {
            self.is_suspended = true;
            self.estimated_memory_bytes = 1024 * 64; // Suspended footprint (~64KB)
        }
    }

    /// Resumes tab state from suspension.
    pub fn resume(&mut self, timestamp: u64) {
        self.is_suspended = false;
        self.last_accessed_timestamp = timestamp;
        self.estimated_memory_bytes = 1024 * 1024 * 15;
    }
}

/// Event notification types emitted by tab lifecycle changes.
#[derive(Debug, Clone)]
pub enum TabEvent {
    Created(TabId),
    Activated(TabId),
    Closed(TabId),
    Updated { id: TabId, url: String, title: String },
    Pinned { id: TabId, pinned: bool },
    Suspended(TabId),
    AiContextUpdated(TabId),
}

/// Trait implemented by desktop UI or OS compositors to observe tab lifecycle events.
pub trait TabObserver {
    fn on_tab_event(&mut self, event: TabEvent);
}

/// Manages multiple tabs, tab reordering, activation, and memory optimization.
#[derive(Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_index: Option<usize>,
    next_id: u64,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TabManager {
    /// Creates a new empty `TabManager`.
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: None,
            next_id: 1,
        }
    }

    /// Opens a new tab with the given URL and sets it as active if requested.
    pub fn create_tab(&mut self, url: String, timestamp: u64, make_active: bool) -> TabId {
        let id = TabId(self.next_id);
        self.next_id += 1;

        let tab = Tab::new(id, url, timestamp);
        self.tabs.push(tab);

        let new_index = self.tabs.len() - 1;
        if make_active || self.active_index.is_none() {
            self.active_index = Some(new_index);
        }

        id
    }

    /// Returns the currently active Tab reference.
    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_index.and_then(|idx| self.tabs.get(idx))
    }

    /// Returns the currently active mutable Tab reference.
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.active_index.and_then(|idx| self.tabs.get_mut(idx))
    }

    /// Returns the active tab index.
    pub fn active_index(&self) -> Option<usize> {
        self.active_index
    }

    /// Gets a tab by its `TabId`.
    pub fn get_tab(&self, id: TabId) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == id)
    }

    /// Gets a mutable tab reference by its `TabId`.
    pub fn get_tab_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    /// Switches active focus to the tab with the specified `TabId`.
    pub fn activate_tab(&mut self, id: TabId, current_time: u64) -> BrowserResult<()> {
        if let Some(index) = self.tabs.iter().position(|t| t.id == id) {
            self.active_index = Some(index);
            if let Some(tab) = self.tabs.get_mut(index) {
                if tab.is_suspended {
                    tab.resume(current_time);
                } else {
                    tab.last_accessed_timestamp = current_time;
                }
            }
            Ok(())
        } else {
            Err(BrowserError::TabNotFound(id))
        }
    }

    /// Closes the specified tab and adjusts active tab selection gracefully.
    pub fn close_tab(&mut self, id: TabId) -> BrowserResult<Option<TabId>> {
        let index = self
            .tabs
            .iter()
            .position(|t| t.id == id)
            .ok_or(BrowserError::TabNotFound(id))?;

        self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.active_index = None;
            Ok(None)
        } else {
            let new_active_index = if let Some(current) = self.active_index {
                if current >= self.tabs.len() {
                    self.tabs.len() - 1
                } else if current > index {
                    current - 1
                } else {
                    current
                }
            } else {
                0
            };

            self.active_index = Some(new_active_index);
            Ok(Some(self.tabs[new_active_index].id))
        }
    }

    /// Moves a tab from one index position to another.
    pub fn reorder_tab(&mut self, from_index: usize, to_index: usize) -> BrowserResult<()> {
        if from_index >= self.tabs.len() || to_index >= self.tabs.len() {
            return Err(BrowserError::NavigationFailed("Index out of bounds".into()));
        }

        let tab = self.tabs.remove(from_index);
        self.tabs.insert(to_index, tab);

        // Adjust active index tracking
        if let Some(active) = self.active_index {
            if active == from_index {
                self.active_index = Some(to_index);
            } else if from_index < active && to_index >= active {
                self.active_index = Some(active - 1);
            } else if from_index > active && to_index <= active {
                self.active_index = Some(active + 1);
            }
        }

        Ok(())
    }

    /// Toggles pinning state for a tab and moves pinned tabs to the front.
    pub fn set_tab_pinned(&mut self, id: TabId, pinned: bool) -> BrowserResult<()> {
        let tab_mut = self.get_tab_mut(id).ok_or(BrowserError::TabNotFound(id))?;
        tab_mut.is_pinned = pinned;

        // Partition pinned tabs to the start of the list
        self.tabs.sort_by_key(|t| !t.is_pinned);
        Ok(())
    }

    /// Suspends background tabs that have not been accessed for `idle_threshold_secs`.
    pub fn suspend_dormant_tabs(&mut self, current_time: u64, idle_threshold_secs: u64) -> usize {
        let mut count = 0;
        let active_id = self.active_tab().map(|t| t.id);

        for tab in self.tabs.iter_mut() {
            if Some(tab.id) != active_id
                && !tab.is_pinned
                && !tab.is_suspended
                && !tab.is_audio_playing
            {
                if current_time.saturating_sub(tab.last_accessed_timestamp) >= idle_threshold_secs {
                    tab.suspend();
                    count += 1;
                }
            }
        }
        count
    }

    /// Returns a vector of references to all current tabs.
    pub fn all_tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Returns total tab count.
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Checks if there are no open tabs.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Calculates total estimated memory consumed by all tabs in bytes.
    pub fn total_memory_usage(&self) -> usize {
        self.tabs.iter().map(|t| t.estimated_memory_bytes).sum()
    }
}
