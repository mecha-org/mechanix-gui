use super::wireless_model::WirelessModel;
use crate::{
    components::{detail_row, single_detail_row, DetailRow},
    gui::{Message, NetworkScreenRoutes, Routes},
    header_node,
    utils::truncate,
};

use mctk_core::widgets::{HDivider, Scrollable};
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
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;

#[derive(Debug)]
pub struct NetworkDetailsState {}

#[derive(Debug)]
pub struct UnknownNetworkDetails {
    mac: String,
}

impl UnknownNetworkDetails {
    pub fn new(mac: String) -> Self {
        Self { mac }
    }

    fn get_ip_address(&self) -> Option<String> {
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (interface, info) in &networks {
            if interface.starts_with("wl") {
                for network in info.ip_networks().iter() {
                    if network.addr.is_ipv4() {
                        return Some(network.addr.to_string());
                    }
                }
            }
        }
        None
    }
}

impl Component for UnknownNetworkDetails {
    fn init(&mut self) {
        WirelessModel::update();
    }

    fn view(&self) -> Option<Node> {
        let ip_address = if let Some(ip_address) = self.get_ip_address() {
            ip_address
        } else {
            "-".to_string()
        };
        let mut text_color = Color::WHITE;
        let connected_network_option = WirelessModel::get()
            .scan_result
            .get()
            .wireless_network
            .clone()
            .into_iter()
            .find(|network| network.mac == self.mac)
            .clone();
        let mut security = "-".to_string();
        let mut signal_strength = "-";
        let network = match connected_network_option {
            Some(connected_network_option) => {
                security = connected_network_option.flags.clone();
                if let Ok(signal_int) = connected_network_option.signal.parse::<i32>() {
                    if signal_int < 30_i32 {
                        signal_strength = "Weak";
                    } else if signal_int >= 30 && signal_int < 70 {
                        signal_strength = "Good";
                    } else if signal_int >= 70 {
                        signal_strength = "Excellent";
                    }
                };
                connected_network_option
            }
            None => WirelessInfoResponse {
                name: "-".to_string(),
                mac: "-".to_string(),
                flags: "-".to_string(),
                frequency: "-".to_string(),
                signal: "-".to_string(),
            },
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
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [0., 0., 0., 0.],
            ]
        );

        let mut scrollable_section = node!(
            Scrollable::new(size!(440, 340)),
            lay![
                size: [440, 340],
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

        let truncate_network_name = truncate(network.clone().name.clone(), 20);

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
            key: "Name".to_string(),
            value: truncate(network.clone().name.clone(), 15),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Status".to_string(),
            value: "Not Connected".to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ))
        .push(single_detail_row(DetailRow {
            key: "Frequency".to_string(),
            value: if network.frequency.starts_with("2") {
                "2.4 GHz"
            } else {
                "5 GHz"
            }
            .to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ))
        .push(single_detail_row(DetailRow {
            key: "Signal".to_string(),
            value: signal_strength.to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ))
        .push(single_detail_row(DetailRow {
            key: "MAC Address".to_string(),
            value: network.mac.to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ))
        .push(single_detail_row(DetailRow {
            key: "Security".to_string(),
            value: security.to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));
        scrollable_section = scrollable_section.push(rows_node);

        base = base.push(header_node!(
            truncate_network_name.clone(),
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::Networking
                    }
                })
            })
        ));

        content_node = content_node.push(scrollable_section);
        base = base.push(content_node);

        Some(base)
    }
}
