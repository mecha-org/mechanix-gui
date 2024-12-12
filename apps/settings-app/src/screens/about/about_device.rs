use std::hash::Hash;

use crate::screens::network::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{components::*, header_node, tab_item_node};
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
        WirelessModel::update_mac_addresses();
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        DeviceModel::get().is_provisioned.get().hash(hasher);
        DeviceModel::get().provision_id.get().hash(hasher);
        DeviceModel::get().provision_name.get().hash(hasher);
        DeviceModel::get().provision_icon_url.get().hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let os_info_option = DeviceModel::get().os_info.get().clone();
        let os_info = if let Some(os_info_option) = os_info_option {
            os_info_option
        } else {
            OSInfo {
                name: "-".to_string(),
                version: "-".to_string(),
                hostname: "-".to_string(),
            }
        };

        let provisioned_status = DeviceModel::get().is_provisioned.get().clone();
        let provision_machine_id = DeviceModel::get().provision_id.get().clone();
        let provision_machine_name = DeviceModel::get().provision_name.get().clone();
        let provision_machine_icon_url = DeviceModel::get().provision_icon_url.get().clone();
        let ip_adderss = DeviceModel::get().ip_address.get().clone();

        let wifi_mac_address: String = WirelessModel::get().wireless_mac_address.get().clone();
        let ethernet_mac_address: String = WirelessModel::get().ethernet_mac_address.get().clone();

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let mut scrollable = node!(
            Scrollable::new(size!(440, 380)),
            lay![
                size: [440, 380],
            ]
        );

        let mut content_node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let provision_device_image = node!(
            Div::new(),
            lay![
                size_pct: [100, 18],
                direction: Direction::Row,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Start,
            ]
        )
        .push(node!(
            widgets::Image::new("device_icon"),
            lay![
                size: [70, 70],
            ]
        ));
        let provision_device_details_row_1 = node!(
            Div::new(),
            lay![
            size_pct: [100, 6],
            direction: Direction::Row,
            axis_alignment: Alignment::Center,
            cross_alignment: Alignment::Center,
            ]
        )
        .push(node!(
            Text::new(txt!(provision_machine_name))
                .style("color", Color::WHITE)
                .style("size", 18.0)
                .style("line_height", 20.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![]
        ));

        let provision_device_details_row_2 = node!(
            Div::new(),
            lay![
            size_pct: [100, 6],
            direction: Direction::Row,
            axis_alignment: Alignment::Center,
            cross_alignment: Alignment::Start,
            ]
        )
        .push(node!(
            Text::new(txt!(provision_machine_id.clone()))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Medium),
            lay![]
        ));

        let details_row_1 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Start,
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
                Text::new(txt!("Host name".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(format!("{}.local", os_info.hostname)))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![
                    padding: [4.0, 0.0, 4.0, 0.0],

                ]
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
                Text::new(txt!("IP Address".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(ip_adderss))
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
                Text::new(txt!("Make".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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
                Text::new(txt!("Model".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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
                Text::new(txt!("Wi-Fi MAC Address".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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
                Text::new(txt!("Ethernet MAC Address".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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

        let details_row_4 = node!(
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
                Text::new(txt!("OS".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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
                Text::new(txt!("Kernel".to_uppercase()))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 14.0)
                    .style("line_height", 18.)
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
                margin: [2.0, 0.0, 0.0, 0.0],
            ]
        );

        // content_node = content_node.push(start_node);
        content_node = content_node.push(provision_device_image);
        content_node = content_node.push(provision_device_details_row_1);
        if provision_machine_id.clone().len() > 0 {
            content_node = content_node.push(provision_device_details_row_2);
        }

        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [15., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [10., 0., 20., 0.]
            ]
        ));

        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [20., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_4);
        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));

        scrollable = scrollable.push(content_node);

        base = base.push(header_node!(
            "About Device",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::SettingsList
            }))
        ));
        base = base.push(scrollable);

        Some(base)
    }
}
