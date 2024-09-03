use std::{collections::HashMap, fmt};

use mctk_core::AssetParams;

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

pub fn get_rotation_icons_map(icon_paths: RotationIconPaths) -> HashMap<String, AssetParams> {
    let mut assets = HashMap::new();

    assets.insert(
        RotationStatus::Portrait.to_string(),
        AssetParams::new(icon_paths.portrait.clone()),
    );
    assets.insert(
        RotationStatus::Landscape.to_string(),
        AssetParams::new(icon_paths.landscape.clone()),
    );

    assets
}
