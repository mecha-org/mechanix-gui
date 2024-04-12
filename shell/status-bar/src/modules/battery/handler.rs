use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::{io, str::FromStr, time::Duration};
use tokio::time;

use crate::{types::BatteryStatus, AppMessage};

use super::service::BatteryService;
use tracing::{error, info};

pub struct BatteryServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BatteryServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match BatteryService::get_battery_level().await {
                Ok((capacity, status)) => {
                    let _ = self.app_channel.send(AppMessage::Battery {
                        level: capacity,
                        status: BatteryStatus::from_str(status.as_ref()).unwrap(),
                    });
                }
                Err(e) => {
                    error!("error while getting battery level {}", e);
                    let _ = self.app_channel.send(AppMessage::Battery {
                        level: 0,
                        status: BatteryStatus::Unknown,
                    });
                }
            };
        }
    }
}
