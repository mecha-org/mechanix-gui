use std::time::Duration;

use crate::AppMessage;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use sysinfo::System;

pub struct UptimeHandle {
    app_channel: Sender<AppMessage>,
}

impl UptimeHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        UptimeHandle { app_channel }
    }

    pub async fn run(&self) {
        loop {
            let up = System::uptime();
            let mut uptime = up;
            let days = uptime / 86400;
            uptime -= days * 86400;
            let hours = uptime / 3600;
            uptime -= hours * 3600;
            let minutes = uptime / 60;
            let uptime = format!("{:?}d{:?}h{:?}m", days, hours, minutes);
            let _ = &self.app_channel.send(AppMessage::Uptime { uptime });
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
