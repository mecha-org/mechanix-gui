use anyhow::Result;
use interfaces::{sound_event_notification_stream, PowerBusInterface, SoundBusInterface};
use tokio::task::JoinHandle;
use zbus::connection;
mod config;
mod interfaces;

use config::read_configs_yml;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match read_configs_yml() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error reading configs: {}", e);
            std::process::exit(1);
        }
    };

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

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

    for handle in handles {
        handle.await?;
    }

    // wait in server loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
