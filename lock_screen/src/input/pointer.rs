use iced::mouse::Button;

pub fn button_to_iced_button(button: u32) -> Option<Button> {
    match button {
        0x110 => Some(Button::Left),
        0x111 => Some(Button::Right),
        0x112 => Some(Button::Middle),
        _ => None,
    }
}
