use anyhow::{bail, Result};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse,
    WirelessInfoResponse,
    WirelessScanListResponse,
    WirelessService as WirelessZbusClient, // WirelessService,
};
use regex::Regex;
use tokio::{select, sync::mpsc::Receiver};

use super::errors::{WirelessServiceError, WirelessServiceErrorCodes};

use crate::{
    // types::{WirelessConnectedState, WirelessStatus},
    AppMessage,
    WirelessMessage,
};

#[derive(Debug, Clone)]
pub struct WirelessDetailsItem {
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
        // match WirelessService::wireless_status().await {
        match WirelessService::get_wireless_status().await {
            Ok(wireless_status) => {
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: Some(wireless_status),
                    },
                });
            }
            Err(e) => {
                println!("error while getting wireless status {}", e);
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: Some(false),
                    },
                });
            }
        };

        let connected_network_info: Option<WirelessInfoResponse> =
            match WirelessService::get_connected_network().await {
                Ok(response) => {
                    // modified_wireless_network.find(|&p| p.name == name_to_find);

                    let _ = self.app_channel.send(AppMessage::Wireless {
                        message: WirelessMessage::ConnectedNetworkName {
                            name: response.clone().name.clone(),
                        },
                    });
                    Some(response)
                }
                Err(e) => {
                    println!("error while getting wireless info {}", e);
                    let _ = self.app_channel.send(AppMessage::NotFound);
                    None
                }
            };

        match WirelessService::scan_networks().await {
            Ok(response) => {
                let security_protocols = vec!["WPA-PSK", "WPA2-PSK", "WPA3-PSK"];

                let security_protocol_regex = Regex::new(r"\[(WPA[2-]?-PSK)\-").unwrap();
                let encryption_regex = Regex::new(r"\-(CCMP|TKIP)\]").unwrap();

                let modified_wireless_network: Vec<WirelessDetailsItem> = response
                    .wireless_network
                    .into_iter()
                    .map(|item| {
                        let mut new_item: WirelessDetailsItem = WirelessDetailsItem {
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

                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::AvailableNetworksList {
                        list: modified_wireless_network.clone(),
                    },
                });

                match connected_network_info {
                    Some(response) => {
                        let connected_wireless: Option<WirelessDetailsItem> =
                            modified_wireless_network
                                .iter()
                                .find(|p| {
                                    p.scan_info.name.to_lowercase() == response.name.to_lowercase()
                                })
                                .cloned();

                        let _ = self.app_channel.send(AppMessage::Wireless {
                            message: WirelessMessage::ConnectedNetworkDetails {
                                details: connected_wireless.clone(),
                            },
                        });
                    }
                    None => (),
                }
            }
            Err(e) => {
                println!("error while getting wireless scan list {}", e);
            }
        }

        match WirelessService::get_known_networks().await {
            Ok(response) => {
                println!(
                    " WirelessService::get_known_networks {:?} ",
                    response.known_network.clone()
                );

                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::KnownNetworksList {
                        list: response.known_network.clone(),
                    },
                });
            }
            Err(e) => {
                println!("error while getting wireless known list {}", e);
            }
        };

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
                        WirelessMessage::getStatus => {
                            match WirelessService::get_wireless_status().await {
                                Ok(wireless_status) => {
                                    let _ = self.app_channel.send(AppMessage::Wireless {
                                        message: WirelessMessage::Status {
                                            status: Some(wireless_status),
                                        },
                                    });
                                }
                                Err(e) => {
                                    println!("error while getting wireless status {}", e);
                                    let _ = self.app_channel.send(AppMessage::Wireless {
                                        message: WirelessMessage::Status {
                                            status: Some(false),
                                        },
                                    });
                                }
                            };
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<bool> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wireless_status().await {
            Ok(v) => v,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::GetWirelessNetworkStatus,
                e.to_string(),
            )),
        };
        if !wireless_on {
            return Ok(false);
        };

        Ok(wireless_on)
    }

    pub async fn enable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::enable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::EnableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }

    pub async fn disable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::disable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::DisableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }

    pub async fn get_connected_network() -> Result<WirelessInfoResponse> {
        let task = "get_connected_network";
        let list = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::GetCurrentWirelessNetwork,
                e.to_string(),
            )),
        };

        Ok(list)
    }

    pub async fn scan_networks() -> Result<WirelessScanListResponse> {
        let task = "scan_networks";
        let list = match WirelessZbusClient::scan().await {
            Ok(v) => v,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::ScanNetworks,
                e.to_string(),
            )),
        };

        Ok(list)
    }

    pub async fn get_known_networks() -> Result<KnownNetworkListResponse> {
        let task = "get_known_networks";
        let list = match WirelessZbusClient::known_networks().await {
            Ok(v) => v,
            Err(e) => bail!(WirelessServiceError::new(
                WirelessServiceErrorCodes::KnownNetworks,
                e.to_string(),
            )),
        };

        Ok(list)
    }
}
