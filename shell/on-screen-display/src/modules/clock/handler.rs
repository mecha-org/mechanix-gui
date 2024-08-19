use chrono::Timelike;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time::{self, interval_at, Instant};

use crate::AppMessage;

use super::service::ClockService;

#[derive(Debug)]

pub struct ClockServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl ClockServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, format: String) {
        let remaining_seconds = 60 - chrono::Local::now().second() as u64;
        let start = Instant::now() + Duration::from_secs(remaining_seconds);
        let mut interval = interval_at(start, Duration::from_secs(60));
        self.send_event(&format);

        // let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            self.send_event(&format);
        }
    }

    pub fn send_event(&self, format: &String) {
        let time = ClockService::get_current_time(&format);
        let date = ClockService::get_current_time("%e %B  %A");
        let _ = self.app_channel.send(AppMessage::Clock {
            time: time.clone(),
            date,
        });
    }
}
