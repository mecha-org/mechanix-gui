use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct BluetoothPairingVerifyCode {}
impl Component for BluetoothPairingVerifyCode {
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

        main_node = main_node.push(header_node("Verify the Code for Mecha"));
        let text_node = node!(
            widgets::Text::new(txt!("1234 5678"))
                .style("color", Color::WHITE)
                .style("size", 40.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![
                margin: [22.5, 125.0, 0.0, 10.0],
                size_pct: [100, 15],
                cross_alignment: layout::Alignment::Center,
                axis_alignment: layout::Alignment::Center
            ]
        );
        main_node = main_node.push(node!(HDivider { size: 1. }).key(9));
        main_node = main_node.push(text_node);
        main_node = main_node.push(node!(HDivider { size: 1. }).key(10));
        base = base.push(footer_node!(
            Routes::BluetoothScreen,
            "tick_icon",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::BluetoothScreen,
            }))
        ));
        base = base.push(main_node);
        Some(base)
    }
}
