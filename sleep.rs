//! LensOS v0.1 - Sleep and Power-Saving Controller
//!
//! Controls automatic sleep entry, sleep duration tracking, hibernate escalation,
//! and automatic system shutdown after prolonged sleep states to preserve battery.

use crate::power::PowerManager;

/// Configuration parameters for sleep management
#[derive(Debug, Clone)]
pub struct SleepConfig {
    /// Inactivity threshold in seconds before entering Sleep (0 = disabled)
    pub idle_timeout_secs: u64,
    /// Duration in seconds of continuous sleep before initiating automatic shutdown
    /// Default: 7200 seconds (2 hours)
    pub auto_shutdown_after_sleep_secs: u64,
    /// Flag enabling hibernation when battery is critically low
    pub enable_auto_hibernate: bool,
    /// Critical battery percentage threshold triggering hibernate
    pub critical_battery_threshold: u8,
}

impl Default for SleepConfig {
    fn default() -> Self {
        Self {
            idle_timeout_secs: 1800,              // 30 minutes
            auto_shutdown_after_sleep_secs: 7200, // 2 hours long sleep -> auto shutdown
            enable_auto_hibernate: true,
            critical_battery_threshold: 5,
        }
    }
}

/// Sleep Manager monitoring system sleep lifecycles and enforcing auto-shutdown policies
#[derive(Debug)]
pub struct SleepManager {
    config: SleepConfig,
    sleep_entered_at: Option<u64>,
    total_sleep_time_accumulated: u64,
    is_in_sleep: bool,
}

impl SleepManager {
    /// Creates a new `SleepManager` with custom or default config
    pub fn new(config: SleepConfig) -> Self {
        Self {
            config,
            sleep_entered_at: None,
            total_sleep_time_accumulated: 0,
            is_in_sleep: false,
        }
    }

    /// Returns current sleep configuration reference
    pub fn config(&self) -> &SleepConfig {
        &self.config
    }

    /// Mutable reference to sleep configuration
    pub fn config_mut(&mut self) -> &mut SleepConfig {
        &mut self.config
    }

    /// Enter sleep state and record timestamp
    pub fn enter_sleep(&mut self, power_manager: &mut PowerManager) -> Result<(), String> {
        if self.is_in_sleep {
            return Ok(());
        }

        power_manager.sleep()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.sleep_entered_at = Some(now);
        self.is_in_sleep = true;
        Ok(())
    }

    /// Wake up from sleep state
    pub fn wake_up(&mut self, power_manager: &mut PowerManager) {
        if !self.is_in_sleep {
            return;
        }

        if let Some(start) = self.sleep_entered_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            if now >= start {
                self.total_sleep_time_accumulated += now - start;
            }
        }

        self.is_in_sleep = false;
        self.sleep_entered_at = None;
        power_manager.wake();
    }

    /// Returns whether the system is currently in sleep mode
    pub fn is_sleeping(&self) -> bool {
        self.is_in_sleep
    }

    /// Checks if sleep duration has exceeded `auto_shutdown_after_sleep_secs`.
    /// If so, executes automatic power shutdown to safeguard hardware battery.
    pub fn check_long_sleep_auto_shutdown(&mut self, power_manager: &mut PowerManager) -> Result<bool, String> {
        if !self.is_in_sleep {
            return Ok(false);
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if let Some(entered_at) = self.sleep_entered_at {
            let duration = now.saturating_sub(entered_at);
            if duration >= self.config.auto_shutdown_after_sleep_secs {
                // Auto-shutdown threshold reached after long sleep!
                power_manager.shutdown()?;
                self.is_in_sleep = false;
                self.sleep_entered_at = None;
                return Ok(true); // Auto-shutdown triggered
            }
        }

        // Check critical battery threshold during sleep
        let (battery_level, ac_connected) = power_manager.battery_status();
        if !ac_connected && battery_level <= self.config.critical_battery_threshold {
            if self.config.enable_auto_hibernate {
                power_manager.hibernate()?;
            } else {
                power_manager.shutdown()?;
            }
            return Ok(true);
        }

        Ok(false)
    }
}
