//! LensOS v0.1 - Startup and Boot Sequence Manager
//!
//! Orchestrates the multi-stage system startup sequence, verifying compatibility
//! and initializing subsystem components (`boot`, `kernel`, `desktop`, `ui`,
//! `files`, `settings`, `browser`, `lens_ai`).

use std::collections::VecDeque;

/// Modular boot and startup stages in LensOS v0.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StartupStage {
    /// Handshake with `boot` module and loader verify
    BootHandshake,
    /// Connect to microkernel (`kernel`) IPC primitives
    KernelInit,
    /// Load core system daemons (power, permissions, notifications)
    SystemServices,
    /// Mount filesystems via `files` module
    FileSystemMount,
    /// Load system configuration from `settings` module
    SettingsLoad,
    /// Initialize `desktop` and `ui` compositors
    DesktopCompositor,
    /// Pre-warm `lens_ai` AI engine and quick-launch handlers
    AIPrewarm,
    /// System boot complete, login prompt ready
    UserReady,
}

/// Information record for a startup initialization task
#[derive(Debug, Clone)]
pub struct StartupTask {
    pub name: String,
    pub target_module: String,
    pub stage: StartupStage,
    pub completed: bool,
}

/// Startup Sequence Manager
#[derive(Debug)]
pub struct StartupManager {
    current_stage: StartupStage,
    tasks: VecDeque<StartupTask>,
    boot_logs: Vec<String>,
}

impl Default for StartupManager {
    fn default() -> Self {
        let mut manager = Self {
            current_stage: StartupStage::BootHandshake,
            tasks: VecDeque::new(),
            boot_logs: Vec::new(),
        };
        manager.register_default_tasks();
        manager
    }
}

impl StartupManager {
    /// Creates a new `StartupManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers the default system tasks required for LensOS v0.1 startup
    fn register_default_tasks(&mut self) {
        self.register_task("Verify Boot Image Signature", "boot", StartupStage::BootHandshake);
        self.register_task("Initialize Kernel Memory Manager", "kernel", StartupStage::KernelInit);
        self.register_task("Start System Permissions Daemon", "system", StartupStage::SystemServices);
        self.register_task("Mount System and User Volumes", "files", StartupStage::FileSystemMount);
        self.register_task("Load LensOS Preferences", "settings", StartupStage::SettingsLoad);
        self.register_task("Initialize Window Manager", "ui", StartupStage::DesktopCompositor);
        self.register_task("Launch Desktop Environment", "desktop", StartupStage::DesktopCompositor);
        self.register_task("Pre-warm Lens AI Engine", "lens_ai", StartupStage::AIPrewarm);
    }

    /// Adds a task to the startup sequence queue
    pub fn register_task(&mut self, name: impl Into<String>, target_module: impl Into<String>, stage: StartupStage) {
        self.tasks.push_back(StartupTask {
            name: name.into(),
            target_module: target_module.into(),
            stage,
            completed: false,
        });
    }

    /// Returns current startup stage
    pub fn current_stage(&self) -> StartupStage {
        self.current_stage
    }

    /// Executes the full startup sequence, logging progress
    pub fn execute_startup(&mut self) -> Result<(), String> {
        self.boot_logs.push("[BOOT] Initiating LensOS v0.1 system startup sequence...".to_string());

        let stages = [
            StartupStage::BootHandshake,
            StartupStage::KernelInit,
            StartupStage::SystemServices,
            StartupStage::FileSystemMount,
            StartupStage::SettingsLoad,
            StartupStage::DesktopCompositor,
            StartupStage::AIPrewarm,
            StartupStage::UserReady,
        ];

        for stage in stages {
            self.current_stage = stage;
            self.boot_logs.push(format!("[STARTUP STAGE] {:?}", stage));

            for task in self.tasks.iter_mut() {
                if task.stage == stage && !task.completed {
                    self.boot_logs.push(format!("  -> Executing: {} (Module: {})", task.name, task.target_module));
                    task.completed = true;
                }
            }
        }

        self.boot_logs.push("[BOOT] Startup sequence completed successfully. LensOS ready.".to_string());
        Ok(())
    }

    /// Returns all collected boot/startup log lines
    pub fn boot_logs(&self) -> &[String] {
        &self.boot_logs
    }
}
