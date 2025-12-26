//! Example file to show all notifications
use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let notification_service = desktop_dbus::NotificationService::new().await?;
    let all_notifications = notification_service.fetch_all().await;
    
    for notification in all_notifications? {
        println!("{:?}", notification);
    }

    Ok(())
}
