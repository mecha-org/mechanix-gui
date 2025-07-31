use anyhow::Result;
use async_trait::async_trait;

/// Represents the current status of the Wireless connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WifiStatus {
    /// Successfully connected to a network, containing the SSID.
    Connected { ssid: String },
    /// Currently attempting to connect to a network containing the SSID.
    Connecting { ssid: String },
    /// Not connected to any network.
    Disconnected,
    /// Wireless hardware or software is disabled.
    Disabled,
    /// The status could not be determined.
    Unknown,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NMState {
    #[default]
    Unknown = 0,
    Asleep = 10,
    Disconnected = 20,
    Disconnecting = 30,
    Connecting = 40,
    ConnectedLocal = 50,
    ConnectedSite = 60,
    ConnectedGlobal = 70,
}

impl From<u32> for NMState {
    fn from(value: u32) -> Self {
        match value {
            0 => NMState::Unknown,
            10 => NMState::Asleep,
            20 => NMState::Disconnected,
            30 => NMState::Disconnecting,
            40 => NMState::Connecting,
            50 => NMState::ConnectedLocal,
            60 => NMState::ConnectedSite,
            70 => NMState::ConnectedGlobal,
            _ => NMState::Unknown,
        }
    }
}
// impl fmt::Display for NMState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             WifiState::Connecting => write!(f, "Connecting.."),
//             WifiState::Connected => write!(f, "Connected"),
//             WifiState::Disconnected => write!(f, "Disconnected"),
//             WifiState::Disconnecting => write!(f, "Disconnecting.."),
//             WifiState::Unknown => write!(f, ""),
//         }
//     }
// }

/// Raw information about a detected Wireless access point.
///
/// This struct holds low-level details as reported by the network hardware or driver.
#[derive(Debug, Clone, Default)]
pub struct RawAccessPointInfo {
    /// Flags indicating access point capabilities.
    pub flags: u32,
    /// WPA-specific flags.
    pub wpa_flags: u32,
    /// RSN (WPA2/3) specific flags.
    pub rsn_flags: u32,
    /// SSID of the access point as a sequence of bytes.
    pub ssid: String,
    /// Whether the access point is currently active.
    pub is_active: bool,
    /// Operating frequency (in MHz).
    pub frequency: u32,
    /// Hardware (MAC) address of the access point.
    pub hw_address: String,
    /// Mode of the access point (e.g., infrastructure, ad-hoc).
    pub mode: u32,
    /// Maximum supported bitrate (in kbps).
    pub max_bitrate: u32,
    /// Channel bandwidth (in MHz).
    pub bandwidth: u32,
    /// Signal strength (0-100).
    pub strength: u8,
    /// Time (in seconds) since the access point was last seen.
    pub last_seen: i64,
}

bitflags::bitflags! {
    /// Flags describing capabilities and features of a Wireless access point.
    #[derive(Default)]
    pub struct NM80211ApFlags: u32 {
        /// Access point has no special capabilities.
        const NONE      = 0x00000000;
        /// Access point requires authentication and encryption (usually means WEP).
        const PRIVACY   = 0x00000001;
        /// Access point supports some WPS method.
        const WPS       = 0x00000002;
        /// Access point supports push-button WPS.
        const WPS_PBC   = 0x00000004;
        /// Access point supports PIN-based WPS.
        const WPS_PIN   = 0x00000008;
    }
}

impl RawAccessPointInfo {
    /// Returns the parsed `NM80211ApFlags` for this access point.
    ///
    /// This method interprets the raw `flags` field and converts it to a strongly-typed bitflags struct.
    pub fn nm80211_flags(&self) -> NM80211ApFlags {
        NM80211ApFlags::from_bits_truncate(self.flags)
    }
}

/// High-level, user-friendly information about a Wireless network.
#[derive(Debug, Clone)]
pub struct WirelessNetworkInfo {
    /// SSID (network name) as a UTF-8 string.
    pub ssid: String,
    /// Signal strength (0-100).
    pub signal_strength: u8,
    /// Security type (e.g., "Open", "Protected").
    pub security: String,
    /// Hardware (MAC) address of the access point.
    pub hw_address: String,

    pub is_active: bool,
    // Additional fields can be added as needed.
}

/// Trait for objects that can interact with the system's network manager.
///
/// This trait is intended to be implemented by types that can list available Wireless networks and perform other network management tasks.
#[async_trait]
pub trait NetworkManagerInterface: Send + Sync {
    /// Lists all available Wireless networks.
    ///
    /// Returns a vector of `WirelessNetworkInfo` describing each visible network.
    async fn list_networks(&self) -> Result<Vec<WirelessNetworkInfo>>;
}

#[derive(Debug, Default)]
pub enum EventType {
    #[default]
    Added,
    Removed,
}
#[derive(Debug, Default)]
pub struct AccessPointEvent {
    pub event_type: EventType,
    pub access_point_path: String,
    pub raw_access_point_info: Option<RawAccessPointInfo>,
}
