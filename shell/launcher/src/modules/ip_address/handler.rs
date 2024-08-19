use std::time::Duration;

use crate::AppMessage;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;

pub struct IpAddressHandle {
    app_channel: Sender<AppMessage>,
}

impl IpAddressHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        IpAddressHandle { app_channel }
    }

    pub async fn run(&self) {
        loop {
            let mut ip_address: Option<String> = None;
            let mut networks = sysinfo::Networks::new();
            networks.refresh_list();
            for (interface_name, network) in &networks {
                match interface_name.as_str() {
                    "enp5s0" => {
                        let ip_networks = network.ip_networks();
                        for ip_network in ip_networks {
                            if ip_network.addr.is_ipv4() {
                                ip_address = Some(ip_network.addr.to_string());
                                break;
                            }
                        }
                    }
                    "wlan0" => {
                        let ip_networks = network.ip_networks();
                        for ip_network in ip_networks {
                            if ip_network.addr.is_ipv4() {
                                ip_address = Some(ip_network.addr.to_string());
                                break;
                            }
                        }
                    }
                    _ => (),
                }
            }

            if let Some(address) = ip_address {
                let _ = &self.app_channel.send(AppMessage::IpAddress { address });
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
