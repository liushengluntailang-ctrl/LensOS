//! # LensOS Animation Module
//!
//! Provides loading animations and progress indicators during the boot sequence.
//! Includes animated spinner frames and progress bars for system startup feedback.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::logo::colors;

/// Spinner animation frame characters for smooth terminal loading loops.
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Loading animation controller for LensOS startup.
pub struct LoadingAnimation {
    /// Text label shown during initialization (e.g. "Starting system...")
    label: String,
}

impl LoadingAnimation {
    /// Creates a new loading animation with a given label.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    /// Runs an animated loading loop for a target duration in milliseconds.
    ///
    /// # Arguments
    /// * `duration_ms` - Total millisecond duration for this animation phase.
    /// * `start_pct`   - Starting percentage (0 - 100).
    /// * `end_pct`     - Ending percentage (0 - 100).
    pub fn animate_stage(&self, duration_ms: u64, start_pct: u32, end_pct: u32) -> io::Result<()> {
        let frame_delay = Duration::from_millis(50);
        let total_frames = (duration_ms / 50).max(1);

        let mut stdout = io::stdout();

        for frame in 0..total_frames {
            let spinner_char = SPINNER_FRAMES[(frame as usize) % SPINNER_FRAMES.len()];
            let progress = start_pct + ((end_pct - start_pct) * (frame as u32 + 1) / total_frames as u32);

            // Construct 20-character progress bar: [████████░░░░░░░░░░░░]
            let filled_width = (progress as usize * 20) / 100;
            let empty_width = 20 - filled_width;
            let bar_fill = "█".repeat(filled_width);
            let bar_empty = "░".repeat(empty_width);

            // Print overwriting single line using carriage return '\r'
            write!(
                stdout,
                "\r   {} [{}{}{}{}] {} {}%  {} {}  {} {} {}",
                colors::GLOW_CYAN_BLUE,
                colors::DEEP_BLUE,
                bar_fill,
                colors::DIM_GRAY,
                bar_empty,
                colors::BRIGHT_WHITE,
                progress,
                colors::GLOW_CYAN_BLUE,
                spinner_char,
                colors::BRIGHT_WHITE,
                self.label,
                colors::RESET
            )?;
            stdout.flush()?;

            thread::sleep(frame_delay);
        }

        Ok(())
    }

    /// Clears the loading line after completion.
    #[allow(dead_code)]
    pub fn finish_line() -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "\r{}\r", " ".repeat(80))?;
        stdout.flush()?;
        Ok(())
    }
}
