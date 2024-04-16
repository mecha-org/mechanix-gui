use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    widgets::{Div, Image, Svg},
    Node,
};

use crate::{
    gui::Message,
    settings::BatteryIconPaths,
    types::BatteryLevel,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Debug)]
pub struct BatteryComponent {
    pub level: BatteryLevel,
    pub percentage: u8,
}

impl Component for BatteryComponent {
    fn view(&self) -> Option<Node> {
        let mut percentage = "".to_string();
        let mut subscript = "".to_string();

        if self.percentage > 0 {
            percentage = self.percentage.to_string();
            subscript = "%".to_string();
        }

        Some(node!(ClickableSetting::new(
            self.level.to_string(),
            "Battery".to_string(),
            SettingText::Subscript(percentage, subscript),
        )
        .click_disabled(true)))
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
