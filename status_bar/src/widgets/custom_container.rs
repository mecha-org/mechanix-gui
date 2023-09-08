use iced::Color;
use iced::widget::container;
use iced_style::container::Appearance;
use iced_style::Theme;

pub struct CustomContainer {
    appearance: Appearance,
}

impl CustomContainer {
    pub fn new(params: Appearance) -> Self {
        Self {
            appearance: params
        }
    }
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            Theme::Light => Appearance {
                text_color: None,
                background: Color::TRANSPARENT.into(),
                border_radius: 4.0,
                border_width: 2.0,
                border_color: Color::TRANSPARENT,
            },
            _ => self.appearance
        }
    }
}