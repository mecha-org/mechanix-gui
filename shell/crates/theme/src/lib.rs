use futures::channel::mpsc;

use gpui::*;

mod colors;
mod event_listeners;
mod helpers;
mod manager;
mod theme;

use theme::Theme;

use crate::{
    event_listeners::{listen_dispatcher, listen_theme_channel},
    manager::ThemeManager,
    prelude::{ColorsSetting, ThemeMode},
    theme::Fonts,
};

pub mod prelude {
    pub use crate::ActiveTheme;
    pub use crate::colors::*;
    pub use crate::theme::*;
}

pub fn init(cx: &mut App) {
    ThemeManager::init(cx);
    ThemeManager::global_mut(cx).set_mode(ThemeMode::Dark);
    ThemeManager::global_mut(cx).set_colors(ColorsSetting::default());
    ThemeManager::apply(cx);
    Fonts::init(cx);

    let (theme_tx, theme_rx) = mpsc::channel::<ThemeEvents>(120);
    listen_dispatcher(cx, theme_tx);
    listen_theme_channel(cx, theme_rx);
}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

pub trait ActiveFonts {
    fn fonts(&self) -> &Fonts;
}

impl ActiveFonts for App {
    fn fonts(&self) -> &Fonts {
        Fonts::global(self)
    }
}

pub enum ThemeEvents {
    SetThemeColors {
        accent: String,
    },
    SetThemeMode(String),
    SetPrimaryFont(String),
    SetSecondaryFont(String),
    SetTertiaryFont(String),
}
