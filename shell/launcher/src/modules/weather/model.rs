use chrono::{Local, Timelike};
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use networkmanager::network_manager::NetworkManagerProxy;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::{interval_at, Instant};

use crate::utils::get_ip_address;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref IpAddress: IpAddressModel = IpAddressModel {
        ip_address: Context::new(None),
        is_streaming: Context::new(false)
    };
}

#[derive(Model)]
pub struct IpAddressModel {
    pub ip_address: Context<Option<String>>,
    is_streaming: Context<bool>,
}

impl IpAddressModel {
    pub fn get() -> &'static Self {
        &IpAddress
    }

    pub fn on_wireless_change() {
        RUNTIME.spawn(async {
            let ip_address = get_ip_address().await;
            println!("IpAddressModel::on_wireless_change() {:?}", ip_address);
            Self::get().ip_address.set(ip_address);
        });
    }

    fn stream_ip_address() {
        RUNTIME.spawn(async {
            let connection = zbus::Connection::system().await.unwrap();
            let proxy = SettingsProxy::new(&connection).await.unwrap();
            let nm_proxy = NetworkManagerProxy::new(&connection).await.unwrap();
        });
    }

    pub fn start_streaming() {
        if *IpAddressModel::get().is_streaming.get() {
            return;
        }

        IpAddressModel::get().is_streaming.set(true);
        // Self::stream_ip_address();
    }
}
