use tokio::time::{self, Duration};
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};

use mechanix_network_ctl::wireless::WirelessNetworkControl;

#[derive(Clone)]
pub struct WirelessBusInterface {
    pub path: String,
}

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

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone)]
/// A wireless notification event.
#[zvariant(signature = "a{sv}")]
pub struct WirelessNotificationEvent {
    pub signal_strength: String,
    pub is_connected: bool,
    pub is_enabled: bool,
    pub frequency: String,
    pub ssid: String,
}

#[interface(name = "org.mechanix.services.Wireless")]
impl WirelessBusInterface {
    pub async fn status(&self) -> Result<bool, ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let result = wireless.status().await;

        Ok(result)
    }

    pub async fn connect(&self, ssid: &str, password: &str) -> Result<(), ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let wifi_result = match wireless.connect(ssid, password).await {
            Ok(result) => result,
            Err(_) => return Err(ZbusError::Failed("Failed to connect Wifi".to_string())),
        };
        Ok(wifi_result)
    }

    pub async fn disconnect(&self, network_id: &str) -> Result<(), ZbusError> {
        let network_id = match network_id.parse::<usize>() {
            Ok(result) => result,
            Err(_) => return Err(ZbusError::Failed("Failed to parse network_id".to_string())),
        };
        let wifi_result =
            match WirelessNetworkControl::remove_wireless_network(&self.path, network_id).await {
                Ok(result) => result,
                Err(_) => return Err(ZbusError::Failed("Failed to disconnect Wifi".to_string())),
            };

        Ok(wifi_result)
    }

    pub async fn select_network(&self, network_id: &str) -> Result<(), ZbusError> {
        let network_id = match network_id.parse::<usize>() {
            Ok(result) => result,
            Err(_) => return Err(ZbusError::Failed("Failed to parse network_id".to_string())),
        };
        let wifi_result =
            match WirelessNetworkControl::select_wireless_network(&self.path, network_id).await {
                Ok(result) => result,
                Err(_) => return Err(ZbusError::Failed("Failed to select network".to_string())),
            };

        Ok(wifi_result)
    }

    pub async fn info(&self) -> Result<WirelessInfoResponse, ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let wifi_result = match wireless.info().await {
            Ok(result) => {
                let wifi_info = WirelessInfoResponse {
                    mac: result.mac,
                    frequency: result.frequency,
                    signal: result.signal.to_string(),
                    flags: result.flags,
                    name: result.name,
                };
                wifi_info
            }
            Err(_) => return Err(ZbusError::Failed("Failed to get Wifi info".to_string())),
        };

        Ok(wifi_result)
    }

    #[zbus(signal)]
    async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: WirelessNotificationEvent,
    ) -> Result<(), zbus::Error>;

    pub async fn scan(&self) -> Result<WirelessScanListResponse, ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let wifi_result = match wireless.scan().await {
            Ok(result) => {
                let mut wifi_scan_list = Vec::new();
                for wifi in result {
                    let wifi_info = WirelessInfoResponse {
                        mac: wifi.mac,
                        frequency: wifi.frequency,
                        signal: wifi.signal.to_string(),
                        flags: wifi.flags,
                        name: wifi.name,
                    };
                    wifi_scan_list.push(wifi_info);
                }
                let wifi_scan = WirelessScanListResponse {
                    wireless_network: wifi_scan_list,
                };
                wifi_scan
            }
            Err(_) => return Err(ZbusError::Failed("Failed to scan Wifi".to_string())),
        };

        Ok(wifi_result)
    }

    pub async fn known_networks(&self) -> Result<KnownNetworkListResponse, ZbusError> {
        let wifi_result = match WirelessNetworkControl::known_network(&self.path).await {
            Ok(result) => {
                let mut known_network_list = Vec::new();
                for known_network in result {
                    let known_network_info = KnownNetworkResponse {
                        network_id: known_network.network_id.to_string(),
                        ssid: known_network.ssid,
                        flags: known_network.flags,
                    };
                    known_network_list.push(known_network_info);
                }
                let known_network = KnownNetworkListResponse {
                    known_network: known_network_list,
                };
                known_network
            }
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get known networks".to_string(),
                ))
            }
        };

        Ok(wifi_result)
    }

    pub async fn enable(&self) -> Result<bool, ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let wifi_result = match wireless.enable().await {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                return Err(ZbusError::Failed("Failed to enable Wifi".to_string()));
            }
        };

        Ok(wifi_result)
    }

    pub async fn disable(&self) -> Result<bool, ZbusError> {
        let wireless = WirelessNetworkControl::new(self.path.as_str());
        let wifi_result = match wireless.disable().await {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err);
                return Err(ZbusError::Failed("Failed to enable Wifi".to_string()));
            }
        };

        Ok(wifi_result)
    }
}

#[allow(dead_code, unused)]
pub async fn wireless_event_notification_stream(
    wireless_path: String,
    wireless_bus: &WirelessBusInterface,
    conn: &zbus::Connection,
) -> Result<(), ZbusError> {
    let mut interval = time::interval(Duration::from_secs(15));
    let mut previous_is_enabled: Option<bool> = None;
    let mut previous_is_connected: Option<bool> = None;
    let mut previous_signal_strength: Option<String> = None;
    let mut previous_frequency: Option<String> = None;
    let mut previous_ssid: Option<String> = None;

    loop {
        interval.tick().await;
        let wireless = WirelessNetworkControl::new(wireless_path.as_str());

        // Check if WiFi is enabled
        let is_enabled = wireless.status().await;

        let is_connected;
        let signal_strength;
        let ssid;
        let frequency;

        // If WiFi is enabled, check if it's connected and get signal strength, SSID, and frequency
        if is_enabled {
            let wifi_info = wireless.info().await;

            if let Ok(info) = wifi_info {
                is_connected = true;
                signal_strength = info.signal.clone().to_string();
                ssid = info.name.clone(); // Assuming `ssid` is a field in the info struct
                frequency = info.frequency.clone(); // Assuming `frequency` is a field in the info struct
            } else {
                is_connected = false;
                signal_strength = "".to_string();
                ssid = "".to_string();
                frequency = "".to_string();
            }
        } else {
            is_connected = false;
            signal_strength = "".to_string();
            ssid = "".to_string();
            frequency = "".to_string();
        }

        // Trigger notification if any value has changed
        if previous_is_enabled != Some(is_enabled)
            || previous_is_connected != Some(is_connected)
            || previous_signal_strength != Some(signal_strength.clone())
            || previous_ssid != Some(ssid.clone())
            || previous_frequency != Some(frequency.clone())
        {
            let ctxt = SignalContext::new(conn, "/org/mechanix/services/Wireless")?;
            wireless_bus
                .notification(
                    &ctxt,
                    WirelessNotificationEvent {
                        signal_strength: signal_strength.clone(),
                        is_connected,
                        is_enabled,
                        // Add ssid and frequency to the event
                        ssid: ssid.clone(),
                        frequency: frequency.clone(),
                    },
                )
                .await?;

            // Update previous values
            previous_is_enabled = Some(is_enabled);
            previous_is_connected = Some(is_connected);
            previous_signal_strength = Some(signal_strength);
            previous_ssid = Some(ssid);
            previous_frequency = Some(frequency);
        }
    }
}
