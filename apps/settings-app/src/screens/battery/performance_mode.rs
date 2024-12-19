use font_cache::TextSegment;

use crate::{components::*, radio_node, screens::battery::battery_model::BatteryModel};

#[derive(Debug)]
pub struct PerformanceMode {}
impl Component for PerformanceMode {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let modes: Vec<String> = BatteryModel::get().available_modes.get().clone();
        let current_mode = BatteryModel::get().cureent_mode.get().clone();

        let mut available_modes_txt = vec![];

        for (i, mode) in modes.into_iter().enumerate() {
            available_modes_txt.push((txt!(mode.clone()), txt!(mode.clone())));
        }
        main_node = main_node.push(radio_node!(
            available_modes_txt,
            txt!(current_mode),
            Box::new(|x| msg!(BatteryModel::set_mode(&x)))
        ));

        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_node("Higher performance will use battery faster"));
        main_node = main_node.push(text_node("and Check ambient temperature before"));
        main_node = main_node.push(text_node("proceeding. increase the temperature of the"));
        main_node = main_node.push(text_node("device significantly."));

        let sub_header = sub_header_node("Performance Mode");

        base = base.push(sub_header);
        base = base.push(main_node);
        Some(base)
    }
}
