use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength_percent: u8,
    pub is_secured: bool,
    pub is_saved: bool,
    pub is_connected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VpnProtocol {
    WireGuard,
    OpenVPN,
    LensShieldProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VpnConfig {
    pub id: String,
    pub name: String,
    pub server_address: String,
    pub protocol: VpnProtocol,
    pub auto_connect: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkSettings {
    pub wifi_enabled: bool,
    pub connected_wifi: Option<WifiNetwork>,
    pub available_networks: Vec<WifiNetwork>,
    pub ethernet_connected: bool,
    pub ethernet_interface_name: String,
    pub ip_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
    pub primary_dns: String,
    pub secondary_dns: String,
    pub vpn_configurations: Vec<VpnConfig>,
    pub active_vpn_id: Option<String>,
    pub hotspot_enabled: bool,
    pub hotspot_ssid: String,
    pub hotspot_password: String,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        let sample_wifi = WifiNetwork {
            ssid: "LensOS-Fiber-5G".to_string(),
            bssid: "00:11:22:33:44:55".to_string(),
            signal_strength_percent: 92,
            is_secured: true,
            is_saved: true,
            is_connected: true,
        };

        let other_wifi = WifiNetwork {
            ssid: "GlassCoffee_Guest".to_string(),
            bssid: "66:77:88:99:AA:BB".to_string(),
            signal_strength_percent: 65,
            is_secured: false,
            is_saved: false,
            is_connected: false,
        };

        let sample_vpn = VpnConfig {
            id: "vpn_wireguard_01".to_string(),
            name: "LensShield Secure Mesh".to_string(),
            server_address: "mesh.lensos.org:51820".to_string(),
            protocol: VpnProtocol::WireGuard,
            auto_connect: true,
            is_active: false,
        };

        Self {
            wifi_enabled: true,
            connected_wifi: Some(sample_wifi.clone()),
            available_networks: vec![sample_wifi, other_wifi],
            ethernet_connected: false,
            ethernet_interface_name: "eth0".to_string(),
            ip_address: Some("192.168.1.142".to_string()),
            subnet_mask: Some("255.255.255.0".to_string()),
            gateway: Some("192.168.1.1".to_string()),
            primary_dns: "1.1.1.1".to_string(), // Cloudflare DNS
            secondary_dns: "8.8.8.8".to_string(), // Google DNS
            vpn_configurations: vec![sample_vpn],
            active_vpn_id: None,
            hotspot_enabled: false,
            hotspot_ssid: "LensOS-Hotspot".to_string(),
            hotspot_password: "lens-glass-pass".to_string(),
        }
    }
}

impl NetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_wifi(&mut self, enabled: bool) {
        self.wifi_enabled = enabled;
        if !enabled {
            if let Some(ref mut current) = self.connected_wifi {
                current.is_connected = false;
            }
            self.connected_wifi = None;
        }
    }

    pub fn connect_wifi(&mut self, ssid: &str) -> Result<(), String> {
        if !self.wifi_enabled {
            return Err("Wi-Fi is currently disabled".to_string());
        }
        if let Some(net) = self.available_networks.iter_mut().find(|n| n.ssid == ssid) {
            net.is_connected = true;
            net.is_saved = true;
            self.connected_wifi = Some(net.clone());
            Ok(())
        } else {
            Err(format!("Wi-Fi network '{}' not found", ssid))
        }
    }

    pub fn set_dns(&mut self, primary: String, secondary: String) {
        self.primary_dns = primary;
        self.secondary_dns = secondary;
    }

    pub fn toggle_vpn(&mut self, vpn_id: &str) -> Result<bool, String> {
        if let Some(vpn) = self.vpn_configurations.iter_mut().find(|v| v.id == vpn_id) {
            vpn.is_active = !vpn.is_active;
            if vpn.is_active {
                self.active_vpn_id = Some(vpn_id.to_string());
            } else if self.active_vpn_id.as_deref() == Some(vpn_id) {
                self.active_vpn_id = None;
            }
            Ok(vpn.is_active)
        } else {
            Err(format!("VPN config '{}' not found", vpn_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_defaults() {
        let net = NetworkSettings::default();
        assert!(net.wifi_enabled);
        assert_eq!(net.primary_dns, "1.1.1.1");
        assert!(net.connected_wifi.is_some());
    }

    #[test]
    fn test_wifi_toggle() {
        let mut net = NetworkSettings::default();
        net.toggle_wifi(false);
        assert!(!net.wifi_enabled);
        assert!(net.connected_wifi.is_none());
    }

    #[test]
    fn test_vpn_toggle() {
        let mut net = NetworkSettings::default();
        assert!(net.toggle_vpn("vpn_wireguard_01").unwrap());
        assert_eq!(net.active_vpn_id.as_deref(), Some("vpn_wireguard_01"));
    }
}
