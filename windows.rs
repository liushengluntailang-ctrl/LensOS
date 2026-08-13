//! LensOS Window Component & Window Management Engine
//!
//! Provides desktop window primitives, header titlebars, window states (normal, minimized,
//! maximized, fullscreen), snapping logic, z-index elevation stacking, and focus handling.

use crate::glass::GlassMaterial;
use crate::icons::IconType;

/// 2D Coordinate Point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// 2D Dimension Size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// 2D Bounding Rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x <= self.x + self.width && p.y >= self.y && p.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// Operating system window state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Closing,
}

/// Desktop tile snapping alignment zones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSnapPosition {
    None,
    LeftHalf,
    RightHalf,
    TopHalf,
    BottomHalf,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Window operational behavioral configuration flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowFlags {
    pub is_resizable: bool,
    pub is_draggable: bool,
    pub is_always_on_top: bool,
    pub has_glass_backdrop: bool,
    pub has_titlebar: bool,
    pub show_close: bool,
    pub show_minimize: bool,
    pub show_maximize: bool,
}

impl Default for WindowFlags {
    fn default() -> Self {
        Self {
            is_resizable: true,
            is_draggable: true,
            is_always_on_top: false,
            has_glass_backdrop: true,
            has_titlebar: true,
            show_close: true,
            show_minimize: true,
            show_maximize: true,
        }
    }
}

/// Titlebar Header component for LensOS glass windows.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowHeader {
    pub title: String,
    pub icon: Option<IconType>,
    pub height: f32,
}

impl WindowHeader {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            height: 38.0,
        }
    }
}

/// Individual LensOS Desktop Window Instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub id: u64,
    pub header: WindowHeader,
    pub bounds: Rect,
    pub restored_bounds: Rect,
    pub state: WindowState,
    pub glass_material: GlassMaterial,
    pub flags: WindowFlags,
    pub is_focused: bool,
    pub z_index: u32,
    pub min_size: Size,
    pub snap_position: WindowSnapPosition,
}

impl Window {
    pub fn new(id: u64, title: impl Into<String>, x: f32, y: f32, width: f32, height: f32) -> Self {
        let bounds = Rect::new(x, y, width, height);
        Self {
            id,
            header: WindowHeader::new(title),
            bounds,
            restored_bounds: bounds,
            state: WindowState::Normal,
            glass_material: GlassMaterial::deep_acrylic(),
            flags: WindowFlags::default(),
            is_focused: false,
            z_index: 0,
            min_size: Size::new(320.0, 240.0),
            snap_position: WindowSnapPosition::None,
        }
    }

    pub fn set_state(&mut self, state: WindowState, desktop_viewport: Rect) {
        match state {
            WindowState::Maximized => {
                if self.state != WindowState::Maximized {
                    self.restored_bounds = self.bounds;
                    self.bounds = desktop_viewport;
                }
            }
            WindowState::Normal => {
                if self.state == WindowState::Maximized {
                    self.bounds = self.restored_bounds;
                }
            }
            _ => {}
        }
        self.state = state;
    }

    pub fn contains_point(&self, p: Point) -> bool {
        self.bounds.contains(p)
    }
}

/// Window Manager coordinating z-ordering, dragging, snapping, and focus state.
#[derive(Debug, Default)]
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: u64,
    focused_id: Option<u64>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            focused_id: None,
        }
    }

    pub fn create_window(&mut self, title: &str, width: f32, height: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let offset = (self.windows.len() as f32) * 24.0;
        let mut window = Window::new(id, title, 100.0 + offset, 80.0 + offset, width, height);
        window.z_index = self.windows.len() as u32 + 1;

        self.windows.push(window);
        self.focus_window(id);
        id
    }

    pub fn focus_window(&mut self, id: u64) {
        self.focused_id = Some(id);
        let max_z = self.windows.len() as u32;

        for win in self.windows.iter_mut() {
            if win.id == id {
                win.is_focused = true;
                win.z_index = max_z + 10;
            } else {
                win.is_focused = false;
            }
        }

        self.reorder_z_indices();
    }

    fn reorder_z_indices(&mut self) {
        self.windows.sort_by_key(|w| w.z_index);
        for (i, win) in self.windows.iter_mut().enumerate() {
            win.z_index = (i + 1) as u32;
        }
    }

    pub fn windows_in_render_order(&self) -> &[Window] {
        &self.windows
    }

    pub fn close_window(&mut self, id: u64) {
        self.windows.retain(|w| w.id != id);
        if self.focused_id == Some(id) {
            self.focused_id = self.windows.last().map(|w| w.id);
        }
    }
}
