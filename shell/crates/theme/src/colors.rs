use crate::prelude::*;
use gpui::*;
use palette::{Darken, IntoColor, Lighten, Oklcha};

#[derive(Debug, Clone, Default)]
pub struct ThemeColors {
    pub accent_0: Rgba,
    pub accent_100: Rgba,
    pub accent_200: Rgba,
    pub accent_300: Rgba,
    pub accent_400: Rgba,
    pub accent_500: Rgba,
    pub accent_600: Rgba,
    pub accent_700: Rgba,
    pub accent_800: Rgba,
    pub accent_900: Rgba,
    pub accent_1000: Rgba,
    pub background_0: Rgba,
    pub background_100: Rgba,
    pub background_200: Rgba,
    pub background_300: Rgba,
    pub background_400: Rgba,
    pub background_500: Rgba,
    pub background_600: Rgba,
    pub background_700: Rgba,
    pub background_800: Rgba,
    pub background_900: Rgba,
    pub background_1000: Rgba,
    pub foreground_0: Rgba,
    pub foreground_100: Rgba,
    pub foreground_200: Rgba,
    pub foreground_300: Rgba,
    pub foreground_400: Rgba,
    pub foreground_500: Rgba,
    pub foreground_600: Rgba,
    pub foreground_700: Rgba,
    pub foreground_800: Rgba,
    pub foreground_900: Rgba,
    pub foreground_1000: Rgba,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorsSetting {
    pub accent_color: Oklcha,
    pub background_color: Oklcha,
    pub foreground_color: Oklcha,
}

impl Default for ColorsSetting {
    fn default() -> Self {
        Self {
            accent_color: Oklcha::new(0.6388, 0.1435, 64.8, 98.0),
            background_color: Oklcha::new(0.1638, 0.0, 0.0, 100.0),
            foreground_color: Oklcha::new(1.0, 0.0, 0.0, 100.0),
        }
    }
}

impl ThemeColors {
    pub fn apply_setting(&mut self, mode: ThemeMode, colors: ColorsSetting) {
        let ColorsSetting {
            accent_color,
            foreground_color,
            background_color,
            ..
        } = colors;

        //For Dark theme
        self.accent_0 = oklcha_to_rgba(accent_color.lighten_fixed(0.1));
        self.accent_100 = oklcha_to_rgba(accent_color.lighten_fixed(0.05));
        self.accent_200 = oklcha_to_rgba(accent_color);
        self.accent_300 = oklcha_to_rgba(accent_color.darken_fixed(0.05));
        self.accent_400 = oklcha_to_rgba(accent_color.darken_fixed(0.1));
        self.accent_500 = oklcha_to_rgba(accent_color.darken_fixed(0.15));
        self.accent_600 = oklcha_to_rgba(accent_color.darken_fixed(0.20));
        self.accent_700 = oklcha_to_rgba(accent_color.darken_fixed(0.25));
        self.accent_800 = oklcha_to_rgba(accent_color.darken_fixed(0.30));
        self.accent_900 = oklcha_to_rgba(accent_color.darken_fixed(0.35));
        self.accent_1000 = oklcha_to_rgba(accent_color.darken_fixed(0.40));

        self.background_0 = oklcha_to_rgba(background_color.lighten_fixed(0.50));
        self.background_100 = oklcha_to_rgba(background_color.lighten_fixed(0.45));
        self.background_200 = oklcha_to_rgba(background_color.lighten_fixed(0.40));
        self.background_300 = oklcha_to_rgba(background_color.lighten_fixed(0.35));
        self.background_400 = oklcha_to_rgba(background_color.lighten_fixed(0.30));
        self.background_500 = oklcha_to_rgba(background_color.lighten_fixed(0.25));
        self.background_600 = oklcha_to_rgba(background_color.lighten_fixed(0.20));
        self.background_700 = oklcha_to_rgba(background_color.lighten_fixed(0.15));
        self.background_800 = oklcha_to_rgba(background_color.lighten_fixed(0.1));
        self.background_900 = oklcha_to_rgba(background_color.lighten_fixed(0.05));
        self.background_1000 = oklcha_to_rgba(background_color);

        self.foreground_0 = oklcha_to_rgba(foreground_color);
        self.foreground_100 = oklcha_to_rgba(foreground_color.darken_fixed(0.05));
        self.foreground_200 = oklcha_to_rgba(foreground_color.darken_fixed(0.1));
        self.foreground_300 = oklcha_to_rgba(foreground_color.darken_fixed(0.15));
        self.foreground_400 = oklcha_to_rgba(foreground_color.darken_fixed(0.20));
        self.foreground_500 = oklcha_to_rgba(foreground_color.darken_fixed(0.25));
        self.foreground_600 = oklcha_to_rgba(foreground_color.darken_fixed(0.30));
        self.foreground_700 = oklcha_to_rgba(foreground_color.darken_fixed(0.35));
        self.foreground_800 = oklcha_to_rgba(foreground_color.darken_fixed(0.40));
        self.foreground_900 = oklcha_to_rgba(foreground_color.darken_fixed(0.45));
        self.foreground_1000 = oklcha_to_rgba(foreground_color.darken_fixed(0.50));
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
