use iced::{Alignment, BorderRadius, Color, Command, Element, Length, Renderer};
use iced::widget::{Column, container, image, mouse_area, MouseArea, Row, Text, vertical_space};
use iced_style::container::Appearance;
use serde::{Deserialize, Serialize};

use crate::widgets::styled_container::StyledContainer;

#[derive(Clone, Debug)]
pub enum Message {
    AppCloseClicked(String),
    AppClicked(String),
}


#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct AppWidget {
    pub app_id: String,
    pub alias: String,
    pub name: String,
    pub instance_name: String,
    pub icon: String,
    pub close_button_icon: Option<String>,
}

impl AppWidget {
    pub fn new(app_id: String, alias: String, name: String, instance_name: String, icon: String, close_button_icon: Option<String>) -> Self {
        return Self {
            app_id,
            alias,
            name,
            instance_name,
            icon,
            close_button_icon
        };
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        // match message {
        //     Message::AppCloseClicked(state) => { self.value = state; }
        // }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {
        let image_icon = Row::new().push(
            container(image(self.icon.to_owned()).width(208))
                .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
                    border_radius: BorderRadius::from(11.0),
                    border_color: Color::from_rgb8(21, 23, 29),
                    border_width: 1.0,
                    ..Default::default()
                }))))
        ).padding([0, 8]);
        let mut app_name_row =
            Row::new()
                .align_items(Alignment::Center)
                .push(Text::new(self.name.to_string()).size(12).width(Length::FillPortion(1)))
                .padding([0, 0, 0, 8]);
        ;
        match self.close_button_icon.clone() {
            Some(icon) => app_name_row = app_name_row.push(MouseArea::new(image(icon)).on_release(Message::AppCloseClicked(self.app_id.to_owned()))),
            None => (),
        };

        let app_instance_name_text =
            Row::new()
                .push(Text::new(self.instance_name.to_string()).size(14).style(iced::theme::Text::Color(Color::from_rgb8(132, 141, 166))))
                .padding([0, 8]);
        ;
        ;

        let items = Column::new()
            .push(app_name_row)
            .push(vertical_space(5))
            .push(image_icon)
            .push(vertical_space(5))
            .push(app_instance_name_text)
            .push(vertical_space(10));

        mouse_area(container((items))
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
                border_radius: BorderRadius::from(5.0),
                border_color: Color::from_rgb8(21, 23, 29),
                border_width: 1.0,
                ..Default::default()
            }))))
            .width(208)
        )
            .on_press(Message::AppClicked(self.app_id.to_owned()))
            .into()
    }
}


