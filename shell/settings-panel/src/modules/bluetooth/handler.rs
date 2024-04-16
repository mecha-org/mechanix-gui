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
        println!("BluetoothServiceHandle::run()");
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            select! {
                tick = interval.tick() => {
                    match BluetoothService::get_bluetooth_status().await {
                        Ok(bluetooth_status) => {
                            let _ = self.app_channel.send(AppMessage::Bluetooth {
                                message: BluetoothMessage::Status {
                                    status: bluetooth_status,
                                },
                            });
                        }
                        Err(e) => {
                            // error!(task, "error while getting bluetooth status {}", e);
                            let _ = self.app_channel.send(AppMessage::Bluetooth {
                                message: BluetoothMessage::Status {
                                    status: BluetoothStatus::NotFound,
                                },
                            });
                        }
                    };
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
