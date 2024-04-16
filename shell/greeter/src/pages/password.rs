use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{HorizontalPosition, Styled, VerticalPosition},
    txt,
    widgets::{Button, Div, IconButton, Svg, Text, TextBox, TextBoxVariant},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};
use smithay_client_toolkit::reexports::calloop::channel::Sender;

use crate::{
    gui::{Message, PasswordAuthMessage, PasswordAuthRoute, Routes},
    handlers::login::handler::LoginHandlerMessage,
    AppMessage, AuthSubmit, LoginHandlerEvents, Prompt,
};

#[derive(Debug)]
pub struct Username {}

impl Component for Username {
    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 0.45)),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::BackPressed)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
            .push(node!(
                IconButton::new("next_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::Submit)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        let label = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(Text::new(txt!("Username".to_string()))
            .style("color", Color::rgb(175., 175., 175.))
            .style("size", 23.0)));

        let input = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 76],
                cross_alignment: Alignment::Center,
                axis_alignment:Alignment::Stretch,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(
            TextBox::new(Some("".to_string()))
            .style("background_color", Color::TRANSPARENT)
            .style("font_size", 32.)
            .style("text_color", Color::WHITE)
            .style("border_width", 0.)
            .style("cursor_color", Color::WHITE)
            .style("placeholder_color", Color::rgb(111., 107., 107.))
                .placeholder("Type here")
                .on_change(Box::new(|s| msg!(PasswordAuthMessage::CaptchaChange(s.to_string()))))
                .on_commit(Box::new(|s| msg!(PasswordAuthMessage::CaptchaChange(s.to_string())))),
            [size: [Auto]]
        ));

        Some(
            node!(
                Div::new().bg(Color::rgba(0., 0., 0., 0.80)),
                lay![
                    size_pct: [100]
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    padding: [ 30, 0, 0, 0 ]
                ]
            )
            .push(label)
            .push(input)
            .push(footer),
        )
    }
}

#[derive(Debug)]
pub struct Password {
    pub default_value: String,
}

impl Component for Password {
    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 0.45)),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::BackPressed)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
            .push(node!(
                IconButton::new("submit_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::Submit)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        let label = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(Text::new(txt!("Password".to_string()))
            .style("color", Color::rgb(175., 175., 175.))
            .style("size", 23.0)));

        let input = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 76],
                cross_alignment: Alignment::Center,
                axis_alignment:Alignment::Stretch,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(
            TextBox::new(Some(self.default_value.clone()))
            .style("background_color", Color::TRANSPARENT)
            .style("font_size", 32.)
            .style("text_color", Color::WHITE)
            .style("border_width", 0.)
            .style("cursor_color", Color::WHITE)
            .style("placeholder_color", Color::rgb(111., 107., 107.))
            .placeholder("Type here")
            .variant(TextBoxVariant::Hidden)
            .show_icon("show_icon")
            .hide_icon("hide_icon")
                .on_change(Box::new(|s| msg!(PasswordAuthMessage::PasswordChange(s.to_string()))))
                .on_commit(Box::new(|s| msg!(PasswordAuthMessage::PasswordChange(s.to_string())))),
            [size: [Auto], padding: [0., 0., 0., 20.]]
        ));

        Some(
            node!(
                Div::new().bg(Color::rgba(0., 0., 0., 0.80)),
                lay![
                    size_pct: [100]
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    padding: [ 30, 0, 0, 0 ]
                ]
            )
            .push(label)
            .push(input)
            .push(footer),
        )
    }
}

#[derive(Debug)]
pub struct Captcha {
    pub message: String,
    pub error_message: Option<String>,
    pub default_value: String,
}

impl Component for Captcha {
    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 0.45)),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::BackPressed)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
            .push(node!(
                IconButton::new("next_icon")
                    .on_click(Box::new(|| msg!(PasswordAuthMessage::Submit)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        let label = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(Text::new(txt!(self.message.clone()))
            .style("color", Color::rgb(175., 175., 175.))
            .style("size", 23.0)));

        let input = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [Auto, 76],
                cross_alignment: Alignment::Center,
                axis_alignment:Alignment::Stretch,
                padding: [0., 20., 0., 0.]
            ]
        )
        .push(node!(
            TextBox::new(Some(self.default_value.clone()))
            .style("background_color", Color::TRANSPARENT)
            .style("font_size", 32.)
            .style("text_color", Color::WHITE)
            .style("border_width", 0.)
            .style("cursor_color", Color::WHITE)
            .style("placeholder_color", Color::rgb(111., 107., 107.))
                .placeholder("Type here")
                .on_change(Box::new(|s| msg!(PasswordAuthMessage::CaptchaChange(s.to_string()))))
                .on_commit(Box::new(|s| msg!(PasswordAuthMessage::CaptchaChange(s.to_string())))),
            [size: [Auto]]
        ));

        let error_node = node!(
            Text::new(txt!(self.error_message.clone().unwrap_or_default()))
                .style("color", Color::RED)
                .style("size", 22.0)
                .style("h_alignment", HorizontalPosition::Left)
                .style("v_alignment", VerticalPosition::Center)
        );

        Some(
            node!(
                Div::new().bg(Color::rgba(0., 0., 0., 0.80)),
                lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                ]
            )
            .push(label)
            .push(input)
            .push(error_node)
            .push(footer),
        )
    }
}
