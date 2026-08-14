//! # LensOS v0.2 Runtime Engine
//!
//! The `runtime` module provides the graphics display pipeline, linear framebuffer
//! drivers, software 2D renderer, multi-layer window compositor, cursor rendering,
//! keyboard/mouse input drivers, and QEMU hardware initialization for LensOS v0.2.
//!
//! ## Subsystems
//!
//! - [`display`]: Pixel formats, screen resolution definitions, and display metadata.
//! - [`framebuffer`]: Framebuffer memory representations, color primitives, and blitting.
//! - [`renderer`]: 2D geometric shapes, built-in bitmap fonts, and LensOS brand graphics.
//! - [`compositor`]: Multi-surface z-ordered alpha compositor with drop shadow support.
//! - [`cursor`]: Mouse cursor shapes, hotspot positioning, and software pointer rendering.
//! - [`input`]: Unified input event queue, KeyCodes, KeyEvents, and MouseEvents.
//! - [`keyboard`]: PS/2 scancode sets 1 & 2 decoder, modifier state tracking, and layout mapping.
//! - [`mouse`]: PS/2 and IntelliMouse packet parser with delta sign extension and clamping.
//! - [`startup`]: [`RuntimeManager`] orchestrating the startup lifecycle, main loop, and frame rendering.
//! - [`qemu`]: Bochs VBE graphics register programming, Port 0xE9 debug streaming, and RAMFB support.

pub mod compositor;
pub mod cursor;
pub mod display;
pub mod framebuffer;
pub mod input;
pub mod keyboard;
pub mod mouse;
pub mod qemu;
pub mod renderer;
pub mod startup;

// Re-export primary types for LensOS modules (boot, kernel, desktop, ui, system)
pub use compositor::{Compositor, CompositorConfig, Layer, LayerId, WindowSurface};
pub use cursor::{Cursor, CursorShape, MouseState};
pub use display::{DisplayInfo, DisplayMode, PixelFormat, Resolution};
pub use framebuffer::{Color, Framebuffer, FramebufferConfig};
pub use input::{
    InputDevice, InputEvent, InputManager, KeyCode, KeyEvent, KeyState, ModifierState, MouseButton,
    MouseEvent,
};
pub use keyboard::{Keyboard, KeyboardLayout, Scancode};
pub use mouse::{Mouse, MouseButtonState, MousePacket};
pub use qemu::{QemuBochsVbe, QemuDebugPort, QemuDisplayType, QemuFramebuffer};
pub use renderer::{Font, Renderer};
pub use startup::{RuntimeConfig, RuntimeManager, StartupPhase};

/// LensOS Runtime Version identifier.
pub const RUNTIME_VERSION: &str = "0.2.0";

/// Scancode type alias for convenience in low-level kernel drivers.
pub type Scancode = u8;

/// Device identifier type placeholder for input hardware enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
    Ps2Keyboard,
    Ps2Mouse,
    VirtioInput,
    SerialPort,
    VirtualDevice,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_initialization() {
        let config = RuntimeConfig {
            resolution: Resolution::new(800, 600, 60),
            pixel_format: PixelFormat::Bgra8888,
            enable_double_buffering: true,
            enable_cursor: true,
            enable_qemu_lfb: false,
            qemu_lfb_address: 0xE0000000,
            target_fps: 60,
        };

        let mut runtime = RuntimeManager::new(config);
        assert_eq!(runtime.phase(), StartupPhase::Uninitialized);

        let init_res = runtime.init();
        assert!(init_res.is_ok());
        assert!(runtime.is_running());
        assert_eq!(runtime.phase(), StartupPhase::Running);
    }

    #[test]
    fn test_color_blending() {
        let red = Color::from_rgba(255, 0, 0, 255);
        let semi_blue = Color::from_rgba(0, 0, 255, 128);
        let blended = semi_blue.blend_over(red);

        assert!(blended.r > 0);
        assert!(blended.b > 0);
    }

    #[test]
    fn test_keyboard_scancode_translation() {
        let mut kb = Keyboard::new(KeyboardLayout::UsQwerty);
        // Press 'A' (Set 1 scancode 0x1E)
        let event = kb.process_scancode(0x1E);
        assert!(event.is_some());
        let ev = event.unwrap();
        assert_eq!(ev.key, KeyCode::KeyA);
        assert_eq!(ev.state, KeyState::Pressed);
        assert_eq!(ev.character, Some('a'));

        // Release 'A' (Set 1 scancode 0x1E | 0x80 = 0x9E)
        let rel_event = kb.process_scancode(0x9E);
        assert!(rel_event.is_some());
        let rel_ev = rel_event.unwrap();
        assert_eq!(rel_ev.key, KeyCode::KeyA);
        assert_eq!(rel_ev.state, KeyState::Released);
    }

    #[test]
    fn test_compositor_layers() {
        let mut compositor = Compositor::new(CompositorConfig::default());
        let layer1 = compositor.create_layer("Test Layer", 200, 200);
        assert_eq!(layer1.0, 1);

        assert!(compositor.get_layer(layer1).is_some());
        compositor.bring_to_front(layer1);
        assert!(compositor.remove_layer(layer1));
        assert!(compositor.get_layer(layer1).is_none());
    }
}
