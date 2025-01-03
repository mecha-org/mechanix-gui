use futures::SinkExt;

use crate::components::*;
use crate::radio_node;
use crate::utils::truncate;

use super::sound_model::SoundModel;

#[derive(Debug)]
pub struct OutputDeviceSelector {}
impl Component for OutputDeviceSelector {
    fn init(&mut self) {
        SoundModel::get_output_devices();
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
        .push(sub_header_node("Select Output Device"));

        main_node = main_node.push(sub_header);

        let output_devices = SoundModel::get().output_devices.get().clone();
        let mut options = vec![];

        for (i, device) in output_devices.clone().into_iter().enumerate() {
            options.push((
                txt!(truncate(device.name.clone(), 30)),
                txt!(truncate(device.name.clone(), 30)),
            ));
        }

        if (options.len() > 0) {
            main_node = main_node.push(radio_node!(options, txt!("Speakers")));
        }

        base = base.push(main_node);
        Some(base)
    }
}
