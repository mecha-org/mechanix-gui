use std::collections::HashMap;

use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_system_dbus_client::wireless::{
    self, KnownNetworkListResponse, NotificationStream, WirelessInfoResponse,
    WirelessScanListResponse, WirelessService,
};
use tokio::runtime::Runtime;
use tokio::{select, signal};
use zbus::fdo::ConnectionCredentials;
use zbus::zvariant::Str;

mod access_point;
mod active_connection;
mod connection;
mod device;
mod network_manager;
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
                println!("{}", device);
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

    pub fn connect_to_network(ssid: String, password: String) {
        RUNTIME.spawn(async move {
            println!("Trying to connect to {ssid} with password {password}");
            let _ = WirelessService::connect_to_network(ssid.as_str(), password.as_str()).await;
            WirelessModel::update();
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
                    flags: "[WPA-PSK]".to_string(),
                });
                println!("{access_point}: {signal} {frequency} {mac}");
            }
            WirelessModel::get()
                .scan_result
                .set(WirelessScanListResponse {
                    wireless_network: scan_result,
                });
        });
    }

    fn stream_connection() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = network_manager::NetworkManagerProxy::new(&connection)
                .await
                .unwrap();
            let mut stream = proxy.receive_primary_connection_changed().await;
            while let Some(property) = stream.next().await {
                let mut connected_network = None;
                if let Ok(connection_path) = property.get().await {
                    let active_connection_proxy = active_connection::ActiveConnectionProxy::new(
                        &connection,
                        connection_path.clone(),
                    )
                    .await;
                    if active_connection_proxy.is_err() {
                        continue;
                    }
                    let active_connection_proxy = active_connection_proxy.unwrap();
                    let connection_path = active_connection_proxy.connection().await;
                    if connection_path.is_err() {
                        continue;
                    }
                    let connection_path = connection_path.unwrap();
                    let connection_proxy =
                        connection::ConnectionProxy::new(&connection, connection_path).await;
                    if connection_proxy.is_err() {
                        continue;
                    }
                    let connection_proxy = connection_proxy.unwrap();
                    let connection = connection_proxy.get_settings().await.unwrap();
                    let name = (*connection["connection"]["id"])
                        .downcast_ref::<Str>()
                        .unwrap();
                    connected_network = Some(WirelessInfoResponse {
                        name: name.to_string(),
                        frequency: "5000".to_string(),
                        mac: "00:00:00:00:00".to_string(),
                        signal: "-10".to_string(),
                        flags: "[WPA-PSK]".to_string(),
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
    }

    pub fn select_network(network_id: String) {
        println!("Trying to connect to {network_id}");
        RUNTIME.spawn(async move {
            WirelessService::connect_to_known_network(network_id.as_str())
                .await
                .unwrap();
            WirelessModel::update();
        });
    }
}
