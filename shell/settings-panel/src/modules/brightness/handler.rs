use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{
    select,
    sync::{mpsc::Receiver, oneshot},
    time,
};

use tracing::error;

use crate::{gui::Message, AppMessage, BrightnessMessage};

use super::service::BrightnessService;

pub struct BrightnessServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BrightnessServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut brightness_msg_rx: Receiver<BrightnessMessage>) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            select! {
                tick = interval.tick() => {
                    println!("BrightnessServiceHandle::run() tick");
                    match BrightnessService::get_brightness_value().await {
                        Ok(value) => {
                            let _ = self.app_channel.send(AppMessage::Brightness {
                                message: BrightnessMessage::Value { value },
                            });
                        }
                        Err(e) => {
                            error!(task, "error while getting brightness value {}", e);
                            let _ = self.app_channel.send(AppMessage::Brightness {
                                message: BrightnessMessage::Value { value: 0 },
                            });
                        }
                    };
                }

                msg = brightness_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        BrightnessMessage::Change { value } => {
                            println!("BrightnessServiceHandle::run() value {:?}", value);
                            let _ = BrightnessService::set_brightness_value(value as u8).await;
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
