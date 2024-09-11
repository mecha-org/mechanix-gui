use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use tokio::{select, sync::mpsc::Receiver};

use crate::{
    types::{WirelessConnectedState, WirelessStatus},
    AppMessage, WirelessMessage,
};

use crate::errors::{LauncherError, LauncherErrorCodes};
use anyhow::{bail, Result};
use mechanix_system_dbus_client::wireless::{
    NotificationStream, WirelessService as WirelessZbusClient,
};
use std::fmt;
use tracing::error;

pub struct WirelessServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl WirelessServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut wireless_msg_rx: Receiver<WirelessMessage>) {
        println!("WirelessServiceHandle::run()");
        match WirelessService::get_wireless_status().await {
            Ok(wireless_status) => {
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: wireless_status,
                    },
                });
            }
            Err(e) => {
                println!("error while getting wireless status {}", e);
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: WirelessStatus::NotFound,
                    },
                });
            }
        };

        let mut stream_res = WirelessService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            println!("error while getting wireless stream {}", e);
            let _ = self.app_channel.send(AppMessage::Wireless {
                message: WirelessMessage::Status {
                    status: WirelessStatus::NotFound,
                },
            });
            return;
        }

        loop {
            if wireless_msg_rx.is_closed() {
                break;
            }
            select! {
                    signal = stream_res.as_mut().unwrap().next() => {
                        if signal.is_none() {
                            continue;
                        }


                        if let Ok(args) = signal.unwrap().args() {
                            let event = args.event;
                            println!("event {:?}", event);
                            let mut wireless_status = WirelessStatus::Off;

                            if event.is_enabled {
                                wireless_status = WirelessStatus::On;
                            }

                            if event.is_connected {
                                let signal = event.signal_strength.parse::<i32>().unwrap();
                                wireless_status = if signal <= -80 {
                                    WirelessStatus::Connected(WirelessConnectedState::Low)
                                } else if signal <= -60 {
                                    WirelessStatus::Connected(WirelessConnectedState::Weak)
                                } else if signal <= -40 {
                                    WirelessStatus::Connected(WirelessConnectedState::Good)
                                } else {
                                    WirelessStatus::Connected(WirelessConnectedState::Strong)
                                };
                            }

                            let _ = self.app_channel.send(AppMessage::Wireless {
                                message: WirelessMessage::Status {
                                    status: wireless_status,
                                },
                            });
                        }
                    }
                    msg = wireless_msg_rx.recv() => {
                        if msg.is_none() {
                            continue;
                        }

                        match msg.unwrap() {
                            WirelessMessage::Toggle { value } => {
                            println!("WirelessServiceHandle::run() toggle {:?}", value);
                            if let Some(turn_on) = value {
                                    if turn_on {
                                         let _ = WirelessService::enable_wireless().await;
                                    }
                                    else {
                                        let _ = WirelessService::disable_wireless().await;
                                    }
                                }
                            }
                        _ => ()
                        };
                    }
            }
        }
    }
}

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wireless_status().await {
            Ok(v) => v,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        if !wireless_on {
            return Ok(WirelessStatus::Off);
        };

        let wireless_info = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        let signal = wireless_info.signal.parse::<i32>().unwrap();

        let wireless_status = if signal <= -80 {
            WirelessStatus::Connected(WirelessConnectedState::Low)
        } else if signal <= -60 {
            WirelessStatus::Connected(WirelessConnectedState::Weak)
        } else if signal <= -40 {
            WirelessStatus::Connected(WirelessConnectedState::Good)
        } else {
            WirelessStatus::Connected(WirelessConnectedState::Strong)
        };

        Ok(wireless_status)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = WirelessZbusClient::get_notification_stream().await?;
        Ok(stream)
    }

    pub async fn enable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::enable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::EnableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }

    pub async fn disable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::disable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::DisableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }
}

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum WirelessServiceErrorCodes {
    #[default]
    UnknownError,
    GetWirelessNetworkStatusError,
    GetCurrentWirelessNetworkError,
}

impl fmt::Display for WirelessServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            WirelessServiceErrorCodes::GetWirelessNetworkStatusError => {
                write!(f, "GetWirelessNetworkStatusError")
            }
            WirelessServiceErrorCodes::GetCurrentWirelessNetworkError => {
                write!(f, "GetCurrentWirelessNetworkError")
            }
        }
    }
}

/// # WirelessServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`WirelessServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct WirelessServiceError {
    pub code: WirelessServiceErrorCodes,
    pub message: String,
}

impl WirelessServiceError {
    pub fn new(code: WirelessServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for WirelessServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
