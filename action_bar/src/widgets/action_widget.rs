use iced::{Alignment, Background, BorderRadius, Color, Command, Element, Renderer};
use iced::widget::{Column, container, image, mouse_area, Row, Text, vertical_space};
use iced_style::container::Appearance;
use serde::{Deserialize, Serialize};

use crate::widgets::styled_container::StyledContainer;

#[derive(Clone, Debug)]
pub enum Message {
    ValueChanged(Option<i8>),
    IconChanged(Option<String>),
    TitleChanged(String),
    WidgetClicked,
}


#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PercentageWidget {
    pub title: String,
    pub icon: Option<String>,
    pub value: Option<i8>,
    pub value_subscript: String,
}

impl PercentageWidget {
    pub fn new(title: String, icon: Option<String>, value: Option<i8>, value_subscript: String) -> Self {
        return Self {
            title,
            icon,
            value,
            value_subscript,
        };
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ValueChanged(state) => { self.value = state; }
            Message::IconChanged(state) => { self.icon = state; }
            Message::TitleChanged(state) => { self.title = state; }
            Message::WidgetClicked => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {
        let mut value_text =
            Row::new();
        ;

        if self.value.is_some() && self.value.unwrap().to_string().len() > 0 {
            value_text = value_text.push(Text::new(self.value.unwrap().to_string()).size(18))
                .spacing(7);
        } else {
            value_text = value_text.push(vertical_space(22));
        }

        if self.value_subscript.to_string().len() > 0 {
            value_text = value_text.push(Text::new(self.value_subscript.to_string()).size(10)).align_items(Alignment::End);
        }

        let title_text = Text::new(self.title.to_owned()).size(11);

        let mut items = Column::new();

        match self.icon.to_owned() {
            Some(icon) => items = items.push(image(icon)),
            None => (),
        };

        items = items
            .push(vertical_space(10))
            .push(value_text)
            .push(vertical_space(2))
            .push(title_text);

        mouse_area(container((items))
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
                background: Option::from(Background::Color(Color::from_rgba8(21, 23, 29, 1.0))),
                border_radius: BorderRadius::from(8.0),
                ..Default::default()
            }))))
            .padding(10)
            .width(100)
            .height(100))
            .on_press(Message::WidgetClicked)
            .into()
    }
}


