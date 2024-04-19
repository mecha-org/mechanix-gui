use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;

use super::service::BluetoothService;
use crate::{types::BluetoothStatus, AppMessage};
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
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
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
        }
    }
}
