use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct ScreenOffTime {}
impl Component for ScreenOffTime {
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
            widgets::Div::new().scroll_y(),
            lay![
                size_pct: [100, 80],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        main_node = main_node.push(header_node("Screen off time"));
        for (i, time) in ["10s", "30s", "60s", "5m", "Never"].into_iter().enumerate() {
            main_node = main_node.push(
                tab_item_node!([text_bold_node(time)], [icon_node("right_arrow_icon")])
                    .key((i + 1) as u64),
            );
            main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        }

        base = base.push(main_node);
        base = base.push(footer_node());
        Some(base)
    }
}
