use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{select, sync::mpsc::Receiver, time};

use crate::{
    types::{WirelessConnectedState, WirelessInfo, WirelessStatus},
    AppMessage, WirelessMessage,
};

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
        match WirelessService::get_wireless_status().await {
            Ok(wireless_status) => {
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: wireless_status,
                    },
                });
            }
            Err(e) => {
                println!("error while getting wireless status {}", e);
                let _ = self.app_channel.send(AppMessage::Wireless {
                    message: WirelessMessage::Status {
                        status: WirelessStatus::NotFound,
                    },
                });
            }
        };

        let mut stream_res = WirelessService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            println!("error while getting wireless stream {}", e);
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
                            let event = args.event;
                            println!("event {:?}", event);
                            let mut wireless_status = WirelessStatus::Off;

                            if event.is_enabled {
                                wireless_status = WirelessStatus::On;
                            }

                            if event.is_connected {
                                let signal = event.signal_strength.parse::<i32>().unwrap();
                                let info = WirelessInfo {
                                    ssid: event.ssid.clone(),
                                    frequency: event.frequency.clone(),
                                };
                                wireless_status = if signal <= -80 {
                                    WirelessStatus::Connected(WirelessConnectedState::Low, info)
                                } else if signal <= -60 {
                                    WirelessStatus::Connected(WirelessConnectedState::Weak, info)
                                } else if signal <= -40 {
                                    WirelessStatus::Connected(WirelessConnectedState::Good, info)
                                } else {
                                    WirelessStatus::Connected(WirelessConnectedState::Strong, info)
                                };
                            }

                            let _ = self.app_channel.send(AppMessage::Wireless {
                                message: WirelessMessage::Status {
                                    status: wireless_status,
                                },
                            });
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
                    }
            }
        }
    }
}
