use anyhow::Result;
use evdev::{Device, EventStream};
use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
pub enum Key {
    #[default]
    Unknown,
    Power,
    Home,
    VolumeUp,
    VolumeDown,
    ExtensionDetection,
}

impl From<evdev::Key> for Key {
    fn from(value: evdev::Key) -> Self {
        match value {
            // todo: replace actual keys instead of using shift keys
            evdev::Key::KEY_POWER => Key::Power,
            evdev::Key::KEY_HOME => Key::Home,
            evdev::Key::KEY_F24 => Key::ExtensionDetection,
            evdev::Key::KEY_VOLUMEUP => Key::VolumeUp,
            evdev::Key::KEY_VOLUMEDOWN => Key::VolumeDown,
            _ => Key::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Type)]
pub enum KeyEvent {
    Unknown(Key),
    Pressed(Key),
    Released(Key),
    Pressing(Key),
}

impl Default for KeyEvent {
    fn default() -> Self {
        Self::Unknown(Key::default())
    }
}

pub fn get_device_stream(path: String) -> Result<EventStream> {
    let device_r = Device::open(path);

    if let Err(e) = &device_r {
        println!("Error opening device: {:?}", e);
    }

    let device = device_r.unwrap();

    let stream_r = device.into_event_stream();

    if let Err(e) = &stream_r {
        println!("Error getting stream {:?}", e);
    }

    Ok(stream_r.unwrap())
}
