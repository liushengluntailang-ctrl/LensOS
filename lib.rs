//! # LensOS v0.1 Kernel Core Architecture
//!
//! LensOS is a modular operating system designed with clean separation of concerns.
//! The `kernel/` module serves as the core kernel implementation and exports all
//! fundamental subsystems required for operating system execution.
//!
//! ## Subsystem Architecture Overview
//!
//! - **[`kernel`]**: Central `Kernel` struct that orchestrates subsystem life cycles, boot sequence, and kernel ticks.
//! - **[`memory`]**: Physical RAM discovery, PML4 virtual memory page allocation, and kernel heap management.
//! - **[`scheduler`]**: Multi-queue preemptive process task scheduler, thread state handling, and context switching.
//! - **[`filesystem`]**: Virtual File System (VFS) layer managing root filesystems (`/`), device nodes (`/dev`), and process entries (`/proc`).
//! - **[`graphics`]**: Framebuffer resolution control (GOP/VBE), double buffering, and 2D rendering primitives.
//! - **[`input`]**: PS/2 and USB HID driver abstraction, key scan-code queues, and mouse motion event tracking.
//! - **[`power`]**: ACPI table parsing, power states (S0-S5), battery status tracking, and reboot/shutdown vector controls.
//!
//! ## Usage & Integration with Bootloader (`boot/`)
//!
//! ```rust
//! use lensos_kernel::Kernel;
//!
//! fn main() {
//!     let mut kernel = Kernel::new();
//!     
//!     // Initialize all 6 kernel subsystems and print boot diagnostic logs
//!     if let Err(err) = kernel.initialize() {
//!         eprintln!("Kernel Panic during boot: {}", err);
//!         return;
//!     }
//!
//!     // Kernel operational loop tick simulation
//!     kernel.tick();
//!
//!     // Graceful shutdown sequence
//!     let _ = kernel.shutdown();
//! }
//! ```

pub mod filesystem;
pub mod graphics;
pub mod input;
pub mod kernel;
pub mod memory;
pub mod power;
pub mod scheduler;

// Re-exports for convenience
pub use filesystem::{FileType, VNode, VirtualFileSystem};
pub use graphics::{Color, FrameBufferInfo, GraphicsSubsystem, Resolution};
pub use input::{InputSubsystem, KeyEvent, KeyModifiers, KeyState, MouseEvent};
pub use kernel::{Kernel, KernelSubsystem};
pub use memory::{MemoryManager, MemoryRegion, MemoryStats, RegionType};
pub use power::{PowerManager, PowerSource, PowerState};
pub use scheduler::{Task, TaskPriority, TaskScheduler, TaskState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_lifecycle() {
        let mut kernel = Kernel::new();
        assert!(!kernel.is_initialized());

        // Initialize all subsystems
        let boot_res = kernel.initialize();
        assert!(boot_res.is_ok());
        assert!(kernel.is_initialized());

        // Subsystem status checks
        assert!(kernel.memory.is_initialized());
        assert!(kernel.scheduler.is_initialized());
        assert!(kernel.filesystem.is_initialized());
        assert!(kernel.graphics.is_initialized());
        assert!(kernel.input.is_initialized());
        assert!(kernel.power.is_initialized());

        // Test scheduler tick
        kernel.tick();

        // Graceful shutdown
        let shutdown_res = kernel.shutdown();
        assert!(shutdown_res.is_ok());
        assert!(!kernel.is_initialized());
    }

    #[test]
    fn test_filesystem_operations() {
        let mut vfs = VirtualFileSystem::new();
        assert!(vfs.initialize().is_ok());

        assert!(vfs.write_file("/etc/hostname", b"lensos-box").is_ok());
        let read_data = vfs.read_file("/etc/hostname");
        assert!(read_data.is_ok());
        assert_eq!(read_data.unwrap(), b"lensos-box");

        assert!(vfs.shutdown().is_ok());
    }
}
