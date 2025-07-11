use freedesktop_notifications_server::proxies::mechanix::MechanixNotificationProxy;
use zbus::Connection;
use futures_util::stream::StreamExt;
use tokio::select;
#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = MechanixNotificationProxy::new(&connection).await?;

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
                                println!("Notification Received from: {:?}",args.notification.app_name);
                                let notif_id = args.id;
                                let image_path = format!("notification{}.png",notif_id);
                                if let Some(notification_image) = args.notification.get_image(){
                                    println!("Notiification has an Image/ Saved to {}",image_path);
                                    let _ = notification_image.save_to_path(image_path.into());
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
