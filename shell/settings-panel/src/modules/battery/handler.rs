use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::{io, str::FromStr, time::Duration};
use tokio::time;
use upower::BatteryStatus;

use crate::{AppMessage, BatteryMessage};

use super::service::BatteryService;
use tracing::{error, info};

pub struct BatteryInfo {
    level: u8,
    status: BatteryStatus,
}

pub struct BatteryServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BatteryServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        match BatteryService::get_battery_level().await {
            Ok((percentage, status)) => {
                let _ = self.app_channel.send(AppMessage::Battery {
                    message: BatteryMessage::Status {
                        level: percentage,
                        status,
                    },
                });
            }
            Err(e) => {
                error!("error while getting battery level {}", e);
                let _ = self.app_channel.send(AppMessage::Battery {
                    message: BatteryMessage::Status {
                        level: 0,
                        status: BatteryStatus::Unknown,
                    },
                });
            }
        };

        let battery_r = upower::get_battery().await;

        if let Err(e) = battery_r {
            println!("Error while getting battery {:?}", e);
            return;
        }

        let battery = battery_r.unwrap();

        let mut state_stream = battery.receive_state_changed().await;
        let app_channel = self.app_channel.clone();
        let battery_cloned = battery.clone();
        let state_stream_t = tokio::spawn(async move {
            while let Some(msg) = state_stream.next().await {
                if let Ok(state) = msg.get().await {
                    let status = BatteryStatus::try_from(state).unwrap();
                    let level = battery_cloned.percentage().await.unwrap() as u8;
                    let _ = app_channel.send(AppMessage::Battery {
                        message: BatteryMessage::Status { level, status },
                    });
                };
            }
        });

        let mut percentage_stream = battery.receive_percentage_changed().await;
        let app_channel = self.app_channel.clone();
        let percentage_stream_t = tokio::spawn(async move {
            while let Some(msg) = percentage_stream.next().await {
                if let Ok(percentage) = msg.get().await {
                    let status = BatteryStatus::try_from(battery.state().await.unwrap()).unwrap();
                    let level = percentage as u8;
                    let _ = app_channel.send(AppMessage::Battery {
                        message: BatteryMessage::Status { level, status },
                    });
                };
            }
        });

        state_stream_t.await.unwrap();
        percentage_stream_t.await.unwrap();
    }
}
