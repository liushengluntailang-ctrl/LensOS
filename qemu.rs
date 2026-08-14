//! LensOS v0.2 QEMU Hardware & Virtual Device Subsystem
//!
//! Provides Bochs VBE (Virtual Graphics Extensions) configuration,
//! QEMU Port 0xE9 debug console streaming, PCI Linear Framebuffer address mappings,
//! and QEMU-compatible framebuffer initialization.

use crate::display::{DisplayInfo, DisplayMode, PixelFormat, Resolution};
use crate::framebuffer::Framebuffer;

/// Bochs VBE I/O Port Addresses for QEMU `-vga std` and `-device VGA`.
pub const VBE_DISPI_IOPORT_INDEX: u16 = 0x01CE;
pub const VBE_DISPI_IOPORT_DATA: u16 = 0x01CF;

/// Bochs VBE Register Indices.
pub const VBE_DISPI_INDEX_ID: u16 = 0x0;
pub const VBE_DISPI_INDEX_XRES: u16 = 0x1;
pub const VBE_DISPI_INDEX_YRES: u16 = 0x2;
pub const VBE_DISPI_INDEX_BPP: u16 = 0x3;
pub const VBE_DISPI_INDEX_ENABLE: u16 = 0x4;
pub const VBE_DISPI_INDEX_BANK: u16 = 0x5;
pub const VBE_DISPI_INDEX_VIRT_WIDTH: u16 = 0x6;
pub const VBE_DISPI_INDEX_VIRT_HEIGHT: u16 = 0x7;
pub const VBE_DISPI_INDEX_X_OFFSET: u16 = 0x8;
pub const VBE_DISPI_INDEX_Y_OFFSET: u16 = 0x9;

/// Bochs VBE Mode Control Flags.
pub const VBE_DISPI_DISABLED: u16 = 0x00;
pub const VBE_DISPI_ENABLED: u16 = 0x01;
pub const VBE_DISPI_LFB_ENABLED: u16 = 0x40;
pub const VBE_DISPI_NOCLEARMEM: u16 = 0x80;

/// Default PCI Linear Frame Buffer (LFB) physical address in QEMU.
pub const QEMU_DEFAULT_LFB_ADDRESS: usize = 0xE0000000;

/// QEMU ISA Debug Console I/O Port.
pub const QEMU_DEBUG_PORT: u16 = 0x00E9;

/// Supported QEMU display controller types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuDisplayType {
    /// Bochs VBE / QEMU `-vga std`
    BochsVbe,
    /// VirtIO GPU `-device virtio-gpu-pci`
    VirtioGpu,
    /// RAMFB standalone virtual framebuffer `-device ramfb`
    Ramfb,
    /// QXL high performance SPICE display `-vga qxl`
    Qxl,
    /// Fallback Cirrus Logic GD5446 `-vga cirrus`
    Cirrus,
}

/// Bochs VBE driver for QEMU graphics programming.
pub struct QemuBochsVbe {
    lfb_address: usize,
    width: u16,
    height: u16,
    bpp: u16,
}

impl QemuBochsVbe {
    pub const fn new(lfb_address: usize) -> Self {
        Self {
            lfb_address,
            width: 1024,
            height: 768,
            bpp: 32,
        }
    }

    #[inline]
    pub fn lfb_address(&self) -> usize {
        self.lfb_address
    }

    #[inline]
    pub fn resolution(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    #[inline]
    pub fn bpp(&self) -> u16 {
        self.bpp
    }

    /// Prepares display mode parameters for QEMU VBE.
    pub fn configure_mode(&mut self, width: u16, height: u16, bpp: u16) {
        self.width = width;
        self.height = height;
        self.bpp = bpp;
    }
}

/// QEMU Debug Port (0xE9) text output streamer.
pub struct QemuDebugPort;

impl QemuDebugPort {
    /// Writes a slice of ASCII bytes to the QEMU debug console.
    pub fn write_str(s: &str) {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            for b in s.bytes() {
                unsafe {
                    core::arch::asm!(
                        "out dx, al",
                        in("dx") QEMU_DEBUG_PORT,
                        in("al") b,
                        options(nomem, nostack, preserves_flags)
                    );
                }
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            let _ = s;
        }
    }
}

/// QEMU-compatible Framebuffer Initializer and bridge.
pub struct QemuFramebuffer {
    display_type: QemuDisplayType,
    vbe: QemuBochsVbe,
}

impl QemuFramebuffer {
    pub fn new(display_type: QemuDisplayType, lfb_base: usize) -> Self {
        Self {
            display_type,
            vbe: QemuBochsVbe::new(lfb_base),
        }
    }

    pub fn default_qemu() -> Self {
        Self::new(QemuDisplayType::BochsVbe, QEMU_DEFAULT_LFB_ADDRESS)
    }

    pub fn display_type(&self) -> QemuDisplayType {
        self.display_type
    }

    /// Initializes a Framebuffer instance for the QEMU environment.
    ///
    /// If `use_hardware_lfb` is true, attempts to map the physical LFB address directly.
    /// Otherwise, creates a double-buffered memory buffer suitable for blitting.
    pub fn create_framebuffer(
        &mut self,
        width: usize,
        height: usize,
        use_hardware_lfb: bool,
    ) -> Framebuffer {
        self.vbe.configure_mode(width as u16, height as u16, 32);

        if use_hardware_lfb {
            unsafe {
                Framebuffer::from_raw(
                    self.vbe.lfb_address() as *mut u8,
                    width,
                    height,
                    width,
                    PixelFormat::Bgra8888,
                )
            }
        } else {
            Framebuffer::new(width, height, PixelFormat::Bgra8888)
        }
    }

    /// Produces a matching `DisplayInfo` descriptor for QEMU.
    pub fn create_display_info(&self, width: u32, height: u32) -> DisplayInfo {
        DisplayInfo {
            resolution: Resolution::new(width, height, 60),
            format: PixelFormat::Bgra8888,
            mode: DisplayMode::QemuVga,
            scale_factor: 1.0,
            name: String::from("QEMU Bochs VBE Standard Display"),
            is_hardware_accelerated: false,
        }
    }
}

impl Default for QemuFramebuffer {
    fn default() -> Self {
        Self::default_qemu()
    }
}
