//! LensOS v0.1 - Core Kernel Orchestrator
//!
//! The central `Kernel` struct initializes and manages all underlying subsystems:
//! memory management, task scheduling, virtual file system, graphics, input, and power management.

use crate::filesystem::VirtualFileSystem;
use crate::graphics::GraphicsSubsystem;
use crate::input::InputSubsystem;
use crate::memory::MemoryManager;
use crate::power::PowerManager;
use crate::scheduler::TaskScheduler;

/// Trait defining lifecycle hooks for kernel subsystems.
pub trait KernelSubsystem {
    /// Human-readable identifier for the subsystem.
    fn name(&self) -> &'static str;
    /// Boot phase initialization sequence.
    fn initialize(&mut self) -> Result<(), String>;
    /// Orderly shutdown sequence.
    fn shutdown(&mut self) -> Result<(), String>;
}

/// The primary LensOS Kernel orchestrator.
pub struct Kernel {
    version: &'static str,
    initialized: bool,
    pub memory: MemoryManager,
    pub scheduler: TaskScheduler,
    pub filesystem: VirtualFileSystem,
    pub graphics: GraphicsSubsystem,
    pub input: InputSubsystem,
    pub power: PowerManager,
}

impl Kernel {
    /// Creates a uninitialized instance of `Kernel`.
    pub fn new() -> Self {
        Self {
            version: "0.1.0-alpha",
            initialized: false,
            memory: MemoryManager::new(),
            scheduler: TaskScheduler::new(),
            filesystem: VirtualFileSystem::new(),
            graphics: GraphicsSubsystem::new(),
            input: InputSubsystem::new(),
            power: PowerManager::new(),
        }
    }

    /// Initializes all kernel subsystems in proper boot sequence order.
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("Kernel is already initialized.".to_string());
        }

        println!("============================================================");
        println!("             LensOS v{} Kernel Boot Sequence              ", self.version);
        println!("============================================================");
        println!("[BOOT] Commencing LensOS core initialization...");

        // 1. Power Manager
        self.power.initialize()?;

        // 2. Memory Subsystem (Required before VFS and Scheduler)
        self.memory.initialize()?;

        // 3. Virtual File System
        self.filesystem.initialize()?;

        // 4. Task Scheduler
        self.scheduler.initialize()?;

        // 5. Graphics Subsystem
        self.graphics.initialize()?;

        // 6. Input Subsystem
        self.input.initialize()?;

        self.initialized = true;

        println!("============================================================");
        println!("[BOOT] LensOS Kernel v{} booted successfully!", self.version);
        println!("[BOOT] All 6 core subsystems online. System ready.");
        println!("============================================================");

        Ok(())
    }

    /// Performs an orderly shutdown of all kernel subsystems in reverse boot order.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }

        println!("------------------------------------------------------------");
        println!("[SHUTDOWN] Initiating LensOS Kernel graceful shutdown sequence...");
        println!("------------------------------------------------------------");

        // Request shutdown signal to power subsystem
        let _ = self.power.trigger_shutdown();

        // Shutdown subsystems in reverse order of initialization
        self.input.shutdown()?;
        self.graphics.shutdown()?;
        self.scheduler.shutdown()?;
        self.filesystem.shutdown()?;
        self.memory.shutdown()?;
        self.power.shutdown()?;

        self.initialized = false;
        println!("[SHUTDOWN] All subsystems stopped. System halted securely.");
        Ok(())
    }

    /// Reboots the machine via power manager.
    pub fn restart(&mut self) -> Result<(), String> {
        println!("[REBOOT] Requesting system reboot...");
        self.shutdown()?;
        self.power.trigger_restart()?;
        Ok(())
    }

    /// Simulates a kernel timer interrupt tick across active subsystems.
    pub fn tick(&mut self) {
        if self.initialized {
            self.scheduler.schedule_tick();
            self.graphics.swap_buffers();
        }
    }

    /// Returns the LensOS version string.
    pub fn version(&self) -> &'static str {
        self.version
    }

    /// Returns whether the kernel is initialized and operational.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}
