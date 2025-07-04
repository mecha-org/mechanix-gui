use freedesktop_notifications_server::proxies::mechanix::NotificationsProxy;
use freedesktop_notifications_server::notification::Notification;
use zbus::Connection;
use futures_util::stream::StreamExt;
use tokio::select;
use std::path::PathBuf;
use std::str::FromStr;
#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    let mut received_stream = proxy.receive_notification_received().await?;
    let mut closed_stream = proxy.receive_notification_closed().await?;

    loop {
        select! {
            // Handles notification received
            msg = received_stream.next() => {
                match msg {
                    Some(signal) => {
                        match signal.args() {
                            Ok(args) => {
                                println!("Notification Received: {:?}",args.notification);
                                let notif_id = args.id;
                                // dbg!(notif_id);
                                let image_path = format!("notification{}.png",notif_id);
                                let _saved = args.notification.get_image().unwrap().save_to_path(PathBuf::from_str(&image_path).unwrap()); 
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
