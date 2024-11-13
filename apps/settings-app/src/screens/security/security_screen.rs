use crate::footer_node;
use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct SecurityScreen {}
impl Component for SecurityScreen {
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

        let toggle = node!(
            Toggle::new(false),
            lay![
                size: [0.0],
                margin:[0., 0., 0., 0.],
                padding: [0, 0, 0., 0],
                axis_alignment: Alignment::End,
                cross_alignment: Alignment::Center,
            ]
        );
        main_node = main_node.push(header_node("Security"));
        main_node = main_node.push(tab_item_node!([text_node("Enable Lock")], [toggle]));
        main_node = main_node.push(node!(Div::new(), lay![size: [10]]));
        main_node = main_node.push(node!(HDivider { size: 1. }));
        base = base.push(footer_node!(Routes::SettingsList));
        base = base.push(main_node);
        Some(base)
    }
}
