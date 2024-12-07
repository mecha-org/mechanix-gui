use std::hash::Hash;

use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::renderables::Image;
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

use mechanix_status_bar_components::types::WirelessStatus;
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;
use zbus::message;

use super::device_model::DeviceModel;

#[derive(Debug, Clone)]
pub struct AboutDeviceState {}

#[derive(Debug)]
pub struct AboutDevice {}

impl Component for AboutDevice {
    fn init(&mut self) {
        DeviceModel::update();
    }

    fn view(&self) -> Option<Node> {
        // TODO: dynamic

        let provisioned_status = *DeviceModel::get().is_provisioned.get();
        let device_icon = "device_icon".to_string(); // image
        let wifi_mac_address: String = "28:cd:c4:c2:e8:33".to_string();
        let ethernet_mac_address: String = "c0:3e:ba:3e:94:47".to_string();

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
            Text::new(txt!("About device"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 17.5)
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
                        route: Routes::SettingsList
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
        .push(node!(
            Div::new(),
            lay![
                size_pct: [20, Auto],
                axis_alignment: Alignment::End
            ]
        ));

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        // TODO: if provisioned_status true then show provision_device_details

        let provision_device_details = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size_pct: [100, 30],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                margin: [0., 0., 18., 0.]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [30, Auto]
                ]
            )
            .push(node!(
                widgets::Image::new(device_icon),
                lay![
                    size: [24, 24],
                    margin: [0, 5],
                    padding: [0., 0., 0., 15.]
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [60, Auto],
                    direction: Direction::Column,
                    // cross_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Text::new(txt!("Mecha Comet"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![
                    size_pct: [Auto, 50],
                    cross_alignment: Alignment::Center
                ]
            ))
            .push(node!(
                Text::new(txt!("213213dasda-2332324-a"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![
                    size_pct: [Auto, 50],
                    padding:[10., 0., 0., 0.]
                    // cross_alignment: Alignment::Center
                ]
            )),
        );

        let details_row_1 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Make"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!("Mecha"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Model"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!("Comet"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        let details_row_2 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Wi-Fi MAC Address"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(wifi_mac_address))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Ethernet MAC Address"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(ethernet_mac_address))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        let details_row_3 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("OS"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!("Linux"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Kernel"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!("Linux 6.1"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        let start_node = node!(
            Div::new(),
            lay![
                direction: Direction::Row,
                margin: [10.0, 0.0, 0.0, 0.0],
            ]
        );
        content_node = content_node.push(start_node);

        // content_node = content_node.push(selected_network_node);
        // content_node = content_node.push(node!(HDivider { size: 1. }, lay![
        //     margin: [0.0, 0.0, 30.0, 0.0],
        // ]));

        content_node = content_node.push(provision_device_details);

        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [0., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        // content_node = content_node.push(node!(
        //     HDivider { size: 1. },
        //     lay![
        //         margin: [10., 0., 10., 0.]
        //     ]
        // ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));

        base = base.push(header_node);
        base = base.push(content_node);
        Some(base)
    }
}
