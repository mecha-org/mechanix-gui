use async_broadcast::broadcast;
use futures::StreamExt;
use gpui::*;
use mxconf_dbus::watch_setting;

#[derive(Clone)]
pub struct Dispatcher(pub async_broadcast::Receiver<Message>);

impl Global for Dispatcher {}

#[derive(Debug, Clone)]
pub enum Message {
    SetTheme(SharedString),
    SetKeyboardAlwayson(bool),
}

pub fn init(cx: &mut App) {
    let (tx, rx) = broadcast::<Message>(120);
    cx.set_global(Dispatcher(rx));

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
    pub use async_broadcast::Receiver;
}
