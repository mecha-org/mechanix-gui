use std::fmt::Debug;

use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
use mctk_core::widgets::TextBox;
use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text, Toggle},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

use mechanix_system_dbus_client::wireless::WirelessInfoResponse;
use zbus::message;

use super::wireless_model::WirelessModel;

lazy_static! {
    static ref FORM: Form = Form {
        ssid: Context::new("".to_string()),
        password: Context::new("".to_string()),
    };
}

enum NetworkingMessage {}

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

        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
                // position_type: Absolute,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::Networking
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End
                ]
            )
            .push(node!(
                IconButton::new("tick_icon")
                    .on_click(Box::new(|| {
                        WirelessModel::connect_to_network(
                            FORM.ssid.get().clone(),
                            FORM.password.get().clone(),
                        );
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    }))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                    padding: [0., 0., 0., 2.]
                ]
            )),
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        // Add 'Network name'
        let name_input_text = node!(
            Text::new(txt!("Name (SSID)"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [25.0, 0.0, 10.0, 0.0],
            ]
        );

        let name_input_value = node!(
            TextBox::new(Some(network_name))
                .style("background_color", Color::TRANSPARENT)
                .style("font_size", 20.)
                .style("text_color", Color::WHITE)
                .style("border_color", Color::TRANSPARENT)
                // .style("border_width", 0.)
                .style("cursor_color", Color::WHITE)
                .style("placeholder_color", Color::rgb(107., 107., 107.))
                .on_change(Box::new(|s| {
                    FORM.ssid.set(s.to_string());
                    msg!(())
                }))
                .placeholder("Enter Name"),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch
            ]
        );

        let password_input_text = node!(
            Text::new(txt!("Password"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [25.0, 0.0, 10.0, 0.0],
            ]
        );

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
                .placeholder("Enter Password"),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch
            ]
        );

        content_node = content_node.push(name_input_text);
        content_node = content_node.push(name_input_value);
        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [0.0, 0.0, 10.0, 0.0],
            ]
        ));

        content_node = content_node.push(password_input_text);
        // content_node = content_node.push(node!(HDivider { size: 1. }));
        content_node = content_node.push(password_input_value);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        base = base.push(header_node);
        base = base.push(content_node);
        Some(base)
    }
}
