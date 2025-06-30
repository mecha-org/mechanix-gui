use tokio;
use freedesktop_notifications_server::notification::{ Notification };
use freedesktop_notifications_server::handler::NotificationHandler;

// create a default handler implementation
pub struct DefaultHandler;

// this trait can be further implemented to a GUI element and would be run when each event is recieve Specific Events { Close, Replace, Show}
#[async_trait::async_trait]
impl NotificationHandler for DefaultHandler {
    async fn show_notification(&self, id: u32, notification: &Notification) {
        notification.print(id);
    }
    async fn close_notification(&self, id: u32) {}

    async fn replace_notification(&self, id: u32, notification: &Notification) {
        println!("notification: {id} replaced by {:?} ", notification)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notif = DefaultHandler;
    let event_task = notif.start_service();

    let _ = event_task.await.unwrap();

    Ok(())
}
