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
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
            ]
        );

        let mut scrollable = node!(
            Scrollable::new(size!(440, 350)),
            lay![
                size: [440, 350],
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

        let sub_header = sub_header_node("Screen off time");
        let options = vec!["10s", "20s", "30s", "60s", "5m", "Never"];
        let mut options_vec: Vec<(Vec<TextSegment>, Vec<TextSegment>)> = vec![];
        for (i, option) in options.into_iter().enumerate() {
            options_vec.push((txt!(option), txt!(option)));
        }
        scrollable = scrollable.push(radio_node!(options_vec, txt!("30s")));

        base = base.push(sub_header);

        base = base.push(scrollable);
        Some(base)
    }
}
