use std::hash::Hash;

use crate::gui::{Message, Routes};
use crate::screens::network::wireless_model::WirelessModel;
use crate::{components::*, header_node};

use mctk_core::prelude::ComponentHasher;
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

    // fn render_hash(&self, hasher: &mut ComponentHasher) {
    //     DeviceModel::get().is_provisioned.get().hash(hasher);
    //     DeviceModel::get().provision_id.get().hash(hasher);
    //     DeviceModel::get().provision_name.get().hash(hasher);
    //     DeviceModel::get().provision_icon_url.get().hash(hasher);
    // }

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

        // let provisioned_status = DeviceModel::get().is_provisioned.get().clone();
        // let provision_machine_id = DeviceModel::get().provision_id.get().clone();
        // let provision_machine_icon_url = DeviceModel::get().provision_icon_url.get().clone();

        let provision_machine_name = DeviceModel::get().provision_name.get().clone();
        let ip_adderss = DeviceModel::get().ip_address.get().clone();

        let wireless_mac_address: String = WirelessModel::get().wireless_mac_address.get().clone();
        let ethernet_mac_address: String = WirelessModel::get().ethernet_mac_address.get().clone();

        let wireless_ip_address: String = DeviceModel::get().wireless_ip_address.get().clone();
        let ethernet_ip_address: String = DeviceModel::get().ethernet_ip_address.get().clone();

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
            Scrollable::new(size!(440, 280)),
            lay![
                size: [440, 280],
            ]
        );

        let mut content_node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                padding: [0.0, 8.0, 0.0, 8.0],
            ]
        );

        let provision_device_image = node!(
            Div::new(),
            lay![
                // size_pct: [100, 18],
                size_pct: [100, 100],
                direction: Direction::Row,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(node!(
            widgets::Image::new("device_icon"),
            lay![
                size: [60, 60],
            ]
        ));
        // let provision_device_details_row_1 = node!(
        //     Div::new(),
        //     lay![
        //     size_pct: [100, 8],
        //     direction: Direction::Row,
        //     axis_alignment: Alignment::Start,
        //     cross_alignment: Alignment::Center,
        //     ]
        // )
        // .push(node!(
        //     Text::new(txt!(provision_machine_name.clone()))
        //         .style("color", Color::WHITE)
        //         .style("size", 20.0)
        //         .style("line_height", 24.0)
        //         .style("font", "Space Grotesk")
        //         .style("font_weight", FontWeight::Normal),
        //     lay![]
        // ));

        // let provision_device_details_row_2 = node!(
        //     Div::new(),
        //     lay![
        //     size_pct: [100, 6],
        //     direction: Direction::Row,
        //     axis_alignment: Alignment::Start,
        //     cross_alignment: Alignment::Start,
        //     ]
        // )
        // .push(node!(
        //     Text::new(txt!(provision_machine_id.clone()))
        //         .style("color", Color::WHITE)
        //         .style("size", 16.0)
        //         .style("line_height", 18.0)
        //         .style("font", "Space Grotesk")
        //         .style("font_weight", FontWeight::Bold),
        //     lay![]
        // ));

        // let details_row_1 = detail_row(
        //     DetailRow {
        //         key: "Host name".to_uppercase(),
        //         value: format!("{}.local", os_info.hostname),
        //     },
        //     DetailRow {
        //         key: "IP Address".to_uppercase(),
        //         value: ip_adderss.to_string(),
        //     },
        // );

        let details_row_1 = detail_row(
            DetailRow {
                key: "Make".to_uppercase(),
                value: "Mecha".to_string(),
            },
            DetailRow {
                key: "Model".to_uppercase(),
                value: "Comet".to_string(),
            },
        );

        let details_row_2 = detail_row(
            DetailRow {
                key: "Wireless Ip Address".to_uppercase(),
                value: wireless_ip_address.to_string(),
            },
            DetailRow {
                key: "Ethernet Ip Address".to_uppercase(),
                value: ethernet_ip_address.to_string(),
            },
        );

        let details_row_3 = detail_row(
            DetailRow {
                key: "Wireless MAC Address".to_uppercase(),
                value: wireless_mac_address.to_string(),
            },
            DetailRow {
                key: "Ethernet MAC Address".to_uppercase(),
                value: ethernet_mac_address.to_string(),
            },
        );

        let details_row_4 = detail_row(
            DetailRow {
                key: "OS".to_uppercase(),
                value: os_info.name.to_string(),
            },
            DetailRow {
                key: "Kernel".to_uppercase(),
                value: os_info.version.to_string(),
            },
        );

        let mut machine_detail_node = node!(
            Div::new(),
            lay![
            size_pct: [100, 100],
            direction: Direction::Column,
            axis_alignment: Alignment::Start,
            cross_alignment: Alignment::Stretch
            ]
        )
        .push(node!(
            Text::new(txt!(provision_machine_name))
                .style("color", Color::WHITE)
                .style("size", 20.0)
                .style("line_height", 24.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, 50],
            ]
        ))
        .push(node!(
            Text::new(txt!(os_info.hostname.clone()))
                .style("color", Color::WHITE)
                .style("size", 18.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, 50],
            ]
        ));

        // if provision_machine_id.clone().len() > 0 {
        //     machine_detail_node = machine_detail_node.push(node!(
        //         Text::new(txt!(provision_machine_id))
        //             .style("color", Color::WHITE)
        //             .style("size", 14.0)
        //             .style("line_height", 18.)
        //             .style("font", "Space Grotesk")
        //             .style("font_weight", FontWeight::Normal),
        //         lay![
        //             size_pct: [100, 50],
        //         ]
        //     ));
        // }
        let start_node = node!(
            Div::new(),
            lay![
                size: [440, 60],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
                margin: [0., 0., 20., 0.],
                padding: [0.0, 8.0, 0.0, 8.0],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, 100],
                    axis_alignment: Alignment::Start,

                ]
            )
            .push(provision_device_image),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, 100],
                    axis_alignment: Alignment::Start,

                ]
            )
            .push(machine_detail_node),
        );

        // content_node = content_node.push(start_node);

        // content_node = content_node.push(provision_device_image);
        // content_node = content_node.push(provision_device_details_row_1);
        // if provision_machine_id.clone().len() > 0 {
        //     content_node = content_node.push(provision_device_details_row_2);
        // }

        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));

        // content_node = content_node.push(node!(
        //     HDivider {
        //         size: 0.8,
        //         color: Color::rgba(83., 83., 83., 1.)
        //     },
        //     lay![
        //         margin: [8., 0., 8., 0.]
        //     ]
        // ));
        content_node = content_node.push(node!(
            Div::new(),
            lay![
                size: [440, 15],
                direction: Direction::Row,
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.6,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.6,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        content_node = content_node.push(details_row_4);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));

        scrollable = scrollable.push(content_node);

        let content = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                size: [440, Auto],
                margin: [20., 0., 0., 0.],
                // padding: [0.0, 8.0, 0.0, 8.0],
            ]
        )
        .push(start_node)
        .push(scrollable);

        base = base.push(header_node!(
            "About Device",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::SettingsList
            }))
        ));
        base = base.push(content);

        Some(base)
    }
}
