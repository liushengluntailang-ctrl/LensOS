//! LensOS Dock & Taskbar Component Engine
//!
//! Controls system dock launcher, running app indicators, system tray state
//! (wifi, battery, volume, time), pinned app shortcuts, and frosted glass dock layout.

use crate::glass::GlassMaterial;
use crate::icons::IconType;

/// Taskbar dock edge position on screen viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarPosition {
    Bottom,
    Top,
    Left,
    Right,
}

/// Category classification for taskbar entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarItemKind {
    StartMenuButton,
    AppLauncher,
    RunningTask,
    SystemTrayIcon,
    Clock,
    NotificationCenterToggle,
    Divider,
}

/// Individual item or launcher icon on the taskbar.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskbarItem {
    pub id: String,
    pub title: String,
    pub icon: IconType,
    pub is_active: bool,
    pub is_running: bool,
    pub badge_count: u32,
    pub kind: TaskbarItemKind,
}

impl TaskbarItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>, icon: IconType, kind: TaskbarItemKind) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon,
            is_active: false,
            is_running: false,
            badge_count: 0,
            kind,
        }
    }

    pub fn launcher(id: impl Into<String>, title: impl Into<String>, icon: IconType) -> Self {
        Self::new(id, title, icon, TaskbarItemKind::AppLauncher)
    }
}

/// System Tray indicators state (battery, connectivity, audio volume, clock).
#[derive(Debug, Clone, PartialEq)]
pub struct SystemTray {
    pub wifi_signal_percent: u8,
    pub battery_level_percent: u8,
    pub is_charging: bool,
    pub volume_level_percent: u8,
    pub unread_notifications: u32,
    pub clock_display: String,
}

impl Default for SystemTray {
    fn default() -> Self {
        Self {
            wifi_signal_percent: 92,
            battery_level_percent: 88,
            is_charging: true,
            volume_level_percent: 75,
            unread_notifications: 3,
            clock_display: "10:42 AM".to_string(),
        }
    }
}

/// LensOS Main Taskbar Instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Taskbar {
    pub position: TaskbarPosition,
    pub height: f32,
    pub items: Vec<TaskbarItem>,
    pub system_tray: SystemTray,
    pub glass_material: GlassMaterial,
    pub auto_hide: bool,
    pub start_menu_open: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        let mut items = Vec::new();

        items.push(TaskbarItem::new("start", "LensOS Start", IconType::AppGrid, TaskbarItemKind::StartMenuButton));
        items.push(TaskbarItem::new("div_1", "", IconType::ChevronRight, TaskbarItemKind::Divider));
        items.push(Taskbar::launcher_item("terminal", "Terminal", IconType::Terminal));
        items.push(Taskbar::launcher_item("files", "File Manager", IconType::FileManager));
        items.push(Taskbar::launcher_item("browser", "Web Browser", IconType::Browser));
        items.push(Taskbar::launcher_item("settings", "Settings", IconType::Settings));

        Self {
            position: TaskbarPosition::Bottom,
            height: 52.0,
            items,
            system_tray: SystemTray::default(),
            glass_material: GlassMaterial::taskbar_dock(),
            auto_hide: false,
            start_menu_open: false,
        }
    }

    fn launcher_item(id: &str, title: &str, icon: IconType) -> TaskbarItem {
        let mut item = TaskbarItem::launcher(id, title, icon);
        item.is_running = true;
        item
    }

    pub fn set_active_app(&mut self, app_id: &str) {
        for item in self.items.iter_mut() {
            item.is_active = item.id == app_id;
        }
    }

    pub fn toggle_start_menu(&mut self) {
        self.start_menu_open = !self.start_menu_open;
    }
}

impl Default for Taskbar {
    fn default() -> Self {
        Self::new()
    }
}
