//! Basic example: Scan available Bluetooth devices using freedesktop-bluez-client

use freedesktop_bluez_client::error::BluezError;
use freedesktop_bluez_client::handler::{BluezRequest, BluezClient};
use tokio::sync::{mpsc, oneshot};

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
    let (res_tx, res_rx) = oneshot::channel();
    // Example: Enable Bluetooth
    let request = BluezRequest::SetPoweredOn { reply_to: res_tx };
    bt_tx
        .try_send(request)
        .expect("Failed to send Bluetooth request");
    println!("Bluetooth enable request sent");

    if let Ok(result) = res_rx.await {
        match result {
            Ok(percentage) => println!("bluetooth powered on"),
            Err(e) => match e {
                BluezError::Generic => {
                    println!("Generic error occurred");
                }
                BluezError::ProxyError(err) => {
                    println!("Proxy error occurred: {:?}", err);
                }
                BluezError::CreateSystemBusError(_) => {
                    println!("Failed to create system bus");
                }
                BluezError::CreateBluezProxyError(_) => {
                    println!("Failed to create Bluez proxy");
                }
                BluezError::CreateAdapterProxyError(_) => {
                    println!("Failed to create adapter proxy");
                }
                _ => {
                    println!("An unexpected error occurred: {:?}", e);
                }
            },
        }
    } else {
        eprintln!("Did not receive a response for battery percentage");
    }

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
