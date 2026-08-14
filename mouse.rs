//! LensOS v0.2 Mouse Driver & Packet Decoder
//!
//! Provides PS/2 3-byte standard and 4-byte IntelliMouse (scroll wheel) packet processing,
//! delta sign-extension, sensitivity scaling, and mouse event generation.

use crate::input::{KeyState, MouseButton, MouseEvent};

/// Individual button states for mouse input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MouseButtonState {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

/// Decoded raw PS/2 mouse packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MousePacket {
    pub flags: u8,
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
    pub left_btn: bool,
    pub right_btn: bool,
    pub middle_btn: bool,
}

/// PS/2 and virtual mouse state machine and event generator.
pub struct Mouse {
    packet_buf: [u8; 4],
    packet_idx: usize,
    has_scroll_wheel: bool,
    buttons: MouseButtonState,
    x: i32,
    y: i32,
    screen_width: usize,
    screen_height: usize,
    sensitivity: f32,
}

impl Mouse {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            packet_buf: [0; 4],
            packet_idx: 0,
            has_scroll_wheel: false,
            buttons: MouseButtonState::default(),
            x: (screen_width / 2) as i32,
            y: (screen_height / 2) as i32,
            screen_width,
            screen_height,
            sensitivity: 1.0,
        }
    }

    pub fn set_scroll_wheel_support(&mut self, enabled: bool) {
        self.has_scroll_wheel = enabled;
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.screen_width = width;
        self.screen_height = height;
        self.clamp_position();
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity.max(0.1);
    }

    #[inline]
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn buttons(&self) -> MouseButtonState {
        self.buttons
    }

    /// Feeds a raw byte into the mouse packet accumulator.
    /// Returns a parsed `MouseEvent` once a complete packet has been decoded.
    pub fn process_byte(&mut self, byte: u8) -> Option<MouseEvent> {
        // PS/2 mouse synchronization: Byte 0 must have bit 3 (0x08) set.
        if self.packet_idx == 0 && (byte & 0x08) == 0 {
            // Out of sync - drop byte and wait for valid packet header
            return None;
        }

        self.packet_buf[self.packet_idx] = byte;
        self.packet_idx += 1;

        let expected_len = if self.has_scroll_wheel { 4 } else { 3 };

        if self.packet_idx >= expected_len {
            self.packet_idx = 0;
            return self.decode_packet();
        }

        None
    }

    fn decode_packet(&mut self) -> Option<MouseEvent> {
        let flags = self.packet_buf[0];
        let raw_x = self.packet_buf[1];
        let raw_y = self.packet_buf[2];

        // Check for overflow flags
        let x_overflow = (flags & 0x40) != 0;
        let y_overflow = (flags & 0x80) != 0;
        if x_overflow || y_overflow {
            return None;
        }

        // Sign extension for 9-bit delta values
        let mut dx = raw_x as i32;
        if (flags & 0x10) != 0 {
            dx |= !0xFF; // sign extend negative
        }

        let mut dy = raw_y as i32;
        if (flags & 0x20) != 0 {
            dy |= !0xFF; // sign extend negative
        }

        // Invert Y axis for screen space (PS/2 reports positive Y as upwards)
        dy = -dy;

        // Apply sensitivity scaling
        dx = ((dx as f32) * self.sensitivity) as i32;
        dy = ((dy as f32) * self.sensitivity) as i32;

        let new_left = (flags & 0x01) != 0;
        let new_right = (flags & 0x02) != 0;
        let new_middle = (flags & 0x04) != 0;

        // Check for button state changes
        if new_left != self.buttons.left {
            self.buttons.left = new_left;
            return Some(MouseEvent::Button {
                button: MouseButton::Left,
                state: if new_left { KeyState::Pressed } else { KeyState::Released },
                x: self.x,
                y: self.y,
            });
        }

        if new_right != self.buttons.right {
            self.buttons.right = new_right;
            return Some(MouseEvent::Button {
                button: MouseButton::Right,
                state: if new_right { KeyState::Pressed } else { KeyState::Released },
                x: self.x,
                y: self.y,
            });
        }

        if new_middle != self.buttons.middle {
            self.buttons.middle = new_middle;
            return Some(MouseEvent::Button {
                button: MouseButton::Middle,
                state: if new_middle { KeyState::Pressed } else { KeyState::Released },
                x: self.x,
                y: self.y,
            });
        }

        // Check scroll wheel delta (Byte 3 in IntelliMouse mode)
        if self.has_scroll_wheel && self.packet_buf[3] != 0 {
            let mut dz = self.packet_buf[3] as i8 as i32;
            return Some(MouseEvent::Wheel {
                delta_x: 0,
                delta_y: dz,
            });
        }

        // Motion Event
        if dx != 0 || dy != 0 {
            self.x += dx;
            self.y += dy;
            self.clamp_position();

            return Some(MouseEvent::Moved {
                x: self.x,
                y: self.y,
                dx,
                dy,
            });
        }

        None
    }

    fn clamp_position(&mut self) {
        let max_w = (self.screen_width.saturating_sub(1)) as i32;
        let max_h = (self.screen_height.saturating_sub(1)) as i32;
        self.x = self.x.clamp(0, max_w);
        self.y = self.y.clamp(0, max_h);
    }
}
