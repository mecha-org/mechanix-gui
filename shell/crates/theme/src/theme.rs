use crate::prelude::*;
use gpui::*;

//Theme mode
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    #[default]
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
    pub fn from_str(s: &str) -> ThemeMode {
        match s {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::Dark,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ThemeColors,
}

impl Default for Theme {
    fn default() -> Self {
        Self::from(ThemeColors::default())
    }
}

impl From<ThemeColors> for Theme {
    fn from(colors: ThemeColors) -> Self {
        Theme { colors }
    }
}

impl Global for Theme {}

impl Theme {
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }
}

impl Theme {
    pub fn apply_setting(&mut self, mode: ThemeMode, colors: ColorsSetting) {
        self.colors.apply_setting(mode, colors);
    }
}

#[derive(Debug, Clone)]
pub struct Fonts {
    pub primary: SharedString,
    pub secondary: SharedString,
    pub tertiary: SharedString,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            primary: "Overused Grotesk".into(),
            secondary: "Noto Sans".into(),
            tertiary: "Inter".into(),
        }
    }
}

impl Global for Fonts {}

impl Fonts {
    pub fn init(cx: &mut App) {
        let fonts = Fonts::default();
        cx.set_global(fonts);
    }

    pub fn global(cx: &App) -> &Fonts {
        cx.global::<Fonts>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Fonts {
        cx.global_mut::<Fonts>()
    }
}
