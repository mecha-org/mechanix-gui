use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_desktop_dbus_client::sound::{NotificationStream, Sound};
use tokio::runtime::Runtime;
use tokio::select;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref SOUND_MODEL: SoundModel = SoundModel {
        input_devices: Context::new(vec![]),
        input_volume: Context::new(50.0),
        output_device: Context::new(vec![]),
        output_volume: Context::new(0.0),

        listen_to_stream: Context::new(false)
    };
}

#[derive(Model)]
pub struct SoundModel {
    pub input_devices: Context<Vec<String>>,
    pub input_volume: Context<f64>,

    pub output_device: Context<Vec<String>>,
    pub output_volume: Context<f64>,

    pub listen_to_stream: Context<bool>,
}

impl SoundModel {
    pub fn get() -> &'static Self {
        &SOUND_MODEL
    }

    pub fn set_output_volume(value: f64) {
        RUNTIME.spawn(async move {
            let _ = Sound::set_sound_percentage(value, "".to_string()).await;
        });
    }

    pub fn update() {
        if *SoundModel::get().listen_to_stream.get() {
            return;
        }
        SoundModel::get().listen_to_stream.set(true);
        RUNTIME.spawn(async {
            if let Ok(value) = Sound::get_sound_percentage("".to_string()).await {
                SoundModel::get().output_volume.set(value);
            }
            let mut sound_stream: NotificationStream<'static> =
                Sound::get_notification_stream().await.unwrap();

            loop {
                select! {
                signal = sound_stream.next() => {
                    if signal.is_none() {
                        continue;
                    }
                    if let Ok(args) = signal.unwrap().args() {
                        let event = args.event;
                        SoundModel::get().output_volume.set(event.volume_level);
                    } else {
                        // TODO: Fix/improve temperory Solution
                        // if let Ok(value) = Sound::get_sound_percentage("".to_string()).await {
                        //     SoundModel::get().output_volume.set(value);
                        // }
                        // println!("Error in sound stream");
                    }

                }
                }
            }
        });
    }
}
