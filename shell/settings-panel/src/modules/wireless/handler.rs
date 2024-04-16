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
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            select! {
                tick = interval.tick() => {
                    println!("Wireless handler tick()");

                    match WirelessService::get_wireless_status().await {
                        Ok(wireless_status) => {
                            let _ = self.app_channel.send(AppMessage::Wireless { message: WirelessMessage::Status { status: wireless_status } });
                        }
                        Err(e) => {
                            println!("error while getting wireless status {:?}", e.to_string());
                            let _ = self.app_channel.send(AppMessage::Wireless { message: WirelessMessage::Status { status: WirelessStatus::NotFound } });
                        }
                    };
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
