use anyhow::Result;
use futures_util::stream::StreamExt;
use log::{debug, info};
use mechanix_hw_buttons::{Key, KeyEvent};
use std::time::Instant;
use system_dbus::hw_button_client::HwButton;

pub struct HwButtonHandler {
    pressed_at: Option<Instant>,
}
impl HwButtonHandler {
    pub fn new() -> Self {
        Self { pressed_at: None }
    }

    pub async fn run(mut self) {
        let mut power_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Power".to_string())
                .await
                .unwrap();
        let mut home_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Home".to_string())
                .await
                .unwrap();

        loop {
            tokio::select! {
                maybe_power_signal = power_stream.next() => {
                    if let Some(signal) = maybe_power_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("power button event is {:?}", event);
                            match event {
                                KeyEvent::Pressed(Key::Power) => {
                                    self.pressed_at = Some(Instant::now());
                                }
                                KeyEvent::Released(Key::Power) => {
                                    if let Some(pressed_at) = self.pressed_at {
                                        let power_button_pressed_for =
                                            (Instant::now() - pressed_at).as_secs();
                                        let is_long_press = power_button_pressed_for >= 1;
                                        println!("is_long_press {:?}", is_long_press);
                                        self.pressed_at = None;
                                        if is_long_press {
                                            info!("button long press event")
                                        } else {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                maybe_home_signal = home_stream.next() => {
                    if let Some(signal) = maybe_home_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("home button event is {:?}", event);
                            match event {
                                KeyEvent::Pressed(Key::Home) => {
                                    self.pressed_at = Some(Instant::now());
                                }
                                KeyEvent::Released(Key::Home) => {
                                    if let Some(pressed_at) = self.pressed_at {
                                        let home_button_pressed_for =
                                            (Instant::now() - pressed_at).as_secs();
                                        let is_long_press = home_button_pressed_for >= 1;
                                        println!("is_long_press {:?}", is_long_press);
                                        self.pressed_at = None;
                                        if is_long_press {
                                            info!("button long press event")
                                        } else {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
