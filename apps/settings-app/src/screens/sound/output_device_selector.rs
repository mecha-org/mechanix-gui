use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct OutputDeviceSelector {}
impl Component for OutputDeviceSelector {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [80],
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
                padding: [0.0],
            ]
        );

        main_node = main_node.push(text_node("Select Output Device"));
        main_node = main_node.push(radio_node(vec!["Speakers", "Headphones"]));
        base = base.push(main_node);
        Some(base)
    }
}
