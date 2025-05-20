//! Basic example: Enable Bluetooth using mechanix_debus_client

use freedesktop_bluz_client::handler::{BluetoothRequest, BluezClient};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending Bluetooth requests
    let (bt_tx, bt_rx) = mpsc::channel(10);

    // Spawn the Bluetooth handler in a background task
    let _handler = tokio::spawn(async move {
        let mut bt_handler = BluezClient::new().await.unwrap();
        // Run the handler event loop
        let _ = bt_handler.run(bt_rx).await;
    });
    println!("handler spawned");
    let (res_tx, mut res_rx) = mpsc::channel(10);
    // Example: Enable Bluetooth
    let request = BluetoothRequest::GetAvailableDevices {
        discovery_duration: 5000,
        reply_to: res_tx,
    };
    bt_tx
        .try_send(request)
        .expect("Failed to send Bluetooth request");
    println!("Bluetooth enable request sent");

    let handler = tokio::spawn(async move {
        while let Some(result) = res_rx.recv().await {
            match result {
                Ok(devices) => println!("Available devices: {:?}", devices),
                Err(e) => eprintln!("Error getting available devices: {e}"),
            }
        }
    });
    // Await the result and log it
    handler.await?;

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
