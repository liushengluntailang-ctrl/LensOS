//! LensOS v0.1 - Input Subsystem
//!
//! Manages keyboard scan codes, mouse pointer events, interrupt request (IRQ) dispatchers,
//! and USB/PS2 Human Interface Device (HID) event queues for LensOS.

use std::collections::VecDeque;

/// Key event press/release states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Keyboard key modifier flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Keyboard event representation.
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub scan_code: u16,
    pub key_code: char,
    pub state: KeyState,
    pub modifiers: KeyModifiers,
}

/// Mouse button state bits.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

/// Relative motion or position event for mouse/trackpad.
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub delta_x: i32,
    pub delta_y: i32,
    pub scroll_delta: i8,
    pub buttons: MouseButtons,
}

/// Core Input Manager for LensOS.
pub struct InputSubsystem {
    initialized: bool,
    key_queue: VecDeque<KeyEvent>,
    mouse_queue: VecDeque<MouseEvent>,
    mouse_position: (i32, i32),
    modifiers: KeyModifiers,
}

impl InputSubsystem {
    /// Constructs a new InputSubsystem instance.
    pub fn new() -> Self {
        Self {
            initialized: false,
            key_queue: VecDeque::new(),
            mouse_queue: VecDeque::new(),
            mouse_position: (960, 540), // Centered on 1920x1080
            modifiers: KeyModifiers::default(),
        }
    }

    /// Initializes keyboard (IRQ 1) and mouse (IRQ 12) input controllers.
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("[BOOT][INPUT] Probing PS/2 Controller & USB HID Host drivers...");
        println!("[BOOT][INPUT] Registering Keyboard Interrupt Handler on IRQ 1 (US-QWERTY layout)...");
        println!("[BOOT][INPUT] Registering PS/2 / USB Mouse Interrupt Handler on IRQ 12...");

        self.initialized = true;
        println!("[BOOT][INPUT] Input event queues initialized. Keyboard and Mouse drivers active.");
        Ok(())
    }

    /// Pushes a raw hardware key interrupt event to the queue.
    pub fn push_key_event(&mut self, event: KeyEvent) {
        if !self.initialized {
            return;
        }
        self.key_queue.push_back(event);
    }

    /// Pops the next pending key event from the queue.
    pub fn pop_key_event(&mut self) -> Option<KeyEvent> {
        self.key_queue.pop_front()
    }

    /// Pushes a mouse input packet event to the queue.
    pub fn push_mouse_event(&mut self, event: MouseEvent) {
        if !self.initialized {
            return;
        }
        // Update accumulated cursor position bounded to screen
        self.mouse_position.0 = (self.mouse_position.0 + event.delta_x).clamp(0, 1920);
        self.mouse_position.1 = (self.mouse_position.1 + event.delta_y).clamp(0, 1080);
        self.mouse_queue.push_back(event);
    }

    /// Pops the next pending mouse event from the queue.
    pub fn pop_mouse_event(&mut self) -> Option<MouseEvent> {
        self.mouse_queue.pop_front()
    }

    /// Returns the current (X, Y) desktop cursor coordinates.
    pub fn get_mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    /// Returns active key modifiers.
    pub fn get_modifiers(&self) -> KeyModifiers {
        self.modifiers
    }

    /// Shuts down the input subsystem and unregisters interrupt vectors.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }
        println!("[SHUTDOWN][INPUT] Disabling input IRQs and clearing HID buffers...");
        self.key_queue.clear();
        self.mouse_queue.clear();
        self.initialized = false;
        Ok(())
    }

    /// Checks if input subsystem is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for InputSubsystem {
    fn default() -> Self {
        Self::new()
    }
}
