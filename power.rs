//! LensOS v0.1 - Power Management System
//!
//! Handles ACPI/Hardware power states, shutdown routines, system reboot,
//! sleep, hibernate, and notification hooks for running LensOS services.

/// Power actions supported by LensOS power manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    /// Full ACPI system shutdown
    Shutdown,
    /// Warm hardware reboot
    Restart,
    /// Low-power RAM suspend (Sleep)
    Sleep,
    /// Disk suspend (Hibernate)
    Hibernate,
    /// Emergency/immediate ungraceful shutdown
    ForceShutdown,
}

/// Current state of system power subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// Fully operational
    Online,
    /// Transitioning to low power sleep mode
    Sleeping,
    /// Hibernating state
    Hibernated,
    /// Executing clean system shutdown
    ShuttingDown,
    /// Executing system restart
    Restarting,
}

/// Callback hook signature for system services to save state before power transition
pub type PowerEventCallback = Box<dyn Fn(PowerAction) -> Result<(), String> + Send + Sync>;

/// Core Power Manager coordinating power events across LensOS
pub struct PowerManager {
    current_state: PowerState,
    battery_level_percent: u8,
    is_ac_connected: bool,
    pre_power_callbacks: Vec<PowerEventCallback>,
}

impl std::fmt::Debug for PowerManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PowerManager")
            .field("current_state", &self.current_state)
            .field("battery_level_percent", &self.battery_level_percent)
            .field("is_ac_connected", &self.is_ac_connected)
            .field("callbacks_registered", &self.pre_power_callbacks.len())
            .finish()
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self {
            current_state: PowerState::Online,
            battery_level_percent: 100,
            is_ac_connected: true,
            pre_power_callbacks: Vec::new(),
        }
    }
}

impl PowerManager {
    /// Creates a new `PowerManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns current power state
    pub fn state(&self) -> PowerState {
        self.current_state
    }

    /// Registers a module handler to be called prior to power transitions (e.g. saving state in desktop/files)
    pub fn register_pre_power_hook<F>(&mut self, callback: F)
    where
        F: Fn(PowerAction) -> Result<(), String> + Send + Sync + 'static,
    {
        self.pre_power_callbacks.push(Box::new(callback));
    }

    /// Executes all registered hooks before power operation
    fn execute_hooks(&self, action: PowerAction) -> Result<(), String> {
        for callback in &self.pre_power_callbacks {
            if let Err(err) = callback(action) {
                return Err(format!("Pre-power hook failed for action {:?}: {}", action, err));
            }
        }
        Ok(())
    }

    /// Initiates graceful shutdown sequence
    pub fn shutdown(&mut self) -> Result<(), String> {
        self.execute_hooks(PowerAction::Shutdown)?;
        self.current_state = PowerState::ShuttingDown;
        // Signaling microkernel / power controller to cut power
        Ok(())
    }

    /// Initiates system restart sequence
    pub fn restart(&mut self) -> Result<(), String> {
        self.execute_hooks(PowerAction::Restart)?;
        self.current_state = PowerState::Restarting;
        Ok(())
    }

    /// Puts system into low power Sleep mode
    pub fn sleep(&mut self) -> Result<(), String> {
        self.execute_hooks(PowerAction::Sleep)?;
        self.current_state = PowerState::Sleeping;
        Ok(())
    }

    /// Puts system into Hibernate state
    pub fn hibernate(&mut self) -> Result<(), String> {
        self.execute_hooks(PowerAction::Hibernate)?;
        self.current_state = PowerState::Hibernated;
        Ok(())
    }

    /// Wakes up system from Sleep or Hibernate
    pub fn wake(&mut self) {
        self.current_state = PowerState::Online;
    }

    /// Updates battery metrics
    pub fn update_battery(&mut self, level_percent: u8, ac_connected: bool) {
        self.battery_level_percent = level_percent.min(100);
        self.is_ac_connected = ac_connected;
    }

    /// Retrieves battery state
    pub fn battery_status(&self) -> (u8, bool) {
        (self.battery_level_percent, self.is_ac_connected)
    }
}
