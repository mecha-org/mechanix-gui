use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::StatusBarMessage as AppMessage;

use super::{component::ClockMessage, service::ClockService};

#[derive(Debug)]

pub struct ClockServiceHandle {
    app_channel: Sender<AppMessage>,
    prev_time: Option<String>,
}

impl ClockServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self {
            app_channel,
            prev_time: None,
        }
    }

    pub async fn run(&mut self, format: String) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let current_time = ClockService::get_current_time(&format);

            if let Some(prev_time) = &self.prev_time {
                if prev_time == &current_time {
                    continue;
                }
            }
            self.prev_time = Some(current_time.clone());
            println!("Clock::tick()");
            let _ = self.app_channel.send(AppMessage::Clock { current_time });
        }
    }
}
