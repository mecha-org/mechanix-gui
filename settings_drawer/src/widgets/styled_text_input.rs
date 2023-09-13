use iced::{Color};
use iced::widget::text_input;
use iced_style::core::{Background, BorderRadius};
use iced_style::text_input::Appearance;
use iced_style::Theme;

pub struct StyledTextInput {
    pub appearance: Appearance
}

impl StyledTextInput {
    pub fn new() -> Self {
        Self {
            appearance: Appearance{
                background: Background::from(Color::from_rgb8(21, 23, 29)),
                border_radius: BorderRadius::from([0.0, 8.0, 8.0, 0.0]),
                border_width: 0.0,
                border_color: Default::default(),
                icon_color: Default::default(),
            }
        }
    }
}

impl text_input::StyleSheet for StyledTextInput {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        self.appearance
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.appearance
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        Color::from_rgb8(86, 94, 118)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.appearance
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.appearance
    }
}