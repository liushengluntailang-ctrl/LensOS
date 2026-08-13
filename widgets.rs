//! Desktop Widgets module for LensOS.
//!
//! Manages desktop widgets (clock, calendar, weather, system hardware monitor, notes),
//! spatial positioning, visibility layer toggling, and tick updates.

use crate::desktop::{Position, Rect, Size};

/// Built-in LensOS Widget types.
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetType {
    Clock { format_24h: bool },
    Calendar,
    Weather { location: String, temp_celsius: f32 },
    SystemResourceMonitor { cpu_usage_pct: f32, ram_usage_pct: f32 },
    QuickNotes { text: String },
    AudioPlayer { track_title: String, artist: String, is_playing: bool },
}

impl WidgetType {
    pub fn title(&self) -> &'static str {
        match self {
            WidgetType::Clock { .. } => "Clock",
            WidgetType::Calendar => "Calendar",
            WidgetType::Weather { .. } => "Weather",
            WidgetType::SystemResourceMonitor { .. } => "System Monitor",
            WidgetType::QuickNotes { .. } => "Quick Notes",
            WidgetType::AudioPlayer { .. } => "Audio Player",
        }
    }
}

/// Individual Desktop Widget instance.
#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    pub id: u64,
    pub widget_type: WidgetType,
    pub bounds: Rect,
    pub is_visible: bool,
    pub is_pinned_to_desktop: bool,
}

impl Widget {
    pub fn new(id: u64, widget_type: WidgetType, bounds: Rect) -> Self {
        Self {
            id,
            widget_type,
            bounds,
            is_visible: true,
            is_pinned_to_desktop: true,
        }
    }
}

/// Desktop Widget Manager handling grid placements, updates, and interactions.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetManager {
    pub widgets: Vec<Widget>,
    pub layer_visible: bool,
    next_widget_id: u64,
}

impl WidgetManager {
    /// Creates an empty WidgetManager.
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            layer_visible: true,
            next_widget_id: 1,
        }
    }

    /// Creates a WidgetManager populated with a clean default desktop layout.
    pub fn new_default_layout(display_size: Size) -> Self {
        let mut mgr = Self::new();
        let right_margin = display_size.width - 280.0;

        // Top-right Clock Widget
        mgr.add_widget(
            WidgetType::Clock { format_24h: true },
            Rect::new(right_margin, 40.0, 240.0, 100.0),
        );

        // System Resource Monitor Widget
        mgr.add_widget(
            WidgetType::SystemResourceMonitor {
                cpu_usage_pct: 12.5,
                ram_usage_pct: 38.2,
            },
            Rect::new(right_margin, 156.0, 240.0, 140.0),
        );

        // Weather Widget
        mgr.add_widget(
            WidgetType::Weather {
                location: "Lens City".to_string(),
                temp_celsius: 21.5,
            },
            Rect::new(right_margin, 312.0, 240.0, 110.0),
        );

        mgr
    }

    /// Spawns and registers a new desktop widget.
    pub fn add_widget(&mut self, widget_type: WidgetType, bounds: Rect) -> u64 {
        let id = self.next_widget_id;
        self.next_widget_id += 1;
        let widget = Widget::new(id, widget_type, bounds);
        self.widgets.push(widget);
        id
    }

    /// Removes a widget by ID.
    pub fn remove_widget(&mut self, id: u64) -> bool {
        let len_before = self.widgets.len();
        self.widgets.retain(|w| w.id != id);
        self.widgets.len() < len_before
    }

    /// Updates widget position coordinates.
    pub fn move_widget(&mut self, id: u64, new_pos: Position) {
        if let Some(w) = self.widgets.iter_mut().find(|w| w.id == id) {
            w.bounds.position = new_pos;
        }
    }

    /// Toggles overall desktop widget layer visibility.
    pub fn toggle_layer_visibility(&mut self) -> bool {
        self.layer_visible = !self.layer_visible;
        self.layer_visible
    }

    /// Returns list of currently visible widgets.
    pub fn visible_widgets(&self) -> Vec<&Widget> {
        if !self.layer_visible {
            return Vec::new();
        }
        self.widgets.iter().filter(|w| w.is_visible).collect()
    }

    /// Updates dynamic widget parameters on clock ticks.
    pub fn update(&mut self, delta_time_secs: f32) {
        for widget in self.widgets.iter_mut() {
            match &mut widget.widget_type {
                WidgetType::SystemResourceMonitor { cpu_usage_pct, ram_usage_pct: _ } => {
                    // Simulate subtle live metric fluctuation
                    let variation = (delta_time_secs * 2.0).sin() * 0.5;
                    *cpu_usage_pct = (*cpu_usage_pct + variation).clamp(1.0, 99.0);
                }
                _ => {}
            }
        }
    }

    /// Repositions desktop widgets to adapt to new display size.
    pub fn reposition_for_screen(&mut self, new_size: Size) {
        let right_margin = new_size.width - 280.0;
        for (idx, widget) in self.widgets.iter_mut().enumerate() {
            widget.bounds.position.x = right_margin;
            widget.bounds.position.y = 40.0 + (idx as f32) * 130.0;
        }
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}
