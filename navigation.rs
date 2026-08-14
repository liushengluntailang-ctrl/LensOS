//! # Navigation Controller & Address Bar Module (`navigation.rs`)
//!
//! Handles URL parsing, address bar validation, navigation history stacks (back/forward/reload),
//! LensOS internal scheme routing (`lens://`), and protocol normalization.

use crate::{BrowserError, BrowserResult};

/// Categorizes the user's intent when entering text into the address bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressInputIntent {
    /// A valid web URL (e.g., "https://lensos.org" or "example.com").
    Url(String),
    /// A search engine query (e.g., "rust modular browser architecture").
    SearchQuery(String),
    /// Internal LensOS system pages (e.g., "lens://settings", "lens://history", "lens://bookmarks", "lens://downloads").
    LensInternal(String),
    /// Local filesystem paths (e.g., "file:///home/lens/documents/index.html").
    LocalFile(String),
}

/// Address bar UI state and input validator.
#[derive(Debug, Clone)]
pub struct AddressBar {
    /// Current display string in the address bar.
    pub raw_input: String,
    /// Whether the address bar text field currently has user focus.
    pub is_focused: bool,
    /// Whether input is currently being edited.
    pub is_editing: bool,
}

impl Default for AddressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressBar {
    /// Creates a new empty AddressBar instance.
    pub fn new() -> Self {
        Self {
            raw_input: String::new(),
            is_focused: false,
            is_editing: false,
        }
    }

    /// Sets raw input string in address bar.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.raw_input = text.into();
    }

    /// Parses user address bar entry into an `AddressInputIntent`.
    pub fn parse_intent(&self) -> AddressInputIntent {
        let trimmed = self.raw_input.trim();

        if trimmed.is_empty() {
            return AddressInputIntent::LensInternal("lens://newtab".to_string());
        }

        // LensOS internal scheme check
        if trimmed.starts_with("lens://") || trimmed.starts_with("about:") {
            return AddressInputIntent::LensInternal(trimmed.to_string());
        }

        // Local file path check
        if trimmed.starts_with("file://") || trimmed.starts_with('/') || trimmed.starts_with("./") {
            return AddressInputIntent::LocalFile(trimmed.to_string());
        }

        // Detect explicit protocol scheme
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return AddressInputIntent::Url(trimmed.to_string());
        }

        // Domain heuristic: contains dot and no spaces
        if !trimmed.contains(' ') && (trimmed.contains('.') || trimmed.contains("localhost")) {
            let canonical_url = format!("https://{}", trimmed);
            return AddressInputIntent::Url(canonical_url);
        }

        // Fallback to search query
        AddressInputIntent::SearchQuery(trimmed.to_string())
    }
}

/// Represents a single navigation entry in a tab's session history stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationEntry {
    pub url: String,
    pub title: String,
    pub timestamp: u64,
}

impl NavigationEntry {
    pub fn new(url: impl Into<String>, title: impl Into<String>, timestamp: u64) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            timestamp,
        }
    }
}

/// Navigation actions triggered from UI controls or hotkeys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationAction {
    NavigateTo(String),
    GoBack,
    GoForward,
    Reload { hard: bool },
    Stop,
    GoHome,
}

/// Manages per-tab back/forward stacks and session navigation state.
#[derive(Debug, Clone, Default)]
pub struct NavigationState {
    back_stack: Vec<NavigationEntry>,
    forward_stack: Vec<NavigationEntry>,
    current_entry: Option<NavigationEntry>,
}

impl NavigationState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the active current navigation entry.
    pub fn current(&self) -> Option<&NavigationEntry> {
        self.current_entry.as_ref()
    }

    /// Returns whether backward navigation is possible.
    pub fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }

    /// Returns whether forward navigation is possible.
    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    /// Navigates to a new URL, pushing current entry onto back_stack and clearing forward_stack.
    pub fn navigate(&mut self, url: String, title: String, timestamp: u64) -> NavigationEntry {
        if let Some(prev) = self.current_entry.take() {
            self.back_stack.push(prev);
        }
        self.forward_stack.clear();

        let new_entry = NavigationEntry::new(url, title, timestamp);
        self.current_entry = Some(new_entry.clone());
        new_entry
    }

    /// Performs backward navigation through session stack.
    pub fn go_back(&mut self) -> Option<NavigationEntry> {
        if !self.can_go_back() {
            return None;
        }

        if let Some(curr) = self.current_entry.take() {
            self.forward_stack.push(curr);
        }

        let target = self.back_stack.pop()?;
        self.current_entry = Some(target.clone());
        Some(target)
    }

    /// Performs forward navigation through session stack.
    pub fn go_forward(&mut self) -> Option<NavigationEntry> {
        if !self.can_go_forward() {
            return None;
        }

        if let Some(curr) = self.current_entry.take() {
            self.back_stack.push(curr);
        }

        let target = self.forward_stack.pop()?;
        self.current_entry = Some(target.clone());
        Some(target)
    }

    /// Returns length of back history.
    pub fn back_count(&self) -> usize {
        self.back_stack.len()
    }

    /// Returns length of forward history.
    pub fn forward_count(&self) -> usize {
        self.forward_stack.len()
    }
}

/// Navigation controller coordinating address bar state and state transitions.
#[derive(Debug, Default)]
pub struct NavigationController {
    pub address_bar: AddressBar,
    pub state: NavigationState,
}

impl NavigationController {
    pub fn new() -> Self {
        Self {
            address_bar: AddressBar::new(),
            state: NavigationState::new(),
        }
    }

    /// Executes a navigation command.
    pub fn execute_action(&mut self, action: NavigationAction, current_time: u64) -> BrowserResult<Option<String>> {
        match action {
            NavigationAction::NavigateTo(raw_url) => {
                self.address_bar.set_text(&raw_url);
                let intent = self.address_bar.parse_intent();
                let canonical_url = match intent {
                    AddressInputIntent::Url(u) => u,
                    AddressInputIntent::LensInternal(u) => u,
                    AddressInputIntent::LocalFile(u) => u,
                    AddressInputIntent::SearchQuery(q) => format!("https://search.lensos.org/?q={}", q),
                };

                let entry = self.state.navigate(canonical_url.clone(), canonical_url.clone(), current_time);
                Ok(Some(entry.url))
            }
            NavigationAction::GoBack => {
                if let Some(entry) = self.state.go_back() {
                    self.address_bar.set_text(&entry.url);
                    Ok(Some(entry.url))
                } else {
                    Err(BrowserError::NavigationFailed("No back history".into()))
                }
            }
            NavigationAction::GoForward => {
                if let Some(entry) = self.state.go_forward() {
                    self.address_bar.set_text(&entry.url);
                    Ok(Some(entry.url))
                } else {
                    Err(BrowserError::NavigationFailed("No forward history".into()))
                }
            }
            NavigationAction::Reload { .. } => {
                if let Some(curr) = self.state.current() {
                    Ok(Some(curr.url.clone()))
                } else {
                    Ok(None)
                }
            }
            NavigationAction::Stop => Ok(None),
            NavigationAction::GoHome => {
                let home_url = "lens://newtab".to_string();
                self.execute_action(NavigationAction::NavigateTo(home_url), current_time)
            }
        }
    }
}
