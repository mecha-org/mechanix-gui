use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;

use tracing::error;

use crate::{gui::Message, AppMessage, MemoryMessage};

use super::service::MemoryService;

pub struct MemoryServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl MemoryServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        match MemoryService::get_memory_usage().await {
            Ok(memory_info) => {
                let _ = self.app_channel.send(AppMessage::Memory {
                    message: MemoryMessage::Usage {
                        used: memory_info.used_memory,
                        total: memory_info.total_memory,
                    },
                });
            }
            Err(e) => {
                error!(task, "error while getting memory status {}", e);
                let _ = self.app_channel.send(AppMessage::Memory {
                    message: MemoryMessage::Usage { used: 0, total: 4 },
                });
            }
        };

        let mut stream_res = MemoryService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!("error while getting memory stream {}", e);
            let _ = self.app_channel.send(AppMessage::Memory {
                message: MemoryMessage::Usage { used: 0, total: 4 },
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let event = args.event;
                let _ = self.app_channel.send(AppMessage::Memory {
                    message: MemoryMessage::Usage {
                        used: event.available_memory,
                        total: event.total_memory,
                    },
                });
            }
        }
    }
}
