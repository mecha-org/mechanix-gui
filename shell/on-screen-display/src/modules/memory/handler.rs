use std::time::Duration;

use crate::AppMessage;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use sysinfo::System;

pub struct MemoryHandle {
    app_channel: Sender<AppMessage>,
}

impl MemoryHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        MemoryHandle { app_channel }
    }

    pub async fn run(&self) {
        let mut sys = System::new();

        loop {
            sys.refresh_memory(); // Refreshing Memory usage.
            let total = sys.total_memory();
            let used = sys.used_memory();
            let _ = &self.app_channel.send(AppMessage::Memory { total, used });
            // Sleeping to let time for the system to run for long
            // enough to have useful information.
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
