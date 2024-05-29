use std::thread::{self, JoinHandle};

use anyhow::Result;
use interfaces::{sound_event_notification_stream, PowerBusInterface, SoundBusInterface};
use session::SessionHandler;
use tokio::runtime::Builder;
use zbus::connection;
mod interfaces;
use mechanix_desktop_settings::{
    idle_notify::IdleNotifySettings, read_settings_yml, DesktopServerSettings,
};

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
    let session_handler = SessionHandler::new(settings);
    let session_handle = tokio::spawn(async move {
        session_handler.run().await;
    });
    handles.push(session_handle);

    for handle in handles {
        handle.await?;
    }

    // wait in server loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
