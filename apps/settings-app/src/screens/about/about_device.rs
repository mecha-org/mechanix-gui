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

#[derive(Debug)]
pub struct AboutDevice {}

impl Component for AboutDevice {
    fn init(&mut self) {
        DeviceModel::update();
        WirelessModel::update_mac_addresses();
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

        let mut content_node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let mut scrollable_section = node!(
            Scrollable::new(size!(440, 280)),
            lay![
                size: [440, 280],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
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

        let start_node = node!(
            Div::new(),
            lay![
                size: [440, 80],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
                padding: [0., 10., 0., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, 100],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!(provision_machine_name.clone()))
                    .style("color", Color::WHITE)
                    .style("font", "Inter")
                    .with_class("text-2xl leading-7 font-medium"),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [1, 100],
                    cross_alignment: Alignment::Stretch
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
                    size_pct: [19, 100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )
            .push(node!(
                widgets::Image::new("device_icon"),
                lay![
                    size: [60, 60],
                ]
            )),
        );

        let rows_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
            ],
        )
        .push(single_detail_row(DetailRow {
            key: "Host name".to_string(),
            value: format!("{}.local ", os_info.hostname),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Make / Model".to_string(),
            value: "Mecha Comet".to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Wireless IP".to_string(),
            value: wireless_ip_address.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Ethernet IP".to_string(),
            value: ethernet_ip_address.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Wireless MAC".to_string(),
            value: wireless_mac_address.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Ethernet MAC".to_string(),
            value: ethernet_mac_address.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "OS".to_string(),
            value: os_info.name.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Kernel".to_string(),
            value: os_info.version.to_string(),
        }));

        base = base.push(header_node!(
            "About Device",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::SettingsList
            }))
        ));
        content_node = content_node.push(start_node);
        content_node = content_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        scrollable_section = scrollable_section.push(rows_node);
        content_node = content_node.push(scrollable_section);
        content_node = content_node.push(node!(HDivider {
            size: 1.,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        base = base.push(content_node);
        Some(base)
    }
}
