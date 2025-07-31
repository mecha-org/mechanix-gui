//! Example to get output devices using `get_sinks` method

use tokio::sync::mpsc;
use freedesktop_pulseaudio_client::service::PulseAudioService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = PulseAudioService::new()?;
    match service.server.get_sinks() {
        Ok(sinks) => println!("Available sinks: {sinks:?}"),
        Err(e) => println!("Error getting sinks: {e}"),
    }
    Ok(())
}
