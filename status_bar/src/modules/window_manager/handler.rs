use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;

use super::service::WindowManagerService;

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

pub struct WindowManagerServiceHandle {
    status: ServiceStatus,
}

impl WindowManagerServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match WindowManagerService::get_current_window().await {
                Ok(current_window) => {
                    let _ = sender.send(Message::CurrentWindowTitleUpdate(current_window));
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
