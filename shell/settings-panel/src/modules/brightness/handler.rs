use futures::StreamExt;
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
        match BrightnessService::get_brightness_value().await {
            Ok(value) => {
                let _ = self.app_channel.send(AppMessage::Brightness {
                    message: BrightnessMessage::Value { value },
                });
            }
            Err(e) => {
                error!(task, "error while getting brightness value {}", e);
            }
        };
        let mut stream_res = BrightnessService::get_notification_stream().await;
        let mut interval = time::interval(Duration::from_secs(5));
        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting brightness stream {}", e);
            let _ = self.app_channel.send(AppMessage::Brightness {
                message: BrightnessMessage::Value { value: 0 },
            });
            return;
        }
        loop {
            select! {
                signal = stream_res.as_mut().unwrap().next() => {
                    if signal.is_none() {
                        continue;
                    }

                    if let Ok(args) = signal.unwrap().args() {
                        let event = args.event;
                        let _ = self.app_channel.send(AppMessage::Brightness {
                            message: BrightnessMessage::Value { value: event.brightness_percentage },
                        });
                    }

                }

                msg = brightness_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        BrightnessMessage::Change { value } => {
                            println!("BrightnessServiceHandle::run() BrightnessMessage::Change {:?}", value);
                            interval.reset();
                            let _ = BrightnessService::set_brightness_value(value as u8).await;
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
