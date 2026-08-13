//! LensOS Vector Icon Management Engine
//!
//! Provides vector iconography primitives, path definitions, icon styling,
//! size scaling, and rendering path command buffers for LensOS.

use crate::colors::Color;

/// Categorized list of all OS system icons supported in LensOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconType {
    WindowClose,
    WindowMinimize,
    WindowMaximize,
    WindowRestore,
    AppGrid,
    Settings,
    Terminal,
    FileManager,
    Browser,
    CpuMonitor,
    Battery,
    Wifi,
    Volume,
    Search,
    Bell,
    User,
    Power,
    ChevronRight,
    ChevronDown,
    Check,
    AlertCircle,
    Folder,
    FileText,
    Play,
    Pause,
    Refresh,
}

/// Standardized icon dimension presets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(f32),
}

impl IconSize {
    pub fn pixels(&self) -> f32 {
        match self {
            IconSize::Small => 16.0,
            IconSize::Medium => 24.0,
            IconSize::Large => 32.0,
            IconSize::ExtraLarge => 48.0,
            IconSize::Custom(px) => *px,
        }
    }
}

/// Style attributes for vector icon strokes and fills.
#[derive(Debug, Clone, PartialEq)]
pub struct IconStyle {
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
    pub rotation_degrees: f32,
}

impl IconStyle {
    pub fn default_stroke(color: Color) -> Self {
        Self {
            stroke_color: color,
            stroke_width: 2.0,
            fill_color: None,
            rotation_degrees: 0.0,
        }
    }

    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }
}

/// Vector graphics command path instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum VectorCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CubicTo(f32, f32, f32, f32, f32, f32),
    QuadTo(f32, f32, f32, f32),
    Close,
}

/// Sequence of vector commands representing a 24x24 normalized icon path.
#[derive(Debug, Clone, PartialEq)]
pub struct VectorPath {
    pub commands: Vec<VectorCommand>,
}

impl VectorPath {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

impl Default for VectorPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Renderable Icon instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub icon_type: IconType,
    pub size: IconSize,
    pub style: IconStyle,
}

impl Icon {
    pub fn new(icon_type: IconType, color: Color) -> Self {
        Self {
            icon_type,
            size: IconSize::Medium,
            style: IconStyle::default_stroke(color),
        }
    }

    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    /// Obtains normalized 24x24 vector path definition for this icon.
    pub fn vector_path(&self) -> VectorPath {
        IconPathLibrary::get_path(self.icon_type)
    }
}

/// Normalized Vector Path Library for LensOS system icons.
pub struct IconPathLibrary;

impl IconPathLibrary {
    pub fn get_path(icon_type: IconType) -> VectorPath {
        let mut path = VectorPath::new();
        match icon_type {
            IconType::WindowClose => {
                path.commands.push(VectorCommand::MoveTo(18.0, 6.0));
                path.commands.push(VectorCommand::LineTo(6.0, 18.0));
                path.commands.push(VectorCommand::MoveTo(6.0, 6.0));
                path.commands.push(VectorCommand::LineTo(18.0, 18.0));
            }
            IconType::WindowMinimize => {
                path.commands.push(VectorCommand::MoveTo(5.0, 12.0));
                path.commands.push(VectorCommand::LineTo(19.0, 12.0));
            }
            IconType::WindowMaximize => {
                path.commands.push(VectorCommand::MoveTo(5.0, 5.0));
                path.commands.push(VectorCommand::LineTo(19.0, 5.0));
                path.commands.push(VectorCommand::LineTo(19.0, 19.0));
                path.commands.push(VectorCommand::LineTo(5.0, 19.0));
                path.commands.push(VectorCommand::Close);
            }
            IconType::ChevronRight => {
                path.commands.push(VectorCommand::MoveTo(9.0, 18.0));
                path.commands.push(VectorCommand::LineTo(15.0, 12.0));
                path.commands.push(VectorCommand::LineTo(9.0, 6.0));
            }
            IconType::Search => {
                path.commands.push(VectorCommand::MoveTo(11.0, 11.0));
                path.commands.push(VectorCommand::LineTo(21.0, 21.0));
            }
            _ => {
                // Generic fallback square outline
                path.commands.push(VectorCommand::MoveTo(4.0, 4.0));
                path.commands.push(VectorCommand::LineTo(20.0, 4.0));
                path.commands.push(VectorCommand::LineTo(20.0, 20.0));
                path.commands.push(VectorCommand::LineTo(4.0, 20.0));
                path.commands.push(VectorCommand::Close);
            }
        }
        path
    }
}
