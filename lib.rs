//! LensOS Desktop Environment Module
//!
//! A modular, dark minimalist desktop system with an elegant blue accent theme.
//!
//! # Architecture
//!
//! - [`desktop`]: Main orchestrator, theme palette (`Theme::dark_minimal()`), geometric types, and event dispatch loop.
//! - [`taskbar`]: Taskbar alignment, pinned launcher items, active window indicators, system clock, and click triggers.
//! - [`start_menu`]: Application directory index, fuzzy search filtering, category tabs, and recent file access.
//! - [`wallpaper`]: Background mode manager supporting gradients, solid dark tones, and slideshow transitions.
//! - [`window_manager`]: Window state stack, z-ordering, layout bounds, resizing, tiling, and focus management.
//! - [`notifications`]: Toast notifications, urgency routing, Do Not Disturb mode, and sidebar history logging.
//! - [`widgets`]: Desktop widgets layer featuring clock, system resource metrics, weather, notes, and music player.

pub mod desktop;
pub mod notifications;
pub mod start_menu;
pub mod taskbar;
pub mod wallpaper;
pub mod widgets;
pub mod window_manager;

// Convenience top-level re-exports
pub use desktop::{Color, Desktop, DesktopConfig, DesktopEvent, Position, Rect, Size, Theme};
pub use notifications::{
    Notification, NotificationAction, NotificationCenter, NotificationUrgency,
};
pub use start_menu::{AppItem, Category, PowerAction, StartMenu, UserProfile};
pub use taskbar::{
    Taskbar, TaskbarAlignment, TaskbarClickAction, TaskbarItem, TaskbarPosition,
};
pub use wallpaper::{Wallpaper, WallpaperFit, WallpaperManager, WallpaperMode};
pub use widgets::{Widget, WidgetManager, WidgetType};
pub use window_manager::{Window, WindowId, WindowManager, WindowState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_colors() {
        let theme = Theme::dark_minimal();
        assert_eq!(theme.accent, Color::hex(0x2563EB)); // Signature LensOS Blue Accent
        assert_eq!(theme.background, Color::hex(0x0B0F19)); // Dark Slate Background
    }

    #[test]
    fn test_desktop_initialization() {
        let config = DesktopConfig::default();
        let mut desktop = Desktop::new(config);

        assert_eq!(desktop.window_manager.windows.len(), 0);
        assert!(!desktop.start_menu.is_open());

        // Launch an app
        let win_id = desktop.launch_app("terminal");
        assert!(win_id.is_some());
        assert_eq!(desktop.window_manager.windows.len(), 1);
    }

    #[test]
    fn test_window_focus_and_tiling() {
        let mut wm = WindowManager::new(Size::new(1920.0, 1080.0));
        let win1 = wm.create_window("Terminal", "terminal", Size::new(800.0, 600.0));
        let win2 = wm.create_window("Browser", "globe", Size::new(800.0, 600.0));

        assert_eq!(wm.focused_window_id, Some(win2));

        wm.tile_left(win1);
        assert_eq!(wm.windows[0].state, WindowState::TiledLeft);
    }

    #[test]
    fn test_notifications_and_dnd() {
        let mut nc = NotificationCenter::new();
        nc.send("Test", "Body", "App", NotificationUrgency::Normal);
        assert_eq!(nc.unread_count(), 1);

        nc.toggle_dnd();
        assert!(nc.do_not_disturb);
    }
}
