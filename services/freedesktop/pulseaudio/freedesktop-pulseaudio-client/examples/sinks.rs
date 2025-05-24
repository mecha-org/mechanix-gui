//! Example to get output devices using `get_sinks` method

use freedesktop_pulseaudio_client::handler::{PulseAudioRequest, PulseAudioClient};
use tokio::sync::mpsc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending PulseAudio requests
    let (pactl_tx, pactl_rx) = mpsc::channel(10);

    // Spawn the PulseAudio handler in a background task
    let _handler = tokio::spawn(async move {
        let mut bt_handler = PulseAudioClient::new();
        // Run the handler event loop
        let _ = bt_handler.run(pactl_rx).await;
    });
    println!("handler spawned");
    let (res_tx, res_rx) = mpsc::channel(10);
    let request = PulseAudioRequest::GetSinks { reply_to: res_tx };
    pactl_tx
        .try_send(request)
        .expect("Failed to send PulseAudio request");
    println!("PulseAudio get_sink request sent");

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
