use serde::{Deserialize, Serialize};

/// Active view tab in the LensAI interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActiveView {
    Chat,
    Assistant,
    Translation,
    Summarizer,
    ImageProcessor,
    Settings,
    History,
}

impl std::fmt::Display for ActiveView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveView::Chat => write!(f, "Chat"),
            ActiveView::Assistant => write!(f, "Assistant Mode"),
            ActiveView::Translation => write!(f, "Translation"),
            ActiveView::Summarizer => write!(f, "Summarizer"),
            ActiveView::ImageProcessor => write!(f, "Vision & Image Utility"),
            ActiveView::Settings => write!(f, "Settings"),
            ActiveView::History => write!(f, "Conversation History"),
        }
    }
}

impl Default for ActiveView {
    fn default() -> Self {
        ActiveView::Chat
    }
}

/// Visual theme parameters for LensOS Frosted Glass UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITheme {
    pub background_blur_px: u32,
    pub surface_color: String,
    pub border_color: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub accent_glow: String,
}

impl Default for UITheme {
    fn default() -> Self {
        Self {
            background_blur_px: 24,
            surface_color: "rgba(15, 23, 42, 0.75)".to_string(), // Slate-900 at 75% opacity
            border_color: "rgba(255, 255, 255, 0.12)".to_string(),
            text_primary: "#F8FAFC".to_string(),
            text_secondary: "#94A3B8".to_string(),
            accent_glow: "#38BDF8".to_string(),
        }
    }
}

/// UI State container for LensAI view layout, controls, and active panel flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    pub active_view: ActiveView,
    pub theme: UITheme,
    pub sidebar_open: bool,
    pub input_buffer: String,
    pub is_processing: bool,
    pub status_message: Option<String>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::Chat,
            theme: UITheme::default(),
            sidebar_open: true,
            input_buffer: String::new(),
            is_processing: false,
            status_message: Some("LensAI Ready".to_string()),
        }
    }

    pub fn switch_view(&mut self, view: ActiveView) {
        self.active_view = view;
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Generates a structural string specification of the Frosted Glass layout.
    pub fn render_frame_spec(&self) -> String {
        format!(
            "[LensOS Frosted Glass Window Frame]\n\
             ├─ Active View: {}\n\
             ├─ Theme: Sophisticated Dark ({})\n\
             ├─ Backdrop Blur: {}px | Border: {}\n\
             ├─ Sidebar Collapsed: {}\n\
             ├─ Processing: {}\n\
             └─ Status: {}",
            self.active_view,
            self.theme.surface_color,
            self.theme.background_blur_px,
            self.theme.border_color,
            !self.sidebar_open,
            self.is_processing,
            self.status_message.as_deref().unwrap_or("Idle")
        )
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}
