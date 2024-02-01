use iced::widget::container;
use iced::{Background, Color};
use iced_style::container::Appearance;
use iced_style::Theme;

pub struct CustomContainer {
    pub(crate) appearance: Appearance,
}

impl CustomContainer {
    pub fn new(params: Appearance) -> Self {
        Self { appearance: params }
    }
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Some(Background::Color(iced::Color::TRANSPARENT)),
                border_radius: 4.0.into(),
                border_width: 12.0,
                border_color: Color::TRANSPARENT.into(),
            },
            _ => self.appearance,
        }
    }
}
