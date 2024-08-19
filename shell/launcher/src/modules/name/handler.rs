use std::time::Duration;

use crate::AppMessage;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use sysinfo::System;

pub struct MachineNameHandle {
    app_channel: Sender<AppMessage>,
}

impl MachineNameHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        MachineNameHandle { app_channel }
    }

    pub async fn run(&self) {
        loop {
            let machine_name = System::host_name();
            if let Some(name) = machine_name {
                let _ = &self.app_channel.send(AppMessage::MachineName { name });
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
