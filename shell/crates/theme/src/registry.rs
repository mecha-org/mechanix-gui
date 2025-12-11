use gpui::*;
use settings::prelude::*;
use std::collections::HashMap;

use crate::{Theme, prelude::ThemesSettings, settings::ThemeSetting};

pub fn init(cx: &mut App) {
    let registry = ThemeRegistry::new();
    cx.set_global(registry);
}

#[derive(Default, Debug, Clone)]
pub struct ThemeRegistry {
    default: SharedString,
    themes: HashMap<SharedString, ThemeSetting>,
}

impl Global for ThemeRegistry {}

impl ThemeRegistry {
    pub fn new() -> Self {
        let theme_settings = load_settings::<ThemesSettings>(config_paths_for("themes.toml"));
        let ThemesSettings {
            default, themes, ..
        } = theme_settings;
        Self { default, themes }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<ThemeRegistry>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<ThemeRegistry>()
    }

    pub fn themes(&self) -> HashMap<SharedString, ThemeSetting> {
        self.themes.clone()
    }

    pub fn get_theme(&self, name: SharedString) -> Option<ThemeSetting> {
        self.themes.get(&name).cloned()
    }

    pub fn default_theme(&self) -> SharedString {
        self.default.clone()
    }
}
