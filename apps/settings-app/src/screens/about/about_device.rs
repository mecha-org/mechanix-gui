use std::hash::Hash;

use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::prelude::ComponentHasher;
use mctk_core::renderables::Image;
use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text},
    Color, Node,
};

use super::device_model::{DeviceModel, OSInfo};

#[derive(Debug, Clone)]
pub struct AboutDeviceState {}

#[derive(Debug)]
pub struct AboutDevice {}

impl Component for AboutDevice {
    fn init(&mut self) {
        DeviceModel::update();
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        DeviceModel::get().is_provisioned.get().hash(hasher);
        DeviceModel::get().provision_id.get().hash(hasher);
        // DeviceModel::get().provision_name.get().hash(hasher);
        // DeviceModel::get().provision_icon_url.get().hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let os_info_option = DeviceModel::get().os_info.get().clone();
        let os_info = if let Some(os_info_option) = os_info_option {
            os_info_option
        } else {
            OSInfo {
                name: "-".to_string(),
                version: "-".to_string(),
            }
        };

        let provisioned_status = DeviceModel::get().is_provisioned.get().clone();
        let provision_machine_id = DeviceModel::get().provision_id.get().clone();
        let provision_machine_name = DeviceModel::get().provision_name.get().clone();
        let provision_machine_icon_url = DeviceModel::get().provision_icon_url.get().clone();

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
            ), // .push(node!(    // TODO : Img
               //     widgets::Image::new(device_icon),
               //     lay![
               //         size: [24, 24],
               //         margin: [0, 5],
               //         padding: [0., 0., 0., 15.]
               //     ]
               // )),
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
                Text::new(txt!(provision_machine_name)) // name
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
                Text::new(txt!(provision_machine_id))
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
                Text::new(txt!(os_info.name))
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
                Text::new(txt!(os_info.version))
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

        if provisioned_status.clone() == true {
            content_node = content_node.push(provision_device_details);
        }

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
