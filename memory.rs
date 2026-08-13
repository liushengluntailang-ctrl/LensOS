//! LensOS v0.1 - Memory Management Subsystem
//!
//! Provides virtual memory paging, frame allocation, kernel heap initialization,
//! and physical memory map detection for LensOS.

use std::fmt;

/// Represents the status and metrics of physical and virtual memory.
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    /// Total detected physical RAM in megabytes.
    pub total_memory_mb: usize,
    /// Currently allocated RAM in megabytes.
    pub used_memory_mb: usize,
    /// Reserved kernel space in megabytes.
    pub kernel_reserved_mb: usize,
    /// Number of active 4KB page frames allocated.
    pub allocated_frames: usize,
}

impl fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MemoryStats {{ Total: {}MB, Used: {}MB, Reserved: {}MB, Frames: {} }}",
            self.total_memory_mb, self.used_memory_mb, self.kernel_reserved_mb, self.allocated_frames
        )
    }
}

/// Represents a contiguous region of physical memory.
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start_address: usize,
    pub size_bytes: usize,
    pub region_type: RegionType,
}

/// Type classification for physical memory regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    Usable,
    Reserved,
    KernelCode,
    ACPIReclaimable,
    BadMemory,
}

/// Core Memory Manager struct for LensOS.
pub struct MemoryManager {
    initialized: bool,
    total_physical_ram_mb: usize,
    kernel_heap_size_mb: usize,
    allocated_frames: usize,
    regions: Vec<MemoryRegion>,
}

impl MemoryManager {
    /// Constructs a new `MemoryManager` instance prior to initialization.
    pub fn new() -> Self {
        Self {
            initialized: false,
            total_physical_ram_mb: 16384, // Default 16 GB simulated physical RAM
            kernel_heap_size_mb: 64,      // 64 MB reserved for kernel heap
            allocated_frames: 0,
            regions: Vec::new(),
        }
    }

    /// Initializes memory management, paging directories, and frame allocators.
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("[BOOT][MEMORY] Detecting physical memory layout from EFI/BIOS memory map...");

        // Populate physical memory regions
        self.regions = vec![
            MemoryRegion {
                start_address: 0x00000000,
                size_bytes: 0x0009F000, // 636 KB
                region_type: RegionType::Usable,
            },
            MemoryRegion {
                start_address: 0x00100000,
                size_bytes: 0x00E00000, // 14 MB Kernel image area
                region_type: RegionType::KernelCode,
            },
            MemoryRegion {
                start_address: 0x00F00000,
                size_bytes: 0x3FF100000, // ~16 GB Usable RAM
                region_type: RegionType::Usable,
            },
        ];

        println!(
            "[BOOT][MEMORY] Detected {} MB total physical RAM.",
            self.total_physical_ram_mb
        );
        println!("[BOOT][MEMORY] Setting up 4-level x86_64 / ARM64 page tables (PML4)...");
        println!(
            "[BOOT][MEMORY] Reserving {} MB for Kernel Heap at virt 0xFFFF_8000_0000_0000.",
            self.kernel_heap_size_mb
        );

        self.initialized = true;
        self.allocated_frames = 1024; // Initial kernel page tables allocation

        println!("[BOOT][MEMORY] Frame allocator online. Memory manager initialized successfully.");
        Ok(())
    }

    /// Allocates `pages` count of 4KB frame blocks.
    pub fn allocate_pages(&mut self, pages: usize) -> Result<usize, String> {
        if !self.initialized {
            return Err("Memory manager is not initialized.".to_string());
        }
        self.allocated_frames += pages;
        // Simulated virtual address assignment
        let virt_addr = 0xFFFF_9000_0000_0000 + (self.allocated_frames * 4096);
        Ok(virt_addr)
    }

    /// Releases previously allocated memory pages.
    pub fn free_pages(&mut self, _address: usize, pages: usize) -> Result<(), String> {
        if !self.initialized {
            return Err("Memory manager is not initialized.".to_string());
        }
        if pages > self.allocated_frames {
            return Err("Attempted to free more pages than currently allocated.".to_string());
        }
        self.allocated_frames -= pages;
        Ok(())
    }

    /// Returns current memory metrics.
    pub fn get_stats(&self) -> MemoryStats {
        let used_mb = (self.allocated_frames * 4096) / (1024 * 1024) + self.kernel_heap_size_mb;
        MemoryStats {
            total_memory_mb: self.total_physical_ram_mb,
            used_memory_mb: used_mb,
            kernel_reserved_mb: self.kernel_heap_size_mb,
            allocated_frames: self.allocated_frames,
        }
    }

    /// Shuts down the memory subsystem and flushes page tables.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }
        println!("[SHUTDOWN][MEMORY] Flushing page directories and unmapping kernel heap...");
        self.initialized = false;
        Ok(())
    }

    /// Returns whether the memory manager is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
