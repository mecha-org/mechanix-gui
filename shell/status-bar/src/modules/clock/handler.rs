use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::{AppMessage, Message};

use super::{component::ClockMessage, service::ClockService};

#[derive(Debug)]

pub struct ClockServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl ClockServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, format: String) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let _ = self.app_channel.send(AppMessage::Clock {
                current_time: ClockService::get_current_time(&format),
            });
        }
    }
}
