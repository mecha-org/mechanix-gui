use std::fmt::Debug;

use crate::gui::{Message, NetworkScreenRoutes, Routes};
use crate::header_node;
use crate::utils::truncate;

use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_core::widgets::{HDivider, TextBox};
use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconButton, IconType, Text},
    Color, Node,
};
use mctk_macros::component;

use super::wireless_model::WirelessModel;

lazy_static! {
    static ref FORM: Form = Form {
        ssid: Context::new("".to_string()),
        password: Context::new("".to_string()),
    };
}

struct Form {
    pub ssid: Context<String>,
    pub password: Context<String>,
}

impl Debug for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("ssid", &self.ssid.get())
            .field("password", &self.password.get())
            .finish()
    }
}

#[derive(Debug)]
pub struct NetworkScreenState {
    pub name: String,
}

#[derive(Debug)]
#[component(State = "NetworkScreenState")]
pub struct AddNetwork {
    form: &'static Form,
    name: String,
}

impl AddNetwork {
    pub fn new(name: String) -> Self {
        Self {
            state: Some(NetworkScreenState { name: name.clone() }),
            dirty: false,
            form: &FORM,
            name,
        }
    }
}

impl Component for AddNetwork {
    fn init(&mut self) {
        FORM.ssid.set(self.name.clone());
    }

    fn view(&self) -> Option<Node> {
        let network_name: String = self.state_ref().name.clone();

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let text_node = node!(
            Text::new(txt!("Add Network"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
            ]
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [10., 0., 0., 0.],
                padding: [0., 8., 0., 8.]
            ]
        );

        let name_input_text = node!(
            Div::new(),
            lay![
                size: [Auto, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 0., 0., 0.],
            ]
        )
        .push(node!(
            Text::new(txt!("Name (SSID)"))
                .with_class("text-l leading-6 font-space-grotesk font-normal")
                .style("color", Color::rgb(197., 197., 197.)),
            lay![
                padding: [5., 0., 0., 0.],
            ]
        ));

        let name_input_value = if FORM.ssid.get().clone().len() == 0 {
            node!(
                TextBox::new(Some("".to_string()))
                    .style("background_color", Color::TRANSPARENT)
                    .style("font_size", 20.)
                    .style("text_color", Color::WHITE)
                    .style("border_color", Color::TRANSPARENT)
                    .style("cursor_color", Color::WHITE)
                    .style("placeholder_color", Color::rgb(107., 107., 107.))
                    .on_change(Box::new(|s| {
                        FORM.password.set(s.to_string());
                        msg!(())
                    }))
                    .placeholder("Enter name"),
                lay![
                    size_pct: [100, 8],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch
                ]
            )
        } else {
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [100, 8],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::End
                ]
            )
            .push(node!(Text::new(txt!(network_name.clone()))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 20.0)
                .style("line_height", 22.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),))
        };

        let password_input_text = node!(
            Div::new(),
            lay![
                size: [Auto, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 0., 0., 0.],
            ]
        )
        .push(node!(
            Text::new(txt!("Password"))
                .with_class("text-l leading-6 font-space-grotesk font-normal")
                .style("color", Color::rgb(197., 197., 197.)),
            lay![
                padding: [5., 0., 0., 0.],
            ]
        ));

        let password_input_value = node!(
            TextBox::new(Some("".to_string()))
                .style("background_color", Color::TRANSPARENT)
                .style("font_size", 20.)
                .style("text_color", Color::WHITE)
                .style("border_color", Color::TRANSPARENT)
                .style("cursor_color", Color::WHITE)
                .style("placeholder_color", Color::rgb(107., 107., 107.))
                .on_change(Box::new(|s| {
                    FORM.password.set(s.to_string());
                    msg!(())
                }))
                .placeholder("Enter password"),
            lay![
                size_pct: [100, 8],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch
            ]
        );

        content_node = content_node.push(name_input_text);
        content_node = content_node.push(name_input_value);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [2.0, 0.0, 25.0, 0.0],
            ]
        ));

        content_node = content_node.push(password_input_text);
        content_node = content_node.push(password_input_value);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [2.0, 0.0, 10.0, 0.0],
            ]
        ));

        let header_text = if FORM.ssid.get().clone().len() == 0 {
            "Add Network"
        } else {
            &truncate(network_name.clone(), 20).to_string()
        };

        base = base.push(header_node!(
            header_text,
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::Networking
                    }
                })
            }),
            "confirm_icon",
            Box::new(|| {
                if !FORM.password.get().clone().is_empty() {
                    WirelessModel::connect_to_network(
                        FORM.ssid.get().clone(),
                        FORM.password.get().clone(),
                    );
                    return msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::Networking
                        }
                    });
                } else {
                    println!("HANDLE PWD VALIDATION!");
                }
                Box::new(())
            })
        ));

        base = base.push(content_node);
        Some(base)
    }
}
