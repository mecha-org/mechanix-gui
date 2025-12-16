//! Example file for a notification client, access the stream for new notifications
use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let notification_service = desktop_dbus::MechanixNotificationService::new().await?;
    let mut stream = notification_service.stream_receive_notification().await;
    while let Some(notification) = stream.next().await {
        println!("Notification received: {notification:?}");
    }

    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
