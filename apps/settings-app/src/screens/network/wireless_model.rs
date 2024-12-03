use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_core::reexports::smithay_client_toolkit::reexports::client::Connection;
use mctk_macros::Model;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, NotificationStream, WirelessInfoResponse, WirelessService,
};
use tokio::runtime::Runtime;
use tokio::select;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref WIRELESS_MODEL: WirelessModel = WirelessModel {
        known_networks: Context::new(KnownNetworkListResponse {
            known_network: vec![]
        }),
        available_networks: Context::new(vec![]),
        connected_network: Context::new(None),
        is_enabled: Context::new(false),
    };
}

#[derive(Model)]
pub struct WirelessModel {
    pub known_networks: Context<KnownNetworkListResponse>,
    pub available_networks: Context<Vec<String>>,
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
