use freedesktop_notifications_server::proxies::mechanix::NotificationsProxy;
use freedesktop_notifications_server::interfaces::freedesktop::Notification;
use zbus::Connection;
use futures_util::stream::StreamExt;
use tokio::select;

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
