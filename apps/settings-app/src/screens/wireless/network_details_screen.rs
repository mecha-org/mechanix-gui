use super::component::NetworkRowComponent;
use crate::shared::h_divider::HDivider;
use mctk_core::style::Styled;
use mctk_core::*;

// use mctk_core::{
//     component::Component,
//     lay,
//     layout::{Alignment, Direction},
//     node, rect, size, size_pct,
//     style::{FontWeight, Styled},
//     txt,
//     widgets::{Div, Image, Text},
//     Color, Node,
// };

macro_rules! row {
    ($key:expr, $value:expr) => {{
        let base = node!(
            widgets::Div::new(),
            lay![
                padding: [20, 0, 0, 0],
                size: [430, 60],
                direction: layout::Direction::Column,
                axis_alignment: layout::Alignment::Stretch,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );
        let row = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                direction: layout::Direction::Row,
                axis_alignment: layout::Alignment::Stretch,
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
pub struct NetworkDetailsScreen {}
impl component::Component for NetworkDetailsScreen {
    fn view(&self) -> Option<Node> {
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
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );

        let mut c_node = node!(
            widgets::Div::new()
                .scroll_y()
                .style("bar_width", 0.)
                .style("bar_color", Color::TRANSPARENT)
                .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100, 80],
                axis_alignment: layout::Alignment::Stretch,
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 20.0, 0.0, 20.0]
            ],
        );

        //Title
        let mut header = node!(
            widgets::Div::new(),
            // Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 15],
                axis_alignment: layout::Alignment::Stretch,
                // cross_alignment: Alignment::Center,
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [5.0, 0.0, 0.0, 0.0],
                margin: [25., 0., 0., 10.]
            ]
        );
        let header_text = node!(
            widgets::Text::new(txt!("Actonate 5G"))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", style::FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
                axis_alignment: layout::Alignment::Stretch,
            ]
        );
        header = header.push(header_text);
        header = header.push(node!(HDivider { size: 1. }));

        let mut footer = node!(
            widgets::Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 20],
                axis_alignment: layout::Alignment::End,
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                direction: layout::Direction::Column
            ]
        );
        footer = footer.push(node!(HDivider { size: 1. }));
        footer = footer.push(node!(
            widgets::Image::new("back_icon"),
            lay![
                size: [24, 24],
                direction: layout::Direction::Row,
                axis_alignment: layout::Alignment::Stretch,
            ]
        ));
        c_node = c_node.push(header);
        c_node = c_node.push(row!("Network SSID", "Actonate 5G"));
        c_node = c_node.push(row!("Network ID", "2"));
        c_node = c_node.push(row!("Passphrase", "WPA2"));
        c_node = c_node.push(row!("Frequency", "5GHz"));
        c_node = c_node.push(row!("", ""));
        c_node = c_node.push(row!("IP Address", "192.168.100.100"));
        c_node = c_node.push(row!("Subnet Mask", "255.255.255.0"));
        c_node = c_node.push(row!("Gateway", "192.168.100.1"));

        main_node = main_node.push(c_node);
        base = base.push(main_node);
        // base = base.push(footer);

        Some(base)
    }
}
