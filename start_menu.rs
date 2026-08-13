//! Start Menu system module for LensOS.
//!
//! Provides application categorization, fuzzy search filtering, pinned quick-launch
//! shortcuts, recent files, and power control actions.

use crate::desktop::{Position, Rect};

/// Categorization tags for LensOS desktop applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    All,
    System,
    Productivity,
    Media,
    Utilities,
    Development,
    Games,
}

impl Category {
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::All => "All Apps",
            Category::System => "System",
            Category::Productivity => "Productivity",
            Category::Media => "Media",
            Category::Utilities => "Utilities",
            Category::Development => "Development",
            Category::Games => "Games",
        }
    }
}

/// Represents an application entry registered within LensOS.
#[derive(Debug, Clone, PartialEq)]
pub struct AppItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub category: Category,
    pub executable: String,
    pub is_pinned: bool,
    pub keywords: Vec<String>,
}

impl AppItem {
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        icon: &str,
        category: Category,
        executable: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            icon: icon.to_string(),
            category,
            executable: executable.to_string(),
            is_pinned: false,
            keywords: vec![name.to_lowercase(), id.to_lowercase()],
        }
    }

    /// Checks if app matches the given search filter query.
    pub fn matches_query(&self, query: &str) -> bool {
        if query.trim().is_empty() {
            return true;
        }
        let q = query.to_lowercase();
        self.name.to_lowercase().contains(&q)
            || self.description.to_lowercase().contains(&q)
            || self.keywords.iter().any(|kw| kw.contains(&q))
    }
}

/// User Profile display details for the start menu header/footer.
#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub username: String,
    pub display_name: String,
    pub avatar_icon: String,
    pub status_message: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            username: "lens_user".to_string(),
            display_name: "LensOS User".to_string(),
            avatar_icon: "user-circle".to_string(),
            status_message: "Active".to_string(),
        }
    }
}

/// Start Menu power state options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Lock,
    Sleep,
    Restart,
    Shutdown,
}

/// Start Menu Manager struct.
#[derive(Debug, Clone, PartialEq)]
pub struct StartMenu {
    pub open: bool,
    pub search_query: String,
    pub selected_category: Category,
    pub apps: Vec<AppItem>,
    pub recent_files: Vec<String>,
    pub user_profile: UserProfile,
}

impl StartMenu {
    /// Creates a new StartMenu instance.
    pub fn new() -> Self {
        Self {
            open: false,
            search_query: String::new(),
            selected_category: Category::All,
            apps: Vec::new(),
            recent_files: vec![
                "~/Documents/project_notes.md".to_string(),
                "~/Pictures/wallpaper_dark.png".to_string(),
                "~/Downloads/lensos_config.json".to_string(),
            ],
            user_profile: UserProfile::default(),
        }
    }

    /// Opens the start menu and focuses search input.
    pub fn open(&mut self) {
        self.open = true;
        self.search_query.clear();
    }

    /// Closes the start menu.
    pub fn close(&mut self) {
        self.open = false;
        self.search_query.clear();
    }

    /// Toggles start menu visibility state.
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Checks if the start menu is currently visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Sets search input query string.
    pub fn set_search_query(&mut self, query: &str) {
        self.search_query = query.to_string();
    }

    /// Registers a new application in the LensOS app index.
    pub fn register_app(&mut self, app: AppItem) {
        if !self.apps.iter().any(|a| a.id == app.id) {
            self.apps.push(app);
        }
    }

    /// Retrieves an application by ID.
    pub fn get_app(&self, app_id: &str) -> Option<&AppItem> {
        self.apps.iter().find(|a| a.id == app_id)
    }

    /// Returns applications filtered by active category and search query.
    pub fn filtered_apps(&self) -> Vec<&AppItem> {
        self.apps
            .iter()
            .filter(|app| {
                let matches_category = self.selected_category == Category::All
                    || app.category == self.selected_category;
                let matches_search = app.matches_query(&self.search_query);
                matches_category && matches_search
            })
            .collect()
    }

    /// Returns pinned application quick-shortcuts.
    pub fn pinned_apps(&self) -> Vec<&AppItem> {
        self.apps.iter().filter(|app| app.is_pinned).collect()
    }

    /// Pins an application in the start menu.
    pub fn pin_app(&mut self, app_id: &str) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == app_id) {
            app.is_pinned = true;
        }
    }

    /// Unpins an application in the start menu.
    pub fn unpin_app(&mut self, app_id: &str) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == app_id) {
            app.is_pinned = false;
        }
    }

    /// Processes clicks inside the start menu bounds and returns app_id if an app item was clicked.
    pub fn handle_click(&self, click_pos: Position, menu_bounds: Rect) -> Option<String> {
        if !menu_bounds.contains(click_pos) {
            return None;
        }

        let filtered = self.filtered_apps();
        let item_height = 48.0;
        let start_y = menu_bounds.y() + 80.0; // Header & search offset

        for (idx, app) in filtered.iter().enumerate() {
            let item_rect = Rect::new(
                menu_bounds.x() + 16.0,
                start_y + (idx as f32) * item_height,
                menu_bounds.width() - 32.0,
                item_height - 4.0,
            );

            if item_rect.contains(click_pos) {
                return Some(app.id.clone());
            }
        }

        None
    }
}

impl Default for StartMenu {
    fn default() -> Self {
        Self::new()
    }
}
