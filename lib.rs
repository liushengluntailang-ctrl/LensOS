//! # LensOS UI System
//!
//! LensOS UI is a pure Rust, high-performance, modular frosted glass design system
//! and desktop environment module crafted for LensOS.
//!
//! ## Architecture
//! - **colors**: RGBA channels, color palettes, dark obsidian theme tokens, WCAG contrast.
//! - **typography**: Font weights, scales, layout engine, line wrapping.
//! - **glass**: Real-time translucent frosted glass physics, blur matrix kernels, specular highlights.
//! - **theme**: Theme modes, design tokens, spacing grid, elevation shadows, ThemeManager.
//! - **icons**: System vector icon paths, styling, scaling, and rendering primitives.
//! - **buttons**: Primary, secondary, ghost, frosted glass, icon buttons, states, and builders.
//! - **windows**: Desktop windows, headers, snapping, z-index elevation, and WindowManager.
//! - **taskbar**: Dock launcher, system tray indicators, app shortcuts, and active task tracking.
//! - **animations**: Easing curves, spring physics solvers, dynamic value transitions, timeline controller.

pub mod animations;
pub mod buttons;
pub mod colors;
pub mod glass;
pub mod icons;
pub mod taskbar;
pub mod theme;
pub mod typography;
pub mod windows;

/// Convenient prelude re-exporting core LensOS UI traits, structs, and tokens.
pub mod prelude {
    pub use crate::animations::{AnimationController, AnimationState, Easing, Transition};
    pub use crate::buttons::{Button, ButtonBuilder, ButtonSize, ButtonState, ButtonVariant};
    pub use crate::colors::{Color, ColorPalette};
    pub use crate::glass::{BlurKernel, GlassBlurAlgorithm, GlassLayer, GlassMaterial};
    pub use crate::icons::{Icon, IconSize, IconStyle, IconType, VectorCommand, VectorPath};
    pub use crate::taskbar::{SystemTray, Taskbar, TaskbarItem, TaskbarItemKind, TaskbarPosition};
    pub use crate::theme::{CornerRadiusScale, ElevationScale, SpacingScale, Theme, ThemeManager, ThemeMode};
    pub use crate::typography::{FontWeight, TypographyScale, TypographyStyle};
    pub use crate::windows::{Point, Rect, Size, Window, WindowFlags, WindowManager, WindowSnapPosition, WindowState};
}

use prelude::*;

/// Unified Master LensOS UI Context coordinating active desktop components.
#[derive(Debug)]
pub struct LensUiContext {
    pub theme_manager: ThemeManager,
    pub window_manager: WindowManager,
    pub taskbar: Taskbar,
    pub animation_controller: AnimationController,
    pub viewport: Rect,
}

impl LensUiContext {
    /// Initializes a new LensOS UI desktop engine instance.
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        let mut window_manager = WindowManager::new();

        // Seed initial system windows
        let w1 = window_manager.create_window("System Monitor", 640.0, 420.0);
        let w2 = window_manager.create_window("Terminal", 580.0, 360.0);
        window_manager.focus_window(w2);
        let _ = w1;

        Self {
            theme_manager: ThemeManager::new(),
            window_manager,
            taskbar: Taskbar::new(),
            animation_controller: AnimationController::new(),
            viewport: Rect::new(0.0, 0.0, viewport_width, viewport_height),
        }
    }

    /// Advances desktop timeline by delta time `dt_secs`.
    pub fn tick(&mut self, dt_secs: f32) {
        self.animation_controller.tick_all(dt_secs);
    }

    /// Resizes OS desktop viewport layout.
    pub fn resize_viewport(&mut self, width: f32, height: f32) {
        self.viewport = Rect::new(0.0, 0.0, width, height);
    }
}
