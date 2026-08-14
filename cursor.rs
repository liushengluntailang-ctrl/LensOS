//! LensOS v0.2 Mouse Cursor Subsystem
//!
//! Provides cursor bitmaps, shape definitions, hotspot calculations,
//! state tracking, and software cursor rendering routines.

use crate::framebuffer::{Color, Framebuffer};

/// Available cursor pointer shapes in LensOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Arrow,
    Pointer,
    Text,
    Crosshair,
    Move,
    ResizeHorizontal,
    ResizeVertical,
    Busy,
    Hidden,
}

impl Default for CursorShape {
    fn default() -> Self {
        CursorShape::Arrow
    }
}

/// Standard 16x16 / 12x19 Arrow Cursor Bitmap.
/// '.' = Transparent, 'X' = Black Border, '#' = White Fill
const ARROW_BITMAP: [&str; 19] = [
    "X...............",
    "XX..............",
    "X#X.............",
    "X##X............",
    "X###X...........",
    "X####X..........",
    "X#####X.........",
    "X######X........",
    "X#######X.......",
    "X########X......",
    "X#####XXXX......",
    "X##X##X.........",
    "X#X.X##X........",
    "XX...X##X.......",
    "X.....X##X......",
    ".......X##X.....",
    "........XX......",
    "................",
    "................",
];

const POINTER_BITMAP: [&str; 16] = [
    "...XX...........",
    "..X##X..........",
    "..X##X...XX.....",
    "..X##X..X##X....",
    "..X##X.X####X...",
    "..X##X.X####X...",
    ".X####X######X..",
    "X#############X.",
    "X#############X.",
    ".X############X.",
    "..X###########X.",
    "...X#########X..",
    "...X#########X..",
    "....X#######X...",
    ".....XXXXXXX....",
    "................",
];

const TEXT_BEAM_BITMAP: [&str; 16] = [
    "XXXXX.XXXXX.....",
    "..X.....X.......",
    "....X.X.........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    ".....X..........",
    "....X.X.........",
    "..X.....X.......",
    "XXXXX.XXXXX.....",
    "................",
];

const CROSSHAIR_BITMAP: [&str; 16] = [
    ".......X........",
    ".......X........",
    ".......X........",
    "................",
    "................",
    "................",
    "...X.......X....",
    "XXX.........XXX.",
    "...X.......X....",
    "................",
    "................",
    "................",
    ".......X........",
    ".......X........",
    ".......X........",
    "................",
];

/// Tracks live cursor position and button status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub prev_x: i32,
    pub prev_y: i32,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub scroll_delta: i32,
}

impl MouseState {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            prev_x: x,
            prev_y: y,
            left_button: false,
            right_button: false,
            middle_button: false,
            scroll_delta: 0,
        }
    }

    /// Updates cursor position with delta movement and clamps to display boundaries.
    pub fn update_pos(&mut self, dx: i32, dy: i32, max_w: usize, max_h: usize) {
        self.prev_x = self.x;
        self.prev_y = self.y;

        self.x = (self.x + dx).clamp(0, (max_w.saturating_sub(1)) as i32);
        self.y = (self.y + dy).clamp(0, (max_h.saturating_sub(1)) as i32);
    }

    /// Sets absolute cursor position.
    pub fn set_pos(&mut self, x: i32, y: i32, max_w: usize, max_h: usize) {
        self.prev_x = self.x;
        self.prev_y = self.y;

        self.x = x.clamp(0, (max_w.saturating_sub(1)) as i32);
        self.y = y.clamp(0, (max_h.saturating_sub(1)) as i32);
    }
}

/// Mouse Cursor manager and renderer.
pub struct Cursor {
    shape: CursorShape,
    visible: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            shape: CursorShape::Arrow,
            visible: true,
        }
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }

    pub fn set_shape(&mut self, shape: CursorShape) {
        self.shape = shape;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Returns the hotspot offset (hot_x, hot_y) for the current shape.
    pub fn hotspot(&self) -> (i32, i32) {
        match self.shape {
            CursorShape::Arrow => (0, 0),
            CursorShape::Pointer => (4, 0),
            CursorShape::Text => (5, 7),
            CursorShape::Crosshair => (7, 7),
            CursorShape::Move => (8, 8),
            CursorShape::ResizeHorizontal => (8, 4),
            CursorShape::ResizeVertical => (4, 8),
            CursorShape::Busy => (8, 8),
            CursorShape::Hidden => (0, 0),
        }
    }

    /// Renders the mouse cursor directly onto the target framebuffer at (pos_x, pos_y).
    pub fn render_to_framebuffer(&self, fb: &mut Framebuffer, pos_x: usize, pos_y: usize) {
        if !self.visible || self.shape == CursorShape::Hidden {
            return;
        }

        let (hot_x, hot_y) = self.hotspot();
        let draw_x = (pos_x as i32) - hot_x;
        let draw_y = (pos_y as i32) - hot_y;

        let bitmap: &[&str] = match self.shape {
            CursorShape::Arrow | CursorShape::Move | CursorShape::ResizeHorizontal | CursorShape::ResizeVertical => &ARROW_BITMAP,
            CursorShape::Pointer => &POINTER_BITMAP,
            CursorShape::Text => &TEXT_BEAM_BITMAP,
            CursorShape::Crosshair | CursorShape::Busy => &CROSSHAIR_BITMAP,
            CursorShape::Hidden => return,
        };

        for (row_idx, row) in bitmap.iter().enumerate() {
            let py = draw_y + row_idx as i32;
            if py < 0 || py >= fb.height() as i32 {
                continue;
            }

            for (col_idx, ch) in row.chars().enumerate() {
                let px = draw_x + col_idx as i32;
                if px < 0 || px >= fb.width() as i32 {
                    continue;
                }

                match ch {
                    'X' => fb.put_pixel(px as usize, py as usize, Color::CURSOR_OUTLINE),
                    '#' => fb.put_pixel(px as usize, py as usize, Color::CURSOR_FILL),
                    _ => {} // Transparent
                }
            }
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
