use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct ConnectNetworkEnterCode {
    pub network_name: String,
}
impl Component for ConnectNetworkEnterCode {
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
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        // TODO :  get requested network here
        main_node = main_node.push(header_node("Enter Password for Network"));
        let text_box = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, 15]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                //axis_alignment: Alignment::Stretch,
                // padding: [ 30, 0, 0, 0 ]
            ]
        )
        .push(node!(
            TextBox::new(Some("".to_string()))
                .style("background_color", Color::TRANSPARENT)
                .style("font_size", 20.)
                .style("text_color", Color::WHITE)
                .style("border_color", Color::TRANSPARENT)
                // .style("border_width", 0.)
                .style("cursor_color", Color::WHITE)
                .style("placeholder_color", Color::rgb(107., 107., 107.))
                .placeholder("Enter code"),
            lay![
                size_pct: [100],
                padding: [0, 10, 0, 10]
            ]
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }).key(9));
        main_node = main_node.push(text_box);
        main_node = main_node.push(node!(HDivider { size: 1. }).key(10));
        // base = base.push(footer_node!(
        //     Routes::BluetoothScreen,
        //     "tick_icon",
        //     Box::new(|| msg!(Message::ChangeRoute {
        //         route: Routes::BluetoothScreen,
        //     }))
        // ));
        base = base.push(main_node);
        Some(base)
    }
}
