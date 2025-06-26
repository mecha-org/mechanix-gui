use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{CoreSlider, hover::Hovering};
use bevy_styled_widgets::prelude::ThemeManager;

use crate::{
    settings_panel::{SettingsItem, SettingsItemText},
    utils::Icon,
};

use super::{StyledSlider, components::AccessibleName};

#[derive(Default)]
pub struct SliderBuilder {
    min: f32,
    max: f32,
    value: f32,
    root_color: Option<Color>,
    indicator_color: Option<Color>,
    icon: String,
    font: Option<Handle<Font>>,
    pub on_change: Option<SystemId<In<f32>>>,
}

impl SliderBuilder {
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn icon(mut self, icon: String) -> Self {
        self.icon = icon;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn build(self) -> impl Bundle {
        let max = if self.value.clone() > 10.0 {
            100.
        } else {
            10.0
        };
        let progress_value = (self.value / max) * 100.0;

        (
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(60.0),
                max_height: Val::Px(60.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                margin: UiRect::top(Val::Px(14.0)),
                padding: UiRect::left(Val::Px(16.)),
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
            BorderRadius::all(Val::Px(12.0)),
            SettingsItem,
            Name::new("Slider"),
            AccessibleName("Slider".to_string()),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Default),
            StyledSlider {
                min: self.min,
                max: self.max,
                value: self.value.clone(),
                root_color: self.root_color.clone(),
                indicator_color: self.indicator_color.clone(),
                on_change: self.on_change,
            },
            CoreSlider {
                min: self.min,
                max: self.max,
                value: self.value,
                on_change: self.on_change,
                thumb_size: 0.0,
                ..default()
            },
            TabIndex(0),
            children![
                (
                    Node {
                        display: Display::Flex,
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        height: Val::Percent(100.0),
                        width: Val::Percent(progress_value), // Indicator width based on value
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgba(1., 1., 1., 0.8)),
                    BorderRadius {
                        top_left: Val::Px(12.),
                        top_right: Val::Auto,
                        bottom_left: Val::Px(12.),
                        bottom_right: Val::Auto
                    },
                    // ProgressIndicator
                ),
                (
                    Text::new(self.icon.clone()),
                    TextFont {
                        font: self.font.unwrap_or_default(),
                        ..Default::default()
                    },
                    TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
                    SettingsItemText { font_size: 32. },
                ),
            ],
        )
    }
}
