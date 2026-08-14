//! LensOS v0.2 Framebuffer Subsystem
//!
//! Provides pixel color representations, linear framebuffer abstractions,
//! fast memory blitting, and color blending routines.

use crate::display::PixelFormat;

/// RGBA 32-bit color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // Basic Color Palette
    pub const TRANSPARENT: Color = Color::from_rgba(0, 0, 0, 0);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const RED: Color = Color::from_rgb(239, 68, 68);
    pub const GREEN: Color = Color::from_rgb(34, 197, 94);
    pub const BLUE: Color = Color::from_rgb(59, 130, 246);
    pub const YELLOW: Color = Color::from_rgb(234, 179, 8);
    pub const CYAN: Color = Color::from_rgb(6, 182, 212);
    pub const MAGENTA: Color = Color::from_rgb(217, 70, 239);

    // LensOS Theme Color Palette
    /// Dark desktop background base (Deep Slate #0F172A)
    pub const DESKTOP_DARK: Color = Color::from_rgb(15, 23, 42);
    /// Dark desktop background top gradient (#1E293B)
    pub const DESKTOP_DARK_ELEVATED: Color = Color::from_rgb(30, 41, 59);
    /// LensOS Signature Aperture Cyan (#38BDF8)
    pub const LENS_CYAN: Color = Color::from_rgb(56, 189, 248);
    /// LensOS Signature Sapphire Blue (#2563EB)
    pub const LENS_BLUE: Color = Color::from_rgb(37, 99, 235);
    /// LensOS Light Teal Glow (#67E8F9)
    pub const LENS_GLOW: Color = Color::from_rgb(103, 232, 249);
    /// LensOS Neutral Border Gray (#334155)
    pub const LENS_BORDER: Color = Color::from_rgb(51, 65, 85);
    /// LensOS Text Secondary (#94A3B8)
    pub const TEXT_SECONDARY: Color = Color::from_rgb(148, 163, 184);
    /// LensOS Text Primary (#F8FAFC)
    pub const TEXT_PRIMARY: Color = Color::from_rgb(248, 250, 252);
    /// Cursor Fill (White)
    pub const CURSOR_FILL: Color = Color::from_rgb(255, 255, 255);
    /// Cursor Outline (Black)
    pub const CURSOR_OUTLINE: Color = Color::from_rgb(15, 23, 42);

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn from_u32_argb(val: u32) -> Self {
        Self {
            a: ((val >> 24) & 0xFF) as u8,
            r: ((val >> 16) & 0xFF) as u8,
            g: ((val >> 8) & 0xFF) as u8,
            b: (val & 0xFF) as u8,
        }
    }

    #[inline]
    pub const fn to_u32_argb(&self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }

    #[inline]
    pub const fn to_u32_rgba(&self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | (self.a as u32)
    }

    #[inline]
    pub const fn to_u32_bgra(&self) -> u32 {
        ((self.b as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.r as u32) << 8)
            | (self.a as u32)
    }

    #[inline]
    pub const fn to_pixel_val(&self, format: PixelFormat) -> u32 {
        match format {
            PixelFormat::Argb8888 => self.to_u32_argb(),
            PixelFormat::Rgba8888 => self.to_u32_rgba(),
            PixelFormat::Bgra8888 | PixelFormat::Bgrx8888 => self.to_u32_bgra(),
            PixelFormat::Rgb888 => {
                ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
            }
            PixelFormat::Bgr888 => {
                ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
            }
            PixelFormat::Rgb565 => {
                let r = ((self.r as u16) >> 3) & 0x1F;
                let g = ((self.g as u16) >> 2) & 0x3F;
                let b = ((self.b as u16) >> 3) & 0x1F;
                ((r << 11) | (g << 5) | b) as u32
            }
        }
    }

    /// Performs alpha blending over a destination color: `src OVER dst`.
    #[inline]
    pub fn blend_over(self, dst: Color) -> Color {
        if self.a == 255 {
            return self;
        }
        if self.a == 0 {
            return dst;
        }

        let alpha = self.a as u32;
        let inv_alpha = 255 - alpha;

        let r = ((self.r as u32 * alpha + dst.r as u32 * inv_alpha) / 255) as u8;
        let g = ((self.g as u32 * alpha + dst.g as u32 * inv_alpha) / 255) as u8;
        let b = ((self.b as u32 * alpha + dst.b as u32 * inv_alpha) / 255) as u8;
        let a = (alpha + (dst.a as u32 * inv_alpha) / 255).min(255) as u8;

        Color { r, g, b, a }
    }

    /// Linear interpolation between two colors.
    pub fn lerp(start: Color, end: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let r = (start.r as f32 + (end.r as f32 - start.r as f32) * factor) as u8;
        let g = (start.g as f32 + (end.g as f32 - start.g as f32) * factor) as u8;
        let b = (start.b as f32 + (end.b as f32 - start.b as f32) * factor) as u8;
        let a = (start.a as f32 + (end.a as f32 - start.a as f32) * factor) as u8;
        Color { r, g, b, a }
    }
}

/// Configuration settings for instantiating a framebuffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramebufferConfig {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub format: PixelFormat,
    pub base_address: usize,
}

/// Primary Framebuffer memory buffer representation.
pub struct Framebuffer {
    data: Vec<u32>,
    raw_ptr: Option<*mut u8>,
    width: usize,
    height: usize,
    stride: usize,
    format: PixelFormat,
}

// Safety: The raw pointer can be safely sent across threads if properly synchronized.
unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

impl Framebuffer {
    /// Creates a new allocated Framebuffer in memory.
    pub fn new(width: usize, height: usize, format: PixelFormat) -> Self {
        let data = vec![0u32; width * height];
        Self {
            data,
            raw_ptr: None,
            width,
            height,
            stride: width,
            format,
        }
    }

    /// Creates a Framebuffer wrapping a raw memory pointer (e.g. QEMU VBE or physical BAR).
    ///
    /// # Safety
    /// The caller must ensure that `raw_ptr` points to a valid mapped memory region of sufficient size.
    pub unsafe fn from_raw(
        raw_ptr: *mut u8,
        width: usize,
        height: usize,
        stride: usize,
        format: PixelFormat,
    ) -> Self {
        Self {
            data: Vec::new(),
            raw_ptr: Some(raw_ptr),
            width,
            height,
            stride,
            format,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }

    #[inline]
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    #[inline]
    pub fn is_raw(&self) -> bool {
        self.raw_ptr.is_some()
    }

    /// Writes a single pixel with alpha blending.
    #[inline]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        if color.a == 0 {
            return;
        }

        let final_color = if color.a < 255 {
            let current = self.get_pixel(x, y);
            color.blend_over(current)
        } else {
            color
        };

        self.put_pixel_opaque(x, y, final_color);
    }

    /// Writes a pixel directly without alpha blending.
    #[inline]
    pub fn put_pixel_opaque(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let pixel_val = color.to_pixel_val(self.format);
        let offset = y * self.stride + x;

        if let Some(ptr) = self.raw_ptr {
            unsafe {
                let dst_ptr = ptr as *mut u32;
                dst_ptr.add(offset).write_volatile(pixel_val);
            }
        } else if offset < self.data.len() {
            self.data[offset] = pixel_val;
        }
    }

    /// Reads a single pixel from the framebuffer.
    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return Color::TRANSPARENT;
        }

        let offset = y * self.stride + x;
        let val = if let Some(ptr) = self.raw_ptr {
            unsafe {
                let src_ptr = ptr as *const u32;
                src_ptr.add(offset).read_volatile()
            }
        } else if offset < self.data.len() {
            self.data[offset]
        } else {
            0
        };

        match self.format {
            PixelFormat::Argb8888 => Color::from_u32_argb(val),
            PixelFormat::Rgba8888 => Color {
                r: ((val >> 24) & 0xFF) as u8,
                g: ((val >> 16) & 0xFF) as u8,
                b: ((val >> 8) & 0xFF) as u8,
                a: (val & 0xFF) as u8,
            },
            PixelFormat::Bgra8888 | PixelFormat::Bgrx8888 => Color {
                b: ((val >> 24) & 0xFF) as u8,
                g: ((val >> 16) & 0xFF) as u8,
                r: ((val >> 8) & 0xFF) as u8,
                a: (val & 0xFF) as u8,
            },
            _ => Color::from_u32_argb(val),
        }
    }

    /// Clears the entire framebuffer with a uniform color.
    pub fn clear(&mut self, color: Color) {
        let pixel_val = color.to_pixel_val(self.format);

        if let Some(ptr) = self.raw_ptr {
            for y in 0..self.height {
                for x in 0..self.width {
                    let offset = y * self.stride + x;
                    unsafe {
                        (ptr as *mut u32).add(offset).write_volatile(pixel_val);
                    }
                }
            }
        } else {
            self.data.fill(pixel_val);
        }
    }

    /// Draws a solid filled rectangle.
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        if color.a == 0 {
            return;
        }

        let end_x = (x + w).min(self.width);
        let end_y = (y + h).min(self.height);

        for curr_y in y..end_y {
            for curr_x in x..end_x {
                self.put_pixel(curr_x, curr_y, color);
            }
        }
    }

    /// Draws an outlined rectangle with specified border thickness.
    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
        thickness: usize,
    ) {
        if w == 0 || h == 0 || thickness == 0 {
            return;
        }

        // Top and Bottom
        self.fill_rect(x, y, w, thickness, color);
        if h > thickness {
            self.fill_rect(x, y + h - thickness, w, thickness, color);
        }

        // Left and Right
        self.fill_rect(x, y, thickness, h, color);
        if w > thickness {
            self.fill_rect(x + w - thickness, y, thickness, h, color);
        }
    }

    /// Copies a block from another framebuffer into this framebuffer.
    pub fn blit(
        &mut self,
        src: &Framebuffer,
        src_x: usize,
        src_y: usize,
        dst_x: usize,
        dst_y: usize,
        width: usize,
        height: usize,
    ) {
        let copy_w = width.min(src.width.saturating_sub(src_x)).min(self.width.saturating_sub(dst_x));
        let copy_h = height.min(src.height.saturating_sub(src_y)).min(self.height.saturating_sub(dst_y));

        for row in 0..copy_h {
            for col in 0..copy_w {
                let pixel = src.get_pixel(src_x + col, src_y + row);
                self.put_pixel(dst_x + col, dst_y + row, pixel);
            }
        }
    }

    /// Copies an entire allocated buffer into a destination buffer (e.g. backbuffer to raw FB).
    pub fn copy_all_to(&self, dst: &mut Framebuffer) {
        let copy_w = self.width.min(dst.width);
        let copy_h = self.height.min(dst.height);

        if !self.is_raw() && !dst.is_raw() && self.stride == dst.stride && self.width == dst.width {
            let total = copy_w * copy_h;
            if total <= self.data.len() && total <= dst.data.len() {
                dst.data[..total].copy_from_slice(&self.data[..total]);
                return;
            }
        }

        for y in 0..copy_h {
            for x in 0..copy_w {
                let color = self.get_pixel(x, y);
                dst.put_pixel_opaque(x, y, color);
            }
        }
    }

    /// Returns a slice to internal pixel storage.
    #[inline]
    pub fn as_slice(&self) -> &[u32] {
        &self.data
    }

    /// Returns a mutable slice to internal pixel storage.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u32] {
        &mut self.data
    }
}
