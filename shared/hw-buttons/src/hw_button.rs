use evdev::{EventStream, EventType};

use crate::utils::{get_device_stream, Key, KeyEvent};

pub struct HwButton {
    pub event_stream: EventStream,
}

impl HwButton {
    pub fn new(path: String) -> Self {
        let event_stream = get_device_stream(path).unwrap();
        Self { event_stream }
    }

    pub async fn poll(&mut self) -> KeyEvent {
        loop {
            let event = self.event_stream.next_event().await.unwrap();
            match event.event_type() {
                EventType::KEY => {
                    let key = event.code();
                    let key: Key = evdev::Key(key).into();
                    if key == Key::Unknown {
                        continue;
                    }
                    return match event.value() {
                        0 => KeyEvent::Released(key),
                        1 => KeyEvent::Pressed(key),
                        2 => KeyEvent::Pressing(key),
                        _ => unreachable!(),
                    };
                }
                _ => {}
            }
        }
    }
}
