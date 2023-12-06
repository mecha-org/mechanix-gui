use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;

use super::service::ClockService;

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

pub struct ClockServiceHandle {
    status: ServiceStatus,
}

impl ClockServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, format: String, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let _ = sender.send(Message::TimeTick(ClockService::get_current_time(&format)));
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
