use freedesktop_notifications_server::proxies::mechanix::NotificationsProxy;
use freedesktop_notifications_server::database::{get_all_notifications_from_db};
use zbus::Connection;
use futures_util::stream::StreamExt;
use tokio::select;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    let mut received_stream = proxy.receive_notification_received().await?;
    let mut closed_stream = proxy.receive_notification_closed().await?;

    // Print existing notifications count on startup
    match get_all_notifications_from_db().await {
        Ok(existing_notifications) => {
            println!("Database contains {} notifications", existing_notifications.len());
        }
        Err(e) => {
            eprintln!("Error getting existing notifications: {}", e);
        }
    }

    loop {
        select! {
            // Handles notification received
            msg = received_stream.next() => {
                match msg {
                    Some(signal) => {
                        match signal.args() {
                            Ok(args) => {
                                println!("Notification Received from: {:?}", args.notification.app_name);

                                // Save image if present
                                let image_path = format!("notification{}.png", args.id);
                                if let Some(notification_image) = args.notification.get_image() {
                                    println!("Notification has an Image/ Saved to {}", image_path);
                                    let _ = notification_image.save_to_path(image_path.into());
                                }
                                
                                // Print total notifications count
                                match get_all_notifications_from_db().await {
                                    Ok(all_notifications) => {
                                        println!("Total notifications in database: {}", all_notifications.len());
                                    }
                                    Err(e) => eprintln!("Error getting all notifications: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Error parsing notification_received: {}", e),
                        }
                    }
                    None => {
                        println!("notification_received stream ended");
                        break;
                    }
                }
            }
            
            // Handle notification closed
            msg = closed_stream.next() => {
                match msg {
                    Some(signal) => {
                        match signal.args() {
                            Ok(args) => {
                                println!("Notification Closed: ID {}", args.id);
                                
                                // Print current notifications count
                                match get_all_notifications_from_db().await {
                                    Ok(all_notifications) => {
                                        println!("Current notifications in database: {}", all_notifications.len());
                                    }
                                    Err(e) => eprintln!("Error getting all notifications: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Error parsing notification_closed: {}", e),
                        }
                    }
                    None => {
                        println!("notification_closed stream ended");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
