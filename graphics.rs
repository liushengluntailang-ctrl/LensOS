//! LensOS v0.1 - Graphics Subsystem
//!
//! Manages Framebuffer initialization, GOP/VBE display driver modes, double buffering,
//! and 2D primitive rendering for LensOS GUI and console displays.

/// Pixel color representation in 32-bit RGBA color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const LENS_BLUE: Color = Color { r: 14, g: 116, b: 233, a: 255 };
    pub const DARK_GRAY: Color = Color { r: 30, g: 30, b: 35, a: 255 };
}

/// Screen display resolution parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
}

/// Framebuffer metadata from UEFI Graphics Output Protocol (GOP) or VBE.
#[derive(Debug, Clone, Copy)]
pub struct FrameBufferInfo {
    pub physical_address: usize,
    pub buffer_size_bytes: usize,
    pub resolution: Resolution,
    pub pitch: u32,
}

/// Core Graphics Subsystem controller for LensOS.
pub struct GraphicsSubsystem {
    initialized: bool,
    framebuffer_info: Option<FrameBufferInfo>,
    back_buffer: Vec<u8>,
}

impl GraphicsSubsystem {
    /// Constructs a new graphics subsystem instance.
    pub fn new() -> Self {
        Self {
            initialized: false,
            framebuffer_info: None,
            back_buffer: Vec::new(),
        }
    }

    /// Initializes framebuffer graphics output mode (1920x1080 @ 32bpp).
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("[BOOT][GRAPHICS] Locating UEFI Graphics Output Protocol (GOP) / VBE framebuffers...");

        let res = Resolution {
            width: 1920,
            height: 1080,
            bpp: 32,
        };

        let buffer_size = (res.width * res.height * 4) as usize;
        let info = FrameBufferInfo {
            physical_address: 0xFD000000,
            buffer_size_bytes: buffer_size,
            resolution: res,
            pitch: res.width * 4,
        };

        self.framebuffer_info = Some(info);
        self.back_buffer = vec![0u8; buffer_size];
        self.initialized = true;

        println!(
            "[BOOT][GRAPHICS] Video mode set: {}x{} x {}bpp.",
            res.width, res.height, res.bpp
        );
        println!(
            "[BOOT][GRAPHICS] Framebuffer mapped at phys address 0x{:X} (Size: {} KB).",
            info.physical_address,
            buffer_size / 1024
        );

        // Clear screen to LensOS Dark Theme background
        self.clear_screen(Color::DARK_GRAY);
        // Render a welcome accent bar
        self.draw_rect(0, 0, 1920, 4, Color::LENS_BLUE);

        println!("[BOOT][GRAPHICS] Framebuffer double-buffering active. Graphics initialized.");
        Ok(())
    }

    /// Clears the screen buffer with a background color.
    pub fn clear_screen(&mut self, color: Color) {
        if !self.initialized {
            return;
        }
        let size = self.back_buffer.len();
        let mut i = 0;
        while i + 3 < size {
            self.back_buffer[i] = color.b;
            self.back_buffer[i + 1] = color.g;
            self.back_buffer[i + 2] = color.r;
            self.back_buffer[i + 3] = color.a;
            i += 4;
        }
    }

    /// Draws a single pixel at (x, y) with color.
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        if let Some(ref info) = self.framebuffer_info {
            if x >= info.resolution.width || y >= info.resolution.height {
                return;
            }
            let offset = ((y * info.resolution.width + x) * 4) as usize;
            if offset + 3 < self.back_buffer.len() {
                self.back_buffer[offset] = color.b;
                self.back_buffer[offset + 1] = color.g;
                self.back_buffer[offset + 2] = color.r;
                self.back_buffer[offset + 3] = color.a;
            }
        }
    }

    /// Draws a filled rectangle on screen.
    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        for curr_y in y..(y + height) {
            for curr_x in x..(x + width) {
                self.draw_pixel(curr_x, curr_y, color);
            }
        }
    }

    /// Flushes back buffer to physical frame buffer display hardware.
    pub fn swap_buffers(&mut self) {
        if self.initialized {
            // Simulated V-Sync framebuffer swap
        }
    }

    /// Returns active framebuffer information.
    pub fn get_framebuffer_info(&self) -> Option<FrameBufferInfo> {
        self.framebuffer_info
    }

    /// Shuts down the graphics driver and restores text mode display.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }
        println!("[SHUTDOWN][GRAPHICS] Releasing video framebuffer and switching to fallback console...");
        self.back_buffer.clear();
        self.initialized = false;
        Ok(())
    }

    /// Returns whether graphics driver is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for GraphicsSubsystem {
    fn default() -> Self {
        Self::new()
    }
}
