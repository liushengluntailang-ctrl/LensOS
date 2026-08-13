//! LensOS v0.1 - Lens Key Event Handler
//!
//! Captures hardware/keyboard events for the dedicated physical "Lens Key".
//! On a single press, instantly invokes the `AILauncher` to trigger Lens AI.

use crate::ai_launcher::{AILaunchContext, AILauncher, AILaunchMode};

/// Action type triggered by the Lens hardware key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensKeyAction {
    /// Single press trigger: Instant invocation of Lens AI overlay/assistant
    SinglePress,
    /// Double press trigger: Toggle Lens AI voice mode or contextual query
    DoublePress,
    /// Long press trigger: Open full Lens AI dashboard / settings
    LongPress,
    /// Key combination hold (e.g. LensKey + Space)
    ShortcutCombo,
}

/// Lens Key Event payload
#[derive(Debug, Clone)]
pub struct LensKeyEvent {
    pub action: LensKeyAction,
    pub timestamp: u64,
    pub active_window_title: Option<String>,
}

/// Handler for the Lens hardware key
#[derive(Debug, Default)]
pub struct LensKeyHandler {
    is_enabled: bool,
    total_press_count: u64,
}

impl LensKeyHandler {
    /// Creates a new `LensKeyHandler`
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            total_press_count: 0,
        }
    }

    /// Enables or disables the Lens key listener
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    /// Returns whether Lens key handling is active
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Handles a hardware Lens Key press event.
    /// A single press instantly launches the Lens AI assistant via `AILauncher`.
    pub fn handle_key_event(
        &mut self,
        action: LensKeyAction,
        ai_launcher: &mut AILauncher,
    ) -> Result<bool, String> {
        if !self.is_enabled {
            return Ok(false);
        }

        self.total_press_count += 1;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        match action {
            LensKeyAction::SinglePress => {
                // Requirement: The Lens key must be able to launch Lens AI with a single press.
                let context = AILaunchContext {
                    launch_mode: AILaunchMode::Overlay,
                    prompt_seed: None,
                    screen_context: Some("LensKey Instant Launch Trigger".to_string()),
                    triggered_at: now,
                };
                ai_launcher.launch_lens_ai(context)?;
                Ok(true)
            }
            LensKeyAction::DoublePress => {
                let context = AILaunchContext {
                    launch_mode: AILaunchMode::Sidebar,
                    prompt_seed: None,
                    screen_context: Some("LensKey Voice Assistant Mode".to_string()),
                    triggered_at: now,
                };
                ai_launcher.launch_lens_ai(context)?;
                Ok(true)
            }
            LensKeyAction::LongPress => {
                let context = AILaunchContext {
                    launch_mode: AILaunchMode::Fullscreen,
                    prompt_seed: None,
                    screen_context: Some("LensKey Full Workspace Mode".to_string()),
                    triggered_at: now,
                };
                ai_launcher.launch_lens_ai(context)?;
                Ok(true)
            }
            LensKeyAction::ShortcutCombo => {
                let context = AILaunchContext {
                    launch_mode: AILaunchMode::BackgroundService,
                    prompt_seed: None,
                    screen_context: Some("LensKey Background Query".to_string()),
                    triggered_at: now,
                };
                ai_launcher.launch_lens_ai(context)?;
                Ok(true)
            }
        }
    }

    /// Returns total times Lens Key has been pressed in session
    pub fn total_press_count(&self) -> u64 {
        self.total_press_count
    }
}
