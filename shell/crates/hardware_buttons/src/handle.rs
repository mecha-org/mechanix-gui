use std::time::Duration;
use dispatcher::Message;
use hw_buttons::{Key, KeyEvent};
use log::info;
use rusb::{
    ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor,
    InterfaceDescriptor, Language, Result, Speed, UsbContext,
};
use mxconf_dbus::get_setting;

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    MechaGamepad,
    MechaKeyboard,
    MechaGpio,
}

pub fn build_message_for_event(event: KeyEvent) -> Option<Message> {
    info!("hardware_buttons: received hardware button event: {event:?}");

    let mut message: Option<Message> = None;

    match event {
        KeyEvent::Pressed(Key::Power) => {
            info!("hardware_buttons: power button pressed");
            message = Some(Message::ShowLockscreen(true));
        }
        KeyEvent::Pressing(Key::Power) => {
            info!("hardware_buttons: power button pressing");
            message = Some(Message::ShowPowerOptions(true));
        }
        KeyEvent::Unknown(Key::Power) => {
            info!("hardware_buttons: power button unknown event");
        }
        KeyEvent::Released(Key::Power) => {
            info!("hardware_buttons: power button released");
        }

        KeyEvent::Pressed(Key::Home) => {
            message = Some(Message::MinimizeToHome);
            info!("hardware_buttons: home button pressed");
        }
        KeyEvent::Pressing(Key::Home) => {
            info!("hardware_buttons: home button pressing");
        }
        KeyEvent::Released(Key::Home) => {
            info!("hardware_buttons: home button released");
        }
        KeyEvent::Unknown(Key::Home) => {
            info!("hardware_buttons: home button unknown event");
        }

        KeyEvent::Pressed(Key::VolumeUp) => {
            info!("hardware_buttons: volume up pressed");
            message = Some(Message::VolumeUp);
        }
        KeyEvent::Pressing(Key::VolumeUp) => {
            info!("hardware_buttons: volume up pressing");
            message = Some(Message::VolumeUp);
        }
        KeyEvent::Released(Key::VolumeUp) => {
            info!("hardware_buttons: volume up released");
        }
        KeyEvent::Unknown(Key::VolumeUp) => {
            info!("hardware_buttons: volume up unknown event");
            message = Some(Message::VolumeUp);
        }

        KeyEvent::Pressed(Key::VolumeDown) => {
            info!("hardware_buttons: volume down pressed");
            message = Some(Message::VolumeDown);
        }
        KeyEvent::Pressing(Key::VolumeDown) => {
            info!("hardware_buttons: volume down pressing");
            message = Some(Message::VolumeDown);
        }
        KeyEvent::Released(Key::VolumeDown) => {
            info!("hardware_buttons: volume down released");
        }
        KeyEvent::Unknown(Key::VolumeDown) => {
            info!("hardware_buttons: volume down unknown event");
            message = Some(Message::VolumeDown);
        }

        KeyEvent::Pressed(Key::ExtensionDetection) => {
            println!("HARDWARE EXTENTION DETCTION EVENT:PRESSED");
            list_devices();
            info!("hardware_buttons: extension detection pressed");
        }
        KeyEvent::Pressing(Key::ExtensionDetection) => {
            println!("HARDWARE EXTENTION DETCTION EVENT:PREssing");
            info!("hardware_buttons: extension detection pressing");
        }
        KeyEvent::Released(Key::ExtensionDetection) => {
            println!("HARDWARE EXTENTION DETCTION EVENT:released");
            info!("hardware_buttons: extension detection released");
        }
        KeyEvent::Unknown(Key::ExtensionDetection) => {
            info!("hardware_buttons: extension detection unknown event");
        }

        KeyEvent::Pressed(Key::Unknown) => {
            info!("hardware_buttons: unknown key pressed");
        }
        KeyEvent::Pressing(Key::Unknown) => {
            info!("hardware_buttons: unknown key pressing");
        }
        KeyEvent::Released(Key::Unknown) => {
            info!("hardware_buttons: unknown key released");
        }
        KeyEvent::Unknown(Key::Unknown) => {
            info!("hardware_buttons: unknown key unknown event");
        }
    }

    message
}

fn list_devices() -> Result<()> {
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
        }
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
        );

        for n in 0..device_desc.num_configurations() {
            let config_desc = match device.config_descriptor(n) {
                Ok(c) => c,
                Err(_) => continue,
            };
        }
    }

    Ok(())
}

pub fn extension_from_vid_pid(
    vendor_id: u16,
    product_id: u16,
) -> Option<Extension> {
    match (vendor_id, product_id) {
        (0x1234, 0x0001) => Some(Extension::MechaGamepad),
        (0x1234, 0x0002) => Some(Extension::MechaKeyboard),
        (0x1234, 0x0003) => Some(Extension::MechaGpio),
        _ => Some(Extension::MechaGpio),
    }
}