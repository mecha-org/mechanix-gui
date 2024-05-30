use std::{
    collections::HashMap, fmt::Error, io, process::Child, str::FromStr, string::ParseError,
    time::Duration,
};

use logind::session_lock;
use mechanix_system_dbus_client::display::Display;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use wayland_protocols_async::ext_idle_notify_v1::handler::{IdleNotifyEvent, IdleNotifyHandler};

use crate::settings::idle_notify::IdleNotifySettings;
#[derive(Debug, Clone, Copy)]
pub enum NotifyEvents {
    Display,
    Lock,
}

impl ToString for NotifyEvents {
    fn to_string(&self) -> String {
        match self {
            NotifyEvents::Display => "Display".to_string(),
            NotifyEvents::Lock => "Lock".to_string(),
        }
    }
}

impl FromStr for NotifyEvents {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Display" => Ok(NotifyEvents::Display),
            "Lock" => Ok(NotifyEvents::Lock),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
}

pub struct IdleNotifyHandlerClient {
    configs: IdleNotifySettings,
}

impl IdleNotifyHandlerClient {
    pub fn new(configs: IdleNotifySettings) -> IdleNotifyHandlerClient {
        IdleNotifyHandlerClient { configs }
    }

    pub async fn run(mut self) {
        println!("IdleNotifyHandler::run()");
        // create mpsc channel for receiving events from the idle_notify handler
        let (idle_notify_event_tx, mut idle_notify_event_rx) = mpsc::channel(128);

        let periods = self.configs.periods.clone();

        let mut subscribers = HashMap::new();

        subscribers.insert(
            NotifyEvents::Display.to_string(),
            Duration::from_secs(periods.display_off as u64),
        );
        subscribers.insert(
            NotifyEvents::Lock.to_string(),
            Duration::from_secs(periods.screen_lock as u64),
        );

        // create the handler instance
        let mut idle_notify_handler = IdleNotifyHandler::new(subscribers, idle_notify_event_tx);

        // start the idle_notify handler
        let idle_notify_t = tokio::spawn(async move {
            let _ = idle_notify_handler.run().await;
        });

        // receive all idle_notify events
        let idle_notify_event_t = tokio::spawn(async move {
            loop {
                let msg = idle_notify_event_rx.recv().await;
                if msg.is_none() {
                    continue;
                }

                if let Some(event) = msg {
                    println!("IdleNotifyHandlerClient::run event {:?}", event);
                    match event {
                        IdleNotifyEvent::Idled { key } => {
                            if let Ok(notify_event) = NotifyEvents::from_str(&key) {
                                match notify_event {
                                    NotifyEvents::Display => {
                                        //Turn backlight off
                                        println!("Turning off display");
                                        let _ = Display::set_backlight_off().await;
                                    }
                                    NotifyEvents::Lock => {
                                        println!("Locking");
                                        let _ = session_lock().await;
                                    }
                                }
                            };
                        }
                        IdleNotifyEvent::Resumed { key } => {
                            if let Ok(notify_event) = NotifyEvents::from_str(&key) {
                                match notify_event {
                                    NotifyEvents::Display => {
                                        println!("Turning on display");
                                        //Turn backlight on
                                        let _ = Display::set_backlight_on().await;
                                    }
                                    NotifyEvents::Lock => {
                                        //Ignore
                                    }
                                }
                            };
                        }
                    }
                }
            }
        });

        let _ = idle_notify_t.await.unwrap();
        let _ = idle_notify_event_t.await.unwrap();
    }
}
