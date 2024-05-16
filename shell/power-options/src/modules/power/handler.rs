use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;
use tokio::{select, sync::mpsc::Receiver};

use tracing::error;

use crate::modules::power::service::PowerService;
use crate::AppMessage;

pub struct PowerServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl PowerServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut power_msg_rx: Receiver<AppMessage>) {
        let task = "run";

        loop {
            select! {
                msg = power_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        AppMessage::PowerOff => {
                            let _ = PowerService::power_off().await;
                        }
                        AppMessage::Reboot => {
                            let _ = PowerService::reboot().await;
                        }
                        AppMessage::Logout => {
                            let _ = PowerService::logout().await;
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
