use gpui::*;
mod settings;
use settings::{Settings, load_settings};

use crate::prelude::config_paths_for;

impl Settings {
    pub fn new() -> Self {
        let settings = load_settings::<Settings>(config_paths_for("settings.toml"));
        settings
    }
}

pub fn init(cx: &mut App) {
    let settings = Settings::new();
    cx.set_global(settings);
}

impl Global for Settings {}

pub mod prelude {
    pub use crate::settings::*;
}
