//! # LensOS Boot Module - Main Entry Point
//!
//! Entry point for the LensOS operating system boot simulation.
//! Coordinates the modular boot process, initializing hardware abstraction,
//! microkernel subsystems, and visual boot screens.

mod animation;
mod bootloader;
mod logo;

use bootloader::Bootloader;
use std::process::ExitCode;

/// Main function for the LensOS boot module.
///
/// Instantiates the LensOS `Bootloader` and triggers the ~3-second boot sequence.
/// On completion, displays success messaging and returns clean exit code.
fn main() -> ExitCode {
    let bootloader = Bootloader::new();

    match bootloader.execute_boot() {
        Ok(_) => {
            // Explicit required output upon completion
            println!("Boot completed successfully");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("LensOS Boot Error: Failed during boot sequence: {}", err);
            ExitCode::FAILURE
        }
    }
}
