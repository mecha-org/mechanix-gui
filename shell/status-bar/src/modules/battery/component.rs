use std::{collections::HashMap, sync::mpsc::Sender};

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    widgets::{Div, Image, Svg},
    Node,
};

use crate::{
    gui::{BatteryLevel, Message},
    settings::BatteryIconPaths,
};

#[derive(Debug, Clone)]
pub enum BatteryMessage {
    BatteryLevelUpdate(BatteryLevel),
}

#[derive(Debug)]
pub struct BatteryComponent {
    pub level: BatteryLevel,
}

impl Component for BatteryComponent {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                //  .bg(0xFF00FFFF)
                ,
                [
                    size: [24, 24],
                    cross_alignment: Alignment::Center,
                    axis_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                Svg::new(self.level.to_string()),
                lay![
                    size: [16, 20],
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
