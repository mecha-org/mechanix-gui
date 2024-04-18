use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;

use crate::{types::WirelessStatus, AppMessage};

use super::service::WirelessService;
use tracing::error;

pub struct WirelessServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl WirelessServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match WirelessService::get_wireless_status().await {
                Ok(wireless_status) => {
                    let _ = self.app_channel.send(AppMessage::Wireless {
                        status: wireless_status,
                    });
                }
                Err(e) => {
                    error!(task, "error while getting wireless status {}", e);
                    let _ = self.app_channel.send(AppMessage::Wireless {
                        status: WirelessStatus::NotFound,
                    });
                }
            };
        }
    }
}
