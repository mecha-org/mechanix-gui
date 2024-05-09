use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;

use crate::{
    types::{WirelessConnectedState, WirelessStatus},
    AppMessage,
};

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
            println!("got args {:?}", args);
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
