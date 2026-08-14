use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseChannel {
    Stable,
    Beta,
    Developer,
    Nightly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpdateAvailable,
    Downloading { progress_percent: u8 },
    ReadyToInstall,
    Installing { stage: String, progress_percent: u8 },
    RequiresRestart,
    UpToDate,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemUpdateInfo {
    pub version: String,
    pub build_number: String,
    pub release_notes: String,
    pub size_bytes: u64,
    pub is_security_patch: bool,
    pub published_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateSettings {
    pub auto_check_updates: bool,
    pub auto_download: bool,
    pub update_channel: ReleaseChannel,
    pub last_checked_timestamp: u64,
    pub current_version: String,
    pub available_update: Option<SystemUpdateInfo>,
    pub status: UpdateStatus,
}

impl Default for UpdateSettings {
    fn default() -> Self {
        Self {
            auto_check_updates: true,
            auto_download: true,
            update_channel: ReleaseChannel::Stable,
            last_checked_timestamp: 1700020000,
            current_version: "1.0.0-Frosted".to_string(),
            available_update: None,
            status: UpdateStatus::UpToDate,
        }
    }
}

impl UpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_channel(&mut self, channel: ReleaseChannel) {
        self.update_channel = channel;
    }

    pub fn check_for_updates(&mut self) -> UpdateStatus {
        self.status = UpdateStatus::Checking;
        self.last_checked_timestamp = 1700025000;

        // Simulate update availability based on channel
        match self.update_channel {
            ReleaseChannel::Stable => {
                self.status = UpdateStatus::UpToDate;
                self.available_update = None;
            }
            ReleaseChannel::Beta | ReleaseChannel::Developer | ReleaseChannel::Nightly => {
                let update = SystemUpdateInfo {
                    version: "1.1.0-Beta1".to_string(),
                    build_number: "lens-2026-08".to_string(),
                    release_notes: "Enhanced frosted glass blur algorithms, kernel IPC low latency, Gemini 2.5 Flash optimizations.".to_string(),
                    size_bytes: 482_149_888, // ~460 MB
                    is_security_patch: true,
                    published_date: "2026-08-10".to_string(),
                };
                self.available_update = Some(update);
                self.status = UpdateStatus::UpdateAvailable;
            }
        }
        self.status.clone()
    }

    pub fn start_download(&mut self) -> Result<(), String> {
        if self.available_update.is_none() {
            return Err("No update available to download".to_string());
        }
        self.status = UpdateStatus::Downloading { progress_percent: 0 };
        Ok(())
    }

    pub fn update_download_progress(&mut self, percent: u8) {
        if percent >= 100 {
            self.status = UpdateStatus::ReadyToInstall;
        } else {
            self.status = UpdateStatus::Downloading { progress_percent: percent };
        }
    }

    pub fn install_update(&mut self) -> Result<(), String> {
        match self.status {
            UpdateStatus::ReadyToInstall => {
                self.status = UpdateStatus::Installing {
                    stage: "Applying kernel patch".to_string(),
                    progress_percent: 50,
                };
                Ok(())
            }
            _ => Err("Update is not ready to install".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_defaults() {
        let upd = UpdateSettings::default();
        assert_eq!(upd.current_version, "1.0.0-Frosted");
        assert_eq!(upd.update_channel, ReleaseChannel::Stable);
    }

    #[test]
    fn test_beta_channel_check() {
        let mut upd = UpdateSettings::default();
        upd.set_channel(ReleaseChannel::Beta);
        let status = upd.check_for_updates();
        assert_eq!(status, UpdateStatus::UpdateAvailable);
        assert!(upd.available_update.is_some());
    }
}
