use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};

#[derive(Debug, DeserializeDict, SerializeDict, Type, Clone, Default)]
// `Type` treats `WirelessInfoResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct WirelessInfoResponse {
    pub mac: String,
    pub frequency: String,
    pub signal: String,
    pub flags: String,
    pub name: String,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
// `Type` treats `WirelessScanResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct WirelessScanListResponse {
    pub wireless_network: Vec<WirelessInfoResponse>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone)]
/// A known WiFi network.
#[zvariant(signature = "a{sv}")]
pub struct KnownNetworkResponse {
    pub network_id: String,
    pub ssid: String,
    pub flags: String,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone)]
/// A known WiFi networkList
#[zvariant(signature = "a{sv}")]
pub struct KnownNetworkListResponse {
    pub known_network: Vec<KnownNetworkResponse>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, PartialEq)]
/// A wireless notification event.
#[zvariant(signature = "a{sv}")]
pub struct WirelessNotificationEvent {
    pub signal_strength: String,
    pub is_connected: bool,
    pub is_enabled: bool,
    pub frequency: String,
    pub ssid: String,
}
