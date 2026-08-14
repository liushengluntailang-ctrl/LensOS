//! LensOS v0.2 Display Subsystem
//!
//! Provides display configuration, pixel formats, resolution presets,
//! and display device management.

/// Supported pixel color formats across different hardware and virtual display devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 32-bit ARGB (8-bit Alpha, 8-bit Red, 8-bit Green, 8-bit Blue)
    Argb8888,
    /// 32-bit RGBA (8-bit Red, 8-bit Green, 8-bit Blue, 8-bit Alpha)
    Rgba8888,
    /// 32-bit BGRA (8-bit Blue, 8-bit Green, 8-bit Red, 8-bit Alpha) - Standard for QEMU / UEFI
    Bgra8888,
    /// 32-bit BGRX (24-bit color, 8-bit unused padding)
    Bgrx8888,
    /// 24-bit RGB (8-bit Red, 8-bit Green, 8-bit Blue)
    Rgb888,
    /// 24-bit BGR (8-bit Blue, 8-bit Green, 8-bit Red)
    Bgr888,
    /// 16-bit RGB 5:6:5 High Color
    Rgb565,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel for this format.
    #[inline]
    pub const fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::Argb8888
            | PixelFormat::Rgba8888
            | PixelFormat::Bgra8888
            | PixelFormat::Bgrx8888 => 4,
            PixelFormat::Rgb888 | PixelFormat::Bgr888 => 3,
            PixelFormat::Rgb565 => 2,
        }
    }

    /// Returns the number of bits per pixel.
    #[inline]
    pub const fn bits_per_pixel(&self) -> usize {
        self.bytes_per_pixel() * 8
    }
}

/// Represents a screen resolution and optional refresh rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl Resolution {
    pub const RES_640X480: Resolution = Resolution::new(640, 480, 60);
    pub const RES_800X600: Resolution = Resolution::new(800, 600, 60);
    pub const RES_1024X768: Resolution = Resolution::new(1024, 768, 60);
    pub const RES_1280X720: Resolution = Resolution::new(1280, 720, 60);
    pub const RES_1280X800: Resolution = Resolution::new(1280, 800, 60);
    pub const RES_1440X900: Resolution = Resolution::new(1440, 900, 60);
    pub const RES_1920X1080: Resolution = Resolution::new(1920, 1080, 60);
    pub const RES_2560X1440: Resolution = Resolution::new(2560, 1440, 60);

    pub const fn new(width: u32, height: u32, refresh_rate: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate,
        }
    }

    #[inline]
    pub const fn total_pixels(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    #[inline]
    pub const fn aspect_ratio(&self) -> (u32, u32) {
        let gcd = Self::gcd(self.width, self.height);
        (self.width / gcd, self.height / gcd)
    }

    const fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self::RES_1024X768
    }
}

/// Display operation modes for LensOS runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Windowed / Emulated surface
    Windowed,
    /// Native Fullscreen hardware console
    Fullscreen,
    /// Headless mode (rendering into virtual backbuffer only)
    Headless,
    /// QEMU virtualized standard graphics output
    QemuVga,
}

/// Information describing the active display device.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayInfo {
    pub resolution: Resolution,
    pub format: PixelFormat,
    pub mode: DisplayMode,
    pub scale_factor: f32,
    pub name: String,
    pub is_hardware_accelerated: bool,
}

impl DisplayInfo {
    pub fn new(resolution: Resolution, format: PixelFormat) -> Self {
        Self {
            resolution,
            format,
            mode: DisplayMode::Fullscreen,
            scale_factor: 1.0,
            name: String::from("LensOS Display Engine"),
            is_hardware_accelerated: false,
        }
    }

    pub fn qemu_default() -> Self {
        Self {
            resolution: Resolution::RES_1024X768,
            format: PixelFormat::Bgra8888,
            mode: DisplayMode::QemuVga,
            scale_factor: 1.0,
            name: String::from("QEMU Bochs/Standard VGA"),
            is_hardware_accelerated: false,
        }
    }
}

impl Default for DisplayInfo {
    fn default() -> Self {
        Self::new(Resolution::default(), PixelFormat::Bgra8888)
    }
}
