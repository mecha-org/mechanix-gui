use futures_util::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;

use super::service::BluetoothService;
use crate::{types::BluetoothStatus, StatusBarMessage as AppMessage};
use tracing::error;

pub struct BluetoothServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BluetoothServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        match BluetoothService::get_bluetooth_status().await {
            Ok(bluetooth_status) => {
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    status: bluetooth_status,
                });
            }
            Err(e) => {
                error!(task, "error while getting bluetooth status {}", e);
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    status: BluetoothStatus::NotFound,
                });
            }
        };

        let mut stream_res = BluetoothService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting bluetooth status {}", e);
            let _ = self.app_channel.send(AppMessage::Bluetooth {
                status: BluetoothStatus::NotFound,
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let mut status = BluetoothStatus::Off;

                let event = args.event;

                if event.is_enabled {
                    status = BluetoothStatus::On;
                }

                if event.is_connected {
                    status = BluetoothStatus::Connected
                }

                let _ = self.app_channel.send(AppMessage::Bluetooth { status });
            }
        }
    }
}
