mod errors;
pub mod handlers;
mod interfaces;

use crate::interfaces::freedesktop::FreedesktopNotificationService;
use crate::interfaces::mechanix::NotificationService;
use anyhow::Result;
use log::info;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::ObjectPath;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("desktop session service: initializing...");

    // Setup freedesktop notification service
    let (freedesktop_connection, _service, receiver) =
        FreedesktopNotificationService::create_connection()
            .await
            .map_err(|e| {
                zbus::Error::Failure(format!("Failed to create freedesktop connection: {}", e))
            })?;

    let freedesktop_signal_emitter = SignalEmitter::from_parts(
        freedesktop_connection.clone(),
        ObjectPath::try_from("/org/freedesktop/Notifications")
            .map_err(|e| zbus::Error::Failure(format!("Invalid object path: {}", e)))?,
    );

    // Setup mechanix notification service
    let mut notificationbus = NotificationService::new_from_database()
        .await
        .expect("Unable to initialize notificaion Service");
    let notifications_clone = notificationbus.notifications.clone();

    notificationbus.set_signal_emmiter(freedesktop_signal_emitter);
    let connection = Connection::session().await?;

    connection
        .object_server()
        .at("/org/mechanix/NotificationManager", notificationbus.clone())
        .await?;

    connection
        .request_name("org.mechanix.NotificationManager")
        .await?;

    // gets the signal emitter for org.mechanix.NotificationManager
    let mechanix_signal_emitter = SignalEmitter::from_parts(
        connection.clone(),
        ObjectPath::try_from("/org/mechanix/NotificationManager")
            .map_err(|e| zbus::Error::Failure(format!("Invalid object path: {}", e)))?,
    );

    // starts event handling
    NotificationService::handle_event(
        receiver,
        mechanix_signal_emitter,
        notifications_clone,
    )
    .await;

    std::future::pending::<()>().await;
    Ok(())
}
