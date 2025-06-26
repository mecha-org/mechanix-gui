use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{CoreButton, hover::Hovering};

use crate::settings_panel::{SettingsItem, SettingsItemText};

use super::{
    ButtonSize, StyledButtonText,
    components::{AccessibleName, ButtonVariant, StyledButton},
};

#[derive(Default)]
pub struct ButtonBuilder {
    variant: ButtonVariant,
    on_click: Option<SystemId>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    hover_background_color: Option<Color>,
    hover_border_color: Option<Color>,
    text_color: Option<Color>,
    text: Option<String>,
    icon: Option<String>,
    size: Option<ButtonSize>,
    disabled: bool,
    font: Option<Handle<Font>>,
    width: Option<Val>,
    height: Option<Val>,
    border_radius: Option<f32>,
    font_size: Option<f32>,
}

impl ButtonBuilder {
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn on_click(mut self, system_id: SystemId) -> Self {
        self.on_click = Some(system_id);
        self
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn hover_background_color(mut self, color: Color) -> Self {
        self.hover_background_color = Some(color);
        self
    }

    pub fn hover_border_color(mut self, color: Color) -> Self {
        self.hover_border_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    pub fn border_radius(mut self, border_radius: f32) -> Self {
        self.border_radius = Some(border_radius);
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(60.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
            BorderRadius::all(Val::Px(12.0)),
            SettingsItem,
            Name::new("Button"),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            StyledButton {
                text: self.text.clone(),
                icon: self.icon.clone(),
                font: self.font.clone(),
                variant: self.variant,
                size: self.size,
                on_click: self.on_click,
                background_color: self.background_color,
                border_color: self.border_color,
                hover_background_color: self.hover_background_color,
                hover_border_color: self.hover_border_color,
                text_color: self.text_color,
                disabled: self.disabled,
                width: self.width,
                height: self.height,
                border_radius: self.border_radius,
            },
            CoreButton {
                on_click: self.on_click,
            },
            AccessibleName(self.text.clone().unwrap_or_else(|| "Button".to_string())),
            TabIndex(0),
            Children::spawn(Spawn((
                Text::new(self.icon.unwrap_or_default()),
                TextFont {
                    font: self.font.unwrap_or_default(),
                    ..Default::default()
                },
                TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
                StyledButtonText,
                SettingsItemText {
                    font_size: self.font_size.unwrap_or(32.0),
                },
            ))),
        )
    }
}
