use dispatcher::Message;
use hw_buttons::{Key, KeyEvent};
use log::info;

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
            info!("hardware_buttons: extension detection pressed");
        }
        KeyEvent::Pressing(Key::ExtensionDetection) => {
            info!("hardware_buttons: extension detection pressing");
        }
        KeyEvent::Released(Key::ExtensionDetection) => {
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
