use super::component::NetworkRowComponent;
use super::networking::ClicableIconComponent;
use super::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
use mctk_core::widgets::Scrollable;
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

enum NetworkingMessage {
    handleClickOnMore,
    handleClickOnBack,
}

#[derive(Debug)]
pub struct NetworkSettingsState {
    // pub loading: bool,
    // list
}

#[derive(Debug)]
// #[component(State = "NetworkSettingsState")]
pub struct NetworkSettings {}

impl Component for NetworkSettings {
    fn view(&self) -> Option<Node> {
        let mut text_color = Color::WHITE;

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
            Text::new(txt!("Saved Networks"))
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
        );

        // let mut content_node = node!(
        //     Div::new(),
        //     lay![
        //         size_pct: [100, 90],
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // );

        let mut scrollable_section = node!(
            Scrollable::new(),
            lay![
                size: [440, 400],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        for (i, network) in WirelessModel::get()
            .known_networks
            .get()
            .known_network
            .clone()
            .into_iter()
            .enumerate()
        {
            let mut icon = if network.flags.contains("WPA") {
                "secured_wireless_strong".to_string()
            } else {
                "wireless_strong".to_string()
            };
            let row = node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                    // padding: [5., 0., 12., 0.],
                ],
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || {
                        WirelessModel::select_network(network.network_id.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    }))
                })
                .push(node!(
                    widgets::Image::new(icon),
                    lay![
                        size: [24, 24],
                        margin:[0., 0., 0., 20.],
                    ]
                ))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(node!(
                        Text::new(txt!(network.ssid.clone()))
                            .style("color", Color::WHITE)
                            .style("size", 18.0)
                            .style("line_height", 20.0)
                            .style("font", "Space Grotesk")
                            .style("font_weight", FontWeight::Normal),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    )), // .push(node!(
                        //     // mini status
                        //     Text::new(txt!("Saved"))
                        //         .style("color", Color::WHITE)
                        //         .style("size", 14.0)
                        //         .style("line_height", 18.)
                        //         .style("font", "Space Grotesk")
                        //         .style("font_weight", FontWeight::Normal),
                        //     lay![
                        //         direction: Direction::Row,
                        //         axis_alignment: Alignment::Start,
                        //         cross_alignment: Alignment::Center,
                        //     ]
                        // )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("delete_icon")
                        .on_click(Box::new(move || {
                            WirelessModel::forget_saved_network(network.ssid.clone());
                            msg!(())
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
                    ]
                )),
            )
            .key(i as u64);

            let row = node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Column,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                    // padding: [5., 0., 12., 0.],
                ],
            )
            .push(row)
            .push(node!(HDivider { size: 1.0 }))
            .key(2 * i as u64);

            scrollable_section = scrollable_section.push(row);
        }
        base = base.push(header_node);
        base = base.push(scrollable_section);

        Some(base)
    }
}
