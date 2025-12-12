use crate::prelude::*;
use gpui::*;

#[derive(Debug, Clone)]
pub struct ThemeManager {
    pub mode: ThemeMode,
    pub colors: ColorsSetting,
}

impl Global for ThemeManager {}

impl ThemeManager {
    pub fn global(cx: &App) -> &ThemeManager {
        cx.global::<ThemeManager>()
    }

    pub fn global_mut(cx: &mut App) -> &mut ThemeManager {
        cx.global_mut::<ThemeManager>()
    }
}

impl ThemeManager {
    pub fn init(cx: &mut App) {
        let mut theme_manager = ThemeManager::new();
        cx.set_global(theme_manager);
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ColorsSetting::default(),
        }
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
    }

    pub fn set_colors(&mut self, colors: ColorsSetting) {
        self.colors = colors;
    }

    pub fn apply(cx: &mut App) {
        if !cx.has_global::<Theme>() {
            cx.set_global(Theme::default());
        }
        let ThemeManager { mode, colors, .. } = ThemeManager::global(cx).clone();
        let theme = cx.global_mut::<Theme>();
        theme.apply_setting(mode, colors);
        cx.refresh_windows();
    }
}
