use anyhow::Result;
use command::spawn_command;
use futures_util::stream::StreamExt;
use mechanix_desktop_settings::home_button::HomeButtonSettings;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use std::time::Instant;

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
        if let Ok(mut stream) =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Home".to_string())
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
                                let _ =
                                    app_switcher(self.configs.run_commands.app_switcher.clone())
                                        .await;
                            } else {
                                let _ = minimize_all();
                            }
                        }
                        _ => {}
                    }
                }
            }
        };
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

fn minimize_all() {}
