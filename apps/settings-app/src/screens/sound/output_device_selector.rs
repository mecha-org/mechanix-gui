use crate::radio_node;
use crate::components::*;

#[derive(Debug)]
pub struct OutputDeviceSelector {}
impl Component for OutputDeviceSelector {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, Auto],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
                padding: [5.0, 0.0, 5.0, 0.0],
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

        main_node = main_node.push(sub_header_node("Select Output Device"));
        let options = vec![
            (txt!("Speakers".to_string()), txt!("Speakers".to_string())),
            (
                txt!("Headphones".to_string()),
                txt!("Headphones".to_string()),
            ),
        ];
        main_node = main_node.push(radio_node!(options, txt!("Speakers")));
        base = base.push(main_node);
        Some(base)
    }
}
