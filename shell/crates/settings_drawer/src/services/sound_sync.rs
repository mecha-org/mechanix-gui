use crate::events::{AppEvents, VolumeEvents};
use futures::{SinkExt, StreamExt, channel::mpsc, select};
use pulseaudio::service::PulseAudioService;

pub async fn audio_event_handler(
    mut tx: mpsc::Sender<AppEvents>,
    mut volume_rx: mpsc::Receiver<VolumeEvents>,
) {
    let pulse_service = match PulseAudioService::new() {
        Ok(service) => service,
        Err(e) => {
            eprintln!("Failed to create PulseAudioService: {}", e);
            return;
        }
    };

    if let Ok(device_info) = pulse_service.handle.get_default_sink().await {
        let _ = tx.send(AppEvents::OutputSoundDevice { device_info }).await;
    }

    loop {
        select! {
            event = volume_rx.next() => {
                if let Some(VolumeEvents::VolumeChanged { name, value }) = event {
                    match pulse_service.handle.set_sink_volume_by_name(&name, &value).await {
                        Ok(_) => {
                            if let Ok(updated_info) = pulse_service.handle.get_default_sink().await {
                                tx.send(AppEvents::OutputSoundDevice {
                                    device_info: updated_info,
                                }).await.ok();
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to set volume: {}", e);
                        }
                    }
                }
            }
        }
    }
}
