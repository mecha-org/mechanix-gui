use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use super::service::BluetoothService;
use crate::Message;

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

pub struct BluetoothServiceHandle {
    status: ServiceStatus,
}

impl BluetoothServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match BluetoothService::get_bluetooth_status().await {
                Ok(bluetooth_status) => {
                    let _ = sender.send(Message::BluetoothStateUpdate(bluetooth_status));
                }
                Err(_e) => {}
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
