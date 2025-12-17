//! Example file to show all notifications
use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let notification_service = desktop_dbus::MechanixNotificationService::new().await?;
    let all_notifications = notification_service.get_all_notifications().await;
    
    for notification in all_notifications? {
        println!("{:?}", notification);
    }

    Ok(())
}

// Place here the get_setting function as you defined it (as in your code sample),
// along with the ConfigServerProxy definition, or import them from your module.
