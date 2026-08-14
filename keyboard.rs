//! LensOS v0.2 Keyboard Driver & Scancode Decoder
//!
//! Provides PS/2 Scancode Set 1 and Set 2 translation, modifier state tracking,
//! extended key sequences (0xE0), and ASCII character mapping.

use crate::input::{KeyCode, KeyEvent, KeyState, ModifierState};

/// Keyboard layout representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayout {
    UsQwerty,
    UkQwerty,
}

/// PS/2 Scancode parser and keyboard handler.
pub struct Keyboard {
    layout: KeyboardLayout,
    modifiers: ModifierState,
    extended_0xe0: bool,
    extended_0xe1: bool,
}

impl Keyboard {
    pub fn new(layout: KeyboardLayout) -> Self {
        Self {
            layout,
            modifiers: ModifierState::default(),
            extended_0xe0: false,
            extended_0xe1: false,
        }
    }

    pub fn modifiers(&self) -> ModifierState {
        self.modifiers
    }

    pub fn layout(&self) -> KeyboardLayout {
        self.layout
    }

    /// Processes a raw byte from the PS/2 or virtual keyboard controller (Port 0x60).
    /// Returns a parsed `KeyEvent` if a complete keystroke was processed.
    pub fn process_scancode(&mut self, scancode: u8) -> Option<KeyEvent> {
        // Extended byte escape prefixes
        if scancode == 0xE0 {
            self.extended_0xe0 = true;
            return None;
        }
        if scancode == 0xE1 {
            self.extended_0xe1 = true;
            return None;
        }

        let is_extended = self.extended_0xe0;
        self.extended_0xe0 = false;
        self.extended_0xe1 = false;

        let is_release = (scancode & 0x80) != 0;
        let base_code = scancode & 0x7F;
        let state = if is_release {
            KeyState::Released
        } else {
            KeyState::Pressed
        };

        let key_code = if is_extended {
            Self::decode_extended_scancode(base_code)
        } else {
            Self::decode_standard_scancode(base_code)
        };

        // Update internal modifiers
        self.update_modifiers(key_code, state);

        // Derive printable ASCII character
        let character = if state == KeyState::Pressed {
            self.scancode_to_char(key_code)
        } else {
            None
        };

        Some(KeyEvent {
            key: key_code,
            state,
            modifiers: self.modifiers,
            scancode,
            character,
        })
    }

    fn update_modifiers(&mut self, key: KeyCode, state: KeyState) {
        let is_pressed = state == KeyState::Pressed;
        match key {
            KeyCode::LeftShift | KeyCode::RightShift => self.modifiers.shift = is_pressed,
            KeyCode::LeftControl | KeyCode::RightControl => self.modifiers.ctrl = is_pressed,
            KeyCode::LeftAlt | KeyCode::RightAlt => self.modifiers.alt = is_pressed,
            KeyCode::LeftMeta | KeyCode::RightMeta => self.modifiers.meta = is_pressed,
            KeyCode::CapsLock if is_pressed => self.modifiers.caps_lock = !self.modifiers.caps_lock,
            KeyCode::NumLock if is_pressed => self.modifiers.num_lock = !self.modifiers.num_lock,
            KeyCode::ScrollLock if is_pressed => self.modifiers.scroll_lock = !self.modifiers.scroll_lock,
            _ => {}
        }
    }

    fn decode_standard_scancode(code: u8) -> KeyCode {
        match code {
            0x01 => KeyCode::Escape,
            0x02 => KeyCode::Key1,
            0x03 => KeyCode::Key2,
            0x04 => KeyCode::Key3,
            0x05 => KeyCode::Key4,
            0x06 => KeyCode::Key5,
            0x07 => KeyCode::Key6,
            0x08 => KeyCode::Key7,
            0x09 => KeyCode::Key8,
            0x0A => KeyCode::Key9,
            0x0B => KeyCode::Key0,
            0x0C => KeyCode::Minus,
            0x0D => KeyCode::Equal,
            0x0E => KeyCode::Backspace,
            0x0F => KeyCode::Tab,
            0x10 => KeyCode::KeyQ,
            0x11 => KeyCode::KeyW,
            0x12 => KeyCode::KeyE,
            0x13 => KeyCode::KeyR,
            0x14 => KeyCode::KeyT,
            0x15 => KeyCode::KeyY,
            0x16 => KeyCode::KeyU,
            0x17 => KeyCode::KeyI,
            0x18 => KeyCode::KeyO,
            0x19 => KeyCode::KeyP,
            0x1A => KeyCode::LeftBracket,
            0x1B => KeyCode::RightBracket,
            0x1C => KeyCode::Return,
            0x1D => KeyCode::LeftControl,
            0x1E => KeyCode::KeyA,
            0x1F => KeyCode::KeyS,
            0x20 => KeyCode::KeyD,
            0x21 => KeyCode::KeyF,
            0x22 => KeyCode::KeyG,
            0x23 => KeyCode::KeyH,
            0x24 => KeyCode::KeyJ,
            0x25 => KeyCode::KeyK,
            0x26 => KeyCode::KeyL,
            0x27 => KeyCode::Semicolon,
            0x28 => KeyCode::Apostrophe,
            0x29 => KeyCode::Grave,
            0x2A => KeyCode::LeftShift,
            0x2B => KeyCode::Backslash,
            0x2C => KeyCode::KeyZ,
            0x2D => KeyCode::KeyX,
            0x2E => KeyCode::KeyC,
            0x2F => KeyCode::KeyV,
            0x30 => KeyCode::KeyB,
            0x31 => KeyCode::KeyN,
            0x32 => KeyCode::KeyM,
            0x33 => KeyCode::Comma,
            0x34 => KeyCode::Period,
            0x35 => KeyCode::Slash,
            0x36 => KeyCode::RightShift,
            0x38 => KeyCode::LeftAlt,
            0x39 => KeyCode::Space,
            0x3A => KeyCode::CapsLock,
            0x3B => KeyCode::F1,
            0x3C => KeyCode::F2,
            0x3D => KeyCode::F3,
            0x3E => KeyCode::F4,
            0x3F => KeyCode::F5,
            0x40 => KeyCode::F6,
            0x41 => KeyCode::F7,
            0x42 => KeyCode::F8,
            0x43 => KeyCode::F9,
            0x44 => KeyCode::F10,
            0x45 => KeyCode::NumLock,
            0x46 => KeyCode::ScrollLock,
            0x57 => KeyCode::F11,
            0x58 => KeyCode::F12,
            other => KeyCode::Unknown(other),
        }
    }

    fn decode_extended_scancode(code: u8) -> KeyCode {
        match code {
            0x1C => KeyCode::Return,
            0x1D => KeyCode::RightControl,
            0x38 => KeyCode::RightAlt,
            0x47 => KeyCode::Home,
            0x48 => KeyCode::ArrowUp,
            0x49 => KeyCode::PageUp,
            0x4B => KeyCode::ArrowLeft,
            0x4D => KeyCode::ArrowRight,
            0x4F => KeyCode::End,
            0x50 => KeyCode::ArrowDown,
            0x51 => KeyCode::PageDown,
            0x52 => KeyCode::Insert,
            0x53 => KeyCode::Delete,
            0x5B => KeyCode::LeftMeta,
            0x5C => KeyCode::RightMeta,
            other => KeyCode::Unknown(other),
        }
    }

    fn scancode_to_char(&self, key: KeyCode) -> Option<char> {
        let shift = self.modifiers.shift;
        let caps = self.modifiers.caps_lock;
        let uppercase = shift ^ caps;

        let ch = match key {
            KeyCode::KeyA => if uppercase { 'A' } else { 'a' },
            KeyCode::KeyB => if uppercase { 'B' } else { 'b' },
            KeyCode::KeyC => if uppercase { 'C' } else { 'c' },
            KeyCode::KeyD => if uppercase { 'D' } else { 'd' },
            KeyCode::KeyE => if uppercase { 'E' } else { 'e' },
            KeyCode::KeyF => if uppercase { 'F' } else { 'f' },
            KeyCode::KeyG => if uppercase { 'G' } else { 'g' },
            KeyCode::KeyH => if uppercase { 'H' } else { 'h' },
            KeyCode::KeyI => if uppercase { 'I' } else { 'i' },
            KeyCode::KeyJ => if uppercase { 'J' } else { 'j' },
            KeyCode::KeyK => if uppercase { 'K' } else { 'k' },
            KeyCode::KeyL => if uppercase { 'L' } else { 'l' },
            KeyCode::KeyM => if uppercase { 'M' } else { 'm' },
            KeyCode::KeyN => if uppercase { 'N' } else { 'n' },
            KeyCode::KeyO => if uppercase { 'O' } else { 'o' },
            KeyCode::KeyP => if uppercase { 'P' } else { 'p' },
            KeyCode::KeyQ => if uppercase { 'Q' } else { 'q' },
            KeyCode::KeyR => if uppercase { 'R' } else { 'r' },
            KeyCode::KeyS => if uppercase { 'S' } else { 's' },
            KeyCode::KeyT => if uppercase { 'T' } else { 't' },
            KeyCode::KeyU => if uppercase { 'U' } else { 'u' },
            KeyCode::KeyV => if uppercase { 'V' } else { 'v' },
            KeyCode::KeyW => if uppercase { 'W' } else { 'w' },
            KeyCode::KeyX => if uppercase { 'X' } else { 'x' },
            KeyCode::KeyY => if uppercase { 'Y' } else { 'y' },
            KeyCode::KeyZ => if uppercase { 'Z' } else { 'z' },

            KeyCode::Key1 => if shift { '!' } else { '1' },
            KeyCode::Key2 => if shift { '@' } else { '2' },
            KeyCode::Key3 => if shift { '#' } else { '3' },
            KeyCode::Key4 => if shift { '$' } else { '4' },
            KeyCode::Key5 => if shift { '%' } else { '5' },
            KeyCode::Key6 => if shift { '^' } else { '6' },
            KeyCode::Key7 => if shift { '&' } else { '7' },
            KeyCode::Key8 => if shift { '*' } else { '8' },
            KeyCode::Key9 => if shift { '(' } else { '9' },
            KeyCode::Key0 => if shift { ')' } else { '0' },

            KeyCode::Space => ' ',
            KeyCode::Return => '\n',
            KeyCode::Tab => '\t',

            KeyCode::Minus => if shift { '_' } else { '-' },
            KeyCode::Equal => if shift { '+' } else { '=' },
            KeyCode::LeftBracket => if shift { '{' } else { '[' },
            KeyCode::RightBracket => if shift { '}' } else { ']' },
            KeyCode::Backslash => if shift { '|' } else { '\\' },
            KeyCode::Semicolon => if shift { ':' } else { ';' },
            KeyCode::Apostrophe => if shift { '"' } else { '\'' },
            KeyCode::Grave => if shift { '~' } else { '`' },
            KeyCode::Comma => if shift { '<' } else { ',' },
            KeyCode::Period => if shift { '>' } else { '.' },
            KeyCode::Slash => if shift { '?' } else { '/' },

            _ => return None,
        };

        Some(ch)
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new(KeyboardLayout::UsQwerty)
    }
}
