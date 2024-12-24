use super::wireless_model::WirelessModel;
use crate::{
    components::{detail_row, DetailRow},
    gui::{Message, NetworkScreenRoutes, Routes},
    header_node,
    utils::truncate,
};

use mctk_core::widgets::HDivider;
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

        let text_node = node!(
            Text::new(txt!("Network Information"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 17.5)
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
            ]
        );

        let details_row_1 = detail_row(
            DetailRow {
                key: "NAME".to_uppercase(),
                value: truncate(network.name.clone(), 17),
            },
            DetailRow {
                key: "Status".to_uppercase(),
                value: "Not Connected".to_string(),
            },
        );

        let details_row_2 = detail_row(
            DetailRow {
                key: "Frequency".to_uppercase(),
                value: if network.frequency.starts_with("2") {
                    "2.4 GHz"
                } else {
                    "5 GHz"
                }
                .to_string(),
            },
            DetailRow {
                key: "Signal".to_uppercase(),
                value: signal_strength.to_string(),
            },
        );

        let details_row_3 = detail_row(
            DetailRow {
                key: "MAC Address".to_uppercase(),
                value: network.mac.to_string(),
            },
            DetailRow {
                key: "Security".to_uppercase(),
                value: security.to_string(),
            },
        );

        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.5,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider {
                size: 0.5,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider {
                size: 1.,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));

        base = base.push(header_node!(
            "Network Information",
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::Networking
                    }
                })
            })
        ));
        base = base.push(content_node);

        Some(base)
    }
}
