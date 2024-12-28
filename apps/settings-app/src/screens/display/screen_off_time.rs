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

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, 90],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                margin: [10., 0., 0., 0.],
            ]
        );

        let mut scrollable = node!(
            Scrollable::new(size!(440, 310)),
            lay![
                size: [440, 310],
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

        let sub_header = node!(
            Div::new(),
            lay![
                margin: [0., 8., 0., 8.]
            ]
        )
        .push(sub_header_node("Screen off time"));

        let options = vec!["10s", "20s", "30s", "60s", "5m", "Never"];
        let mut options_vec: Vec<(Vec<TextSegment>, Vec<TextSegment>)> = vec![];
        for (i, option) in options.into_iter().enumerate() {
            options_vec.push((txt!(option), txt!(option)));
        }
        scrollable = scrollable.push(radio_node!(options_vec, txt!("30s")));

        main_node = main_node.push(sub_header);
        main_node = main_node.push(scrollable);

        base = base.push(main_node);
        Some(base)
    }
}
