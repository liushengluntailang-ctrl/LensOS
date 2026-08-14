//! LensOS Sidebar Navigation Module (`src/sidebar.rs`)
//!
//! Manages system quick-access locations (Home, Desktop, Documents, Downloads, Pictures)
//! and custom user bookmarks for the LensOS Files left navigation pane.

use std::path::{Path, PathBuf};

/// Enumeration of standard LensOS system locations and custom paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarLocation {
    Home,
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Custom(String, PathBuf),
}

/// Represents a single quick-access item rendered in the left sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub path: PathBuf,
    pub location: SidebarLocation,
    pub badge_count: Option<usize>,
    pub is_selected: bool,
}

/// Managing component for sidebar navigation links and active location highlighting.
#[derive(Debug, Clone)]
pub struct Sidebar {
    pub items: Vec<SidebarItem>,
    pub selected_id: Option<String>,
}

impl Sidebar {
    /// Constructs the standard LensOS sidebar containing Home, Desktop, Documents, Downloads, and Pictures.
    pub fn new(home_dir: &Path) -> Self {
        let home_path = home_dir.to_path_buf();
        let desktop_path = home_path.join("Desktop");
        let documents_path = home_path.join("Documents");
        let downloads_path = home_path.join("Downloads");
        let pictures_path = home_path.join("Pictures");

        let items = vec![
            SidebarItem {
                id: "sidebar-home".to_string(),
                label: "Home".to_string(),
                icon: "home".to_string(),
                path: home_path,
                location: SidebarLocation::Home,
                badge_count: None,
                is_selected: true,
            },
            SidebarItem {
                id: "sidebar-desktop".to_string(),
                label: "Desktop".to_string(),
                icon: "monitor".to_string(),
                path: desktop_path,
                location: SidebarLocation::Desktop,
                badge_count: None,
                is_selected: false,
            },
            SidebarItem {
                id: "sidebar-documents".to_string(),
                label: "Documents".to_string(),
                icon: "file-text".to_string(),
                path: documents_path,
                location: SidebarLocation::Documents,
                badge_count: None,
                is_selected: false,
            },
            SidebarItem {
                id: "sidebar-downloads".to_string(),
                label: "Downloads".to_string(),
                icon: "download".to_string(),
                path: downloads_path,
                location: SidebarLocation::Downloads,
                badge_count: None,
                is_selected: false,
            },
            SidebarItem {
                id: "sidebar-pictures".to_string(),
                label: "Pictures".to_string(),
                icon: "image".to_string(),
                path: pictures_path,
                location: SidebarLocation::Pictures,
                badge_count: None,
                is_selected: false,
            },
        ];

        Self {
            items,
            selected_id: Some("sidebar-home".to_string()),
        }
    }

    /// Selects a sidebar item by its `SidebarLocation` enum variant and returns its path.
    pub fn select_by_location(&mut self, location: &SidebarLocation) -> Option<PathBuf> {
        let mut target_path = None;

        for item in &mut self.items {
            let is_match = match (&item.location, location) {
                (SidebarLocation::Home, SidebarLocation::Home) => true,
                (SidebarLocation::Desktop, SidebarLocation::Desktop) => true,
                (SidebarLocation::Documents, SidebarLocation::Documents) => true,
                (SidebarLocation::Downloads, SidebarLocation::Downloads) => true,
                (SidebarLocation::Pictures, SidebarLocation::Pictures) => true,
                (SidebarLocation::Custom(_, p1), SidebarLocation::Custom(_, p2)) => p1 == p2,
                _ => false,
            };

            if is_match {
                item.is_selected = true;
                self.selected_id = Some(item.id.clone());
                target_path = Some(item.path.clone());
            } else {
                item.is_selected = false;
            }
        }

        target_path
    }

    /// Selects a sidebar item by its identifier string.
    pub fn select_by_id(&mut self, id: &str) -> Option<PathBuf> {
        let mut target_path = None;

        for item in &mut self.items {
            if item.id == id {
                item.is_selected = true;
                self.selected_id = Some(item.id.clone());
                target_path = Some(item.path.clone());
            } else {
                item.is_selected = false;
            }
        }

        target_path
    }

    /// Adds a custom user location bookmark to the sidebar list.
    pub fn add_bookmark(&mut self, label: &str, path: PathBuf, icon: &str) {
        let id = format!("sidebar-custom-{}", self.items.len());
        let item = SidebarItem {
            id,
            label: label.to_string(),
            icon: icon.to_string(),
            path: path.clone(),
            location: SidebarLocation::Custom(label.to_string(), path),
            badge_count: None,
            is_selected: false,
        };
        self.items.push(item);
    }

    /// Removes a custom bookmark by ID. Returns true if removed.
    pub fn remove_bookmark(&mut self, id: &str) -> bool {
        let original_len = self.items.len();
        self.items.retain(|item| item.id != id);
        self.items.len() < original_len
    }

    /// Updates the badge counter for a specific sidebar item (e.g., number of downloads).
    pub fn set_badge_count(&mut self, id: &str, count: Option<usize>) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.badge_count = count;
        }
    }
}
