use std::time::Duration;

use crate::AppMessage;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use sysinfo::System;

pub struct CPUServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl CPUServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        CPUServiceHandle { app_channel }
    }

    pub async fn run(&self) {
        let mut sys = System::new();

        loop {
            sys.refresh_cpu_usage(); // Refreshing CPU usage.
            let usage = sys.global_cpu_usage();
            let _ = &self.app_channel.send(AppMessage::CPUUsage { usage });
            // Sleeping to let time for the system to run for long
            // enough to have useful information.
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
