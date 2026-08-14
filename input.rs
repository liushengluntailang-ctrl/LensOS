//! LensOS v0.2 Input Subsystem
//!
//! Provides abstract input events, keycodes, modifier states,
//! mouse interaction types, and the centralized InputManager event queue.

use std::collections::VecDeque;

/// Standard Key Codes supported by LensOS input subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Alphanumeric
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ,
    KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT,
    KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Function Keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Controls & Modifiers
    Escape,
    Return,
    Space,
    Backspace,
    Tab,
    CapsLock,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftMeta,
    RightMeta,

    // Navigation & Editing
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Punctuation & Symbols
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Period,
    Slash,

    // Lock & State
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,

    // Unknown Key
    Unknown(u8),
}

/// Key transition state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
    Repeated,
}

/// Keyboard modifier status mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
}

/// High-level Keyboard Event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub state: KeyState,
    pub modifiers: ModifierState,
    pub scancode: u8,
    pub character: Option<char>,
}

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

/// High-level Mouse Event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEvent {
    Moved {
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
    },
    Button {
        button: MouseButton,
        state: KeyState,
        x: i32,
        y: i32,
    },
    Wheel {
        delta_x: i32,
        delta_y: i32,
    },
}

/// Unified Input Event Enum for LensOS kernel and applications.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    DeviceConnected(&'static str),
    DeviceDisconnected(&'static str),
    Custom(u32, u64),
}

/// Centralized Input Manager with a FIFO Event Queue.
pub struct InputManager {
    event_queue: VecDeque<InputEvent>,
    max_queue_size: usize,
    modifiers: ModifierState,
}

impl InputManager {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            event_queue: VecDeque::with_capacity(max_queue_size),
            max_queue_size,
            modifiers: ModifierState::default(),
        }
    }

    pub fn push_event(&mut self, event: InputEvent) {
        if self.event_queue.len() >= self.max_queue_size {
            self.event_queue.pop_front();
        }
        self.event_queue.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<InputEvent> {
        self.event_queue.pop_front()
    }

    pub fn peek_event(&self) -> Option<&InputEvent> {
        self.event_queue.front()
    }

    pub fn has_events(&self) -> bool {
        !self.event_queue.is_empty()
    }

    pub fn queue_len(&self) -> usize {
        self.event_queue.len()
    }

    pub fn clear(&mut self) {
        self.event_queue.clear();
    }

    pub fn current_modifiers(&self) -> ModifierState {
        self.modifiers
    }

    pub fn set_modifiers(&mut self, modifiers: ModifierState) {
        self.modifiers = modifiers;
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new(256)
    }
}
