use zbus::{ object_server::SignalEmitter, Connection, interface };
use zvariant::ObjectPath;
use tokio::sync::mpsc::Receiver;
use crate::interfaces::freedesktop::NotificationService;
use crate::interfaces::freedesktop::{ Event, Notification };
#[derive(Debug, Clone)]
pub struct MechanixNotificationService {}

impl MechanixNotificationService {
    pub fn new() -> Self {
        Self {}
    }

    // Start processing events in a background task - static method
    pub async fn handle_event(
        mut event_receiver: Receiver<Event>,
        signal_emitter: SignalEmitter<'static>
    ) {
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                match event {
                    Event::Show(id, notification) => {
                        // dbg!(&notification);
                        let _ = Self::notification_received(
                            &signal_emitter,
                            id,
                            &notification
                        ).await;
                    }
                    Event::Replace(id, notification) => {
                        let _ = Self::notification_replaced(
                            &signal_emitter,
                            id,
                            &notification
                        ).await;
                    }
                    Event::Close(id) => {
                        let _ = Self::notification_closed(&signal_emitter, id).await;
                    }
                }
            }
        });
    }

    /// Start the complete notification service including freedesktop and mechanix interfaces
    pub async fn start_service() -> zbus::Result<()> {
        // Setup freedesktop notification service
        let (freedesktop_connection, service, receiver) =
            NotificationService::create_connection().await.map_err(|e|
                zbus::Error::Failure(format!("Failed to create freedesktop connection: {}", e))
            )?;

        // Setup mechanix notification service
        let notificationbus = Self::new();
        let connection = Connection::session().await?;

        connection
            .object_server()
            .at("/org/mechanix/Notifications", notificationbus.clone()).await?;

        connection.request_name("org.mechanix.Notifications").await?;

        // gets the signal emitter for org.mechanix.Notifications 
        let signal_emitter = SignalEmitter::from_parts(
            connection.clone(),
            ObjectPath::try_from("/org/mechanix/Notifications").map_err(|e|
                zbus::Error::Failure(format!("Invalid object path: {}", e))
            )?
        );

        // starts event handling
        Self::handle_event(receiver, signal_emitter).await;

        std::future::pending::<()>().await;
        Ok(())
    }
}

#[interface(name = "org.mechanix.Notifications")]
impl MechanixNotificationService {
    async fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    #[zbus(signal)]
    async fn notification_received(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        notification: &Notification
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_replaced(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        notification: &Notification
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(signal_ctxt: &SignalEmitter<'_>, id: u32) -> zbus::Result<()>;
}
