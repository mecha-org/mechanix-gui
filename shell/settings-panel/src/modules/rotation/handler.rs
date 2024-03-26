use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::{gui::Message, modules::rotation::component::RotationStatus};

use super::{component::RotationMessage, service::RotationService};
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

pub struct RotationServiceHandle {
    status: ServiceStatus,
}

impl RotationServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match RotationService::get_rotation_status().await {
                Ok(Rotation_status) => {
                    let _ = sender.send(Message::Rotation {
                        status: Rotation_status,
                    });
                }
                Err(e) => {
                    error!(task, "error while getting Rotation status {}", e);
                    let _ = sender.send(Message::Rotation {
                        status: RotationStatus::NotFound,
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
