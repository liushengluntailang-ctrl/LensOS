//! LensOS v0.1 - System Update Manager
//!
//! Manages Over-The-Air (OTA) operating system updates, signature verification,
//! staged system partition updates, release channel management, and rollback safety.

/// Update release channels for LensOS v0.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    /// Stable release builds
    Stable,
    /// Pre-release feature testing channel
    Beta,
    /// Cutting-edge daily developer builds
    Nightly,
}

impl Default for UpdateChannel {
    fn default() -> Self {
        UpdateChannel::Stable
    }
}

/// Operational state of system update engine
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateState {
    /// Idle, no update actively processing
    Idle,
    /// Contacting LensOS update servers for update manifests
    Checking,
    /// New version available
    Available { version: String, release_notes: String },
    /// Transferring signed OS payload
    Downloading { version: String, progress_percent: f32 },
    /// Payload verified and staged on passive system partition
    ReadyToInstall { version: String },
    /// System reboot queued to switch partition slots
    Installing,
    /// Update check or download failed
    Failed(String),
}

/// System Update Package metadata
#[derive(Debug, Clone)]
pub struct UpdatePackage {
    pub version: String,
    pub channel: UpdateChannel,
    pub download_size_bytes: u64,
    pub release_notes: String,
    pub sha256_signature: String,
}

/// System Update Manager
#[derive(Debug)]
pub struct SystemUpdateManager {
    current_version: String,
    active_channel: UpdateChannel,
    state: UpdateState,
    auto_check_enabled: bool,
}

impl Default for SystemUpdateManager {
    fn default() -> Self {
        Self {
            current_version: "0.1.0".to_string(),
            active_channel: UpdateChannel::Stable,
            state: UpdateState::Idle,
            auto_check_enabled: true,
        }
    }
}

impl SystemUpdateManager {
    /// Creates a new `SystemUpdateManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether automatic update checks are enabled
    pub fn is_auto_check_enabled(&self) -> bool {
        self.auto_check_enabled
    }

    /// Sets automatic update checks status
    pub fn set_auto_check_enabled(&mut self, enabled: bool) {
        self.auto_check_enabled = enabled;
    }
    pub fn current_version(&self) -> &str {
        &self.current_version
    }

    /// Returns active update release channel
    pub fn active_channel(&self) -> UpdateChannel {
        self.active_channel
    }

    /// Sets active release channel
    pub fn set_channel(&mut self, channel: UpdateChannel) {
        self.active_channel = channel;
    }

    /// Returns current update process state
    pub fn state(&self) -> &UpdateState {
        &self.state
    }

    /// Checks remote update server for newer LensOS builds
    pub fn check_for_updates(&mut self) -> Result<Option<UpdatePackage>, String> {
        self.state = UpdateState::Checking;

        // Mock update payload check logic for LensOS v0.1
        let mock_package = UpdatePackage {
            version: "0.1.1".to_string(),
            channel: self.active_channel,
            download_size_bytes: 148_576_000, // 148.5 MB
            release_notes: "LensOS v0.1.1 - Performance optimizations for Lens AI instant launcher, kernel memory efficiency improvements, and power bug fixes.".to_string(),
            sha256_signature: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        };

        self.state = UpdateState::Available {
            version: mock_package.version.clone(),
            release_notes: mock_package.release_notes.clone(),
        };

        Ok(Some(mock_package))
    }

    /// Simulates downloading the update payload
    pub fn download_update(&mut self, package: &UpdatePackage) -> Result<(), String> {
        self.state = UpdateState::Downloading {
            version: package.version.clone(),
            progress_percent: 100.0,
        };

        // Transition to ready to install
        self.state = UpdateState::ReadyToInstall {
            version: package.version.clone(),
        };
        Ok(())
    }

    /// Stages the update and triggers reboot installation
    pub fn apply_update(&mut self) -> Result<(), String> {
        let version_to_install = if let UpdateState::ReadyToInstall { version } = &self.state {
            Some(version.clone())
        } else {
            None
        };

        if let Some(version) = version_to_install {
            self.state = UpdateState::Installing;
            self.current_version = version;
            self.state = UpdateState::Idle;
            Ok(())
        } else {
            Err("No update ready to install".to_string())
        }
    }
}
