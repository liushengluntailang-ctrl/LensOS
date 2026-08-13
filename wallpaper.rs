//! Wallpaper management module for LensOS.
//!
//! Handles background rendering modes (solid colors, dark minimalist gradients, custom image files),
//! aspect fitting modes, dark overlay tinting for icon legibility, and slideshow updates.

use crate::desktop::Color;

/// Aspect ratio scaling behavior for image wallpapers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperFit {
    Cover,
    Contain,
    Stretch,
    Center,
    Tile,
}

/// Rendering style mode for desktop wallpaper.
#[derive(Debug, Clone, PartialEq)]
pub enum WallpaperMode {
    SolidColor(Color),
    LinearGradient {
        start_color: Color,
        end_color: Color,
        angle_degrees: f32,
    },
    Image {
        file_path: String,
        fit: WallpaperFit,
    },
    DynamicTimeBased {
        day_color: Color,
        night_color: Color,
    },
}

/// Wallpaper configuration parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct Wallpaper {
    pub mode: WallpaperMode,
    pub blur_radius: f32,
    pub dim_opacity: f32, // Dark tint overlay (0.0 - 1.0) for high contrast text
}

impl Wallpaper {
    /// Creates a default LensOS signature dark gradient wallpaper.
    pub fn dark_minimal_gradient() -> Self {
        Self {
            mode: WallpaperMode::LinearGradient {
                start_color: Color::hex(0x0B0F19), // Deep slate black
                end_color: Color::hex(0x111827),   // Subtle deep dark blue-slate
                angle_degrees: 135.0,
            },
            blur_radius: 0.0,
            dim_opacity: 0.1,
        }
    }

    /// Creates a solid color wallpaper.
    pub fn solid(color: Color) -> Self {
        Self {
            mode: WallpaperMode::SolidColor(color),
            blur_radius: 0.0,
            dim_opacity: 0.0,
        }
    }

    /// Creates an image wallpaper with custom scaling.
    pub fn image(path: &str, fit: WallpaperFit) -> Self {
        Self {
            mode: WallpaperMode::Image {
                file_path: path.to_string(),
                fit,
            },
            blur_radius: 0.0,
            dim_opacity: 0.15,
        }
    }
}

impl Default for Wallpaper {
    fn default() -> Self {
        Self::dark_minimal_gradient()
    }
}

/// Wallpaper Manager system handling wallpaper switching, slideshows, and dynamic tinting.
#[derive(Debug, Clone, PartialEq)]
pub struct WallpaperManager {
    pub current_wallpaper: Wallpaper,
    pub fallback_color: Color,
    pub slideshow_files: Vec<String>,
    pub slideshow_interval_secs: f32,
    pub slideshow_timer_secs: f32,
    pub slideshow_enabled: bool,
    pub current_slideshow_index: usize,
}

impl WallpaperManager {
    /// Constructs a WallpaperManager with default dark gradient.
    pub fn new() -> Self {
        Self {
            current_wallpaper: Wallpaper::dark_minimal_gradient(),
            fallback_color: Color::hex(0x0B0F19),
            slideshow_files: Vec::new(),
            slideshow_interval_secs: 300.0, // 5 minutes default
            slideshow_timer_secs: 0.0,
            slideshow_enabled: false,
            current_slideshow_index: 0,
        }
    }

    /// Constructs a WallpaperManager with specific gradient colors.
    pub fn new_gradient(start: Color, end: Color) -> Self {
        let mut mgr = Self::new();
        mgr.current_wallpaper = Wallpaper {
            mode: WallpaperMode::LinearGradient {
                start_color: start,
                end_color: end,
                angle_degrees: 135.0,
            },
            blur_radius: 0.0,
            dim_opacity: 0.1,
        };
        mgr
    }

    /// Sets the active wallpaper.
    pub fn set_wallpaper(&mut self, wallpaper: Wallpaper) {
        self.current_wallpaper = wallpaper;
    }

    /// Sets wallpaper to a image file path.
    pub fn set_image(&mut self, path: &str, fit: WallpaperFit) {
        self.set_wallpaper(Wallpaper::image(path, fit));
    }

    /// Adjusts the dark dimming overlay opacity (0.0 - 1.0).
    pub fn set_dim_opacity(&mut self, opacity: f32) {
        self.current_wallpaper.dim_opacity = opacity.clamp(0.0, 1.0);
    }

    /// Enables background wallpaper slideshow mode.
    pub fn enable_slideshow(&mut self, files: Vec<String>, interval_secs: f32) {
        if !files.is_empty() {
            self.slideshow_files = files;
            self.slideshow_interval_secs = interval_secs;
            self.slideshow_timer_secs = 0.0;
            self.slideshow_enabled = true;
            self.current_slideshow_index = 0;
            self.load_current_slideshow_image();
        }
    }

    /// Loads image at current slideshow index.
    fn load_current_slideshow_image(&mut self) {
        if let Some(path) = self.slideshow_files.get(self.current_slideshow_index).cloned() {
            self.set_image(&path, WallpaperFit::Cover);
        }
    }

    /// Clock tick update to progress slideshow or dynamic time calculations.
    pub fn update(&mut self, delta_time_secs: f32) {
        if self.slideshow_enabled && !self.slideshow_files.is_empty() {
            self.slideshow_timer_secs += delta_time_secs;
            if self.slideshow_timer_secs >= self.slideshow_interval_secs {
                self.slideshow_timer_secs = 0.0;
                self.current_slideshow_index =
                    (self.current_slideshow_index + 1) % self.slideshow_files.len();
                self.load_current_slideshow_image();
            }
        }
    }
}

impl Default for WallpaperManager {
    fn default() -> Self {
        Self::new()
    }
}
