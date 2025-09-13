//! Basic example: Scan available Bluetooth devices using freedesktop-bluez-client

use bluez::service::BluetoothService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending Bluetooth requests
    let bluetooth_service = BluetoothService::new().await?;
    let discovery_durations:core::time::Duration = core::time::Duration::from_secs(5);
    let available_devices = match bluetooth_service.get_available_devices(discovery_durations).await {
        Ok(devices) => devices,
        Err(e) => {
            eprintln!("Error getting available devices: {e}");
            return Ok(());
        }
    };

    for device in available_devices {
        println!("Discovered Bluetooth device: {:?}", device);
    }

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
