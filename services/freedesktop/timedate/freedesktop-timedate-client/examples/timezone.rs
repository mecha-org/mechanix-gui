//! Basic example: Get the current timezone using freedesktop-timedate-client

use freedesktop_timedate_client::handler::{TimeDateClient, TimeDateRequest};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending TimeDate requests
    let (tx, rx) = mpsc::channel(10);

    // Spawn the Timedate handler in a background task
    let _handler = tokio::spawn(async move {
        let mut handler = TimeDateClient::new()
            .await
            .expect("Failed to create TimeDate client");
        // Run the handler event loop
        let _ = handler.run(rx).await;
    });
    println!("handler spawned");
    let (res_tx, mut res_rx) = oneshot::channel();
    // Example: Enable Timedate
    let request = TimeDateRequest::GetTimeZone { reply_to: res_tx };
    // Send the request to the handler
    match tx.send(request).await {
        Ok(_) => println!("GetTimeZone request sent"),
        Err(e) => eprintln!("Failed to send GetTimeZone request: {e}"),
    };
    
    let handler = tokio::spawn(async move {
        println!("Waiting for response...");
        match res_rx.await {
            Ok(result) => match result {
                Ok(timezone) => println!("Current timezone: {}", timezone),
                Err(e) => eprintln!("Error getting timezone: {}", e),
            },
            Err(e) => eprintln!("Failed to receive response: {e}"),
        }
    });
    // Await the result and log it
    handler.await?;

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
