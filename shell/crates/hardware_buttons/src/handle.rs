use dispatcher::Message;
use hw_buttons::{Key, KeyEvent};
use mxconf_dbus::set_setting;
use rusb::{
    DeviceHandle, DeviceList
    , Language, Result, UsbContext,
};
use std::fmt;
use std::time::Duration;

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    Gamepad,
    Keyboard,
    Gpio,
    Unknown,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Extension::Gamepad => "GAMEPAD",
            Extension::Keyboard => "KEYBOARD",
            Extension::Gpio => "GPIO",
            Extension::Unknown => "Unknown",
        };
        write!(f, "{s}")
    }
}
pub async fn build_message_for_event(event: KeyEvent) -> Option<Message> {
    println!("hardware_buttons: received hardware button event: {event:?}");

    let mut message: Option<Message> = None;

    match event {
        KeyEvent::Pressed(Key::Power) => {
            println!("hardware_buttons: power button pressed");
            message = Some(Message::ShowLockscreen(true));
        }
        KeyEvent::Pressing(Key::Power) => {
            println!("hardware_buttons: power button pressing");
            message = Some(Message::ShowPowerOptions(true));
        }
        KeyEvent::Unknown(Key::Power) => {
            println!("hardware_buttons: power button unknown event");
        }
        KeyEvent::Released(Key::Power) => {
            println!("hardware_buttons: power button released");
        }

        KeyEvent::Pressed(Key::Home) => {
            message = Some(Message::MinimizeToHome);
            println!("hardware_buttons: home button pressed");
        }
        KeyEvent::Pressing(Key::Home) => {
            println!("hardware_buttons: home button pressing");
        }
        KeyEvent::Released(Key::Home) => {
            println!("hardware_buttons: home button released");
        }
        KeyEvent::Unknown(Key::Home) => {
            println!("hardware_buttons: home button unknown event");
        }

        KeyEvent::Pressed(Key::VolumeUp) => {
            println!("hardware_buttons: volume up pressed");
            message = Some(Message::VolumeUp);
        }
        KeyEvent::Pressing(Key::VolumeUp) => {
            println!("hardware_buttons: volume up pressing");
            message = Some(Message::VolumeUp);
        }
        KeyEvent::Released(Key::VolumeUp) => {
            println!("hardware_buttons: volume up released");
        }
        KeyEvent::Unknown(Key::VolumeUp) => {
            println!("hardware_buttons: volume up unknown event");
            message = Some(Message::VolumeUp);
        }

        KeyEvent::Pressed(Key::VolumeDown) => {
            println!("hardware_buttons: volume down pressed");
            message = Some(Message::VolumeDown);
        }
        KeyEvent::Pressing(Key::VolumeDown) => {
            println!("hardware_buttons: volume down pressing");
            message = Some(Message::VolumeDown);
        }
        KeyEvent::Released(Key::VolumeDown) => {
            println!("hardware_buttons: volume down released");
        }
        KeyEvent::Unknown(Key::VolumeDown) => {
            println!("hardware_buttons: volume down unknown event");
            message = Some(Message::VolumeDown);
        }

        KeyEvent::Pressed(Key::ExtensionDetection) => {
            println!("HARDWARE EXTENTION DETCTION EVENT:PRESSED");
            message = Some(Message::SetExtensionDetected(true));
            let detected_extension_name = match get_detected_extension_name().await {
                Ok(extension) => extension,
                Err(_) => Extension::Unknown,
            };
            message = Some(Message::SetExtensionName(detected_extension_name.to_string()));
            set_setting("org.mechanix.desktop.settings.extension.detected", "true").await;
            set_setting("org.mechanix.desktop.settings.extension.name", &detected_extension_name.to_string()).await;
            println!("hardware_buttons: extension detection pressed");
        }
        KeyEvent::Pressing(Key::ExtensionDetection) => {
            println!("hardware_buttons: extension detection pressing");
            message = Some(Message::SetExtensionDetected(true));
            let detected_extension_name = match get_detected_extension_name().await {
                Ok(extension) => extension,
                Err(_) => Extension::Unknown,
            };
            set_setting("org.mechanix.desktop.settings.extension.detected", "true").await;
            set_setting("org.mechanix.desktop.settings.extension.name", &detected_extension_name.to_string()).await;
            message = Some(Message::SetExtensionName(detected_extension_name.to_string()));
        }
        KeyEvent::Released(Key::ExtensionDetection) => {
            println!("HARDWARE EXTENTION DETCTION EVENT:released");
            message = Some(Message::SetExtensionDetected(false));
            let detected_extension_name = match get_detected_extension_name().await {
                Ok(extension) => extension,
                Err(_) => Extension::Unknown,
            };
            set_setting("org.mechanix.desktop.settings.extension.detected", "false").await;
            set_setting("org.mechanix.desktop.settings.extension.name", &detected_extension_name.to_string()).await;

            message = Some(Message::SetExtensionName(detected_extension_name.to_string()));
            println!("hardware_buttons: extension detection released");
        }
        KeyEvent::Unknown(Key::ExtensionDetection) => {
            println!("hardware_buttons: extension detection unknown event");
        }

        KeyEvent::Pressed(Key::Unknown) => {
            println!("hardware_buttons: unknown key pressed");
        }
        KeyEvent::Pressing(Key::Unknown) => {
            println!("hardware_buttons: unknown key pressing");
        }
        KeyEvent::Released(Key::Unknown) => {
            println!("hardware_buttons: unknown key released");
        }
        KeyEvent::Unknown(Key::Unknown) => {
            println!("hardware_buttons: unknown key unknown event");
        }
    }

    message
}

async fn get_detected_extension_name() -> Result<Extension> {
    let timeout = Duration::from_secs(1);
    for device in DeviceList::new()?.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let mut usb_device = {
            match device.open() {
                Ok(h) => match h.read_languages(timeout) {
                    Ok(l) => {
                        if !l.is_empty() {
                            Some(UsbDevice {
                                handle: h,
                                language: l[0],
                                timeout,
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            }
        };

        let vendor_id = device_desc.vendor_id();
        let product_id = device_desc.product_id();

        if let Some(extension) = extension_from_vid_pid(vendor_id, product_id) {
            println!("  → Detected {:?}", extension);
            return Ok(extension);
        }
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
        );
    }
    Ok(Extension::Unknown)
}

pub fn extension_from_vid_pid(vendor_id: u16, product_id: u16) -> Option<Extension> {
    match (vendor_id, product_id) {
        (0xCE07, 0x0001) => Some(Extension::Keyboard),
        (0xCE07, 0x0002) => Some(Extension::Gamepad),
        (0xCE07, 0x0003) => Some(Extension::Gpio),
        _ => Some(Extension::Gpio),
    }
}
