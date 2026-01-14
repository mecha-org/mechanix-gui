use gpui::*;
mod icons;
use icons::{Icons, load_icons};

use crate::prelude::config_paths_for;

impl Icons {
    pub fn new() -> Self {
        let icons = load_icons::<Icons>(config_paths_for("icons.toml"));
        icons
    }
}

pub fn init(cx: &mut App) {
    let icons = Icons::new();
    cx.set_global(icons);
}

impl Global for Icons {}

pub mod prelude {
    pub use crate::icons::*;
}
