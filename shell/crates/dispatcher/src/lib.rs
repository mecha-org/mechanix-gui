use async_broadcast::{Receiver, Sender, broadcast};
use futures::StreamExt;
use gpui::*;
use mxconf_dbus::watch_setting;

#[derive(Clone)]
pub struct Dispatcher(pub Sender<Message>, pub Receiver<Message>);

impl Dispatcher {
    pub fn channel(&self) -> (Sender<Message>, Receiver<Message>) {
        (self.0.clone(), self.1.clone())
    }
}

impl Global for Dispatcher {}

#[derive(Debug, Clone)]
pub enum Message {
    SetThemeMode(String),
    SetThemeColors {
        accent: String,
        background: String,
        foreground: String,
    },
    SetKeyboardAlwayson(bool),
    ShowPowerOptions(bool),
    VolumeUp,
    VolumeDown,
    LaunchApp {
        app_id: String,
        exec: String,
    },
}

pub fn init(cx: &mut App) {
    let (mut tx, rx) = broadcast(120);
    tx.set_overflow(true);
    cx.set_global(Dispatcher(tx.clone(), rx.clone()));

    cx.background_executor()
        .spawn(async move {
            let schema = "org.mechanix.launcher";
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
