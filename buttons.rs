//! LensOS Button Component Engine
//!
//! Implements primary, secondary, ghost, frosted glass, and icon-only button components,
//! hover states, ripple press effects, focus rings, and action event dispatching.

use crate::colors::Color;
use crate::glass::GlassMaterial;
use crate::icons::IconType;

/// Visual variants of buttons supported in LensOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Glass,
    Outline,
    Danger,
    IconOnly,
}

/// Interactive button operational states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Focused,
    Disabled,
    Loading,
}

/// Button size options defining height and padding tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Compact,
    Regular,
    Large,
}

impl ButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            ButtonSize::Compact => 28.0,
            ButtonSize::Regular => 36.0,
            ButtonSize::Large => 44.0,
        }
    }

    pub fn horizontal_padding(&self) -> f32 {
        match self {
            ButtonSize::Compact => 12.0,
            ButtonSize::Regular => 16.0,
            ButtonSize::Large => 24.0,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Compact => 12.0,
            ButtonSize::Regular => 14.0,
            ButtonSize::Large => 16.0,
        }
    }
}

/// LensOS Button Component Instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub id: String,
    pub label: String,
    pub icon: Option<IconType>,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub state: ButtonState,
    pub glass_material: Option<GlassMaterial>,
    pub custom_accent: Option<Color>,
    pub is_full_width: bool,
    pub corner_radius: f32,
    pub width: f32,
    pub height: f32,
}

impl Button {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        let size = ButtonSize::Regular;
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            variant: ButtonVariant::Primary,
            size,
            state: ButtonState::Normal,
            glass_material: Some(GlassMaterial::frosted_crystal()),
            custom_accent: None,
            is_full_width: false,
            corner_radius: 10.0,
            width: 120.0,
            height: size.height(),
        }
    }

    pub fn glass(id: impl Into<String>, label: impl Into<String>) -> Self {
        let mut btn = Self::new(id, label);
        btn.variant = ButtonVariant::Glass;
        btn.glass_material = Some(GlassMaterial::luminous_popover());
        btn
    }

    pub fn icon_only(id: impl Into<String>, icon: IconType) -> Self {
        let mut btn = Self::new(id, "");
        btn.icon = Some(icon);
        btn.variant = ButtonVariant::IconOnly;
        btn.width = ButtonSize::Regular.height();
        btn
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        if self.state != ButtonState::Disabled && self.state != ButtonState::Loading {
            self.state = if hovered {
                ButtonState::Hovered
            } else {
                ButtonState::Normal
            };
        }
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        if self.state != ButtonState::Disabled && self.state != ButtonState::Loading {
            self.state = if pressed {
                ButtonState::Pressed
            } else {
                ButtonState::Hovered
            };
        }
    }

    /// Calculates background color given active state and variant.
    pub fn compute_background_color(&self, default_accent: Color, glass_tint: Color) -> Color {
        match self.state {
            ButtonState::Disabled => Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.05),
            ButtonState::Pressed => match self.variant {
                ButtonVariant::Primary => default_accent.darken(0.15),
                ButtonVariant::Glass => glass_tint.lighten(0.15),
                _ => Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.20),
            },
            ButtonState::Hovered => match self.variant {
                ButtonVariant::Primary => default_accent.lighten(0.10),
                ButtonVariant::Glass => glass_tint.lighten(0.08),
                _ => Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.12),
            },
            _ => match self.variant {
                ButtonVariant::Primary => default_accent,
                ButtonVariant::Glass => glass_tint,
                ButtonVariant::Ghost => Color::TRANSPARENT,
                _ => Color::rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.06),
            },
        }
    }
}

/// Fluent builder for constructing customizable Button instances.
#[derive(Debug)]
pub struct ButtonBuilder {
    button: Button,
}

impl ButtonBuilder {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            button: Button::new(id, label),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.button.height = size.height();
        self.button.size = size;
        self
    }

    pub fn icon(mut self, icon: IconType) -> Self {
        self.button.icon = Some(icon);
        self
    }

    pub fn glass_material(mut self, mat: GlassMaterial) -> Self {
        self.button.glass_material = Some(mat);
        self.button.variant = ButtonVariant::Glass;
        self
    }

    pub fn build(self) -> Button {
        self.button
    }
}
