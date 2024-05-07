use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;
use tokio::{select, sync::mpsc::Receiver};

use super::service::BluetoothService;
use crate::{types::BluetoothStatus, AppMessage, BluetoothMessage};
use tracing::error;

pub struct BluetoothServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BluetoothServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self, mut bluetooth_msg_rx: Receiver<BluetoothMessage>) {
        let task = "run";
        match BluetoothService::get_bluetooth_status().await {
            Ok(bluetooth_status) => {
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    message: BluetoothMessage::Status {
                        status: bluetooth_status,
                    },
                });
            }
            Err(e) => {
                error!(task, "error while getting bluetooth status {}", e);
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    message: BluetoothMessage::Status {
                        status: BluetoothStatus::NotFound,
                    },
                });
            }
        };

        let mut stream_res = BluetoothService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting bluetooth stream {}", e);
            let _ = self.app_channel.send(AppMessage::Bluetooth {
                message: BluetoothMessage::Status {
                    status: BluetoothStatus::NotFound,
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
                        let mut status = BluetoothStatus::Off;

                        if event.is_enabled {
                            status = BluetoothStatus::On;
                        }

                        if event.is_connected {
                            status = BluetoothStatus::Connected
                        }

                        let _ = self.app_channel.send(AppMessage::Bluetooth { message: BluetoothMessage::Status {status }});
                    }

                }

                msg = bluetooth_msg_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        BluetoothMessage::Toggle { value } => {
                            println!("BluetoothServiceHandle::run() toggle {:?}", value);
                            if let Some(turn_on) = value {
                                if turn_on {
                                   let _ = BluetoothService::enable_bluetooth().await;
                                }
                                else {
                                   let _ = BluetoothService::disable_bluetooth().await;
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
