use futures::StreamExt;
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use mechanix_desktop_dbus_client::sound::{NotificationStream, Sound};
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
use pulsectl::controllers::SourceController;
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
            let mut handler = SinkController::create().unwrap();
            if let Ok(device) = handler.get_default_device() {
                let mut volume = device.volume;
                let mut avg = volume.avg();
                avg.0 = ((value * 0.01) * 65536.0) as u32;
                for i in 1..=volume.len() {
                    volume.set(i, avg);
                }
                handler.set_device_volume_by_index(device.index, &volume);
            }
        });
    }

    pub fn set_input_volume(value: f64) {
        RUNTIME.spawn(async move {
            let mut handler = SourceController::create().unwrap();
            let devices = handler.list_devices().unwrap();
            for device in devices {
                let mut volume = device.volume;
                let mut avg = volume.avg();
                avg.0 = ((value * 0.01) * 65536.0) as u32;
                for i in 1..=volume.len() {
                    volume.set(i, avg);
                }
                handler.set_device_volume_by_index(device.index, &volume);
            }
        });
    }

    pub fn update() {
        if *SoundModel::get().listen_to_stream.get() {
            return;
        }
        SoundModel::get().listen_to_stream.set(true);
        RUNTIME.spawn(async move {
            let mut output_handler = SinkController::create().unwrap();
            let mut input_handler = SourceController::create().unwrap();
            loop {
                if let Ok(device) = output_handler.get_default_device() {
                    let volume = device.volume;
                    let avg = volume.avg();
                    let value = avg.0 as f64 / 65536.0 * 100.0;
                    SoundModel::get().output_volume.set(value);
                }

                if let Ok(device) = input_handler.get_default_device() {
                    let volume = device.volume;
                    let avg = volume.avg();
                    let value = avg.0 as f64 / 65536.0 * 100.0;
                    SoundModel::get().input_volume.set(value);
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}
