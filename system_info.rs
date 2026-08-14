use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub os_codename: String,
    pub kernel_version: String,
    pub architecture: String,
    pub device_model: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub ram_total_mb: u64,
    pub ram_used_mb: u64,
    pub disk_total_gb: u64,
    pub disk_used_gb: u64,
    pub uptime_seconds: u64,
    pub gpu_model: String,
    pub display_resolution: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            os_name: "LensOS".to_string(),
            os_version: "1.0.0".to_string(),
            os_codename: "Frosted Obsidian".to_string(),
            kernel_version: "LensKernel 6.12.0-lens-x86_64".to_string(),
            architecture: "x86_64".to_string(),
            device_model: "LensBook Pro Glass Edition".to_string(),
            cpu_model: "Lens Neural Core v2 @ 3.80GHz".to_string(),
            cpu_cores: 12,
            cpu_threads: 16,
            ram_total_mb: 32_768, // 32 GB
            ram_used_mb: 8_192,   // 8 GB
            disk_total_gb: 1024,  // 1 TB
            disk_used_gb: 248,    // 248 GB
            uptime_seconds: 142_850, // ~1 day 15 hrs
            gpu_model: "Lens GlassRender Accelerator 16GB".to_string(),
            display_resolution: "3840 x 2160 (4K Glass Retina)".to_string(),
        }
    }
}

impl SystemInfo {
    pub fn gather_system_info() -> Self {
        Self::default()
    }

    pub fn format_ram_usage(&self) -> String {
        let used_gb = self.ram_used_mb as f64 / 1024.0;
        let total_gb = self.ram_total_mb as f64 / 1024.0;
        let percentage = (self.ram_used_mb as f64 / self.ram_total_mb as f64) * 100.0;
        format!("{:.1} GB / {:.1} GB ({:.0}%)", used_gb, total_gb, percentage)
    }

    pub fn format_disk_usage(&self) -> String {
        let percentage = (self.disk_used_gb as f64 / self.disk_total_gb as f64) * 100.0;
        format!(
            "{} GB used of {} GB ({:.0}%)",
            self.disk_used_gb, self.disk_total_gb, percentage
        )
    }

    pub fn format_uptime(&self) -> String {
        let days = self.uptime_seconds / 86400;
        let hours = (self.uptime_seconds % 86400) / 3600;
        let minutes = (self.uptime_seconds % 3600) / 60;
        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else {
            format!("{}h {}m", hours, minutes)
        }
    }

    pub fn get_system_summary(&self) -> String {
        format!(
            "{} {} ({}) - Kernel: {} on {}",
            self.os_name, self.os_version, self.os_codename, self.kernel_version, self.architecture
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_formatting() {
        let info = SystemInfo::default();
        assert_eq!(info.format_ram_usage(), "8.0 GB / 32.0 GB (25%)");
        assert_eq!(info.format_disk_usage(), "248 GB used of 1024 GB (24%)");
        assert_eq!(info.format_uptime(), "1d 15h 40m");
        assert!(info.get_system_summary().contains("LensOS 1.0.0"));
    }
}
