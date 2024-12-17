use crate::radio_node;
use crate::components::*;

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

        main_node = main_node.push(text_bold_node("Select Input Device"));
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
