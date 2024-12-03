use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct InputDeviceSelector {}
impl Component for InputDeviceSelector {
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
                size_pct: [80],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0],
            ]
        );

        main_node = main_node.push(header_node("Select Input Device"));
        main_node = main_node.push(radio_node(vec![
            "Inbuilt Microphone",
            "External Microphone",
        ]));
        base = base.push(main_node);
        Some(base)
    }
}
