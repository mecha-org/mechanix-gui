use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{select, sync::mpsc::Receiver, time};

use crate::{types::WirelessStatus, AppMessage, WirelessMessage};

use super::service::WirelessService;
use tracing::error;

pub struct WirelessServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl WirelessServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut wireless_msg_rx: Receiver<WirelessMessage>) {
        println!("WirelessServiceHandle::run()");
        let task = "run";
        let mut stream_res = WirelessService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            println!("error while getting wireless status {:?}", e.to_string());
            let _ = self.app_channel.send(AppMessage::Wireless {
                message: WirelessMessage::Status {
                    status: WirelessStatus::NotFound,
                },
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
                        let notification_event = args.event;
                        // let _ = self.app_channel.send(AppMessage::Wireless { message: WirelessMessage::Status { status: wireless_status } });
                    }

                }

                msg = wireless_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        WirelessMessage::Toggle { value } => {
                            println!("WirelessServiceHandle::run() toggle {:?}", value);
                            if let Some(turn_on) = value {
                                if turn_on {
                                   let _ = WirelessService::enable_wireless().await;
                                }
                                else {
                                   let _ = WirelessService::disable_wireless().await;
                                }
                            }
                        }
                        _ => ()
                    };
                },
            }
        }
    }
}
