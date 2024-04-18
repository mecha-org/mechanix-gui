use std::collections::HashMap;

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size,
    widgets::{Div, Svg},
    Color, Node,
};

use crate::{settings::BatteryIconPaths, types::BatteryLevel};

#[derive(Debug)]
pub struct BatteryComponent {
    pub level: BatteryLevel,
}

impl Component for BatteryComponent {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(Color::LIGHT_GREY)
                ,
                [
                    size: [24, 24],
                    // cross_alignment: Alignment::Center,
                    // axis_alignment: Alignment::Center,
                    padding: [1, 2, 1, 2]
                ],
            )
            .push(node!(
                Svg::new(self.level.to_string()),
                lay![
                    size: [20, 20],
                ],
            )),
        )
    }
}

pub fn get_battery_icons_map(icon_paths: BatteryIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.not_found {
        assets.insert(BatteryLevel::NotFound.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_0 {
        assets.insert(BatteryLevel::Level0.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_10 {
        assets.insert(BatteryLevel::Level10.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_20 {
        assets.insert(BatteryLevel::Level20.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_30 {
        assets.insert(BatteryLevel::Level30.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_40 {
        assets.insert(BatteryLevel::Level40.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_50 {
        assets.insert(BatteryLevel::Level50.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_60 {
        assets.insert(BatteryLevel::Level60.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_70 {
        assets.insert(BatteryLevel::Level70.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_80 {
        assets.insert(BatteryLevel::Level80.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_90 {
        assets.insert(BatteryLevel::Level90.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_100 {
        assets.insert(BatteryLevel::Level100.to_string(), value.clone());
    }

    assets
}

pub fn get_battery_icons_charging_map(icon_paths: BatteryIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.level_0 {
        assets.insert(BatteryLevel::ChargingLevel0.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_10 {
        assets.insert(BatteryLevel::ChargingLevel10.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_20 {
        assets.insert(BatteryLevel::ChargingLevel20.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_30 {
        assets.insert(BatteryLevel::ChargingLevel30.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_40 {
        assets.insert(BatteryLevel::ChargingLevel40.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_50 {
        assets.insert(BatteryLevel::ChargingLevel50.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_60 {
        assets.insert(BatteryLevel::ChargingLevel60.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_70 {
        assets.insert(BatteryLevel::ChargingLevel70.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.level_80 {
        assets.insert(BatteryLevel::ChargingLevel80.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_90 {
        assets.insert(BatteryLevel::ChargingLevel90.to_string(), value.clone());
    }
    if let Some(value) = &icon_paths.level_100 {
        assets.insert(BatteryLevel::ChargingLevel100.to_string(), value.clone());
    }

    assets
}
