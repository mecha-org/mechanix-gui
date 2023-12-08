use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;

use super::service::WirelessService;
use tracing::error;

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

pub struct WirelessServiceHandle {
    status: ServiceStatus,
}

impl WirelessServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match WirelessService::get_wireless_status().await {
                Ok(wireless_status) => {
                    let _ = sender.send(Message::WirelessStateUpdate(wireless_status));
                }
                Err(e) => {
                    error!(task, "error while getting wireless status {}", e);
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
