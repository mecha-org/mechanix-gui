use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;

use crate::{
    types::{WirelessConnectedState, WirelessStatus},
    AppMessage,
};

use crate::errors::{OnScreenDisplayError, OnScreenDisplayErrorCodes};
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

    pub async fn run(&mut self) {
        let task = "WirelessServiceHandle::run()";
        match WirelessService::get_wireless_status().await {
            Ok(wireless_status) => {
                let _ = self.app_channel.send(AppMessage::Wireless {
                    status: wireless_status,
                });
            }
            Err(e) => {
                println!("error while getting wireless status {}", e);
                let _ = self.app_channel.send(AppMessage::Wireless {
                    status: WirelessStatus::NotFound,
                });
            }
        };

        let mut stream_res = WirelessService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            println!("error while getting wireless stream {}", e);
            let _ = self.app_channel.send(AppMessage::Wireless {
                status: WirelessStatus::NotFound,
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            let args_r = signal.args();

            if let Err(e) = &args_r {
                println!("error while parsing args {}", e);
            };

            let args = args_r.unwrap();

            println!("args are {:?}", args);

            let event = args.event;
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
                status: wireless_status,
            });
        }
    }
}

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wireless_status().await {
            Ok(v) => v,
            Err(e) => bail!(OnScreenDisplayError::new(
                OnScreenDisplayErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        if !wireless_on {
            return Ok(WirelessStatus::Off);
        };

        let wireless_info = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(OnScreenDisplayError::new(
                OnScreenDisplayErrorCodes::GetWirelessStatusError,
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
