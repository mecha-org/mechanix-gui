use freedesktop_notifications_server::interfaces::mechanix::MechanixNotificationService;
use tokio;
use zbus::{ Result };

#[tokio::main]
async fn main() -> Result<()> {
    MechanixNotificationService::start_service().await
}
