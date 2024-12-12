use crate::components::*;

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
            Scrollable::new(size!(440, 370)),
            lay![
                size: [440, 370],
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

        let sub_header = text_bold_node("Screen off time");
        scrollable = scrollable.push(radio_node(vec!["10s", "20s", "30s", "60s", "5m", "Never"]));

        base = base.push(sub_header);

        base = base.push(scrollable);
        Some(base)
    }
}
