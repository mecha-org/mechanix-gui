use tokio::sync::mpsc::Receiver;
use crate::service::{ Event, NotificationService };
use crate::notification::Notification;
use crate::errors::{ Result, Error };
use async_trait::async_trait;
use zbus::Connection;

#[async_trait]
pub trait NotificationHandler {
    async fn show_notification(&self, id: u32, notification: &Notification);

    async fn close_notification(&self, id: u32);

    async fn replace_notification(&self, id: u32, notification: &Notification);

    // get the events from the server and calls these functions above accordingly for further extension 
    async fn handle_events(&self, mut receiver: Receiver<Event>) {
        while let Some(event) = receiver.recv().await {
            match event {
                Event::Show(id, notification) => {
                    self.show_notification(id, &notification).await;
                }
                Event::Close(id) => {
                    self.close_notification(id).await;
                }
                Event::Replace(id, notification) => {
                    self.replace_notification(id, &notification).await;
                }
            }
        }
    }

    // creates and returns a D-Bus connection, reciever, and the NotificationService 
    async fn create_connection(
        &self
    ) -> Result<(Connection, NotificationService, Receiver<Event>)> {
        let connection = Connection::session().await.map_err(|e|
            Error::DbusString(format!("Failed to create D-Bus connection: {}", e))
        )?;

        let (notification_service, receiver) = NotificationService::new();
        connection
            .object_server()
            .at("/org/freedesktop/Notifications", notification_service.clone()).await
            .map_err(|e| Error::DbusString(format!("Failed to register object: {}", e)))?;
        connection
            .request_name("org.freedesktop.Notifications").await
            .map_err(|e| Error::DbusString(format!("Failed to request D-Bus name: {}", e)))?;
        Ok((connection, notification_service, receiver))
    }

    async fn start_service(&self) -> Result<()> {
        let (_connection, _service, receiver) = self.create_connection().await?;

        // handles events - this will run indefinitely when awaited
        self.handle_events(receiver).await;

        Ok(())
    }
}
