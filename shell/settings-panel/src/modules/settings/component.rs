use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{settings::SettingsIconPaths, widgets::clickable_setting::ClickableSetting};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SettingsStatus {
    #[default]
    Default,
}

impl fmt::Display for SettingsStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsStatus::Default => write!(f, "SettingsDefault"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    SettingsStatusUpdate(SettingsStatus),
}
#[derive(Debug)]
pub struct SettingsComponent {}

impl Component for SettingsComponent {
    fn view(&self) -> Option<Node> {
        Some(node!(ClickableSetting::new(
            SettingsStatus::Default.to_string(),
            "Settings".to_string(),
            "".to_string(),
            "SpaceGrotesk-Medium".to_string()
        )))
    }
}

pub fn get_settings_icons_map(icon_paths: SettingsIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.default {
        assets.insert(SettingsStatus::Default.to_string(), value.clone());
    }

    assets
}
