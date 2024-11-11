use super::component::NetworkRowComponent;
use crate::{
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};
use mctk_core::{
    component::{self, Component, RootComponent},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect,
    reexports::smithay_client_toolkit::reexports::calloop,
    size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, Image, Text, Toggle},
    Color, Node,
};
#[derive(Debug)]
pub struct NetworkScreen {}
impl Component for NetworkScreen {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        let mut main_node = node!(
            Div::new(),
            // .scroll_y()
            // .style("bar_width", 0.)
            // .style("bar_color", Color::TRANSPARENT)
            // .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );
        let mut c_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 80],
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0]
            ],
        );

        //Title
        let mut header = node!(
            Div::new(),
            // Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                // cross_alignment: Alignment::Center,
                padding: [5.0, 10.0, 10.0, 10.0],
                margin: [0., 0., 15., 0.]
            ]
        );
        let header_text = node!(
            Text::new(txt!("Wireless"))
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
        let toggle = node!(
            Toggle::new(true), // dynamic handle
            lay![
                margin:[0., 0., 0., 28.],
                axis_alignment: Alignment::End
            ]
        );
        header = header.push(header_text);
        header = header.push(toggle);

        let mut network_div = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [15, 0, 0, 0]
            ]
        );
        let network_row = node!(
            NetworkRowComponent {
                title: "Actonate office net".to_string(),
                value: "Mecha-1".to_string(),
                icon_1: "connected_icon".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::SettingsList // TODO : show connected network details
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        network_div = network_div
            .push(node!(HDivider { size: 1. }))
            .push(network_row)
            .push(node!(HDivider { size: 1. }));

        let mut manage_networks_div = node!(
            Div::new(),
            lay![
                margin: [15, 0, 0, 0]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let manage_networks_row = node!(
            NetworkRowComponent {
                title: "Manage Networks".to_string(),
                value: "".to_string(),
                icon_1: "".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::SettingsList // TODO : show connected network details
                }))),
            },
            lay![
                padding: [5., 3., 0., 5.],
            ]
        );
        manage_networks_div = manage_networks_div
            .push(manage_networks_row)
            .push(node!(HDivider { size: 1. }));

        let mut available_networks_div = node!(
            Div::new(),
            lay![
                margin: [0, 0, 10, 0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        let available_networks_row = node!(
            NetworkRowComponent {
                title: "Available Networks".to_string(),
                value: "".to_string(),
                icon_1: "".to_string(),
                icon_2: "right_arrow_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                    route: Routes::SettingsList // TODO : show connected network details
                }))),
            },
            lay![
                padding: [5., 3., 5., 5.],
            ]
        );
        available_networks_div = available_networks_div
            .push(available_networks_row)
            .push(node!(HDivider { size: 1. }));

        let mut footer = node!(
            Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 20],
                axis_alignment: Alignment::End,
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                direction: Direction::Column
            ]
        );
        footer = footer.push(node!(HDivider { size: 1. }));
        footer = footer.push(node!(
            Image::new("back_icon"),
            lay![
                size: [24, 24],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
            ]
        ));
        c_node = c_node.push(header);
        c_node = c_node.push(network_div);
        c_node = c_node.push(manage_networks_div);
        c_node = c_node.push(available_networks_div);

        main_node = main_node.push(c_node);
        base = base.push(main_node);
        base = base.push(footer);

        Some(base)
    }
}
