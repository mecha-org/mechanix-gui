use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    settings::RotationIconPaths,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RotationStatus {
    #[default]
    Portrait,
    Landscape,
    NotFound,
}

impl fmt::Display for RotationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationStatus::Portrait => write!(f, "RotationPortrait"),
            RotationStatus::Landscape => write!(f, "RotationLandscape"),
            RotationStatus::NotFound => write!(f, "RotationNotFound"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RotationMessage {
    RotationStatusUpdate(RotationStatus),
}
#[derive(Debug)]
pub struct RotationComponent {
    pub status: RotationStatus,
    pub loading: bool,
}

impl Component for RotationComponent {
    fn view(&self) -> Option<Node> {
        Some(node!(ClickableSetting::new(
            RotationStatus::Portrait.to_string(),
            "Auto Rotate".to_string(),
            SettingText::Normal("".to_string()),
        )))
    }
}

pub fn get_rotation_icons_map(icon_paths: RotationIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.portrait {
        assets.insert(RotationStatus::Portrait.to_string(), value.clone());
    }

    if let value = &icon_paths.landscape {
        assets.insert(RotationStatus::Landscape.to_string(), value.clone());
    }

    assets
}
