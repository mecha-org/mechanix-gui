use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{
    select,
    sync::{mpsc::Receiver, oneshot},
    time,
};

use tracing::error;

use crate::{gui::Message, AppMessage, SoundMessage};

use super::service::SoundService;

pub struct SoundServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl SoundServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut sound_msg_rx: Receiver<SoundMessage>) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            select! {
                tick = interval.tick() => {
                    match SoundService::get_sound_value().await {
                        Ok(value) => {
                            let _ = self.app_channel.send(AppMessage::Sound {
                                message: SoundMessage::Value { value },
                            });
                        }
                        Err(e) => {
                            error!(task, "error while getting sound status {}", e);
                            let _ = self.app_channel.send(AppMessage::Sound {
                                message: SoundMessage::Value { value: 0 },
                            });
                        }
                    };
                }

                msg = sound_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        SoundMessage::Change { value } => {
                            let _ = SoundService::set_sound_value(value as i32).await;
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
