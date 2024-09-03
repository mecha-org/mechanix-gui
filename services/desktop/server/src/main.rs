use std::thread::{self, JoinHandle};

mod dbus;
mod events;
mod handlers;
mod settings;

use anyhow::Result;
use dbus::interfaces::{
    sound_event_notification_stream, NotificationBusInterface, Notifier, PowerBusInterface,
    SoundBusInterface,
};
use handlers::{
    session::SessionHandler,
    shell::{security::SecurityHandler, upower::UpowerHandler},
};
use settings::{read_settings_yml, DesktopServerSettings};
use tokio::sync::mpsc;
use zbus::connection;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = match read_settings_yml() {
        Ok(s) => {
            println!("settings read successfully");
            s
        }
        Err(e) => {
            println!("error while reading settings.yml {:?}", e);
            DesktopServerSettings::default()
        }
    };

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    let sound_bus = SoundBusInterface {};
    let _sound_bus_connection = connection::Builder::session()?
        .name("org.mechanix.services.Sound")?
        .serve_at("/org/mechanix/services/Sound", sound_bus)?
        .build()
        .await?;

    let sound_handle = tokio::spawn(async move {
        if let Err(e) =
            sound_event_notification_stream(&sound_bus.clone(), &_sound_bus_connection).await
        {
            println!("Error in sound notification stream: {}", e);
        }
    });

    let power_bus = PowerBusInterface {};
    let _power_bus_connection = connection::Builder::session()?
        .name("org.mechanix.services.Power")?
        .serve_at("/org/mechanix/services/Power", power_bus)?
        .build()
        .await?;

    handles.push(sound_handle);

    let (event_tx, event_rx) = mpsc::channel(128);
    let notification_bus = NotificationBusInterface {
        event_tx: event_tx.clone(),
    };
    let _notification_bus_connection = connection::Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", notification_bus.clone())?
        .build()
        .await?;

    let notifier = Notifier::new(settings.notifier.clone());
    let notifier_handle = tokio::spawn(async move { notifier.run(event_tx, event_rx).await });
    handles.push(notifier_handle);

    let session_handler = SessionHandler::new(settings.clone());
    let session_handle = tokio::spawn(async move {
        session_handler.run().await;
    });
    handles.push(session_handle);

    // let home_button_handler = HomeButtonHandler::new(settings.home_button.clone());
    // let home_button_handle = tokio::spawn(async move {
    //     home_button_handler.run().await;
    // });
    // handles.push(home_button_handle);

    let security_handler = SecurityHandler::new();
    let security_handle = tokio::spawn(async move {
        security_handler.run().await;
    });
    handles.push(security_handle);

    let upower_handler = UpowerHandler::new();
    let upower_handle = tokio::spawn(async move {
        upower_handler.run().await;
    });
    handles.push(upower_handle);

    for handle in handles {
        handle.await?;
    }

    // wait in server loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
