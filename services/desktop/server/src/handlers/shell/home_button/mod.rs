use anyhow::Result;
use command::spawn_command;
use futures_util::stream::StreamExt;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use std::time::Instant;
use tokio::sync::{mpsc, oneshot};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelHandler, ToplevelMessage,
};

use crate::settings::home_button::HomeButtonSettings;

pub struct HomeButtonHandler {
    pressed_at: Option<Instant>,
    configs: HomeButtonSettings,
}
impl HomeButtonHandler {
    pub fn new(configs: HomeButtonSettings) -> Self {
        Self {
            pressed_at: None,
            configs,
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
                                let home_button_pressed_for =
                                    (Instant::now() - self.pressed_at.unwrap()).as_secs();
                                println!("home button pressed for {:?}", home_button_pressed_for);
                                let is_long_press =
                                    home_button_pressed_for >= self.configs.min_time_long_press;
                                println!("is_long_press {:?}", is_long_press);
                                self.pressed_at = None;

                                //if long press spawn power options
                                //else lock screen

                                if is_long_press {
                                    let _ = app_switcher(
                                        self.configs.run_commands.app_switcher.clone(),
                                    )
                                    .await;
                                } else {
                                    let _ = minimize_all(toplevel_msg_tx.clone()).await;
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

async fn app_switcher(run_command: String) -> Result<bool> {
    let _ = tokio::spawn(async move {
        if !run_command.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(run_command.clone());
            let res = spawn_command("sh".to_string(), args);
            println!("spawn_command res {:?}", res);
        }
    })
    .await;
    Ok(true)
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
