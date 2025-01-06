use std::fmt::Debug;

use crate::gui::{Message, NetworkScreenRoutes, Routes};
use crate::header_node;
use crate::utils::truncate;

use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_core::widgets::{HDivider, TextBox, VDivider};
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
    form: &'static Form,
}

#[derive(Debug)]
#[component(State = "NetworkScreenState")]
pub struct AddNetwork {
    // form: &'static Form,
    pub ssid: String,
}

impl AddNetwork {
    pub fn new(ssid: String) -> Self {
        Self {
            state: Some(NetworkScreenState { form: &FORM }),
            dirty: false,
            ssid,
        }
    }
}

impl Component for AddNetwork {
    fn init(&mut self) {
        FORM.ssid.set(self.ssid.clone());
        FORM.password.set("".to_string());
    }

    fn view(&self) -> Option<Node> {
        let network_name: String = self.ssid.clone();

        let header_text = if network_name.clone().len() == 0 {
            "Add Network"
        } else {
            &truncate(network_name.clone(), 20).to_string()
        };

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let name_row_node = node!(
            Div::new(),
            lay![
                size: [440, 68],
                direction: Direction::Row,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [38, 100],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                Text::new(txt!("Name"))
                    .style("color", Color::rgba(255., 255., 255., 1.))
                    .style("font", "Inter")
                    .with_class("text-xl leading-6 font-medium"),
                lay![
                    margin: [0.0, 10.0, 0.0, 0.0],
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [2, 100],
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                VDivider {
                    size: 0.8,
                    color: Color::rgba(83., 83., 83., 1.),
                },
                lay![
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [60, 100],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                TextBox::new(Some("".to_string()))
                    .style("background_color", Color::TRANSPARENT)
                    .style("font_size", 20.)
                    .style("text_color", Color::WHITE)
                    .style("border_color", Color::TRANSPARENT)
                    .style("cursor_color", Color::WHITE)
                    .style("placeholder_color", Color::rgb(107., 107., 107.))
                    .on_change(Box::new(|s| {
                        FORM.ssid.set(s.to_string());
                        msg!(())
                    }))
                    .placeholder("Enter SSID"),
                lay![
                    axis_alignment: Alignment::End,
                ]
            )),
        );

        let password_row_node = node!(
            Div::new(),
            lay![
                size: [440, 68],
                direction: Direction::Row,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [38, 100],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                Text::new(txt!("Password"))
                    .style("color", Color::rgba(255., 255., 255., 1.))
                    .style("font", "Inter")
                    .with_class("text-xl leading-6 font-medium"),
                lay![
                    margin: [0.0, 10.0, 0.0, 0.0],
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [2, 100],
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                mctk_core::widgets::VDivider {
                    size: 0.8,
                    color: Color::rgba(83., 83., 83., 1.),
                },
                lay![
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [60, 100],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
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
                    axis_alignment: Alignment::End,
                ]
            )),
        );

        let confirm_icon =
            if FORM.password.get().clone().len() == 0 || FORM.ssid.get().clone().len() == 0 {
                "disable_confirm_icon"
            } else {
                "enable_confirm_icon"
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
            confirm_icon,
            IconType::Svg,
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

        if self.ssid.clone().len() == 0 {
            content_node = content_node.push(name_row_node);
            content_node = content_node.push(node!(HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            }));
        }

        content_node = content_node.push(password_row_node);
        content_node = content_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        base = base.push(content_node);
        Some(base)
    }
}
