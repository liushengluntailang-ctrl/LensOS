//! # LensOS Bootloader Module
//!
//! Orchestrates the operating system boot sequence. Simulates low-level
//! kernel initialization, memory management setup, virtual filesystem mounting,
//! driver stack startup, and user-space initialization within a 3-second timing budget.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::animation::LoadingAnimation;
use crate::logo::{colors, LogoRenderer};

/// Subsystem initialization step in the boot sequence.
pub struct BootStep {
    /// Name/description of the kernel initialization step
    pub name: &'static str,
    /// Duration weight in milliseconds allocated to this step
    pub duration_ms: u64,
}

/// Core Bootloader structure for LensOS.
pub struct Bootloader {
    logo_renderer: LogoRenderer,
    boot_steps: Vec<BootStep>,
}

impl Bootloader {
    /// Constructs a new `Bootloader` instance configured with standard system startup steps.
    pub fn new() -> Self {
        Self {
            logo_renderer: LogoRenderer::new(),
            boot_steps: vec![
                BootStep {
                    name: "Initializing CPU cores & interrupts",
                    duration_ms: 450,
                },
                BootStep {
                    name: "Probing physical RAM & mapping page tables",
                    duration_ms: 550,
                },
                BootStep {
                    name: "Loading LensOS microkernel image",
                    duration_ms: 600,
                },
                BootStep {
                    name: "Mounting Virtual File System (VFS)",
                    duration_ms: 500,
                },
                BootStep {
                    name: "Starting Hardware Abstraction Layer (HAL)",
                    duration_ms: 450,
                },
                BootStep {
                    name: "Launching core system daemons",
                    duration_ms: 450,
                },
            ],
        }
    }

    /// Executes the full LensOS boot sequence.
    ///
    /// Clears screen, displays the blue glowing ASCII logo, animates "Starting system...",
    /// runs through kernel startup stages over ~3 seconds total, and outputs success status.
    pub fn execute_boot(&self) -> io::Result<()> {
        let start_time = Instant::now();

        // Step 1: Render black screen & glowing Lens logo
        self.logo_renderer.render()?;

        let animation = LoadingAnimation::new("Starting system...");

        // Calculate total percentage allocation
        let mut current_pct = 0u32;
        let total_steps = self.boot_steps.len();

        let mut stdout = io::stdout();

        // Step 2: Execute boot stages with loading animation
        for (idx, step) in self.boot_steps.iter().enumerate() {
            let next_pct = ((idx + 1) * 100 / total_steps) as u32;

            // Display current kernel subsystem activity status message
            write!(
                stdout,
                "\x1b[16;1H{}   [BOOT] {}{}",
                colors::DIM_GRAY,
                step.name,
                colors::RESET
            )?;
            stdout.flush()?;

            // Run animation frame loop for stage duration
            animation.animate_stage(step.duration_ms, current_pct, next_pct)?;

            current_pct = next_pct;
        }

        // Slight pause to finish progress bar smoothly at 100%
        thread::sleep(Duration::from_millis(150));

        // Step 3: Print boot completion status message
        writeln!(stdout)?;
        writeln!(stdout)?;

        let elapsed = start_time.elapsed();
        writeln!(
            stdout,
            "{}   ✓ Boot completed successfully in {:.2?}!{}",
            colors::GLOW_CYAN_BLUE,
            elapsed,
            colors::RESET
        )?;

        writeln!(stdout)?;
        stdout.flush()?;

        // Restore cursor
        LogoRenderer::restore_terminal()?;

        Ok(())
    }
}
