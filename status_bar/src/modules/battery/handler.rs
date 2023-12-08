use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::{BatteryState, Message};

use super::service::BatteryService;
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
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match BatteryService::get_battery_status().await {
                Ok(battery_status) => {
                    let _ = sender.send(Message::BatteryStatusUpdate(battery_status));
                }
                Err(e) => {
                    error!("error while getting battery status {}", e);
                    let _ = sender.send(Message::BatteryStatusUpdate(BatteryState::NotFound));
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
