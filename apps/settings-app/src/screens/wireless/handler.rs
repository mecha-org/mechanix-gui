use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mechanix_system_dbus_client::wireless::{WirelessInfoResponse, WirelessService};
use regex::Regex;
use tokio::{select, sync::mpsc::Receiver};

use crate::{
    // types::{WirelessConnectedState, WirelessStatus},
    AppMessage,
    WirelessMessage,
};

#[derive(Debug, Clone)]
pub struct WirelessInfoItem {
    pub scan_info: WirelessInfoResponse,
    pub security: String,
    pub encryption: String,
    // pub is_secured: bool,
}

pub struct WirelessServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl WirelessServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut wireless_msg_rx: Receiver<WirelessMessage>) {
        let task = "WirelessServiceHandle::run()";
        match WirelessService::wireless_status().await {
            Ok(wireless_status) => {
                let _ = self.app_channel.send(AppMessage::NetworkStatus {
                    status: wireless_status,
                });
            }
            Err(e) => {
                println!("error while getting wireless status {}", e);
                let _ = self
                    .app_channel
                    .send(AppMessage::NetworkStatus { status: false });
            }
        };

        let connected_network_info: Option<WirelessInfoResponse> =
            match WirelessService::info().await {
                Ok(response) => {
                    // modified_wireless_network.find(|&p| p.name == name_to_find);

                    let _ = self.app_channel.send(AppMessage::ConnectedNetwork {
                        info: response.clone(),
                    });
                    Some(response)
                }
                Err(e) => {
                    println!("error while getting wireless info {}", e);
                    let _ = self
                        .app_channel
                        .send(AppMessage::NetworkStatus { status: false });
                    None
                }
            };

        match WirelessService::scan().await {
            Ok(response) => {
                let security_protocols = vec!["WPA-PSK", "WPA2-PSK", "WPA3-PSK"];

                let security_protocol_regex = Regex::new(r"\[(WPA[2-]?-PSK)\-").unwrap();
                let encryption_regex = Regex::new(r"\-(CCMP|TKIP)\]").unwrap();

                let modified_wireless_network: Vec<WirelessInfoItem> = response
                    .wireless_network
                    .into_iter()
                    .map(|item| {
                        let mut new_item: WirelessInfoItem = WirelessInfoItem {
                            scan_info: item.clone(),
                            security: "".to_string(),
                            encryption: "".to_string(),
                            // is_secured: false,
                        };
                        // Match the security protocol
                        if let Some(captures) = security_protocol_regex.captures(&item.flags) {
                            new_item.security = captures[1].to_string();
                        }

                        // Match the encryption type (CCMP or TKIP)
                        if let Some(captures) = encryption_regex.captures(&item.flags) {
                            new_item.encryption = captures[1].to_string();
                        }

                        // new_item.is_secured = item
                        //     .security
                        //     .as_ref()
                        //     .map_or(false, |s| security_protocols.contains(&s.as_str()));

                        new_item // Return the modified item
                    })
                    .collect();

                let _ = self.app_channel.send(AppMessage::AvailableNetworksList {
                    list: modified_wireless_network.clone(),
                });

                match connected_network_info {
                    Some(response) => {
                        let connected_wireless: Option<WirelessInfoItem> =
                            modified_wireless_network
                                .iter()
                                .find(|p| {
                                    p.scan_info.name.to_lowercase() == response.name.to_lowercase()
                                })
                                .cloned();

                        // println!("connected_wireless: {:?}", connected_wireless);
                        let _ = self.app_channel.send(AppMessage::ConnectedNetworkDetails {
                            details: connected_wireless.clone(),
                        });
                    }
                    None => (),
                }
            }
            Err(e) => {
                println!("error while getting wireless scan list {}", e);
            }
        }

        loop {
            if wireless_msg_rx.is_closed() {
                break;
            }
            select! {
                msg = wireless_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        WirelessMessage::Toggle { value } => {
                            if let Some(turn_on) = value {
                                if turn_on {
                                    let _ = WirelessService::enable_wireless().await;
                                } else {
                                    let _ = WirelessService::disable_wireless().await;
                                }
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}
