use std::fmt;

use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use tokio::{select, sync::mpsc::Receiver};

use crate::{
    errors::{LauncherError, LauncherErrorCodes},
    types::BluetoothStatus,
    AppMessage, BluetoothMessage,
};
use anyhow::{bail, Result};
use mechanix_system_dbus_client::bluetooth::{
    BluetoothService as BluetoothZbusClient, NotificationStream,
};

use tracing::error;

pub struct BluetoothServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BluetoothServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut bluetooth_msg_rx: Receiver<BluetoothMessage>) {
        let task = "run";
        match BluetoothService::get_bluetooth_status().await {
            Ok(bluetooth_status) => {
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    message: BluetoothMessage::Status {
                        status: bluetooth_status,
                    },
                });
            }
            Err(e) => {
                error!(task, "error while getting bluetooth status {}", e);
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    message: BluetoothMessage::Status {
                        status: BluetoothStatus::NotFound,
                    },
                });
            }
        };

        let mut stream_res = BluetoothService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting bluetooth stream {}", e);
            let _ = self.app_channel.send(AppMessage::Bluetooth {
                message: BluetoothMessage::Status {
                    status: BluetoothStatus::NotFound,
                },
            });
            return;
        }

        loop {
            select! {
                signal = stream_res.as_mut().unwrap().next() => {
                    if signal.is_none() {
                        continue;
                    }

                    if let Ok(args) = signal.unwrap().args() {
                        let event = args.event;
                        let mut status = BluetoothStatus::Off;

                        if event.is_enabled {
                            status = BluetoothStatus::On;
                        }

                        if event.is_connected {
                            status = BluetoothStatus::Connected
                        }

                        let _ = self.app_channel.send(AppMessage::Bluetooth { message: BluetoothMessage::Status {status }});
                    }

                }

                msg = bluetooth_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        BluetoothMessage::Toggle { value } => {
                            println!("BluetoothServiceHandle::run() toggle {:?}", value);
                            if let Some(turn_on) = value {
                                if turn_on {
                                   let _ = BluetoothService::enable_bluetooth().await;
                                }
                                else {
                                   let _ = BluetoothService::disable_bluetooth().await;
                                }
                            }
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothStatus> {
        let task = "get_bluetooth_status";

        let bluetooth_on = match BluetoothZbusClient::status().await {
            Ok(v) => v,
            Err(e) => bail!(BluetoothServiceError::new(
                BluetoothServiceErrorCodes::GetBluetoothStatusError,
                e.to_string(),
                false,
            )),
        };

        if bluetooth_on == 0 {
            return Ok(BluetoothStatus::Off);
        };

        let bluetooth_connected = match BluetoothZbusClient::is_connected().await {
            Ok(v) => v,
            Err(e) => bail!(BluetoothServiceError::new(
                BluetoothServiceErrorCodes::GetBluetoothStatusError,
                e.to_string(),
                false,
            )),
        };

        if bluetooth_connected == 1 {
            return Ok(BluetoothStatus::Connected);
        } else {
            return Ok(BluetoothStatus::On);
        };
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = BluetoothZbusClient::get_notification_stream().await?;
        Ok(stream)
    }

    pub async fn enable_bluetooth() -> Result<()> {
        match BluetoothZbusClient::enable_bluetooth().await {
            Ok(_) => true,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::EnableBluetooth,
                e.to_string(),
            )),
        };
        Ok(())
    }

    pub async fn disable_bluetooth() -> Result<()> {
        match BluetoothZbusClient::disable_bluetooth().await {
            Ok(_) => true,
            Err(e) => bail!(LauncherError::new(
                LauncherErrorCodes::DisableBluetooth,
                e.to_string(),
            )),
        };
        Ok(())
    }
}

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum BluetoothServiceErrorCodes {
    #[default]
    UnknownError,
    CreateBluetoothControllerError,
    GetBluetoothStatusError,
}

impl fmt::Display for BluetoothServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BluetoothServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            BluetoothServiceErrorCodes::CreateBluetoothControllerError => {
                write!(f, "CreateBluetoothControllerError")
            }
            BluetoothServiceErrorCodes::GetBluetoothStatusError => {
                write!(f, "GetBluetoothStatusError")
            }
        }
    }
}

/// # BluetoothServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`BluetoothServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BluetoothServiceError {
    pub code: BluetoothServiceErrorCodes,
    pub message: String,
}

impl BluetoothServiceError {
    pub fn new(code: BluetoothServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BluetoothServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
