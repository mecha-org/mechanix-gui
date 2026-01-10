use dispatcher::Message;
use hw_buttons::{Key, KeyEvent};
use log::info;

pub fn build_message_for_event(event: KeyEvent) -> Option<Message> {
    info!(target: "hw-buttons", "received hardware button event: {event:?}");

    let mut message: Option<Message> = None;

    match event {
        KeyEvent::Pressed(Key::Power)
        | KeyEvent::Pressing(Key::Power)
        | KeyEvent::Unknown(Key::Power) => {
            info!(target: "hw-buttons", "power button pressed/pressing/unknown");
            message = Some(Message::ShowPowerOptions(true));
        }
        KeyEvent::Released(key) => {
            info!(target: "hw-buttons", "released: {key:?}");
        }
        KeyEvent::Pressed(key) => {
            info!(target: "hw-buttons", "pressed: {key:?}");
        }
        KeyEvent::Pressing(key) => {
            info!(target: "hw-buttons", "pressing: {key:?}");
        }
        KeyEvent::Unknown(key) => {
            info!(target: "hw-buttons", "unknown event for key: {key:?}");
        }
    }

    message
}
