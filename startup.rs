//! LensOS v0.2 Runtime Startup & Lifecycle Manager
//!
//! Orchestrates display initialization, double-buffered framebuffer rendering,
//! desktop backdrop drawing, centered LensOS logo rendering, mouse cursor updates,
//! and unified input dispatching.

use crate::compositor::{Compositor, CompositorConfig};
use crate::cursor::{Cursor, MouseState};
use crate::display::{DisplayInfo, PixelFormat, Resolution};
use crate::framebuffer::Framebuffer;
use crate::input::{InputEvent, InputManager, KeyEvent, MouseEvent};
use crate::keyboard::{Keyboard, KeyboardLayout};
use crate::mouse::Mouse;
use crate::qemu::QemuFramebuffer;
use crate::renderer::Renderer;

/// Startup and lifecycle phases for LensOS Runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupPhase {
    Uninitialized,
    InitializingDisplay,
    InitializingFramebuffer,
    InitializingInput,
    InitializingCompositor,
    RenderingSplash,
    Ready,
    Running,
    Shutdown,
}

/// Configuration settings for instantiating the LensOS RuntimeManager.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeConfig {
    pub resolution: Resolution,
    pub pixel_format: PixelFormat,
    pub enable_double_buffering: bool,
    pub enable_cursor: bool,
    pub enable_qemu_lfb: bool,
    pub qemu_lfb_address: usize,
    pub target_fps: u32,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::RES_1024X768,
            pixel_format: PixelFormat::Bgra8888,
            enable_double_buffering: true,
            enable_cursor: true,
            enable_qemu_lfb: false,
            qemu_lfb_address: 0xE0000000,
            target_fps: 60,
        }
    }
}

/// Central orchestrator of the LensOS graphics and input runtime.
pub struct RuntimeManager {
    config: RuntimeConfig,
    phase: StartupPhase,
    display_info: DisplayInfo,
    front_buffer: Framebuffer,
    back_buffer: Option<Framebuffer>,
    compositor: Compositor,
    cursor: Cursor,
    mouse_state: MouseState,
    keyboard: Keyboard,
    mouse: Mouse,
    input_manager: InputManager,
    qemu: QemuFramebuffer,
    frame_count: u64,
}

impl RuntimeManager {
    /// Constructs a new `RuntimeManager` with specified configuration.
    pub fn new(config: RuntimeConfig) -> Self {
        let width = config.resolution.width as usize;
        let height = config.resolution.height as usize;
        let format = config.pixel_format;

        let mut qemu = QemuFramebuffer::default_qemu();
        let front_buffer = qemu.create_framebuffer(width, height, config.enable_qemu_lfb);
        let back_buffer = if config.enable_double_buffering {
            Some(Framebuffer::new(width, height, format))
        } else {
            None
        };

        let compositor_config = CompositorConfig {
            width,
            height,
            format,
            enable_shadows: true,
        };

        let display_info = DisplayInfo::new(config.resolution, format);

        Self {
            config,
            phase: StartupPhase::Uninitialized,
            display_info,
            front_buffer,
            back_buffer,
            compositor: Compositor::new(compositor_config),
            cursor: Cursor::new(),
            mouse_state: MouseState::new((width / 2) as i32, (height / 2) as i32),
            keyboard: Keyboard::new(KeyboardLayout::UsQwerty),
            mouse: Mouse::new(width, height),
            input_manager: InputManager::new(512),
            qemu,
            frame_count: 0,
        }
    }

    /// Initializes all runtime subsystems in sequence.
    pub fn init(&mut self) -> Result<(), &'static str> {
        // Phase 1: Display
        self.phase = StartupPhase::InitializingDisplay;
        self.display_info = self.qemu.create_display_info(
            self.config.resolution.width,
            self.config.resolution.height,
        );

        // Phase 2: Framebuffer
        self.phase = StartupPhase::InitializingFramebuffer;
        let w = self.config.resolution.width as usize;
        let h = self.config.resolution.height as usize;
        self.mouse.set_screen_size(w, h);

        // Phase 3: Input
        self.phase = StartupPhase::InitializingInput;
        self.input_manager.clear();

        // Phase 4: Compositor
        self.phase = StartupPhase::InitializingCompositor;

        // Phase 5: Initial Desktop Frame Render
        self.phase = StartupPhase::RenderingSplash;
        self.render_frame();

        self.phase = StartupPhase::Ready;
        self.phase = StartupPhase::Running;

        Ok(())
    }

    /// Current operational phase.
    #[inline]
    pub fn phase(&self) -> StartupPhase {
        self.phase
    }

    /// True if runtime is actively executing.
    #[inline]
    pub fn is_running(&self) -> bool {
        self.phase == StartupPhase::Running || self.phase == StartupPhase::Ready
    }

    /// Access active display info.
    #[inline]
    pub fn display_info(&self) -> &DisplayInfo {
        &self.display_info
    }

    /// Access compositor.
    #[inline]
    pub fn compositor(&self) -> &Compositor {
        &self.compositor
    }

    /// Access mutable compositor.
    #[inline]
    pub fn compositor_mut(&mut self) -> &mut Compositor {
        &mut self.compositor
    }

    /// Access input manager.
    #[inline]
    pub fn input_manager_mut(&mut self) -> &mut InputManager {
        &mut self.input_manager
    }

    /// Access cursor manager.
    #[inline]
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Access mouse state.
    #[inline]
    pub fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    /// Feeds a keyboard scancode into the keyboard driver and input queue.
    pub fn handle_keyboard_scancode(&mut self, scancode: u8) -> Option<KeyEvent> {
        if let Some(event) = self.keyboard.process_scancode(scancode) {
            self.input_manager.push_event(InputEvent::Key(event.clone()));
            Some(event)
        } else {
            None
        }
    }

    /// Feeds a raw mouse byte into the mouse driver and updates cursor state.
    pub fn handle_mouse_byte(&mut self, byte: u8) -> Option<MouseEvent> {
        if let Some(event) = self.mouse.process_byte(byte) {
            match &event {
                MouseEvent::Moved { x, y, .. } => {
                    let w = self.config.resolution.width as usize;
                    let h = self.config.resolution.height as usize;
                    self.mouse_state.set_pos(*x, *y, w, h);
                }
                MouseEvent::Button { button, state, .. } => {
                    let is_pressed = *state == crate::input::KeyState::Pressed;
                    match button {
                        crate::input::MouseButton::Left => self.mouse_state.left_button = is_pressed,
                        crate::input::MouseButton::Right => self.mouse_state.right_button = is_pressed,
                        crate::input::MouseButton::Middle => self.mouse_state.middle_button = is_pressed,
                        _ => {}
                    }
                }
                MouseEvent::Wheel { delta_y, .. } => {
                    self.mouse_state.scroll_delta += delta_y;
                }
            }

            self.input_manager.push_event(InputEvent::Mouse(event.clone()));
            Some(event)
        } else {
            None
        }
    }

    /// Updates internal animations and state before next frame.
    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
    }

    /// Renders a complete frame:
    /// 1. Draws dark desktop background.
    /// 2. Renders centered LensOS logo and branding.
    /// 3. Composes window layers.
    /// 4. Renders mouse cursor at current pointer coordinates.
    /// 5. Transfers backbuffer to display framebuffer.
    pub fn render_frame(&mut self) {
        let width = self.config.resolution.width as usize;
        let height = self.config.resolution.height as usize;

        if let Some(ref mut backbuffer) = self.back_buffer {
            // 1. Desktop Background
            {
                let mut renderer = Renderer::new(backbuffer);
                renderer.draw_desktop_background();

                // 2. Centered LensOS Logo Placeholder
                let cx = width / 2;
                let cy = (height / 2).saturating_sub(40);
                renderer.draw_lensos_logo(cx, cy, 140);
            }

            // 3. Compositor Layers
            self.compositor.compose(backbuffer);

            // 4. Mouse Cursor
            if self.config.enable_cursor {
                self.cursor.render_to_framebuffer(
                    backbuffer,
                    self.mouse_state.x as usize,
                    self.mouse_state.y as usize,
                );
            }

            // 5. Flip backbuffer to front buffer
            backbuffer.copy_all_to(&mut self.front_buffer);
        } else {
            // Direct frontbuffer rendering
            let mut renderer = Renderer::new(&mut self.front_buffer);
            renderer.draw_desktop_background();
            let cx = width / 2;
            let cy = (height / 2).saturating_sub(40);
            renderer.draw_lensos_logo(cx, cy, 140);

            self.compositor.compose(&mut self.front_buffer);

            if self.config.enable_cursor {
                self.cursor.render_to_framebuffer(
                    &mut self.front_buffer,
                    self.mouse_state.x as usize,
                    self.mouse_state.y as usize,
                );
            }
        }
    }

    /// Performs clean shutdown of the runtime engine.
    pub fn shutdown(&mut self) {
        self.phase = StartupPhase::Shutdown;
    }
}
