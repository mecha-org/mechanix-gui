use dispatcher::Message;
use hw_buttons::{Key, KeyEvent};

pub fn build_message_for_event(event: KeyEvent) -> Option<Message> {
    println!("hardware_buttons: received hardware button event: {event:?}");

    let mut message: Option<Message> = None;

    match event {
        KeyEvent::Pressed(Key::Power) => {
            println!("hardware_buttons: power button pressed");
            message = Some(Message::ShowPowerOptions(true));
        }
        KeyEvent::Pressing(Key::Power) => {
            println!("hardware_buttons: power button pressing");
            message = Some(Message::ShowPowerOptions(true));
        }
        KeyEvent::Unknown(Key::Power) => {
            println!("hardware_buttons: power button unknown event");
            message = Some(Message::ShowPowerOptions(true));
        }

        KeyEvent::Released(Key::Power) => {
            println!("hardware_buttons: power button released");
        }
        
        KeyEvent::Pressed(Key::Home) => {
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
        }
        KeyEvent::Pressing(Key::VolumeUp) => {
            println!("hardware_buttons: volume up pressing");
        }
        KeyEvent::Released(Key::VolumeUp) => {
            println!("hardware_buttons: volume up released");
        }
        KeyEvent::Unknown(Key::VolumeUp) => {
            println!("hardware_buttons: volume up unknown event");
        }

        KeyEvent::Pressed(Key::VolumeDown) => {
            println!("hardware_buttons: volume down pressed");
        }
        KeyEvent::Pressing(Key::VolumeDown) => {
            println!("hardware_buttons: volume down pressing");
        }
        KeyEvent::Released(Key::VolumeDown) => {
            println!("hardware_buttons: volume down released");
        }
        KeyEvent::Unknown(Key::VolumeDown) => {
            println!("hardware_buttons: volume down unknown event");
        }
        _ => {}
    }

    message
}
