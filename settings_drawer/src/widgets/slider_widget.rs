use iced::{Background, BorderRadius, Color, Command, Element, Length, Renderer};
use iced::widget::{Column, Container, image, Row, Slider, Text, vertical_space};
use iced_style::container::Appearance;
use serde::{Deserialize, Serialize};

use crate::widgets::styled_container::StyledContainer;

#[derive(Clone, Debug)]
pub enum Message {
    ValueChanged(f32),
    IconChanged(String),
    TitleChanged(String),
}


#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct SliderWidget {
    pub(crate) title: String,
    icon: String,
    value: f32,
}

impl SliderWidget {
    pub fn new(title: String, icon: String, value: f32) -> Self {
        return Self {
            title,
            icon,
            value,
        };
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ValueChanged(state) => { self.value = state; }
            Message::IconChanged(state) => { self.icon = state; }
            Message::TitleChanged(state) => { self.title = state; }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {
        let image_icon = image(self.icon.to_owned());
        let slider_ui =
            Row::new().push(Slider::new(0.0..=100.0, self.value, Message::ValueChanged))
                .spacing(7);

        let title_text = Text::new(self.title.to_owned()).size(11);

        let items = Column::new()
            .push(image_icon)
            .push(vertical_space(10))
            .push(slider_ui)
            .push(vertical_space(2))
            .push(title_text);

        Container::new(items)
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
                background: Option::from(Background::Color(Color::from_rgba8(21, 23, 29, 1.0))),
                border_radius: BorderRadius::from(8.0),
                ..Default::default()
            }))))
            .padding(10)
            .width(Length::Fixed(211.0))
            .height(100)
            .into()
    }
}


