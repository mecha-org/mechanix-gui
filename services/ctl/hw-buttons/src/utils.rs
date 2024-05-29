use anyhow::Result;
use evdev::{Device, EventStream};
use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum Key {
    Power,
    Home,
    Unknown,
}

impl From<evdev::Key> for Key {
    fn from(value: evdev::Key) -> Self {
        match value {
            evdev::Key::KEY_PAUSE => Key::Power,
            evdev::Key::KEY_FN_1 => Key::Home,
            _ => Key::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Type)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
    Pressing(Key),
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
