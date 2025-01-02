use crate::components::*;
use crate::radio_node;

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
                padding: [10.0, 0.0, 5.0, 0.0],
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

        let sub_header = node!(
            Div::new(),
            lay![
                margin: [0., 8., 0., 8.]
            ]
        )
        .push(sub_header_node("Select Output Device"));

        main_node = main_node.push(sub_header);
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
