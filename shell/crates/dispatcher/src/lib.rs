use async_broadcast::{broadcast, Receiver, Sender};
use futures::StreamExt;
use gpui::*;
use mxconf_dbus::watch_setting;
use serde::{Deserialize, Serialize};
use crate::Message::SetLockscreenWallpaper;

#[derive(Clone)]
pub struct Dispatcher(pub Sender<Message>, pub Receiver<Message>);

impl Dispatcher {
    pub fn channel(&self) -> (Sender<Message>, Receiver<Message>) {
        (self.0.clone(), self.1.clone())
    }
}

impl Global for Dispatcher {}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct ThemeColors {
    pub accent: String,
    pub background: String,
    pub foreground: String,
}

impl ThemeColors {
    pub fn parse(input: &str) -> Result<Self, String> {
        fn extract(input: &str, key: &str) -> Result<String, String> {
            let start = input
                .find(&format!("{key} ="))
                .ok_or_else(|| format!("Missing {}", key))?;

            let after_key = &input[start..];
            let first_quote = after_key.find('"').ok_or("Missing opening quote")?;
            let rest = &after_key[first_quote + 1..];
            let end_quote = rest.find('"').ok_or("Missing closing quote")?;

            Ok(rest[..end_quote].to_string())
        }

        Ok(Self {
            accent: extract(input, "accent")?,
            background: extract(input, "background")?,
            foreground: extract(input, "foreground")?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetThemeMode(String),
    SetThemeColors {
        accent: String,
        background: String,
        foreground: String,
    },
    SetPrimaryFont(String),
    SetSecondaryFont(String),
    SetTertiaryFont(String),
    SetKeyboardAlwayson(bool),
    ShowPowerOptions(bool),
    ShowLockscreen(bool),
    SetLockscreenWallpaper(String),
    ExtensionAttached(String),
    ExtensionDetached(String),
    VolumeUp,
    VolumeDown,
    LaunchApp {
        app_id: String,
        exec: String,
    },
    MinimizeToHome,
}

pub fn init(cx: &mut App) {
    let (mut tx, rx) = broadcast(120);
    tx.set_overflow(true);
    cx.set_global(Dispatcher(tx.clone(), rx.clone()));

    cx.background_executor()
        .spawn(async move {
            let schema = "org.mechanix.desktop";
            let key = None;
            let mut watcher = watch_setting(schema, key.clone()).await.unwrap();
            while let Some(signal) = watcher.next().await {
                if let Ok((_schema, signal_key, value)) = signal.body::<(String, String, String)>()
                {
                    println!("Received change signal for key: {}", signal_key);
                    //keyboard.general.always_on
                    match signal_key.as_str() {
                        "keyboard.general.always_on" => {
                            match tx
                                .broadcast(Message::SetKeyboardAlwayson(match value.as_str() {
                                    "true" => true,
                                    _ => false,
                                }))
                                .await
                            {
                                Ok(_) => {
                                    println!("message broadcasted");
                                }
                                Err(_) => {
                                    println!("message broadcasted failed");
                                }
                            };
                        }
                        "settings.active_theme.theme_colors" => {
                            match ThemeColors::parse(&value) {
                                Ok(theme) => {
                                    match tx
                                        .broadcast(Message::SetThemeColors {
                                            accent: theme.accent,
                                            background: theme.background,
                                            foreground: theme.foreground,
                                        })
                                        .await
                                    {
                                        Ok(_) => {
                                            println!("message broadcasted");
                                        }
                                        Err(_) => {
                                            println!("message broadcasted failed");
                                        }
                                    };
                                }
                                Err(err) => {
                                    eprintln!("Error while parsing theme colors: {}", err);
                                }
                            };
                        }
                        "settings.lockscreen.wallpaper" => {
                            match tx
                                .broadcast(SetLockscreenWallpaper(value))
                                .await
                            {
                                Ok(_) => {
                                    println!("lockscreen wallpaper message broadcasted");
                                }
                                Err(_) => {
                                    println!("lockscreen wallpaper message broadcasted failed");
                                }
                            };
                        }
                        "settings.active_fonts.primary" => {
                            match tx.broadcast(Message::SetPrimaryFont(value)).await {
                                Ok(_) => {
                                    println!("message broadcasted");
                                }
                                Err(_) => {
                                    println!("message broadcasted failed");
                                }
                            };
                        }
                        _ => (),
                    }
                } else {
                    println!("Failed to parse signal body for key: {:?}", key);
                }
            }
        })
        .detach();
}

pub mod prelude {
    pub use crate::Message;
    pub use async_broadcast::{Receiver, Sender};
}
