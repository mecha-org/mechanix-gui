//! Basic example: Enable Wifi using freedesktop-network-manager-client

use networkmanager::service::NetworkManagerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let network_manager = NetworkManagerService::new().await?;
    let receiver = network_manager.stream_access_point_events().await;
    // Spawn the NetworkManager handler in a background task
    // Wait for the response
    let handler = tokio::spawn(async move {
        while let Ok(result) = receiver.recv() {
            match result {
                Ok(new_access_point) => println!("access_point event: {:?}", new_access_point),
                Err(e) => eprintln!("Error getting wifi state: {e}"),
            }
        }
    });

    // Await the result and log it
    handler.await.unwrap();

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
