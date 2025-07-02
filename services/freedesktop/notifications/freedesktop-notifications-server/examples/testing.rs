use freedesktop_notifications_server::interfaces::freedesktop::NotificationService;
use freedesktop_notifications_server::interfaces::mechanix::MechanixNotificationService;
use tokio;
use zbus::{ blocking::object_server, conn, object_server::SignalEmitter, Connection, Result };
use zvariant::ObjectPath;

#[tokio::main]
async fn main() -> Result<()> {
    let (freedesktop_connection, service, receiver) =
        NotificationService::create_connection().await.unwrap();

    let notificationbus = MechanixNotificationService::new();
    let connection = Connection::session().await.unwrap();
    connection
        .object_server()
        .at("/org/mechanix/Notifications", notificationbus.clone()).await
        .unwrap();
    connection.request_name("org.mechanix.Notifications").await.unwrap();

    // Get the signal emitter for the registered interface
    let signal_emitter = SignalEmitter::from_parts(
        connection.clone(),
        ObjectPath::try_from("/org/mechanix/Notifications").unwrap()
    );

    // Start event handling
    MechanixNotificationService::handle_event(receiver, signal_emitter).await;

    // Keep the service running by waiting on the connection
    // This will run indefinitely until the connection is closed
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
