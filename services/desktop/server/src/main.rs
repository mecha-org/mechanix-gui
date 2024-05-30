use std::thread::{self, JoinHandle};

mod dbus;
mod events;
mod handlers;
mod settings;

use anyhow::Result;
use dbus::interfaces::{
    sound_event_notification_stream, PowerBusInterface, SecurityBusInterface, SoundBusInterface,
};
use handlers::{
    session::SessionHandler,
    shell::{home_button::HomeButtonHandler, security::SecurityHandler},
};
use settings::{read_settings_yml, DesktopServerSettings};
use tokio::runtime::Builder;
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
    let session_handler = SessionHandler::new(settings.clone());
    let session_handle = tokio::spawn(async move {
        session_handler.run().await;
    });
    handles.push(session_handle);

    let home_button_handler = HomeButtonHandler::new(settings.home_button);
    let home_button_handle = tokio::spawn(async move {
        home_button_handler.run().await;
    });
    handles.push(home_button_handle);

    let security_bus = SecurityBusInterface {};
    let _security_bus_connection = connection::Builder::session()?
        .name("org.mechanix.services.Security")?
        .serve_at("/org/mechanix/services/Security", security_bus)?
        .build()
        .await?;

    let security_handler = SecurityHandler::new();
    let security_handle = tokio::spawn(async move {
        security_handler.run().await;
    });
    handles.push(security_handle);

    for handle in handles {
        handle.await?;
    }

    // wait in server loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
