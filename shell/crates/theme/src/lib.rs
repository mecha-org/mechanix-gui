use futures::{SinkExt, StreamExt, channel::mpsc};

use dispatcher::Dispatcher;
use gpui::*;

mod colors;
mod registry;
mod settings;

use colors::ThemeColors;

use crate::registry::ThemeRegistry;

pub mod prelude {
    pub use crate::Theme;
    pub use crate::colors::*;
    pub use crate::registry::*;
    pub use crate::settings::*;
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ThemeColors,
    pub font_size: Pixels,
}

impl Default for Theme {
    fn default() -> Self {
        Self::from(ThemeColors::default())
    }
}

impl From<ThemeColors> for Theme {
    fn from(colors: ThemeColors) -> Self {
        Theme {
            colors,
            font_size: px(16.),
        }
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
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

impl Theme {}

pub struct ThemeManager {
    active: SharedString,
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
    fn init(cx: &mut App) {
        let mut theme_manager = ThemeManager::new();
        cx.set_global(theme_manager);
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { active: "".into() }
    }

    pub fn set_active_theme_name(&mut self, name: SharedString) {
        self.active = name.clone();
    }

    pub fn change(cx: &mut App) {
        if !cx.has_global::<Theme>() {
            cx.set_global(Theme::default());
        }
        let active = ThemeManager::global(cx).active_theme_name();
        let active_theme_setting = ThemeRegistry::global(cx).get_theme(active).unwrap();
        let theme = cx.global_mut::<Theme>();
        theme.apply_setting(active_theme_setting);
        cx.refresh_windows();
    }

    pub fn active_theme_name(&self) -> SharedString {
        self.active.clone()
    }
}

pub fn init(cx: &mut App) {
    registry::init(cx);
    ThemeManager::init(cx);
    let default = ThemeRegistry::global(cx).default_theme();
    ThemeManager::global_mut(cx).set_active_theme_name(default);
    ThemeManager::change(cx);
    let (theme_tx, theme_rx) = mpsc::channel::<ThemeEvents>(120);
    listen_dispatcher(cx, theme_tx);
    listen_theme_channel(cx, theme_rx);
}

fn listen_dispatcher(cx: &mut App, mut theme_tx: mpsc::Sender<ThemeEvents>) {
    let mut dispatcher_rx = Dispatcher::global(cx).0.clone();
    _ = cx
        .background_executor()
        .spawn(async move {
            while let Ok(msg) = dispatcher_rx.recv().await {
                match msg {
                    dispatcher::Message::SetTheme(shared_string) => {
                        let _ = theme_tx.send(ThemeEvents::SetTheme(shared_string)).await;
                    }
                    _ => (),
                }
            }
        })
        .detach();
}

pub enum ThemeEvents {
    SetTheme(SharedString),
}

fn listen_theme_channel(cx: &mut App, mut theme_rx: mpsc::Receiver<ThemeEvents>) {
    cx.spawn(async move |app| {
        while let Some(msg) = theme_rx.next().await {
            match msg {
                ThemeEvents::SetTheme(shared_string) => {
                    _ = app.update(|cx| {
                        ThemeManager::global_mut(cx).set_active_theme_name(shared_string);
                        cx.refresh_windows();
                    });
                }
            }
        }
    })
    .detach();
}
