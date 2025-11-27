use crate::events::{AppEvents, VolumeEvents};
use futures::{SinkExt, StreamExt, channel::mpsc, select};
use pulseaudio::service::PulseAudioService;

pub async fn sound_device_events(
    mut tx: mpsc::Sender<AppEvents>,
) {
    println!("Starting to get sound device info...");
    let pulse_service = match PulseAudioService::new() {
        Ok(pulse_service) => pulse_service,
        Err(e) => {
            eprintln!("Failed to create PulseAudioService: {}", e);
            return;
        }
    };
    match pulse_service.handle.get_default_sink().await {
        Ok(device_info) => {
            println!("Fetched sound device info: {:?}", device_info);
            let _ = tx
                .send(AppEvents::OutputSoundDevice {
                    device_info: device_info,
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get volume: {}", e);
            return;
        }
    }

    pulse_service.handle.shutdown().await;
}

pub async fn handle_volume_change(mut volume_rx: mpsc::Receiver<VolumeEvents>, mut tx: mpsc::Sender<AppEvents>) {
    println!("Starting to handle volume change events...");
    let pulse_service = match PulseAudioService::new() {
        Ok(pulse_service) => pulse_service,
        Err(e) => {
            eprintln!("Failed to create PulseAudioService: {}", e);
            return;
        }
    };

    loop {
        select! {
            event = volume_rx.next() => {
                println!("Received volume change event");
                if let Some(event) = event  {
                        match event {
                            VolumeEvents::VolumeChanged { name, value } => {
                                // let _ = pulse_service.handle.set_sink_volume_by_name(&name, &value).await;
                                // sound_device_events(tx.clone()).await;

                                match pulse_service.handle.set_sink_volume_by_name(&name, &value).await{
                                    Ok(_) => {
                                        println!("Successfully set volume to {} for sink {}", value, name);
                                        // After setting the volume, fetch the updated device info
                                       tx.send(AppEvents::OutputSoundDevice {
                                            device_info: pulse_service.handle.get_default_sink().await.unwrap(),
                                        }).await.unwrap();
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
    }

}
