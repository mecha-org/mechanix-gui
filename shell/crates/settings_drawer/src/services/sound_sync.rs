use crate::events::AppEvents;
use futures::{SinkExt, channel::mpsc};
use pulseaudio::service::PulseAudioService;

pub async fn update_device_info(tx: &mut mpsc::Sender<AppEvents>, pulse_service: &PulseAudioService) {
    if let Ok(device_info) = pulse_service.handle.get_default_sink().await {
        let _ = tx.send(AppEvents::OutputSoundDevice { device_info }).await;
    }
}
