use mctk_core::component;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Image;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use std::hash::Hash;

use crate::types::BatteryLevel;
use crate::utils::get_formatted_battery_level;

use super::model::BatteryModel;

#[derive(Debug)]
pub struct Battery {}

impl Component for Battery {
    fn init(&mut self) {
        BatteryModel::run();
    }

    fn view(&self) -> Option<Node> {
        let level = BatteryModel::level();
        let status = BatteryModel::status();
        let battery_level = get_formatted_battery_level(&level, &status);

        Some(
            node!(
                Div::new()
                ,
                [
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ],
            )
            .push(node!(
                Image::new(battery_level.to_string()),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
