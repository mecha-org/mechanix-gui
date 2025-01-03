use font_cache::TextSegment;

use crate::{components::*, radio_node};

#[derive(Debug)]
pub struct ScreenOffTime {}
impl Component for ScreenOffTime {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
                padding: [5.0, 0.0, 5.0, 0.0],
            ]
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let sub_header = node!(
            Div::new(),
            lay![
                margin: [0., 10., 0., 8.]
            ]
        )
        .push(sub_header_node("Screen off time"));

        let mut scrollable = node!(
            Scrollable::new(size!(440, 288)),
            lay![
                size: [440, 288],
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        let options = vec!["10s", "20s", "30s", "60s", "5m", "Never"];
        let mut options_vec: Vec<(Vec<TextSegment>, Vec<TextSegment>)> = vec![];
        for (i, option) in options.into_iter().enumerate() {
            options_vec.push((txt!(option), txt!(option)));
        }
        scrollable = scrollable.push(radio_node!(options_vec, txt!("30s")));

        content_node = content_node.push(sub_header);
        content_node = content_node.push(node!(HDivider {
            size: 1.,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        content_node = content_node.push(scrollable);

        base = base.push(content_node);
        Some(base)
    }
}
