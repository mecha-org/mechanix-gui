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

    pub async fn run(mut self) -> anyhow::Result<()> {
        let mut power_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Power".to_string())
                .await?;
        let mut home_stream =
            HwButton::get_notification_stream("/org/mechanix/services/HwButton/Home".to_string())
                .await?;
        let mut volume_up_stream = HwButton::get_notification_stream(
            "/org/mechanix/services/HwButton/VolumeUp".to_string(),
        )
            .await?;
        let mut volume_down_stream = HwButton::get_notification_stream(
            "/org/mechanix/services/HwButton/VolumeDown".to_string(),
        )
            .await?;
        let mut extension_detection_stream = HwButton::get_notification_stream(
            "/org/mechanix/services/HwButton/ExtensionDetection".to_string(),
        )
            .await?;

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
                                        let power_button_pressed_for = (Instant::now() - pressed_at).as_secs();
                                        let is_long_press = power_button_pressed_for >= 1;
                                        println!("is_long_press {:?}", is_long_press);
                                        self.pressed_at = None;
                                        if is_long_press {
                                            info!("button long press event")
                                        }
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
                                        let home_button_pressed_for = (Instant::now() - pressed_at).as_secs();
                                        let is_long_press = home_button_pressed_for >= 1;
                                        println!("is_long_press {:?}", is_long_press);
                                        self.pressed_at = None;
                                        if is_long_press {
                                            info!("button long press event")
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                maybe_volume_up_signal = volume_up_stream.next() => {
                    if let Some(signal) = maybe_volume_up_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("volume up button event is {:?}", event);
                            // Handle volume up events here
                        }
                    }
                }
                maybe_volume_down_signal = volume_down_stream.next() => {
                    if let Some(signal) = maybe_volume_down_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("volume down button event is {:?}", event);
                            // Handle volume down events here
                        }
                    }
                }
                maybe_extension_detection_signal = extension_detection_stream.next() => {
                    if let Some(signal) = maybe_extension_detection_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("extension detection event is {:?}", event);
                            // Handle extension detection events here
                        }
                    }
                }
            }
        }
    }
}
