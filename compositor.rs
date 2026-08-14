//! LensOS v0.2 Compositor & Window Surface Subsystem
//!
//! Provides multi-layer 2D software compositing, dirty-region tracking,
//! z-index ordering, alpha blending, and window surface management.

use crate::display::PixelFormat;
use crate::framebuffer::{Color, Framebuffer};

/// Unique identifier for a compositor layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LayerId(pub u64);

/// Individual graphical layer managed by the compositor.
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
    pub buffer: Framebuffer,
    pub alpha: u8,
    pub visible: bool,
    pub z_index: i32,
    pub has_shadow: bool,
    pub is_dirty: bool,
}

impl Layer {
    pub fn new(id: LayerId, name: impl Into<String>, width: usize, height: usize, format: PixelFormat) -> Self {
        Self {
            id,
            name: name.into(),
            x: 0,
            y: 0,
            width,
            height,
            buffer: Framebuffer::new(width, height, format),
            alpha: 255,
            visible: true,
            z_index: 0,
            has_shadow: false,
            is_dirty: true,
        }
    }

    #[inline]
    pub fn set_position(&mut self, x: i32, y: i32) {
        if self.x != x || self.y != y {
            self.x = x;
            self.y = y;
            self.is_dirty = true;
        }
    }

    #[inline]
    pub fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            self.is_dirty = true;
        }
    }

    #[inline]
    pub fn set_alpha(&mut self, alpha: u8) {
        if self.alpha != alpha {
            self.alpha = alpha;
            self.is_dirty = true;
        }
    }
}

/// Higher-level managed desktop window surface.
pub struct WindowSurface {
    pub layer_id: LayerId,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
    pub is_focused: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub titlebar_height: usize,
}

impl WindowSurface {
    pub fn new(layer_id: LayerId, title: impl Into<String>, x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            layer_id,
            title: title.into(),
            x,
            y,
            width,
            height,
            is_focused: true,
            is_minimized: false,
            is_maximized: false,
            titlebar_height: 28,
        }
    }
}

/// Configuration settings for the LensOS Compositor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositorConfig {
    pub width: usize,
    pub height: usize,
    pub format: PixelFormat,
    pub enable_shadows: bool,
}

impl Default for CompositorConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            format: PixelFormat::Bgra8888,
            enable_shadows: true,
        }
    }
}

/// Multi-layer software compositor.
pub struct Compositor {
    config: CompositorConfig,
    layers: Vec<Layer>,
    windows: Vec<WindowSurface>,
    next_layer_id: u64,
}

impl Compositor {
    pub fn new(config: CompositorConfig) -> Self {
        Self {
            config,
            layers: Vec::new(),
            windows: Vec::new(),
            next_layer_id: 1,
        }
    }

    /// Allocates and adds a new layer to the compositor.
    pub fn create_layer(&mut self, name: impl Into<String>, width: usize, height: usize) -> LayerId {
        let id = LayerId(self.next_layer_id);
        self.next_layer_id += 1;

        let layer = Layer::new(id, name, width, height, self.config.format);
        self.layers.push(layer);
        id
    }

    /// Removes a layer by its ID.
    pub fn remove_layer(&mut self, id: LayerId) -> bool {
        let initial_len = self.layers.len();
        self.layers.retain(|l| l.id != id);
        self.windows.retain(|w| w.layer_id != id);
        self.layers.len() < initial_len
    }

    /// Gets an immutable reference to a layer.
    pub fn get_layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == id)
    }

    /// Gets a mutable reference to a layer.
    pub fn get_layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    /// Brings a layer to the top of the z-order.
    pub fn bring_to_front(&mut self, id: LayerId) {
        let max_z = self.layers.iter().map(|l| l.z_index).max().unwrap_or(0);
        if let Some(layer) = self.get_layer_mut(id) {
            layer.z_index = max_z + 1;
            layer.is_dirty = true;
        }
    }

    /// Sends a layer to the bottom of the z-order.
    pub fn send_to_back(&mut self, id: LayerId) {
        let min_z = self.layers.iter().map(|l| l.z_index).min().unwrap_or(0);
        if let Some(layer) = self.get_layer_mut(id) {
            layer.z_index = min_z - 1;
            layer.is_dirty = true;
        }
    }

    /// Composes all visible layers onto the destination target framebuffer.
    pub fn compose(&mut self, target_fb: &mut Framebuffer) {
        // Sort indices by z-index
        let mut sorted_indices: Vec<usize> = (0..self.layers.len()).collect();
        sorted_indices.sort_by_key(|&idx| self.layers[idx].z_index);

        for &idx in &sorted_indices {
            let layer = &self.layers[idx];
            if !layer.visible || layer.alpha == 0 {
                continue;
            }

            // Render subtle shadow if enabled
            if self.config.enable_shadows && layer.has_shadow {
                Self::render_drop_shadow(target_fb, layer.x, layer.y, layer.width, layer.height);
            }

            // Blit layer contents onto target framebuffer with alpha modulation
            let start_x = layer.x.max(0) as usize;
            let start_y = layer.y.max(0) as usize;
            let end_x = ((layer.x + layer.width as i32).min(target_fb.width() as i32)).max(0) as usize;
            let end_y = ((layer.y + layer.height as i32).min(target_fb.height() as i32)).max(0) as usize;

            for dy in start_y..end_y {
                let layer_y = (dy as i32 - layer.y) as usize;
                for dx in start_x..end_x {
                    let layer_x = (dx as i32 - layer.x) as usize;
                    let mut pixel = layer.buffer.get_pixel(layer_x, layer_y);

                    if layer.alpha < 255 {
                        pixel.a = ((pixel.a as u32 * layer.alpha as u32) / 255) as u8;
                    }

                    target_fb.put_pixel(dx, dy, pixel);
                }
            }
        }
    }

    fn render_drop_shadow(target_fb: &mut Framebuffer, x: i32, y: i32, w: usize, h: usize) {
        let shadow_offset = 6;
        let shadow_color = Color::from_rgba(0, 0, 0, 45);

        let sx = (x + shadow_offset).max(0) as usize;
        let sy = (y + shadow_offset).max(0) as usize;
        let sw = w.min(target_fb.width().saturating_sub(sx));
        let sh = h.min(target_fb.height().saturating_sub(sy));

        target_fb.fill_rect(sx, sy, sw, sh, shadow_color);
    }
}
