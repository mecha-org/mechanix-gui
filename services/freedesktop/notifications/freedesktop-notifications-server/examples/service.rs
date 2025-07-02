use freedesktop_notifications_server::interfaces::freedesktop::NotificationService;
use freedesktop_notifications_server::interfaces::mechanix::MechanixNotificationService;
use tokio;
use zbus::{ blocking::object_server, conn, object_server::SignalEmitter, Connection, Result };
use zvariant::ObjectPath;

#[tokio::main]
async fn main() -> Result<()> {
    MechanixNotificationService::start_service().await
}
