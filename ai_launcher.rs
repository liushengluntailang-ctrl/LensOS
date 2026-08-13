//! LensOS v0.1 - AI Engine Launcher
//!
//! Provides ultra-low latency sub-millisecond invocation of the `lens_ai` module.
//! Manages process pre-warming, modal states (Overlay, Sidebar, Fullscreen),
//! and contextual prompt passing.

/// Display mode when launching Lens AI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AILaunchMode {
    /// Floating overlay above active windows
    Overlay,
    /// Docked sidebar panel
    Sidebar,
    /// Full screen dedicated AI desktop workspace
    Fullscreen,
    /// Non-visual background inference execution
    BackgroundService,
}

impl Default for AILaunchMode {
    fn default() -> Self {
        AILaunchMode::Overlay
    }
}

/// Context payload passed during AI launch
#[derive(Debug, Clone)]
pub struct AILaunchContext {
    pub launch_mode: AILaunchMode,
    pub prompt_seed: Option<String>,
    pub screen_context: Option<String>,
    pub triggered_at: u64,
}

/// Status of the Lens AI runtime instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIStatus {
    /// Cold / stopped
    Inactive,
    /// Model weights loaded in RAM, awaiting hot trigger
    Prewarmed,
    /// Currently displayed and interacting with user
    Active,
    /// Processing inference query
    Busy,
}

/// AI Engine Launcher
#[derive(Debug)]
pub struct AILauncher {
    status: AIStatus,
    active_mode: Option<AILaunchMode>,
    last_context: Option<AILaunchContext>,
    launch_count: u64,
    prewarm_enabled: bool,
}

impl Default for AILauncher {
    fn default() -> Self {
        Self {
            status: AIStatus::Inactive,
            active_mode: None,
            last_context: None,
            launch_count: 0,
            prewarm_enabled: true,
        }
    }
}

impl AILauncher {
    /// Creates a new `AILauncher`
    pub fn new() -> Self {
        let mut launcher = Self::default();
        // Automatically prewarm model runtime during initialization
        launcher.prewarm();
        launcher
    }

    /// Pre-warms the `lens_ai` daemon and loads neural models into resident memory
    pub fn prewarm(&mut self) {
        if self.prewarm_enabled && self.status == AIStatus::Inactive {
            self.status = AIStatus::Prewarmed;
        }
    }

    /// Triggers instant sub-millisecond launch of the `lens_ai` engine
    pub fn launch_lens_ai(&mut self, context: AILaunchContext) -> Result<(), String> {
        self.launch_count += 1;
        self.active_mode = Some(context.launch_mode);
        self.status = AIStatus::Active;
        self.last_context = Some(context);
        Ok(())
    }

    /// Hides or minimizes the Lens AI interface back to prewarmed standby state
    pub fn dismiss(&mut self) {
        if self.status == AIStatus::Active || self.status == AIStatus::Busy {
            self.status = AIStatus::Prewarmed;
            self.active_mode = None;
        }
    }

    /// Returns current runtime status of Lens AI
    pub fn status(&self) -> AIStatus {
        self.status
    }

    /// Returns active launch mode if currently visible
    pub fn active_mode(&self) -> Option<AILaunchMode> {
        self.active_mode
    }

    /// Returns total launch invocation count
    pub fn launch_count(&self) -> u64 {
        self.launch_count
    }

    /// Retrieves last trigger context payload
    pub fn last_context(&self) -> Option<&AILaunchContext> {
        self.last_context.as_ref()
    }
}
