use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::{io, str::FromStr, time::Duration};
use tokio::time;

use crate::{types::BatteryStatus, AppMessage, BatteryMessage};

use super::service::BatteryService;
use tracing::{error, info};

pub struct BatteryInfo {
    level: u8,
    status: BatteryStatus,
}

impl FromStr for BatteryStatus {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("Unknown") => Ok(BatteryStatus::Unknown),
            _ if s.eq_ignore_ascii_case("Empty") => Ok(BatteryStatus::Empty),
            _ if s.eq_ignore_ascii_case("Full") => Ok(BatteryStatus::Full),
            _ if s.eq_ignore_ascii_case("Charging") => Ok(BatteryStatus::Charging),
            _ if s.eq_ignore_ascii_case("Discharging") => Ok(BatteryStatus::Discharging),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
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
            Ok((capacity, status)) => {
                let _ = self.app_channel.send(AppMessage::Battery {
                    message: BatteryMessage::Status {
                        level: capacity,
                        status: BatteryStatus::from_str(status.as_ref()).unwrap(),
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

        let mut stream_res = BatteryService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!("error while getting battery stream {}", e);
            let _ = self.app_channel.send(AppMessage::Battery {
                message: BatteryMessage::Status {
                    level: 0,
                    status: BatteryStatus::Unknown,
                },
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let event = args.event;
                let _ = self.app_channel.send(AppMessage::Battery {
                    message: BatteryMessage::Status {
                        level: event.percentage as u8,
                        status: BatteryStatus::from_str(&event.status).unwrap(),
                    },
                });
            }
        }
    }
}
