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
    gui::Message, settings::BatteryIconPaths, widgets::clickable_setting::ClickableSetting,
};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BatteryLevel {
    #[default]
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100,
    NotFound,
}

impl fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

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
        Some(node!(ClickableSetting::new(
            self.level.to_string(),
            "Battery".to_string(),
            "65".to_string(),
            "SpaceGrotesk-Medium".to_string()
        )))
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
