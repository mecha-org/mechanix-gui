use crate::components::*;

#[derive(Debug)]
pub struct ScreenOffTime {}
impl Component for ScreenOffTime {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut scrollable = node!(
            Scrollable::new(),
            lay![
                size: [440, 400],
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, Auto],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
            ]
        );
        main_node = main_node.push(radio_node(vec!["10s", "20s", "30s", "60s", "5m", "Never"]));

        base = base.push(header_node("Screen off time"));
        scrollable = scrollable.push(main_node);

        base = base.push(scrollable);
        Some(base)
    }
}
