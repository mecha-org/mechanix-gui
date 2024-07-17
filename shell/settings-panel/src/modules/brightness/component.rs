use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, msg, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    gui::{Message, SliderSettingsNames},
    settings::BrightnessIcons,
    widgets::slidable_setting::SlidableSetting,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BrightnessValue {
    Low,
    Medium,
    High,
}

impl fmt::Display for BrightnessValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrightnessValue::Low => write!(f, "BrightnessValueLow"),
            BrightnessValue::Medium => write!(f, "BrightnessValueMedium"),
            BrightnessValue::High => write!(f, "BrightnessValueHigh"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BrightnessMessage {
    BrightnessValueUpdate(i32),
}
#[derive(Debug)]
pub struct BrightnessComponent {
    pub value: i32,
}

impl Component for BrightnessComponent {
    fn view(&self) -> Option<Node> {
        let icon = if self.value < 10 {
            BrightnessValue::Low.to_string()
        } else {
            BrightnessValue::Medium.to_string()
        };

        Some(node!(SlidableSetting::new(
            icon,
            "Brightness".to_string(),
            self.value,
        )
        .on_slide(Box::new(|value| msg!(Message::SliderChanged(
            SliderSettingsNames::Brightness { value }
        ))))))
    }
}

pub fn get_brightness_icons_map(icon_paths: BrightnessIcons) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.low {
        assets.insert(BrightnessValue::Low.to_string(), value.clone());
    }

    if let value = &icon_paths.medium {
        assets.insert(BrightnessValue::Medium.to_string(), value.clone());
    }

    if let value = &icon_paths.high {
        assets.insert(BrightnessValue::High.to_string(), value.clone());
    }

    assets
}
