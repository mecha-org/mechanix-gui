use std::{collections::HashMap, fmt};

use crate::settings::RotationIconPaths;

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
