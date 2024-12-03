use crate::AppMessage;
use anyhow::Result;
use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Instant};
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::{sleep, Duration};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelHandler, ToplevelMessage,
};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct HomeButtonSettings {
    pub min_time_long_press: u64,
}

pub struct HomeButtonHandler {
    app_channel: Sender<AppMessage>,
    pressed_at: Arc<Mutex<Option<Instant>>>, // Use Arc<Mutex<>> for shared mutable state
    configs: HomeButtonSettings,
    long_press_sent: Arc<Mutex<bool>>, // Track if long press action was sent
}
impl HomeButtonHandler {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self {
            pressed_at: Arc::new(Mutex::new(None)),
            configs: HomeButtonSettings {
                min_time_long_press: 1,
            },
            app_channel,
            long_press_sent: Arc::new(Mutex::new(false)),
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
                        println!("HomeButtonHandler::run() event is {:?}", event);
                        let pressed_at_mutex = Arc::clone(&self.pressed_at);
                        let long_press_sent_mutex = Arc::clone(&self.long_press_sent);
                        match event {
                            KeyEvent::Pressed(Key::Home) => {
                                {
                                    let mut pressed_at = pressed_at_mutex.lock().await;
                                    *pressed_at = Some(Instant::now());
                                }

                                let app_channel = self.app_channel.clone();
                                let min_time_long_press = self.configs.min_time_long_press;

                                // Spawn a task that checks the duration while the button is pressed
                                tokio::spawn(async move {
                                    loop {
                                        let mut pressed_at = None;
                                        {
                                            let p = pressed_at_mutex.lock().await;
                                            pressed_at = *p;
                                        }

                                        if pressed_at.is_none() {
                                            break;
                                        }

                                        let elapsed =
                                            (Instant::now() - pressed_at.unwrap()).as_secs();

                                        let mut long_press_sent = false;
                                        {
                                            let p = long_press_sent_mutex.lock().await;
                                            long_press_sent = *p;
                                        }

                                        if elapsed >= min_time_long_press && !long_press_sent {
                                            // Long press detected
                                            let _ = app_channel.send(AppMessage::RunningApps {
                                                message: crate::RunningAppsMessage::Toggle {
                                                    value: true,
                                                },
                                            });
                                            {
                                                let mut long_press_sent =
                                                    long_press_sent_mutex.lock().await;
                                                *long_press_sent = true;
                                            }
                                            break;
                                        }

                                        // // Sleep for a short duration before checking again
                                        sleep(Duration::from_millis(100)).await;
                                    }
                                });
                            }
                            KeyEvent::Released(Key::Home) => {
                                let mut long_press_sent = false;
                                {
                                    let p = long_press_sent_mutex.lock().await;
                                    long_press_sent = *p;
                                }
                                if !long_press_sent {
                                    // Short press detected, minimize all
                                    let app_channel = self.app_channel.clone();
                                    let _ = app_channel.send(AppMessage::RunningApps {
                                        message: crate::RunningAppsMessage::Toggle { value: false },
                                    });
                                    let _ = minimize_all(toplevel_msg_tx.clone()).await;
                                }
                                {
                                    let mut pressed_at = pressed_at_mutex.lock().await;
                                    let mut long_press_sent = long_press_sent_mutex.lock().await;

                                    *pressed_at = None;
                                    *long_press_sent = false;
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
