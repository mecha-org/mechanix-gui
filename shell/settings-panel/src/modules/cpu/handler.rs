use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use tracing::error;

use crate::{gui::Message, AppMessage, CpuMessage};

use super::service::CpuService;

pub struct CpuServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl CpuServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        let mut stream_res = CpuService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting cpu usage {}", e);
            let _ = self.app_channel.send(AppMessage::Cpu {
                message: CpuMessage::Usage { usage: 0. },
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let notification_event = args.event;
                // let _ = self.app_channel.send(AppMessage::Cpu {
                //     message: CpuMessage::Usage { usage },
                // });
            }
        }
    }
}
