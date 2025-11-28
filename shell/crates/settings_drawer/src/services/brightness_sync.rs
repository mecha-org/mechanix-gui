use crate::events::{AppEvents, BrightnessEvents};
use futures::{SinkExt, StreamExt, channel::mpsc};
use system_dbus::display_client;

const MAX_DEVICE_BRIGHTNESS: u32 = 254;

pub async fn brightness_event_handler(
    mut tx: mpsc::Sender<AppEvents>,
    mut brightness_rx: mpsc::Receiver<BrightnessEvents>,
) {
    if let Ok(value) = display_client::get_brightness().await {
        let value = u8_to_percent(value, MAX_DEVICE_BRIGHTNESS);
        let _ = tx.send(AppEvents::Brightness { value }).await;
    }

    while let Some(event) = brightness_rx.next().await {
        match event {
            BrightnessEvents::BrightnessChanged { value } => {
                let value = if value < 10.0 { 10.0 } else { value };
                let value = percent_to_u8(value, MAX_DEVICE_BRIGHTNESS);
                match display_client::set_brightness(value).await {
                    Ok(_) => {
                        if let Ok(value) = display_client::get_brightness().await {
                            let value = u8_to_percent(value, MAX_DEVICE_BRIGHTNESS);
                            let _ = tx.send(AppEvents::Brightness { value }).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error setting brightness: {:?}", e);
                    }
                }
            }
        }
    }
}

fn u8_to_percent(value: u8, max_u32: u32) -> f32 {
    value as f32 / max_u32 as f32 * 100.0
}
fn percent_to_u8(percent: f32, max_u32: u32) -> u8 {
    ((percent / 100.0) * max_u32 as f32).round() as u8
}
