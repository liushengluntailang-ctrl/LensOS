//! Taskbar management module for LensOS.
//!
//! Handles taskbar item alignment, running window indicators, notification triggers,
//! system tray clock, and user interaction hit-testing.

use crate::desktop::{Position, Rect, Size};
use crate::window_manager::WindowId;

/// Screen edge placement for the taskbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarPosition {
    Bottom,
    Top,
}

/// Alignment style for application launcher icons on the taskbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarAlignment {
    Start,
    Center,
}

/// Actions produced when a user interacts with the taskbar.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskbarClickAction {
    ToggleStartMenu,
    ToggleNotifications,
    FocusWindow(WindowId),
    LaunchApp(String),
}

/// Individual item rendered on the taskbar (Pinned app or running window entry).
#[derive(Debug, Clone, PartialEq)]
pub struct TaskbarItem {
    pub id: String,
    pub app_id: String,
    pub title: String,
    pub icon: String,
    pub is_pinned: bool,
    pub running_windows: Vec<WindowId>,
    pub is_active: bool,
    pub badge_count: usize,
}

impl TaskbarItem {
    pub fn new(app_id: &str, title: &str, icon: &str, is_pinned: bool) -> Self {
        Self {
            id: format!("taskbar-item-{}", app_id),
            app_id: app_id.to_string(),
            title: title.to_string(),
            icon: icon.to_string(),
            is_pinned,
            running_windows: Vec::new(),
            is_active: false,
            badge_count: 0,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.running_windows.is_empty()
    }
}

/// The LensOS Taskbar manager struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Taskbar {
    pub position: TaskbarPosition,
    pub alignment: TaskbarAlignment,
    pub screen_width: f32,
    pub height: f32,
    pub items: Vec<TaskbarItem>,
    pub show_clock: bool,
    pub show_start_button: bool,
    pub show_notification_tray: bool,
    pub autohide: bool,
    pub start_button_bounds: Rect,
    pub notification_tray_bounds: Rect,
    pub clock_bounds: Rect,
}

impl Taskbar {
    /// Constructs a new bottom taskbar for the given screen width.
    pub fn new_bottom(screen_width: f32) -> Self {
        let height = 48.0;
        let y_pos = 1080.0 - height; // Default height offset, dynamically updated in bounds()

        Self {
            position: TaskbarPosition::Bottom,
            alignment: TaskbarAlignment::Center,
            screen_width,
            height,
            items: Vec::new(),
            show_clock: true,
            show_start_button: true,
            show_notification_tray: true,
            autohide: false,
            start_button_bounds: Rect::new(8.0, y_pos + 4.0, 40.0, 40.0),
            notification_tray_bounds: Rect::new(screen_width - 120.0, y_pos + 4.0, 40.0, 40.0),
            clock_bounds: Rect::new(screen_width - 70.0, y_pos + 4.0, 62.0, 40.0),
        }
    }

    /// Returns the global bounding rectangle for the taskbar bar.
    pub fn bounds(&self) -> Rect {
        let y = match self.position {
            TaskbarPosition::Bottom => 1080.0 - self.height, // Assuming default screen height fallback
            TaskbarPosition::Top => 0.0,
        };
        Rect::new(0.0, y, self.screen_width, self.height)
    }

    /// Pins an app to the taskbar.
    pub fn pin_app(&mut self, app_id: &str, title: &str, icon: &str) {
        if let Some(existing) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            existing.is_pinned = true;
        } else {
            self.items.push(TaskbarItem::new(app_id, title, icon, true));
        }
    }

    /// Unpins an app from the taskbar.
    pub fn unpin_app(&mut self, app_id: &str) {
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            item.is_pinned = false;
        }
        // Remove item if no longer pinned and has no running windows
        self.items.retain(|i| i.is_pinned || !i.running_windows.is_empty());
    }

    /// Associates a new running window with an app on the taskbar.
    pub fn add_running_window(&mut self, app_id: &str, window_id: WindowId) {
        if let Some(item) = self.items.iter_mut().find(|i| i.app_id == app_id) {
            if !item.running_windows.contains(&window_id) {
                item.running_windows.push(window_id);
            }
            item.is_active = true;
        } else {
            let mut item = TaskbarItem::new(app_id, app_id, "app", false);
            item.running_windows.push(window_id);
            item.is_active = true;
            self.items.push(item);
        }
    }

    /// Disassociates a closed window from the taskbar items.
    pub fn remove_window(&mut self, window_id: WindowId) {
        for item in self.items.iter_mut() {
            item.running_windows.retain(|&id| id != window_id);
            if item.running_windows.is_empty() {
                item.is_active = false;
            }
        }
        // Retain pinned apps or apps with active windows
        self.items.retain(|i| i.is_pinned || !i.running_windows.is_empty());
    }

    /// Sets which window is currently active/focused.
    pub fn set_active_window(&mut self, active_window_id: Option<WindowId>) {
        for item in self.items.iter_mut() {
            item.is_active = match active_window_id {
                Some(win_id) => item.running_windows.contains(&win_id),
                None => false,
            };
        }
    }

    /// Recalculates screen bounds on display resize.
    pub fn resize(&mut self, new_screen_width: f32) {
        self.screen_width = new_screen_width;
        let y = 1080.0 - self.height;
        self.start_button_bounds = Rect::new(8.0, y + 4.0, 40.0, 40.0);
        self.notification_tray_bounds = Rect::new(new_screen_width - 120.0, y + 4.0, 40.0, 40.0);
        self.clock_bounds = Rect::new(new_screen_width - 70.0, y + 4.0, 62.0, 40.0);
    }

    /// Performs hit-testing on taskbar user clicks and returns appropriate TaskbarClickAction.
    pub fn handle_click(&mut self, click_pos: Position) -> Option<TaskbarClickAction> {
        let taskbar_y = 1080.0 - self.height;
        
        // 1. Check Start Menu Button Click (LensOS logo / launcher trigger)
        let start_rect = Rect::new(12.0, taskbar_y + 4.0, 40.0, 40.0);
        if start_rect.contains(click_pos) {
            return Some(TaskbarClickAction::ToggleStartMenu);
        }

        // 2. Check Notification Center Tray Click
        let notif_rect = Rect::new(self.screen_width - 130.0, taskbar_y + 4.0, 40.0, 40.0);
        if notif_rect.contains(click_pos) {
            return Some(TaskbarClickAction::ToggleNotifications);
        }

        // 3. Check Taskbar Item Launcher / Window Toggle
        let item_width = 44.0;
        let item_spacing = 6.0;
        let total_items_width = (self.items.len() as f32) * (item_width + item_spacing);

        let start_x = match self.alignment {
            TaskbarAlignment::Start => 64.0,
            TaskbarAlignment::Center => (self.screen_width - total_items_width) / 2.0,
        };

        for (idx, item) in self.items.iter().enumerate() {
            let item_x = start_x + (idx as f32) * (item_width + item_spacing);
            let item_rect = Rect::new(item_x, taskbar_y + 4.0, item_width, 40.0);

            if item_rect.contains(click_pos) {
                if let Some(&first_win_id) = item.running_windows.first() {
                    return Some(TaskbarClickAction::FocusWindow(first_win_id));
                } else {
                    return Some(TaskbarClickAction::LaunchApp(item.app_id.clone()));
                }
            }
        }

        None
    }
}
