use anyhow::Result;
use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::sync::{mpsc, oneshot};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelHandler, ToplevelMessage,
};

use crate::AppMessage;

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct HomeButtonSettings {
    pub min_time_long_press: u64,
}

pub struct HomeButtonHandler {
    app_channel: Sender<AppMessage>,
    pressed_at: Option<Instant>,
    configs: HomeButtonSettings,
    long_press_sent: bool,
}
impl HomeButtonHandler {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self {
            pressed_at: None,
            configs: HomeButtonSettings {
                min_time_long_press: 1,
            },
            app_channel,
            long_press_sent: false,
        }
    }

    pub async fn run(mut self) {
        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);
        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, _) = mpsc::channel(128);
        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);
        // start the toplevel handler
        let toplevel_t = tokio::spawn(async move {
            let _ = toplevel_handler.run(toplevel_msg_rx).await;
        });

        let home_button_event_t = tokio::spawn(async move {
            if let Ok(mut stream) = HwButton::get_notification_stream(
                "/org/mechanix/services/HwButton/Home".to_string(),
            )
            .await
            {
                while let Some(signal) = stream.next().await {
                    if let Ok(args) = signal.args() {
                        let event = args.event;
                        // println!("HomeButtonHandler::run() event is {:?}", event);
                        match event {
                            KeyEvent::Pressed(Key::Home) => {
                                self.pressed_at = Some(Instant::now());
                            }
                            KeyEvent::Released(Key::Home) => {
                                self.pressed_at = None;
                                if !self.long_press_sent {
                                    let _ = minimize_all(toplevel_msg_tx.clone()).await;
                                }
                            }
                            KeyEvent::Pressing(Key::Home) => {
                                let home_button_pressed_for =
                                    (Instant::now() - self.pressed_at.unwrap()).as_secs();
                                println!("home button pressed for {:?}", home_button_pressed_for);
                                let is_long_press =
                                    home_button_pressed_for >= self.configs.min_time_long_press;
                                println!("is_long_press {:?}", is_long_press);

                                //if long press spawn power options
                                //else lock screen

                                if is_long_press {
                                    let _ = &self.app_channel.send(AppMessage::RunningApps {
                                        message: crate::RunningAppsMessage::Toggle { value: true },
                                    });
                                    self.long_press_sent = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            };
        });
        home_button_event_t.await.unwrap();
        toplevel_t.await.unwrap();
    }
}

async fn minimize_all(toplevel_msg_tx: mpsc::Sender<ToplevelMessage>) -> Result<bool> {
    let (tx, rx) = oneshot::channel();
    let _ = toplevel_msg_tx
        .send(ToplevelMessage::MinimizeAll { reply_to: tx })
        .await;
    if let Err(e) = rx.await {
        println!("errors while minimizing applications {:?}", e);
        return Ok(false);
    }
    Ok(true)
}
