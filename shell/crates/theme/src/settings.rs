use crate::{Theme, colors::ThemeColors};
use gpui::*;
use palette::{Darken, IntoColor, Lighten, Oklcha};
use serde::Deserialize;
use std::collections::HashMap;
/// Themes settings
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ThemesSettings {
    #[serde(default)]
    pub default: SharedString,
    #[serde(default, deserialize_with = "deserialize_vec_to_map")]
    pub themes: HashMap<SharedString, ThemeSetting>,
}

impl Default for ThemesSettings {
    fn default() -> Self {
        let mut themes = HashMap::new();
        themes.insert("Amber".into(), ThemeSetting::default());
        Self {
            default: "Amber".into(),
            themes,
        }
    }
}

fn deserialize_vec_to_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<SharedString, ThemeSetting>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec: Vec<ThemeSetting> = Deserialize::deserialize(deserializer)?;
    Ok(vec.into_iter().map(|t| (t.name.clone(), t)).collect())
}

//Theme mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::Dark)
    }

    pub fn is_light(&self) -> bool {
        matches!(self, ThemeMode::Light)
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }
}

/// Theme setting
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ThemeSetting {
    #[serde(default)]
    pub name: SharedString,
    #[serde(default)]
    pub mode: ThemeMode,
    #[serde(default)]
    pub colors: ColorsSetting,
}

impl Default for ThemeSetting {
    fn default() -> Self {
        Self {
            name: "Amber".into(),
            mode: ThemeMode::Dark,
            colors: ColorsSetting::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ColorsSetting {
    #[serde(default)]
    pub accent_color: Oklcha,
    #[serde(default)]
    pub background_color: Oklcha,
    #[serde(default)]
    pub foreground_color: Oklcha,
}

impl Default for ColorsSetting {
    fn default() -> Self {
        Self {
            accent_color: Oklcha::new(0.6388, 0.1435, 64.8, 98.0),
            background_color: Oklcha::new(0.1638, 0.0, 0.0, 100.0),
            foreground_color: Oklcha::new(0.8638, 0.0, 0.0, 100.0),
        }
    }
}

impl ThemeColors {
    pub fn apply_setting(&mut self, setting: ThemeSetting) {
        let ThemeSetting { colors, .. } = setting;
        let ColorsSetting {
            accent_color,
            foreground_color,
            background_color,
            ..
        } = colors;

        self.accent_0 = oklcha_to_rgba(accent_color.lighten_fixed(0.6));
        self.accent_100 = oklcha_to_rgba(accent_color.lighten_fixed(0.5));
        self.accent_200 = oklcha_to_rgba(accent_color.lighten_fixed(0.4));
        self.accent_300 = oklcha_to_rgba(accent_color.lighten_fixed(0.3));
        self.accent_400 = oklcha_to_rgba(accent_color.lighten_fixed(0.2));
        self.accent_500 = oklcha_to_rgba(accent_color.lighten_fixed(0.1));
        self.accent_600 = oklcha_to_rgba(accent_color);
        self.accent_700 = oklcha_to_rgba(accent_color.darken_fixed(0.1));
        self.accent_800 = oklcha_to_rgba(accent_color.darken_fixed(0.2));
        self.accent_900 = oklcha_to_rgba(accent_color.darken_fixed(0.3));
        self.accent_1000 = oklcha_to_rgba(accent_color.darken_fixed(0.4));
        self.accent_1100 = oklcha_to_rgba(accent_color.darken_fixed(0.5));
        self.accent_1200 = oklcha_to_rgba(accent_color.darken_fixed(0.6));

        self.background_0 = oklcha_to_rgba(background_color.lighten_fixed(0.6));
        self.background_100 = oklcha_to_rgba(background_color.lighten_fixed(0.5));
        self.background_200 = oklcha_to_rgba(background_color.lighten_fixed(0.4));
        self.background_300 = oklcha_to_rgba(background_color.lighten_fixed(0.3));
        self.background_400 = oklcha_to_rgba(background_color.lighten_fixed(0.2));
        self.background_500 = oklcha_to_rgba(background_color.lighten_fixed(0.1));
        self.background_600 = oklcha_to_rgba(background_color);
        self.background_700 = oklcha_to_rgba(background_color.darken_fixed(0.1));
        self.background_800 = oklcha_to_rgba(background_color.darken_fixed(0.2));
        self.background_900 = oklcha_to_rgba(background_color.darken_fixed(0.3));
        self.background_1000 = oklcha_to_rgba(background_color.darken_fixed(0.4));
        self.background_1100 = oklcha_to_rgba(background_color.darken_fixed(0.5));
        self.background_1200 = oklcha_to_rgba(background_color.darken_fixed(0.6));

        self.foreground_0 = oklcha_to_rgba(foreground_color.lighten_fixed(0.6));
        self.foreground_100 = oklcha_to_rgba(foreground_color.lighten_fixed(0.5));
        self.foreground_200 = oklcha_to_rgba(foreground_color.lighten_fixed(0.4));
        self.foreground_300 = oklcha_to_rgba(foreground_color.lighten_fixed(0.3));
        self.foreground_400 = oklcha_to_rgba(foreground_color.lighten_fixed(0.2));
        self.foreground_500 = oklcha_to_rgba(foreground_color.lighten_fixed(0.1));
        self.foreground_600 = oklcha_to_rgba(foreground_color);
        self.foreground_700 = oklcha_to_rgba(foreground_color.darken_fixed(0.1));
        self.foreground_800 = oklcha_to_rgba(foreground_color.darken_fixed(0.2));
        self.foreground_900 = oklcha_to_rgba(foreground_color.darken_fixed(0.3));
        self.foreground_1000 = oklcha_to_rgba(foreground_color.darken_fixed(0.4));
        self.foreground_1100 = oklcha_to_rgba(foreground_color.darken_fixed(0.5));
        self.foreground_1200 = oklcha_to_rgba(foreground_color.darken_fixed(0.6));
    }
}

impl Theme {
    pub fn apply_setting(&mut self, setting: ThemeSetting) {
        self.colors.apply_setting(setting);
    }
}

pub fn oklcha_to_rgba(color: Oklcha) -> Rgba {
    let rgba: palette::rgb::Rgba = color.into_color();
    Rgba {
        r: rgba.red,
        g: rgba.green,
        b: rgba.blue,
        a: rgba.alpha,
    }
}
