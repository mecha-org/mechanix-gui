use crate::components::*;
use crate::radio_node;

#[derive(Debug)]
pub struct InputDeviceSelector {}
impl Component for InputDeviceSelector {
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
        .push(sub_header_node("Select Input Device"));

        main_node = main_node.push(sub_header);
        let options = vec![
            (
                txt!("Inbuilt Microphone".to_string()),
                txt!("Inbuilt Microphone".to_string()),
            ),
            (
                txt!("External Microphone".to_string()),
                txt!("External Microphone".to_string()),
            ),
        ];
        main_node = main_node.push(radio_node!(options, txt!("Inbuilt Microphone")));
        base = base.push(main_node);
        Some(base)
    }
}
