//! Example to get output devices using `get_sinks` method

use tokio::sync::mpsc;
use freedesktop_pulseaudio_client::service::PulseAudioService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = PulseAudioService::new()?;
    let sinks = service.get_sinks().await?;
    println!("Sinks: {:#?}", sinks);

    Ok(())
}
