use freedesktop_notifications_server::proxies::mechanix::NotificationsProxy;
use zbus::Connection;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    match proxy.get_all_notifications().await {
        Ok(notifications) => {
            println!("Retrieved {} notifications:", notifications.len());
            
            if notifications.is_empty() {
                println!("No active notifications found.");
            } else {
                for (id, notification) in notifications {
                    println!("\n--- Notification ID: {} ---", id);
                    println!("App Name: {}", notification.app_name);
                    println!("Summary: {}", notification.summary);
                    println!("Body: {}", notification.body);
                    println!("App Icon: {}", notification.app_icon);
                    println!("Replaces ID: {}", notification.replaces_id);
                    println!("Actions: {:?}", notification.actions);
                    println!("Expire Timeout: {}", notification.expire_timeout);
                    
                    // Check if notification has an image
                    if let Some(image) = notification.get_image() {
                        println!("Has Image: Yes");
                        
                        // Optionally save the image
                        let image_path = format!("notification_{}.png", id);
                        // match image.save_to_path(image_path.clone().into()) {
                        //     Ok(_) => println!("Image saved to: {}", image_path),
                        //     Err(e) => println!("Failed to save image: {}", e),
                        // }
                    } else {
                        println!("Has Image: No");
                    }
                    
                    println!("Hints count: {}", notification.hints.len());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get notifications: {}", e);
            return Err(e);
        }
    }

    Ok(())
}