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

    update_device_info(&mut tx, &pulse_service).await;

    loop {
        select! {
            event = volume_rx.next() => {
                match event {
                    Some(VolumeEvents::VolumeChanged { name, value }) => {
                        match pulse_service.handle.set_sink_volume_by_name(&name, &value).await {
                            Ok(_) => {
                                update_device_info(&mut tx, &pulse_service).await;
                            }
                            Err(e) => {
                                eprintln!("Failed to set volume: {}", e);
                            }
                        }
                    }
                    Some(VolumeEvents::MuteSink { name }) => {
                        match pulse_service.handle.set_sink_mute_by_name(&name).await {
                            Ok(_) => {
                                update_device_info(&mut tx, &pulse_service).await;
                            }
                            Err(e) => {
                                eprintln!("Failed to set mute: {}", e);
                            }
                        }
                    }
                    Some(VolumeEvents::UnmuteSink { name }) => {
                        match pulse_service.handle.set_sink_unmute_by_name(&name).await {
                            Ok(_) => {
                                update_device_info(&mut tx, &pulse_service).await;
                            }
                            Err(e) => {
                                eprintln!("Failed to unset mute: {}", e);
                            }
                        }
                    }
                    None => break,
                }
               
            }
        }
    }
}

async fn update_device_info(tx: &mut mpsc::Sender<AppEvents>, pulse_service: &PulseAudioService) {
    if let Ok(device_info) = pulse_service.handle.get_default_sink().await {
        let _ = tx.send(AppEvents::OutputSoundDevice { device_info }).await;
    }
}
