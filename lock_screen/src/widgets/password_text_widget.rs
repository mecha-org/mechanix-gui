use iced::{Alignment, Background, BorderRadius, Color, Command, Element, Font, Renderer};
use iced::font::Weight;
use iced::widget::{Column, container, image, mouse_area, Row, Text, vertical_space};
use iced_style::container::Appearance;
use serde::{Deserialize, Serialize};

use crate::widgets::custom_container::CustomContainer;

#[derive(Clone, Debug)]
pub enum Message {
    ToggleFilled,
}


#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PasswordTextWidget {
    pub is_filled: bool
}

impl PasswordTextWidget {
    pub fn new(is_filled: bool) -> Self {
        return Self {
            is_filled
        };
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleFilled => {
                self.is_filled = !self.is_filled;
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {
        container(Text::new(" "))
            .style(iced::theme::Container::Custom(Box::new(CustomContainer::new(Appearance {
                background: match self.is_filled {
                    true => Option::from(Background::Color(Color::from_rgba8(255, 255, 255, 1.0))),
                    false => None
                },
                border_radius:  BorderRadius::from(50.0),
                border_width: match self.is_filled {
                true => 0.0,
                false => 2.0
                },
                border_color: Color::from_rgba8(86, 94, 118, 1.0),
                ..Default::default()
            }))))
            .center_x()
            .center_y()
            .width(18)
            .height(18)
            .into()
    }
}


