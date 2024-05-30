use anyhow::Result;
use command::spawn_command;
use futures_util::stream::StreamExt;
use logind::session_lock;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use std::time::Instant;

use crate::settings::lock_button::LockButtonSettings;

pub struct LockButtonHandler {
    pressed_at: Option<Instant>,
    configs: LockButtonSettings,
}
impl LockButtonHandler {
    pub fn new(configs: LockButtonSettings) -> Self {
        Self {
            pressed_at: None,
            configs,
        }
    }

    pub async fn run(mut self) {
        if let Ok(mut stream) =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Power".to_string())
                .await
        {
            while let Some(signal) = stream.next().await {
                if let Ok(args) = signal.args() {
                    let event = args.event;
                    // println!("LockButtonHandler::run() event is {:?}", event);
                    match event {
                        KeyEvent::Pressed(Key::Power) => {
                            self.pressed_at = Some(Instant::now());
                        }
                        KeyEvent::Released(Key::Power) => {
                            let power_button_pressed_for =
                                (Instant::now() - self.pressed_at.unwrap()).as_secs();
                            println!("power button pressed for {:?}", power_button_pressed_for);
                            let is_long_press =
                                power_button_pressed_for >= self.configs.min_time_long_press;
                            println!("is_long_press {:?}", is_long_press);
                            self.pressed_at = None;

                            //if long press spawn power options
                            //else lock screen

                            if is_long_press {
                                let _ =
                                    power_options(self.configs.run_commands.power_options.clone())
                                        .await;
                            } else {
                                let _ = session_lock().await;
                            }
                        }
                        _ => {}
                    }
                }
            }
        };
    }
}

async fn power_options(run_command: String) -> Result<bool> {
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
