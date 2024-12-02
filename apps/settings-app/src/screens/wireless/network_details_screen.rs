use super::handler::WirelessDetailsItem;
use crate::gui::{Message, NetworkScreenRoutes, Routes};
use crate::shared::h_divider::HDivider;
use mctk_core::*;
use mctk_core::{
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconButton, IconType, Text},
    Color, Node,
};

macro_rules! row {
    ($key:expr, $value:expr) => {{
        let base = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );
        let row = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                direction: layout::Direction::Row,
                axis_alignment: layout::Alignment::Stretch,
                padding: [20., 0, 20., 0],
            ]
        );
        let left_div = node!(
            widgets::Div::new(),
            lay![
                size_pct: [50, 100],
                axis_alignment: layout::Alignment::Start,
            ],
        ).push(node!(widgets::Text::new(txt!($key))
            .style("color", Color::rgb(197.0, 197.0, 197.0))
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", style::FontWeight::Medium)));

        let right_div = node!(
            widgets::Div::new(),
            lay![
                size_pct: [50, 100],
                axis_alignment: layout::Alignment::End,
            ],
        ).push(node!(widgets::Text::new(txt!($value))
            .style("color", Color::WHITE)
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", style::FontWeight::Bold)));
        let row = row.push(left_div)
                     .push(right_div);
        base.push(row)
            .push(node!(HDivider { size: 1. }))
    }};
}

#[derive(Debug)]
pub struct NetworkDetailsScreen {
    pub wireless_details: Option<WirelessDetailsItem>,
}
impl component::Component for NetworkDetailsScreen {
    fn view(&self) -> Option<Node> {
        let mut network_name: String = "Mecha-test".to_string();
        let mut network_ssid: String = "Mecha-test".to_string();
        let mut network_security: String = "-".to_string();
        let mut network_encyption: String = "-".to_string();
        let mut network_frequency: String = "-".to_string();
        let mut mac_address = "-".to_string();
        match self.wireless_details.clone() {
            Some(resp) => {
                network_name = resp.scan_info.name.clone();
                network_ssid = resp.scan_info.name.clone();
                network_security = resp.security.clone();
                network_encyption = resp.encryption.clone();
                network_frequency = resp.scan_info.frequency.clone();
                mac_address = resp.scan_info.mac.clone();
            }
            None => (),
        };

        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, 70],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );

        let mut c_node = node!(
            widgets::Div::new(),
            // .scroll_y()
            // .style("bar_width", 0.)
            // .style("bar_color", Color::TRANSPARENT)
            // .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100, 70],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
                position_type: Relative,
            ],
        );

        //Title
        let mut header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
            ]
        );

        let mut header = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                padding: [5.0, 5.0, 10.0, 10.0],
                margin: [0., 0., 20., 0.],
            ]
        );
        let header_text = node!(
            Text::new(txt!(network_name))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
                axis_alignment: Alignment::Start
            ]
        );
        header = header.push(header_text);
        header_node = header_node.push(header);
        header_node = header_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [0., 10., 0., 10.],
                padding: [0., 0., 20., 0.]
            ]
        ));

        c_node = c_node.push(node!(
            Div::new(),
            lay![
                margin: [10., 0., 0., 0.]
            ]
        ));
        c_node = c_node.push(row!("Network SSID", network_ssid));
        c_node = c_node.push(row!("Security", network_security));
        c_node = c_node.push(row!("Encryption", network_encyption));
        c_node = c_node.push(row!("MAC Address", mac_address));
        // // c_node = c_node.push(row!("Frequency", network_frequency));
        // c_node = c_node.push(row!("", ""));
        // c_node = c_node.push(row!("IP Address", "192.168.100.100"));
        // c_node = c_node.push(row!("Subnet Mask", "255.255.255.0"));
        // c_node = c_node.push(row!("Gateway", "192.168.100.1"));
        // // c_node = c_node.push(row!("Network ID", "2"));
        // // c_node = c_node.push(row!("Passphrase", "WPA2"));

        let mut footer_div = node!(
            Div::new(),
            lay![
                size_pct: [100, 20],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
            ]
        );
        let footer_row: Node = node!(
            Div::new(),
            lay![
                direction: Direction::Row,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50],
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkScreen
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(52.0),
                            height: Dimension::Px(52.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 12.),
                lay![
                    size: [52, 52],
                cross_alignment: Alignment::Center,
                margin: [0., 20., 0., 0.]
                ]
            )),
        );

        footer_div = footer_div
            .push(node!(HDivider { size: 1. }))
            .push(footer_row);

        main_node = main_node.push(c_node);

        base = base.push(header_node);
        base = base.push(main_node);
        base = base.push(footer_div);

        Some(base)
    }
}
