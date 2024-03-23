use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::{gui::Message, modules::battery::component::BatteryLevel, settings::BatteryIconPaths};

use super::{component::BatteryMessage, service::BatteryService};
use tracing::{error, info};

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: oneshot::Sender<u32> },
    Stop { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

pub struct BatteryServiceHandle {
    status: ServiceStatus,
}

impl BatteryServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match BatteryService::get_battery_status().await {
                Ok(level) => {
                    let _ = sender.send(Message::Battery { level });
                }
                Err(e) => {
                    error!("error while getting battery level {}", e);
                    let _ = sender.send(Message::Battery {
                        level: BatteryLevel::NotFound,
                    });
                }
            };
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
