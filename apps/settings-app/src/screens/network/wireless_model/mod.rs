use std::collections::HashMap;

use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_system_dbus_client::security;
use mechanix_system_dbus_client::wireless::{
    self, KnownNetworkListResponse, KnownNetworkResponse, NotificationStream, WirelessInfoResponse,
    WirelessScanListResponse, WirelessService,
};
use tokio::runtime::Runtime;
use tokio::{select, signal};
use uuid::Uuid;
use zbus::fdo::ConnectionCredentials;
use zbus::zvariant::{ObjectPath, Str, Value};

mod access_point;
mod active_connection;
mod connection;
mod device;
mod network_manager;
mod settings;
mod wireless_device;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref WIRELESS_MODEL: WirelessModel = WirelessModel {
        known_networks: Context::new(KnownNetworkListResponse {
            known_network: vec![]
        }),
        scan_result: Context::new(WirelessScanListResponse {
            wireless_network: vec![]
        }),
        connected_network: Context::new(None),
        is_enabled: Context::new(false),
        is_streaming: Context::new(false),
        state: Context::new(WifiState::Disconnected),
        wireless_mac_address: Context::new("".to_string()),
        ethernet_mac_address: Context::new("".to_string()),
    };
}

#[derive(Debug, PartialEq)]
pub enum WifiState {
    Connecting,
    Connected,
    Disconnected,
    Disconnecting,
    Unknown,
}

#[derive(Model)]
pub struct WirelessModel {
    pub known_networks: Context<KnownNetworkListResponse>,
    pub scan_result: Context<WirelessScanListResponse>,
    pub connected_network: Context<Option<WirelessInfoResponse>>,
    pub is_enabled: Context<bool>,
    pub is_streaming: Context<bool>,
    pub state: Context<WifiState>,
    pub wireless_mac_address: Context<String>,
    pub ethernet_mac_address: Context<String>,
}

impl WirelessModel {
    pub fn get() -> &'static Self {
        &WIRELESS_MODEL
    }

    pub fn toggle_wireless() {
        RUNTIME.spawn(async {
            let is_enabled = *WirelessModel::get().is_enabled.get();
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            proxy.set_wireless_enabled(!is_enabled).await.unwrap();
            WirelessModel::update();
        });
    }

    async fn get_wifi_device_path() -> String {
        let connection = zbus::Connection::system().await.unwrap();
        let proxy = network_manager::NetworkManagerProxy::new(&connection)
            .await
            .unwrap();
        let devices = proxy.get_all_devices().await.unwrap();
        for device in devices {
            let device_proxy = device::DeviceProxy::new(&connection, device.clone())
                .await
                .unwrap();
            if device_proxy.device_type().await.unwrap() == 2 {
                return device.to_string();
            }
        }
        "/org/freedesktop/NetworkManager/Devices/2".to_string()
    }

    async fn get_ethernet_device_path() -> String {
        let connection = zbus::Connection::system().await.unwrap();
        let proxy = network_manager::NetworkManagerProxy::new(&connection)
            .await
            .unwrap();
        let devices = proxy.get_all_devices().await.unwrap();
        for device in devices {
            let device_proxy = device::DeviceProxy::new(&connection, device.clone())
                .await
                .unwrap();
            if device_proxy.device_type().await.unwrap() == 1 {
                return device.to_string();
            }
        }
        "/org/freedesktop/NetworkManager/Devices/2".to_string()
    }

    pub fn scan() {
        RUNTIME.spawn(async {
            let conneciton = zbus::Connection::system().await.unwrap();
            let wireless_proxy = wireless_device::WirelessDeviceProxy::new(
                &conneciton,
                Self::get_wifi_device_path().await,
            )
            .await
            .unwrap();
            let options = HashMap::new();
            wireless_proxy.request_scan(options).await.unwrap();
        });
    }

    pub fn update() {
        RUNTIME.spawn(async {
            // let is_enabled = WirelessService::wireless_status().await.unwrap();
            // let connection = zbus::Connection::system().await.unwrap();
            // let proxy = network_manager::NetworkManagerProxy::new(&connection)
            //     .await
            //     .unwrap();
            // let is_enabled = proxy.wireless_enabled().await.unwrap();
            // WirelessModel::get().is_enabled.set(is_enabled);

            // let connected_network = WirelessService::info().await.unwrap();
            // println!("Connected network: {:?}", connected_network);
            // WirelessModel::get()
            //     .connected_network
            //     .set(Some(connected_network));
        });
    }

    pub fn update_mac_addresses() {
        RUNTIME.spawn(async {
            let device_path = Self::get_wifi_device_path().await;
            let device_proxy =
                device::DeviceProxy::new(&zbus::Connection::system().await.unwrap(), device_path)
                    .await
                    .unwrap();
            let wifi_mac_address = device_proxy.hw_address().await.unwrap();

            let device_path = Self::get_ethernet_device_path().await;
            let device_proxy =
                device::DeviceProxy::new(&zbus::Connection::system().await.unwrap(), device_path)
                    .await
                    .unwrap();
            let ethernet_mac_address = device_proxy.hw_address().await.unwrap();

            WirelessModel::get()
                .wireless_mac_address
                .set(wifi_mac_address.to_string());
            WirelessModel::get()
                .ethernet_mac_address
                .set(ethernet_mac_address.to_string());
        });
    }

    pub fn connect_to_saved_network(ssid: String) {
        RUNTIME.spawn(async move {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = settings::SettingsProxy::new(&connection).await.unwrap();
            let nm_proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            let connections = proxy.list_connections().await.unwrap();
            for c in connections {
                let connection_proxy = connection::ConnectionProxy::new(&connection, c.clone())
                    .await
                    .unwrap();
                let settings = connection_proxy.get_settings().await.unwrap();
                let access_point = (*settings["connection"]["id"])
                    .downcast_ref::<Str>()
                    .unwrap()
                    .to_string();

                let device = ObjectPath::try_from(Self::get_wifi_device_path().await).unwrap();
                let specific_object = ObjectPath::try_from("/").unwrap();
                if access_point == ssid {
                    nm_proxy
                        .activate_connection(&c, &device, &specific_object)
                        .await
                        .unwrap();
                    break;
                }
            }
        });
    }

    pub fn forget_saved_network(ssid: String) {
        RUNTIME.spawn(async move {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = settings::SettingsProxy::new(&connection).await.unwrap();
            let nm_proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            let connections = proxy.list_connections().await.unwrap();
            for c in connections {
                let connection_proxy = connection::ConnectionProxy::new(&connection, c.clone())
                    .await
                    .unwrap();
                let settings = connection_proxy.get_settings().await.unwrap();
                let access_point = (*settings["connection"]["id"])
                    .downcast_ref::<Str>()
                    .unwrap()
                    .to_string();

                let device = ObjectPath::try_from(Self::get_wifi_device_path().await).unwrap();
                let specific_object = ObjectPath::try_from("/").unwrap();
                if access_point == ssid {
                    connection_proxy.delete().await;
                    break;
                }
            }
        });
    }

    pub fn disconnect() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let device_proxy =
                device::DeviceProxy::new(&connection, Self::get_wifi_device_path().await)
                    .await
                    .unwrap();
            if device_proxy.disconnect().await.is_ok() {
                println!("Disconnected from wifi");
            }
        });
    }

    pub fn connect_to_network(ssid: String, password: String) {
        RUNTIME.spawn(async move {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();

            let device = ObjectPath::try_from(Self::get_wifi_device_path().await).unwrap();
            let specific_object = ObjectPath::try_from("/").unwrap();
            let mut connection = HashMap::new();

            let mut connection_connection = HashMap::new();
            let binding = Value::from(ssid.clone());
            connection_connection.insert("id", &binding);
            let binding = Value::from("802-11-wireless");
            connection_connection.insert("type", &binding);
            let binding = Value::from(Uuid::new_v4().to_string());
            connection_connection.insert("uuid", &binding);
            connection.insert("connection", connection_connection);

            let mut connection_wireless = HashMap::new();
            let binding = Value::from(ssid.clone().as_bytes().to_vec());
            connection_wireless.insert("ssid", &binding);
            let binding = Value::from("infrastructure");
            connection_wireless.insert("mode", &binding);
            connection.insert("802-11-wireless", connection_wireless);

            let mut connection_wireless_security = HashMap::new();
            let binding = Value::from("wpa-psk");
            connection_wireless_security.insert("key-mgmt", &binding);
            let binding = Value::from(password.clone());
            connection_wireless_security.insert("psk", &binding);
            connection.insert("802-11-wireless-security", connection_wireless_security);

            let mut connection_ipv4 = HashMap::new();
            let binding = Value::from("auto");
            connection_ipv4.insert("method", &binding);
            connection.insert("ipv4", connection_ipv4);

            let mut connection_ipv6 = HashMap::new();
            let binding = Value::from("ignore");
            connection_ipv6.insert("method", &binding);
            connection.insert("ipv6", connection_ipv6);

            proxy
                .add_and_activate_connection(connection, &device, &specific_object)
                .await
                .unwrap();
        });
    }

    fn stream_known_networks() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = settings::SettingsProxy::new(&connection).await.unwrap();
            let mut stream = proxy.receive_connections_changed().await;
            while stream.next().await.is_some() {
                let mut known_networks: Vec<KnownNetworkResponse> = vec![];
                let connections = proxy.list_connections().await.unwrap();
                for c in connections {
                    let connection_proxy = connection::ConnectionProxy::new(&connection, c)
                        .await
                        .unwrap();
                    let settings = connection_proxy.get_settings().await.unwrap();
                    if !settings.contains_key("802-11-wireless") {
                        continue;
                    }
                    let access_point = (*settings["connection"]["id"])
                        .downcast_ref::<Str>()
                        .unwrap()
                        .to_string();
                    println!("Access point: {} {:?}", access_point, settings.keys());
                    let security_flags = if !settings.contains_key("802-11-wireless-security") {
                        "Open".to_string()
                    } else {
                        "WPA-PSK".to_string()
                    };
                    let mut flag = false;
                    for network in known_networks.iter() {
                        if network.ssid == access_point {
                            flag = true;
                            break;
                        }
                    }
                    if flag {
                        continue;
                    }
                    known_networks.push(KnownNetworkResponse {
                        ssid: access_point,
                        network_id: "".to_string(),
                        flags: security_flags,
                    })
                }
                Self::get().known_networks.set(KnownNetworkListResponse {
                    known_network: known_networks,
                });
            }
        });
    }

    fn stream_scan_result() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let wireless_proxy = wireless_device::WirelessDeviceProxy::new(
                &connection,
                Self::get_wifi_device_path().await,
            )
            .await
            .unwrap();
            let mut stream = wireless_proxy.receive_access_points_changed().await;
            while stream.next().await.is_some() {
                let access_points = wireless_proxy.get_all_access_points().await.unwrap();
                let mut scan_result: Vec<WirelessInfoResponse> = vec![];
                for access_point in access_points {
                    let access_point_proxy =
                        access_point::AccessPointProxy::new(&connection, access_point).await;
                    if access_point_proxy.is_err() {
                        continue;
                    }
                    let access_point_proxy = access_point_proxy.unwrap();
                    let access_point = access_point_proxy.ssid().await.unwrap();
                    let access_point = String::from_utf8(access_point).unwrap();
                    let signal = access_point_proxy.strength().await.unwrap();
                    let frequency = access_point_proxy.frequency().await.unwrap();
                    let mac = access_point_proxy.hw_address().await.unwrap();

                    let flags = if access_point_proxy.rsn_flags().await.unwrap() == 0 {
                        "Open"
                    } else {
                        "WPA-PSK"
                    };
                    let mut flag = false;
                    for network in scan_result.iter() {
                        if network.name == access_point {
                            flag = true;
                            break;
                        }
                    }
                    if flag {
                        continue;
                    }
                    scan_result.push(WirelessInfoResponse {
                        name: access_point.clone(),
                        frequency: frequency.to_string(),
                        mac: mac.clone(),
                        signal: signal.to_string(),
                        flags: flags.to_string(),
                    });
                }
                WirelessModel::get()
                    .scan_result
                    .set(WirelessScanListResponse {
                        wireless_network: scan_result,
                    });
            }
        });
    }

    fn stream_connection() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = wireless_device::WirelessDeviceProxy::new(
                &connection,
                Self::get_wifi_device_path().await,
            )
            .await
            .unwrap();
            let mut stream = proxy.receive_active_access_point_changed().await;
            while let Some(access_point) = stream.next().await {
                let mut connected_network = None;
                if let Ok(access_point) = access_point.get().await {
                    if access_point.to_string() == *"/" {
                        continue;
                    }
                    let access_point_proxy =
                        access_point::AccessPointProxy::new(&connection, access_point).await;
                    if access_point_proxy.is_err() {
                        continue;
                    }
                    let access_point_proxy = access_point_proxy.unwrap();
                    let access_point = access_point_proxy.ssid().await.unwrap();
                    let access_point = String::from_utf8(access_point).unwrap();
                    let signal = access_point_proxy.strength().await.unwrap();
                    let frequency = access_point_proxy.frequency().await.unwrap();
                    let mac = access_point_proxy.hw_address().await.unwrap();
                    let flags = if access_point_proxy.rsn_flags().await.unwrap() == 0 {
                        "Open"
                    } else {
                        "WPA-PSK"
                    };
                    connected_network = Some(WirelessInfoResponse {
                        name: access_point.to_string(),
                        frequency: frequency.to_string(),
                        mac: mac.to_string(),
                        signal: signal.to_string(),
                        flags: flags.to_string(),
                    })
                }
                WirelessModel::get()
                    .connected_network
                    .set(connected_network);
            }
        });
    }

    fn stream_wireless_enabled_status() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            let mut stream = proxy.receive_wireless_enabled_changed().await;
            while let Some(property) = stream.next().await {
                if let Ok(is_enabled) = property.get().await {
                    WirelessModel::get().is_enabled.set(is_enabled);
                }
            }
        });
    }

    fn stream_wifi_status() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            let mut stream = proxy.receive_state_changed().await;
            while let Some(state_changed) = stream.next().await {
                if let Ok(state) = state_changed.get().await {
                    let state = match state {
                        20 => WifiState::Disconnected,
                        30 => WifiState::Disconnecting,
                        40 => WifiState::Connecting,
                        (50..=70) => WifiState::Connected,
                        _ => WifiState::Unknown,
                    };
                    if state != WifiState::Connected {
                        WirelessModel::get().connected_network.set(None);
                    }
                    WirelessModel::get().state.set(state);
                }
            }
        });
    }

    pub fn start_streaming() {
        if *WirelessModel::get().is_streaming.get() {
            return;
        }
        WirelessModel::get().is_streaming.set(true);
        Self::stream_wireless_enabled_status();
        Self::stream_connection();
        Self::stream_wifi_status();
        Self::stream_scan_result();
        Self::stream_known_networks();
    }

    pub fn select_network(network_id: String) {
        RUNTIME.spawn(async move {
            WirelessService::connect_to_known_network(network_id.as_str())
                .await
                .unwrap();
            WirelessModel::update();
        });
    }
}
