use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, NotificationStream, WirelessInfoResponse, WirelessScanListResponse,
    WirelessService,
};
use tokio::runtime::Runtime;
use tokio::select;

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
    };
}

#[derive(Model)]
pub struct WirelessModel {
    pub known_networks: Context<KnownNetworkListResponse>,
    pub scan_result: Context<WirelessScanListResponse>,
    pub connected_network: Context<Option<WirelessInfoResponse>>,
    pub is_enabled: Context<bool>,
    pub is_streaming: Context<bool>,
}

impl WirelessModel {
    pub fn get() -> &'static Self {
        &WIRELESS_MODEL
    }

    pub fn toggle_wireless() {
        RUNTIME.spawn(async {
            let is_enabled = *WirelessModel::get().is_enabled.get();
            if is_enabled {
                WirelessService::disable_wireless().await.unwrap();
            } else {
                WirelessService::enable_wireless().await.unwrap();
            }
            WirelessModel::update();
        });
    }

    pub fn scan() {
        RUNTIME.spawn(async {
            let scan_result = WirelessService::scan().await.unwrap();
            WirelessModel::get().scan_result.set(scan_result);

            let known_networks = WirelessService::known_networks().await.unwrap();
            WirelessModel::get().known_networks.set(known_networks);
        });
    }

    pub fn update() {
        RUNTIME.spawn(async {
            let is_enabled = WirelessService::wireless_status().await.unwrap();
            WirelessModel::get().is_enabled.set(is_enabled);

            let connected_network = WirelessService::info().await.unwrap();
            println!("Connected network: {:?}", connected_network);
            WirelessModel::get()
                .connected_network
                .set(Some(connected_network));
        });
    }

    pub fn connect_to_network(ssid: String, password: String) {
        RUNTIME.spawn(async move {
            println!("Trying to connect to {ssid} with password {password}");
            WirelessService::connect_to_network(ssid.as_str(), password.as_str()).await;
            WirelessModel::update();
        });
    }

    pub fn start_streaming() {
        if *WirelessModel::get().is_streaming.get() {
            return;
        }
        WirelessModel::get().is_streaming.set(true);
        RUNTIME.spawn(async {
            let mut stream: NotificationStream =
                WirelessService::get_notification_stream().await.unwrap();
            loop {
                println!("Wireless stream started");
                select! {
                signal = stream.next() => {
                    if signal.is_none() {
                        continue;
                    }
                    if let Ok(args) = signal.unwrap().args() {
                        let event = args.event;
                        WirelessModel::get().is_enabled.set(event.is_enabled);
                        if event.is_connected {
                            WirelessModel::update();
                        } else {
                            WirelessModel::get().connected_network.set(None);
                        }
                    }
                }
                }
            }
        });
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
