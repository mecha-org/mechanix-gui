use iced::{Alignment, Background, BorderRadius, Color, Command, Element, Font, Renderer};
use iced::font::Weight;
use iced::widget::{Column, container, image, mouse_area, Row, Text, vertical_space};
use iced_style::container::Appearance;
use serde::{Deserialize, Serialize};

use crate::widgets::custom_container::CustomContainer;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked(String),
}


#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PasswordKey {
    pub key: String
}

impl PasswordKey {
    pub fn new(key: String) -> Self {
        return Self {
            key
        };
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        // match message {
        //     Message::WidgetClicked(c) => {}
        // }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {

        let key_text = Text::new(self.key.to_owned())
            .size(30).font(Font{
            weight: Weight::Bold,
            ..Default::default()
        });

        // let items = Column::new()
        //     .push(key_text);

        mouse_area(container((key_text))
            .style(iced::theme::Container::Custom(Box::new(CustomContainer::new(Appearance {
                background: Option::from(Background::Color(Color::from_rgba8(32, 36, 49, 0.5))),
                border_radius: BorderRadius::from(16.0),
                border_width: 2.0,
                border_color: Color::from_rgba8(32, 36, 49, 1.0),
                ..Default::default()
            }))))
            .center_x()
            .center_y()
            .width(90)
            .height(90))
            .on_press(Message::WidgetClicked(self.key.to_owned()))
            .into()
    }
}


