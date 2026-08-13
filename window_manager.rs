//! Window Manager system module for LensOS.
//!
//! Manages window life cycles, stack z-ordering, focus state transitions,
//! bounding geometry adjustments, dragging/resizing, and edge snapping.

use crate::desktop::{Position, Rect, Size};

/// Unique numerical identifier for an open window.
pub type WindowId = u64;

/// Display state layout mode for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    TiledLeft,
    TiledRight,
}

/// Represents an individual window instance managed by LensOS.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub icon: String,
    pub bounds: Rect,
    pub restored_bounds: Rect,
    pub state: WindowState,
    pub is_focused: bool,
    pub is_always_on_top: bool,
    pub is_resizable: bool,
    pub opacity: f32,
    pub min_size: Size,
    pub max_size: Option<Size>,
}

impl Window {
    pub fn new(id: WindowId, title: &str, icon: &str, bounds: Rect) -> Self {
        Self {
            id,
            title: title.to_string(),
            icon: icon.to_string(),
            bounds,
            restored_bounds: bounds,
            state: WindowState::Normal,
            is_focused: false,
            is_always_on_top: false,
            is_resizable: true,
            opacity: 1.0,
            min_size: Size::new(320.0, 200.0),
            max_size: None,
        }
    }

    /// Header titlebar region rect (used for dragging/moving).
    pub fn titlebar_rect(&self) -> Rect {
        Rect::new(self.bounds.x(), self.bounds.y(), self.bounds.width(), 36.0)
    }

    /// Close button rect inside window titlebar.
    pub fn close_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x() + self.bounds.width() - 36.0,
            self.bounds.y() + 4.0,
            28.0,
            28.0,
        )
    }

    /// Maximize button rect inside window titlebar.
    pub fn maximize_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x() + self.bounds.width() - 68.0,
            self.bounds.y() + 4.0,
            28.0,
            28.0,
        )
    }

    /// Minimize button rect inside window titlebar.
    pub fn minimize_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x() + self.bounds.width() - 100.0,
            self.bounds.y() + 4.0,
            28.0,
            28.0,
        )
    }
}

/// Window Manager handling z-ordered stack of active windows and layout logic.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowManager {
    pub windows: Vec<Window>,
    pub focused_window_id: Option<WindowId>,
    pub screen_size: Size,
    pub taskbar_height: f32,
    next_window_id: WindowId,
}

impl WindowManager {
    /// Creates a new WindowManager for given screen dimensions.
    pub fn new(screen_size: Size) -> Self {
        Self {
            windows: Vec::new(),
            focused_window_id: None,
            screen_size,
            taskbar_height: 48.0,
            next_window_id: 1,
        }
    }

    /// Available usable work area excluding taskbar.
    pub fn usable_work_area(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            self.screen_size.width,
            self.screen_size.height - self.taskbar_height,
        )
    }

    /// Creates and opens a new window, returning its assigned WindowId.
    pub fn create_window(&mut self, title: &str, icon: &str, size: Size) -> WindowId {
        let id = self.next_window_id;
        self.next_window_id += 1;

        // Cascade window spawn location based on existing window count
        let offset = ((self.windows.len() % 8) as f32) * 28.0;
        let work_area = self.usable_work_area();
        let initial_x = work_area.x() + 60.0 + offset;
        let initial_y = work_area.y() + 60.0 + offset;

        let bounds = Rect::new(initial_x, initial_y, size.width, size.height);
        let mut window = Window::new(id, title, icon, bounds);
        window.is_focused = true;

        // Defocus existing windows
        for win in self.windows.iter_mut() {
            win.is_focused = false;
        }

        self.windows.push(window);
        self.focused_window_id = Some(id);
        id
    }

    /// Focuses a window by ID and brings it to top of z-stack.
    pub fn focus_window(&mut self, id: WindowId) {
        let mut found_idx = None;
        for (idx, win) in self.windows.iter_mut().enumerate() {
            if win.id == id {
                win.is_focused = true;
                if win.state == WindowState::Minimized {
                    win.state = WindowState::Normal;
                }
                found_idx = Some(idx);
            } else {
                win.is_focused = false;
            }
        }

        if let Some(idx) = found_idx {
            let win = self.windows.remove(idx);
            self.windows.push(win); // Bring to front of vector (top z-index)
            self.focused_window_id = Some(id);
        }
    }

    /// Closes a window by ID.
    pub fn close_window(&mut self, id: WindowId) -> bool {
        let len_before = self.windows.len();
        self.windows.retain(|w| w.id != id);

        if self.focused_window_id == Some(id) {
            self.focused_window_id = self.windows.last().map(|w| w.id);
            if let Some(new_focused_id) = self.focused_window_id {
                if let Some(w) = self.windows.iter_mut().find(|w| w.id == new_focused_id) {
                    w.is_focused = true;
                }
            }
        }

        self.windows.len() < len_before
    }

    /// Minimizes a window.
    pub fn minimize_window(&mut self, id: WindowId) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            win.state = WindowState::Minimized;
            win.is_focused = false;
        }
        if self.focused_window_id == Some(id) {
            self.focused_window_id = self
                .windows
                .iter()
                .filter(|w| w.state != WindowState::Minimized)
                .last()
                .map(|w| w.id);
        }
    }

    /// Maximizes or restores a window.
    pub fn toggle_maximize(&mut self, id: WindowId) {
        let work_area = self.usable_work_area();
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            match win.state {
                WindowState::Maximized => {
                    win.bounds = win.restored_bounds;
                    win.state = WindowState::Normal;
                }
                _ => {
                    win.restored_bounds = win.bounds;
                    win.bounds = work_area;
                    win.state = WindowState::Maximized;
                }
            }
        }
    }

    /// Restores a window to its normal bounds.
    pub fn restore_window(&mut self, id: WindowId) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            win.bounds = win.restored_bounds;
            win.state = WindowState::Normal;
        }
    }

    /// Tiles window to left half of screen.
    pub fn tile_left(&mut self, id: WindowId) {
        let work_area = self.usable_work_area();
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            win.restored_bounds = win.bounds;
            win.bounds = Rect::new(
                work_area.x(),
                work_area.y(),
                work_area.width() / 2.0,
                work_area.height(),
            );
            win.state = WindowState::TiledLeft;
        }
    }

    /// Tiles window to right half of screen.
    pub fn tile_right(&mut self, id: WindowId) {
        let work_area = self.usable_work_area();
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            win.restored_bounds = win.bounds;
            win.bounds = Rect::new(
                work_area.x() + work_area.width() / 2.0,
                work_area.y(),
                work_area.width() / 2.0,
                work_area.height(),
            );
            win.state = WindowState::TiledRight;
        }
    }

    /// Relocates window position.
    pub fn move_window(&mut self, id: WindowId, new_pos: Position) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            if win.state == WindowState::Normal {
                win.bounds.position = new_pos;
                win.restored_bounds.position = new_pos;
            }
        }
    }

    /// Resizes window dimensions.
    pub fn resize_window(&mut self, id: WindowId, new_size: Size) {
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == id) {
            if win.state == WindowState::Normal && win.is_resizable {
                let clamped_width = new_size.width.max(win.min_size.width);
                let clamped_height = new_size.height.max(win.min_size.height);
                win.bounds.size = Size::new(clamped_width, clamped_height);
                win.restored_bounds.size = win.bounds.size;
            }
        }
    }

    /// Returns topmost window under given mouse position.
    pub fn window_at_position(&self, pos: Position) -> Option<WindowId> {
        // Iterate backwards from highest z-index
        for win in self.windows.iter().rev() {
            if win.state != WindowState::Minimized && win.bounds.contains(pos) {
                return Some(win.id);
            }
        }
        None
    }

    /// Handles titlebar click actions (Close, Maximize, Minimize).
    pub fn handle_click(&mut self, id: WindowId, click_pos: Position) {
        if let Some(win) = self.windows.iter().find(|w| w.id == id) {
            if win.close_button_rect().contains(click_pos) {
                self.close_window(id);
                return;
            }
            if win.maximize_button_rect().contains(click_pos) {
                self.toggle_maximize(id);
                return;
            }
            if win.minimize_button_rect().contains(click_pos) {
                self.minimize_window(id);
                return;
            }
        }
    }

    /// Checks if window is currently minimized.
    pub fn is_minimized(&self, id: WindowId) -> bool {
        self.windows
            .iter()
            .find(|w| w.id == id)
            .map(|w| w.state == WindowState::Minimized)
            .unwrap_or(false)
    }

    /// Updates screen dimensions on resize.
    pub fn set_screen_size(&mut self, new_size: Size) {
        self.screen_size = new_size;
    }
}
