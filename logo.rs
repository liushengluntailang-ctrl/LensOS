//! # LensOS Logo Module
//!
//! Provides ASCII art rendering and visual display components for the
//! LensOS boot screen. Handles ANSI color codes for glowing blue visuals
//! and black background formatting.

use std::io::{self, Write};

/// Color constants for ANSI terminal formatting.
pub mod colors {
    /// Resets all styling and colors to default terminal settings.
    pub const RESET: &str = "\x1b[0m";
    /// Sets terminal background to solid black.
    pub const BG_BLACK: &str = "\x1b[40m";
    /// Clears the terminal screen completely.
    pub const CLEAR_SCREEN: &str = "\x1b[2J";
    /// Moves cursor to top-left position (row 1, col 1).
    pub const CURSOR_HOME: &str = "\x1b[1;1H";
    /// Hides the terminal cursor during boot sequence.
    pub const HIDE_CURSOR: &str = "\x1b[?25l";
    /// Restores the terminal cursor after boot completes.
    pub const SHOW_CURSOR: &str = "\x1b[?25h";

    /// Bright vibrant cyan-blue for glowing lens highlight.
    pub const GLOW_CYAN_BLUE: &str = "\x1b[1;38;5;39m";
    /// Deep electric blue for primary text styling.
    pub const DEEP_BLUE: &str = "\x1b[1;38;5;33m";
    /// Soft sky-blue for aperture geometry lines.
    pub const SOFT_BLUE: &str = "\x1b[38;5;75m";
    /// Bright white for high-contrast version text.
    pub const BRIGHT_WHITE: &str = "\x1b[1;97m";
    /// Dim gray for subtle status messages.
    pub const DIM_GRAY: &str = "\x1b[38;5;244m";
}

/// Renderer responsible for displaying the glowing LensOS ASCII logo.
pub struct LogoRenderer;

impl LogoRenderer {
    /// Creates a new instance of `LogoRenderer`.
    pub fn new() -> Self {
        LogoRenderer
    }

    /// Clears the screen, applies a black background, and renders the blue glowing logo.
    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Prepare black screen & hide cursor
        write!(
            stdout,
            "{}{}{}{}",
            colors::CLEAR_SCREEN,
            colors::CURSOR_HOME,
            colors::BG_BLACK,
            colors::HIDE_CURSOR
        )?;

        // Top padding lines
        writeln!(stdout)?;
        writeln!(stdout)?;

        // Optical Lens ASCII Graphic (Glowing cyan & soft blue)
        writeln!(
            stdout,
            "{}                  .---.                 {}",
            colors::SOFT_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}                /  . .  \\               {}",
            colors::SOFT_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}               |  ( {}O{} )  |              {}",
            colors::SOFT_BLUE,
            colors::GLOW_CYAN_BLUE,
            colors::SOFT_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}                \\  ' '  /               {}",
            colors::SOFT_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}                  '---'                 {}",
            colors::SOFT_BLUE,
            colors::RESET
        )?;

        writeln!(stdout)?;

        // Glowing Blue "LENS" ASCII Typography
        writeln!(
            stdout,
            "{}    ██╗     ███████╗███╗   ██╗███████╗    ██████╗ ███████╗{}",
            colors::GLOW_CYAN_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}    ██║     ██╔════╝████╗  ██║██╔════╝   ██╔═══██╗██╔════╝{}",
            colors::GLOW_CYAN_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}    ██║     █████╗  ██╔██╗ ██║███████╗   ██║   ██║███████╗{}",
            colors::DEEP_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}    ██║     ██╔══╝  ██║╚██╗██║╚════██║   ██║   ██║╚════██║{}",
            colors::DEEP_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}    ███████╗███████╗██║ ╚████║███████║   ╚██████╔╝███████║{}",
            colors::GLOW_CYAN_BLUE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}    ╚══════╝╚══════╝╚═╝  ╚═══╝╚══════╝    ╚═════╝ ╚══════╝{}",
            colors::GLOW_CYAN_BLUE,
            colors::RESET
        )?;

        writeln!(stdout)?;

        // LensOS Version Tagline & Subtitle
        writeln!(
            stdout,
            "{}                     LensOS v0.1{}",
            colors::BRIGHT_WHITE,
            colors::RESET
        )?;
        writeln!(
            stdout,
            "{}           Next-Gen AI Operating System-{}",
            colors::SOFT_BLUE,
            colors::RESET
        )?;

        writeln!(stdout)?;
        stdout.flush()?;

        Ok(())
    }

    /// Restores standard terminal settings (cursor visible, default colors).
    pub fn restore_terminal() -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "{}{}", colors::SHOW_CURSOR, colors::RESET)?;
        stdout.flush()?;
        Ok(())
    }
}
