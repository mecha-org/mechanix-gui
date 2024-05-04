use futures::StreamExt;
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
        let mut stream_res = WirelessService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!("error while getting wireless stream {}", e);
            let _ = self.app_channel.send(AppMessage::Wireless {
                status: WirelessStatus::NotFound,
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let notification_event = args.event;
                // let _ = self.app_channel.send(AppMessage::Wireless {
                //     status: wireless_status,
                // });
            }
        }
    }
}
