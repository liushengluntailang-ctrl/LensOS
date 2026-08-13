//! Core Desktop Environment module for LensOS.
//!
//! Provides the primary desktop orchestrator, geometric primitives,
//! dark minimalist theme definition, and event dispatch logic.

use crate::notifications::NotificationCenter;
use crate::start_menu::StartMenu;
use crate::taskbar::Taskbar;
use crate::wallpaper::WallpaperManager;
use crate::widgets::WidgetManager;
use crate::window_manager::{WindowId, WindowManager};

/// RGBA Color representation for LensOS UI styling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    /// Creates a new RGBA color.
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque RGB color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Constructs a color from a 24-bit hex integer (e.g. `0x2563EB`).
    pub const fn hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self { r, g, b, a: 1.0 }
    }

    /// Formats the color as a CSS-style RGBA string.
    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
    }

    /// Formats the color as a hex string `#RRGGBB`.
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Adjusts the alpha channel opacity.
    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a.clamp(0.0, 1.0);
        self
    }
}

/// 2D Screen Position coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 2D Screen Dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// 2D Rectangle representing a bounded screen region.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub position: Position,
    pub size: Size,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Position { x, y },
            size: Size { width, height },
        }
    }

    /// Checks if a 2D point falls within this rectangle.
    pub fn contains(&self, p: Position) -> bool {
        p.x >= self.position.x
            && p.x <= self.position.x + self.size.width
            && p.y >= self.position.y
            && p.y <= self.position.y + self.size.height
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }
    pub fn y(&self) -> f32 {
        self.position.y
    }
    pub fn width(&self) -> f32 {
        self.size.width
    }
    pub fn height(&self) -> f32 {
        self.size.height
    }
}

/// LensOS Dark Minimalist Theme Palette with Blue Accent.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// Main background color (`#0B0F19` - Deep Slate)
    pub background: Color,
    /// Container and panel background (`#111827` - Slate Gray)
    pub surface: Color,
    /// Elevated cards, popovers, and taskbar surface (`#1F2937`)
    pub surface_variant: Color,
    /// Primary LensOS Blue Accent (`#2563EB` - Royal Blue)
    pub accent: Color,
    /// Hover blue accent (`#3B82F6` - Bright Blue)
    pub accent_hover: Color,
    /// Active blue accent (`#1D4ED8` - Deep Blue)
    pub accent_active: Color,
    /// Subtle accent tint for selections and glow effects
    pub accent_subtle: Color,
    /// Primary text color (`#F9FAFB` - Pure Soft White)
    pub text_primary: Color,
    /// Secondary muted text color (`#9CA3AF` - Cool Gray)
    pub text_secondary: Color,
    /// Border color (`#374151` - Subtle Divider)
    pub border: Color,
    /// Corner radius for standard windows and cards (in pixels)
    pub corner_radius: f32,
    /// Panel acrylic opacity for dark glassmorphism
    pub panel_opacity: f32,
}

impl Theme {
    /// Constructs the signature LensOS Dark Minimalist Theme with Blue Accent.
    pub fn dark_minimal() -> Self {
        Self {
            background: Color::hex(0x0B0F19),      // Deep obsidian slate
            surface: Color::hex(0x111827),         // Dark gray surface
            surface_variant: Color::hex(0x1F2937), // Elevated panel background
            accent: Color::hex(0x2563EB),          // Elegant LensOS Blue
            accent_hover: Color::hex(0x3B82F6),    // Bright interactive blue
            accent_active: Color::hex(0x1D4ED8),   // Pressed blue state
            accent_subtle: Color::rgba(37, 99, 235, 0.15), // Subtle selection highlight
            text_primary: Color::hex(0xF9FAFB),    // High-contrast primary text
            text_secondary: Color::hex(0x9CA3AF),  // Muted secondary text
            border: Color::hex(0x374151),          // Hairline dark border
            corner_radius: 12.0,
            panel_opacity: 0.92,
        }
    }

    /// Constructs the LensOS Frosted Glass translucent theme palette.
    pub fn frosted_glass() -> Self {
        Self {
            background: Color::hex(0x020617),      // Slate-950 deep obsidian
            surface: Color::rgba(255, 255, 255, 0.05), // Translucent white/5
            surface_variant: Color::rgba(0, 0, 0, 0.40), // Translucent black/40
            accent: Color::hex(0x2563EB),          // Blue-600
            accent_hover: Color::hex(0x3B82F6),    // Blue-500
            accent_active: Color::hex(0x1D4ED8),   // Blue-700
            accent_subtle: Color::rgba(37, 99, 235, 0.20),
            text_primary: Color::hex(0xFFFFFF),    // Pure white
            text_secondary: Color::hex(0x94A3B8),  // Slate-400
            border: Color::rgba(255, 255, 255, 0.10), // Translucent white/10
            corner_radius: 16.0,
            panel_opacity: 0.40,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark_minimal()
    }
}

/// Global Desktop Configuration Settings.
#[derive(Debug, Clone, PartialEq)]
pub struct DesktopConfig {
    pub display_size: Size,
    pub theme: Theme,
    pub show_widgets_layer: bool,
    pub enable_blur: bool,
    pub scaling_factor: f32,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            display_size: Size::new(1920.0, 1080.0),
            theme: Theme::dark_minimal(),
            show_widgets_layer: true,
            enable_blur: true,
            scaling_factor: 1.0,
        }
    }
}

/// Desktop Event representation for input handling.
#[derive(Debug, Clone, PartialEq)]
pub enum DesktopEvent {
    MouseClick { position: Position, button: u8 },
    MouseMove { position: Position },
    KeyPress { key_code: u32, modifiers: u32 },
    WindowClosed { window_id: WindowId },
    AppLaunchRequested { app_id: String },
    ToggleStartMenu,
    ToggleNotificationCenter,
    Tick { delta_time_secs: f32 },
}

/// Central Desktop Engine managing all modular desktop components.
#[derive(Debug)]
pub struct Desktop {
    pub config: DesktopConfig,
    pub window_manager: WindowManager,
    pub taskbar: Taskbar,
    pub start_menu: StartMenu,
    pub wallpaper_manager: WallpaperManager,
    pub notification_center: NotificationCenter,
    pub widget_manager: WidgetManager,
    pub cursor_position: Position,
    pub active_time_secs: f64,
}

impl Desktop {
    /// Instantiates a new modular Desktop environment with default config.
    pub fn new(config: DesktopConfig) -> Self {
        let display_size = config.display_size;
        
        let mut desktop = Self {
            config: config.clone(),
            window_manager: WindowManager::new(display_size),
            taskbar: Taskbar::new_bottom(display_size.width),
            start_menu: StartMenu::new(),
            wallpaper_manager: WallpaperManager::new_gradient(
                config.theme.background,
                Color::hex(0x050811), // Subtle deep dark blue-black gradient
            ),
            notification_center: NotificationCenter::new(),
            widget_manager: WidgetManager::new_default_layout(display_size),
            cursor_position: Position::new(0.0, 0.0),
            active_time_secs: 0.0,
        };

        // Initialize default pinned applications in start menu & taskbar
        desktop.bootstrap_default_apps();
        desktop
    }

    /// Bootstraps standard system applications.
    fn bootstrap_default_apps(&mut self) {
        use crate::start_menu::{AppItem, Category};

        let apps = vec![
            AppItem::new("files", "Files", "LensOS File Explorer", "folder", Category::System, "lensos-files"),
            AppItem::new("terminal", "Terminal", "LensOS System Command Line", "terminal", Category::Development, "lensos-terminal"),
            AppItem::new("browser", "Browser", "LensOS Web Browser", "globe", Category::Productivity, "lensos-browser"),
            AppItem::new("settings", "Settings", "LensOS System Preferences", "settings", Category::System, "lensos-settings"),
        ];

        for app in apps {
            self.start_menu.register_app(app.clone());
            self.taskbar.pin_app(&app.id, &app.name, &app.icon);
        }
    }

    /// Handles incoming desktop events and dispatches to appropriate sub-modules.
    pub fn handle_event(&mut self, event: DesktopEvent) {
        match event {
            DesktopEvent::MouseMove { position } => {
                self.cursor_position = position;
            }
            DesktopEvent::MouseClick { position, button: _ } => {
                self.cursor_position = position;

                // 1. Check Taskbar interaction
                if self.taskbar.bounds().contains(position) {
                    if let Some(action) = self.taskbar.handle_click(position) {
                        self.process_taskbar_action(action);
                    }
                    return;
                }

                // 2. Check Start Menu interaction if open
                if self.start_menu.is_open() {
                    let menu_bounds = Rect::new(16.0, self.config.display_size.height - 560.0 - 56.0, 420.0, 550.0);
                    if !menu_bounds.contains(position) {
                        self.start_menu.close();
                    } else if let Some(app_id) = self.start_menu.handle_click(position, menu_bounds) {
                        self.launch_app(&app_id);
                        self.start_menu.close();
                    }
                    return;
                }

                // 3. Check Window interactions
                if let Some(window_id) = self.window_manager.window_at_position(position) {
                    self.window_manager.focus_window(window_id);
                    self.window_manager.handle_click(window_id, position);
                }
            }
            DesktopEvent::ToggleStartMenu => {
                self.start_menu.toggle();
            }
            DesktopEvent::ToggleNotificationCenter => {
                self.notification_center.toggle();
            }
            DesktopEvent::AppLaunchRequested { app_id } => {
                self.launch_app(&app_id);
            }
            DesktopEvent::WindowClosed { window_id } => {
                self.window_manager.close_window(window_id);
                self.taskbar.remove_window(window_id);
            }
            DesktopEvent::KeyPress { key_code, modifiers: _ } => {
                // Escape key closes start menu or notification center
                if key_code == 27 {
                    self.start_menu.close();
                    self.notification_center.close();
                }
            }
            DesktopEvent::Tick { delta_time_secs } => {
                self.update(delta_time_secs);
            }
        }
    }

    /// Launches an application by ID and spawns its main window.
    pub fn launch_app(&mut self, app_id: &str) -> Option<WindowId> {
        if let Some(app) = self.start_menu.get_app(app_id).cloned() {
            let win_id = self.window_manager.create_window(
                &app.name,
                &app.icon,
                Size::new(960.0, 600.0),
            );
            self.taskbar.add_running_window(app_id, win_id);
            
            // Post notification for app launch
            self.notification_center.send(
                &format!("Launched {}", app.name),
                &format!("Application {} is now active.", app.name),
                "LensOS System",
                crate::notifications::NotificationUrgency::Low,
            );

            Some(win_id)
        } else {
            None
        }
    }

    /// Helper to process actions resulting from taskbar clicks.
    fn process_taskbar_action(&mut self, action: crate::taskbar::TaskbarClickAction) {
        use crate::taskbar::TaskbarClickAction;
        match action {
            TaskbarClickAction::ToggleStartMenu => {
                self.start_menu.toggle();
            }
            TaskbarClickAction::ToggleNotifications => {
                self.notification_center.toggle();
            }
            TaskbarClickAction::FocusWindow(win_id) => {
                if self.window_manager.is_minimized(win_id) {
                    self.window_manager.restore_window(win_id);
                }
                self.window_manager.focus_window(win_id);
            }
            TaskbarClickAction::LaunchApp(app_id) => {
                self.launch_app(&app_id);
            }
        }
    }

    /// Updates desktop sub-modules on each clock tick.
    pub fn update(&mut self, delta_time_secs: f32) {
        self.active_time_secs += delta_time_secs as f64;
        self.wallpaper_manager.update(delta_time_secs);
        self.widget_manager.update(delta_time_secs);
        self.notification_center.update(delta_time_secs);
    }

    /// Adjusts desktop layout when screen resolution changes.
    pub fn resize_display(&mut self, new_size: Size) {
        self.config.display_size = new_size;
        self.window_manager.set_screen_size(new_size);
        self.taskbar.resize(new_size.width);
        self.widget_manager.reposition_for_screen(new_size);
    }
}
