use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, WirelessInfoResponse, WirelessScanListResponse, WirelessService,
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
    };
}

#[derive(Model)]
pub struct WirelessModel {
    pub known_networks: Context<KnownNetworkListResponse>,
    pub scan_result: Context<WirelessScanListResponse>,
    pub connected_network: Context<Option<WirelessInfoResponse>>,
    pub is_enabled: Context<bool>,
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
}
