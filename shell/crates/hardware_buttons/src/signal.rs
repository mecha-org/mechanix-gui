use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use futures_timer::Delay;
use futures_util::stream::FuturesUnordered;
use gpui::App;
use hw_buttons::{Key,KeyEvent};
use log::{error, info, warn};
use std::time::Duration;
use zbus::{proxy, Connection};

const HW_BUTTON_SERVICE: &str = "org.mechanix.services.HwButton";
const BUTTON_PATHS: &[(&str, &str)] = &[
    ("Power", "/org/mechanix/services/HwButton/Power"),
    ("Home", "/org/mechanix/services/HwButton/Home"),
    ("VolumeUp", "/org/mechanix/services/HwButton/VolumeUp"),
    ("VolumeDown", "/org/mechanix/services/HwButton/VolumeDown"),
    (
        "ExtensionDetection",
        "/org/mechanix/services/HwButton/ExtensionDetection",
    ),
];

#[proxy(interface = "org.mechanix.services.HwButton")]
trait HwButtonDbusInterface {
    #[zbus(signal)]
    fn notification(&self, event: KeyEvent);
}

pub fn init(cx: &mut App) {
    let (tx, mut rx) = mpsc::channel::<KeyEvent>(32);

    cx.spawn(async move |app| {
        while let Some(event) = rx.next().await {
            let _ = app.update(|cx| {
                // I have to implement something to send dispatcher signals here
                // handle::handle_event(cx, event);
            });
        }
    })
    .detach();

    let executor = cx.background_executor();
    executor
        .spawn(async move {
            if let Err(err) = spawn_button_watchers(tx.clone()).await {
                error!("hardware button listeners failed: {err:?}");
            }
        })
        .detach();
}

async fn spawn_button_watchers(tx: mpsc::Sender<KeyEvent>) -> Result<()> {
    let mut watchers = FuturesUnordered::new();

    for &(label, path) in BUTTON_PATHS {
        let sender = tx.clone();
        let label = label.to_string();
        let path = path.to_string();
        watchers.push(async move {
            loop {
                match listen_on_path(&label, &path, sender.clone()).await {
                    Ok(()) => break,
                    Err(err) => {
                        warn!("listener for {label} at {path} dropped: {err:?}, retrying shortly");
                        Delay::new(Duration::from_millis(750)).await;
                    }
                }
            }
        });
    }

    while watchers.next().await.is_some() {}
    Ok(())
}

async fn listen_on_path(label: &str, path: &str, mut tx: mpsc::Sender<KeyEvent>) -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = HwButtonDbusInterfaceProxy::builder(&connection)
        .destination(HW_BUTTON_SERVICE)?
        .path(path)?
        .build()
        .await?;

    let mut stream = proxy.receive_notification().await?;
    while let Some(signal) = stream.next().await {
        if let Ok(payload) = signal.args() {
            let event = payload.event;
            info!(
                target: "hw-buttons",
                "received {label} signal from {path}: {event:?}"
            );
            if tx.send(event).await.is_err() {
                warn!("volume event channel closed; stopping {label} listener");
                break;
            }
        } else {
            warn!("{label} notification payload could not be decoded");
        }
    }

    Ok(())
}

pub fn handle_event(cx: &mut App, event: KeyEvent) {
    // let state = cx.global_mut::<HardwareState>();
    match event {
        KeyEvent::Pressed(Key::Home) => {
            println!("[hardware-buttons] Home button short press: {:?}", event);
        }
        KeyEvent::Pressing(Key::Home) => {
            println!(
                "[hardware-buttons] Home button long press detected: {:?}",
                event
            );
        }
        KeyEvent::Released(Key::Home) => {
            println!("[hardware-buttons] Home button released: {:?}", event);
        }
        KeyEvent::Pressed(Key::Power) => {
            println!("[hardware-buttons] Power button short press: {:?}", event);
        }
        KeyEvent::Pressing(Key::Power) => {
            println!("[hardware-buttons] Power button long press detected, showing power overlay");
        }
        KeyEvent::Released(Key::Power) => {
            println!("[hardware-buttons] Power button released: {:?}", event);
        }
        KeyEvent::Pressed(Key::VolumeUp)
        | KeyEvent::Pressing(Key::VolumeUp)
        | KeyEvent::Unknown(Key::VolumeUp) => {}
        KeyEvent::Pressed(Key::VolumeDown)
        | KeyEvent::Pressing(Key::VolumeDown)
        | KeyEvent::Unknown(Key::VolumeDown) => {}
        _ => {}
    }
}
