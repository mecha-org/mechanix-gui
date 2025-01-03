use crate::components::*;
use crate::radio_node;
use crate::utils::truncate;

use super::sound_model::SoundModel;

#[derive(Debug)]
pub struct InputDeviceSelector {}
impl Component for InputDeviceSelector {
    fn init(&mut self) {
        SoundModel::get_input_devices();
    }

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
                margin: [0., 10., 0., 0.]
            ]
        )
        .push(sub_header_node("Select Input Device"));

        main_node = main_node.push(sub_header);

        let input_devices = SoundModel::get().input_devices.get().clone();

        let mut options: Vec<(Vec<font_cache::TextSegment>, Vec<font_cache::TextSegment>)> = vec![];

        for (i, device) in input_devices.clone().into_iter().enumerate() {
            options.push((
                txt!(truncate(device.name.clone(), 30)),
                txt!(truncate(device.name.clone(), 30)),
            ));
        }

        if (options.len() > 0) {
            // println!("CHECK  01--------------- {:?}", options[0].1.clone());
            main_node = main_node.push(radio_node!(options, txt!("")));
        }

        base = base.push(main_node);
        Some(base)
    }
}
