//! LensOS v0.1 - Virtual File System (VFS) Subsystem
//!
//! Handles file node creation, virtual device file mounts (/dev, /proc),
//! directory hierarchies, and file I/O operations for LensOS.

use std::collections::HashMap;

/// File type classification in LensOS VFS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
    CharDevice,
    BlockDevice,
}

/// Node representation in the Virtual File System (VNode).
#[derive(Debug, Clone)]
pub struct VNode {
    pub name: String,
    pub path: String,
    pub node_type: FileType,
    pub size_bytes: usize,
    pub content: Vec<u8>,
}

/// Mounted filesystem information.
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub mount_path: String,
    pub fs_type: String,
}

/// Core Virtual File System manager.
pub struct VirtualFileSystem {
    initialized: bool,
    mounts: Vec<MountPoint>,
    files: HashMap<String, VNode>,
}

impl VirtualFileSystem {
    /// Constructs a new VFS instance.
    pub fn new() -> Self {
        Self {
            initialized: false,
            mounts: Vec::new(),
            files: HashMap::new(),
        }
    }

    /// Initializes VFS tree and mounts root, proc, and dev filesystems.
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("[BOOT][VFS] Initializing Virtual File System (VFS) abstraction layer...");

        // Mount Root Filesystem
        self.mount("/", "LensFS")?;
        // Mount pseudo filesystems
        self.mount("/dev", "DevFS")?;
        self.mount("/proc", "ProcFS")?;
        self.mount("/sys", "SysFS")?;

        // Create initial default directories
        self.create_directory("/")?;
        self.create_directory("/dev")?;
        self.create_directory("/proc")?;
        self.create_directory("/sys")?;
        self.create_directory("/bin")?;
        self.create_directory("/etc")?;

        // Populate essential system device nodes
        self.create_device_node("/dev/tty0", FileType::CharDevice)?;
        self.create_device_node("/dev/null", FileType::CharDevice)?;
        self.create_device_node("/dev/sda1", FileType::BlockDevice)?;

        // System information entry
        self.write_file(
            "/etc/lensos-release",
            b"LensOS v0.1-alpha (Kernel build: 2026.08)",
        )?;

        self.initialized = true;
        println!("[BOOT][VFS] Mounted root '/', '/dev', '/proc', and '/sys' successfully.");
        Ok(())
    }

    /// Mounts a filesystem driver at a specific directory target.
    pub fn mount(&mut self, target: &str, fstype: &str) -> Result<(), String> {
        println!("[VFS] Mounting '{}' driver at '{}'", fstype, target);
        self.mounts.push(MountPoint {
            mount_path: target.to_string(),
            fs_type: fstype.to_string(),
        });
        Ok(())
    }

    /// Creates a directory node at path.
    pub fn create_directory(&mut self, path: &str) -> Result<(), String> {
        let node = VNode {
            name: path.split('/').last().unwrap_or("").to_string(),
            path: path.to_string(),
            node_type: FileType::Directory,
            size_bytes: 0,
            content: Vec::new(),
        };
        self.files.insert(path.to_string(), node);
        Ok(())
    }

    /// Creates a device node at path.
    pub fn create_device_node(&mut self, path: &str, dev_type: FileType) -> Result<(), String> {
        let node = VNode {
            name: path.split('/').last().unwrap_or("").to_string(),
            path: path.to_string(),
            node_type: dev_type,
            size_bytes: 0,
            content: Vec::new(),
        };
        self.files.insert(path.to_string(), node);
        Ok(())
    }

    /// Writes content byte slice to a file path.
    pub fn write_file(&mut self, path: &str, content: &[u8]) -> Result<(), String> {
        let name = path.split('/').last().unwrap_or("file").to_string();
        let node = VNode {
            name,
            path: path.to_string(),
            node_type: FileType::RegularFile,
            size_bytes: content.len(),
            content: content.to_vec(),
        };
        self.files.insert(path.to_string(), node);
        Ok(())
    }

    /// Reads content bytes from a file path.
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        if let Some(node) = self.files.get(path) {
            Ok(node.content.clone())
        } else {
            Err(format!("File not found: {}", path))
        }
    }

    /// Lists active mounts.
    pub fn get_mounts(&self) -> &[MountPoint] {
        &self.mounts
    }

    /// Shuts down VFS and unmounts filesystem drivers.
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Ok(());
        }
        println!("[SHUTDOWN][VFS] Unmounting root filesystem and flushing dirty buffers...");
        self.files.clear();
        self.mounts.clear();
        self.initialized = false;
        Ok(())
    }

    /// Checks if VFS is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self::new()
    }
}
