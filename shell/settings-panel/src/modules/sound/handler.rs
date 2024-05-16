use futures::StreamExt;
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
        match SoundService::get_sound_value("".to_string()).await {
            Ok(value) => {
                let _ = self.app_channel.send(AppMessage::Sound {
                    message: SoundMessage::Value { value },
                });
            }
            Err(e) => {
                error!(task, "error while getting sound value {}", e);
            }
        };
        // let mut stream_res = SoundService::get_notification_stream().await;
        let mut interval = time::interval(Duration::from_secs(5));
        // if let Err(e) = stream_res.as_ref() {
        //     error!(task, "error while getting sound stream {}", e);
        //     let _ = self.app_channel.send(AppMessage::Sound {
        //         message: SoundMessage::Value { value: 0 },
        //     });
        //     return;
        // }
        loop {
            select! {
                // signal = stream_res.as_mut().unwrap().next() => {
                //     if signal.is_none() {
                //         continue;
                //     }

                //     if let Ok(args) = signal.unwrap().args() {
                //         let notification_event = args.event;
                //         // let _ = self.app_channel.send(AppMessage::Sound {
                //         //     message: SoundMessage::Value { value },
                //         // });
                //     }

                // }

                msg = sound_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        SoundMessage::Change { value } => {
                            println!("SoundServiceHandle::run() SoundMessage::Change {:?}", value);
                            interval.reset();
                            let _ = SoundService::set_sound_value(value as u8, "".to_string()).await;
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
