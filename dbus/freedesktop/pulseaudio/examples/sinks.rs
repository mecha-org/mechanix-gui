//! Example to get output devices using `get_sinks` method

use pulseaudio::service::PulseAudioService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = PulseAudioService::new()?;
    let available_sinks = service.handle.get_sinks().await?;
    println!("{:#?}", available_sinks);
    service.handle.shutdown().await;

    Ok(())
}
