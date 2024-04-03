use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, msg, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    gui::Message, settings::CommonLowMediumHighPaths, widgets::slidable_setting::SlidableSetting,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SoundValue {
    Low,
    Medium,
    High,
}

impl fmt::Display for SoundValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoundValue::Low => write!(f, "SoundValueLow"),
            SoundValue::Medium => write!(f, "SoundValueMedium"),
            SoundValue::High => write!(f, "SoundValueHigh"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SoundMessage {
    SoundValueUpdate(i32),
}
#[derive(Debug)]
pub struct SoundComponent {
    pub value: i32,
}

impl Component for SoundComponent {
    fn view(&self) -> Option<Node> {
        println!("SoundComponent view {}", self.value);

        let icon = if self.value < 10 {
            SoundValue::Low.to_string()
        } else {
            SoundValue::Medium.to_string()
        };

        Some(node!(SlidableSetting::new(
            icon,
            "Sound".to_string(),
            self.value,
            "SpaceGrotesk-Medium".to_string()
        )
        .on_slide(Box::new(|value| msg!(Message::Sound { value })))))
    }
}

pub fn get_sound_icons_map(icon_paths: CommonLowMediumHighPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.low {
        assets.insert(SoundValue::Low.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.medium {
        assets.insert(SoundValue::Medium.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.high {
        assets.insert(SoundValue::High.to_string(), value.clone());
    }

    assets
}
