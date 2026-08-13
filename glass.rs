//! LensOS Frosted Glass Design Engine
//!
//! Provides real-time translucent material simulation, backdrop blur matrix calculations,
//! specular border lighting, noise grain anti-banding, and acrylic/mica presets.

use crate::colors::Color;

/// Blur filter kernel algorithms supported by LensOS glass renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlassBlurAlgorithm {
    Gaussian,
    DualKawase,
    BoxBlur,
    Anisotropic,
}

/// Physical material parameters for rendering a frosted glass panel.
#[derive(Debug, Clone, PartialEq)]
pub struct GlassMaterial {
    /// Radius of the blur filter in logical pixels (e.g. `24.0`).
    pub blur_radius: f32,
    /// Color tint applied over the blurred background layer.
    pub tint_color: Color,
    /// Backplate color saturation boost factor (e.g. `1.35` for vibrant backdrop filtering).
    pub backdrop_saturation: f32,
    /// Refraction index offset simulating optical glass thickness (`1.0..=1.1`).
    pub refraction_index: f32,
    /// Specular light highlight color along top/left border edges.
    pub specular_border_top: Color,
    /// Dark ambient shadow border color along bottom/right border edges.
    pub specular_border_bottom: Color,
    /// Procedural micro-grain noise intensity (`0.0..=0.08`) to eliminate color banding artifacts.
    pub noise_grain_intensity: f32,
    /// Ambient drop shadow blur radius.
    pub shadow_blur: f32,
    /// Ambient drop shadow Y-axis vertical offset.
    pub shadow_offset_y: f32,
    /// Color and opacity of the drop shadow.
    pub shadow_color: Color,
    /// Outer corner radius of the glass card/window.
    pub corner_radius: f32,
    /// Preferred blur calculation algorithm.
    pub blur_algorithm: GlassBlurAlgorithm,
}

impl GlassMaterial {
    /// Creates a custom glass material with specified parameters.
    pub fn new(blur_radius: f32, tint_color: Color, corner_radius: f32) -> Self {
        Self {
            blur_radius,
            tint_color,
            backdrop_saturation: 1.30,
            refraction_index: 1.02,
            specular_border_top: Color::rgba(1.0, 1.0, 1.0, 0.25),
            specular_border_bottom: Color::rgba(0.0, 0.0, 0.0, 0.35),
            noise_grain_intensity: 0.025,
            shadow_blur: 32.0,
            shadow_offset_y: 12.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.50),
            corner_radius,
            blur_algorithm: GlassBlurAlgorithm::DualKawase,
        }
    }

    /// LensOS Signature Frosted Crystal Preset.
    pub fn frosted_crystal() -> Self {
        Self::new(28.0, Color::rgba(18.0 / 255.0, 24.0 / 255.0, 38.0 / 255.0, 0.65), 16.0)
    }

    /// Deep Acrylic Dark Sheet Preset for primary OS application windows.
    pub fn deep_acrylic() -> Self {
        let mut mat = Self::new(36.0, Color::rgba(10.0 / 255.0, 14.0 / 255.0, 22.0 / 255.0, 0.82), 14.0);
        mat.backdrop_saturation = 1.40;
        mat.shadow_blur = 48.0;
        mat.shadow_offset_y = 16.0;
        mat.shadow_color = Color::rgba(0.0, 0.0, 0.0, 0.65);
        mat
    }

    /// Ultra-light Mica Layer for transient dialogs and context popovers.
    pub fn luminous_popover() -> Self {
        let mut mat = Self::new(20.0, Color::rgba(30.0 / 255.0, 42.0 / 255.0, 68.0 / 255.0, 0.90), 12.0);
        mat.specular_border_top = Color::rgba(0.0, 240.0 / 255.0, 255.0 / 255.0, 0.45);
        mat.shadow_blur = 24.0;
        mat
    }

    /// Taskbar Glass Dock Preset.
    pub fn taskbar_dock() -> Self {
        let mut mat = Self::new(32.0, Color::rgba(14.0 / 255.0, 18.0 / 255.0, 28.0 / 255.0, 0.70), 20.0);
        mat.specular_border_top = Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.20);
        mat.shadow_blur = 40.0;
        mat.shadow_offset_y = -4.0;
        mat
    }
}

/// Gaussian kernel weight calculations for 1D/2D filter passes.
#[derive(Debug)]
pub struct BlurKernel {
    pub weights: Vec<f32>,
    pub offsets: Vec<f32>,
}

impl BlurKernel {
    /// Generates a 1D Gaussian kernel sampling array for given blur radius.
    pub fn generate_gaussian(radius: f32) -> Self {
        let r = radius.max(1.0) as usize;
        let sigma = radius / 2.0;
        let two_sigma_sq = 2.0 * sigma * sigma;

        let mut weights = Vec::with_capacity(r + 1);
        let mut sum = 0.0;

        for i in 0..=r {
            let weight = (-((i * i) as f32) / two_sigma_sq).exp();
            weights.push(weight);
            sum += if i == 0 { weight } else { 2.0 * weight };
        }

        for w in weights.iter_mut() {
            *w /= sum;
        }

        let offsets: Vec<f32> = (0..=r).map(|i| i as f32).collect();

        Self { weights, offsets }
    }
}

/// Glass surface layer instance situated in 2D OS viewport layout space.
#[derive(Debug, Clone, PartialEq)]
pub struct GlassLayer {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub material: GlassMaterial,
    pub is_visible: bool,
}

impl GlassLayer {
    pub fn new(x: f32, y: f32, width: f32, height: f32, material: GlassMaterial) -> Self {
        Self {
            x,
            y,
            width,
            height,
            material,
            is_visible: true,
        }
    }

    /// Evaluates whether a given point `(px, py)` intersects this glass panel.
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}
