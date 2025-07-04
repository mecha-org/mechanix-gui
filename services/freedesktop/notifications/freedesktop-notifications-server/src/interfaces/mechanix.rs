use zbus::{ object_server::SignalEmitter, Connection, interface, fdo };
use zvariant::ObjectPath;
use tokio::sync::mpsc::Receiver;
use crate::interfaces::freedesktop::NotificationService;
use crate::interfaces::freedesktop::{ Event };
use crate::notification::Notification;

#[derive(Debug, Clone)]
pub struct MechanixNotificationService {
    freedesktop_signal_emitter: Option<SignalEmitter<'static>>,
}

impl MechanixNotificationService {
    pub fn new() -> Self {
        Self { freedesktop_signal_emitter: None }
    }

    pub fn set_signal_emmiter(&mut self, signal_emitter: SignalEmitter<'static>) {
        self.freedesktop_signal_emitter = Some(signal_emitter);
    }

    // Start processing events in a background task 
    pub async fn handle_event(
        mut event_receiver: Receiver<Event>,
        signal_emitter: SignalEmitter<'static>
    ) {
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                match event {
                    Event::Show(id, notification) => {
                        let _ = Self::notification_received(
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

        let freedesktop_signal_emitter = SignalEmitter::from_parts(
            freedesktop_connection.clone(),
            ObjectPath::try_from("/org/freedesktop/Notifications").map_err(|e|
                zbus::Error::Failure(format!("Invalid object path: {}", e))
            )?
        );

        // Setup mechanix notification service
        let mut notificationbus = Self::new();

        notificationbus.set_signal_emmiter(freedesktop_signal_emitter);
        let connection = Connection::session().await?;

        connection
            .object_server()
            .at("/org/mechanix/Notifications", notificationbus.clone()).await?;

        connection.request_name("org.mechanix.Notifications").await?;

        // gets the signal emitter for org.mechanix.Notifications
        let mechanix_signal_emitter = SignalEmitter::from_parts(
            connection.clone(),
            ObjectPath::try_from("/org/mechanix/Notifications").map_err(|e|
                zbus::Error::Failure(format!("Invalid object path: {}", e))
            )?
        );

        // starts event handling
        Self::handle_event(receiver, mechanix_signal_emitter).await;

        std::future::pending::<()>().await;
        Ok(())
    }
}

#[interface(name = "org.mechanix.Notifications")]
impl MechanixNotificationService {
    async fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Invoke an action on a notification, triggering the ActionInvoked signal
    /// on the org.freedesktop.Notifications interface
    async fn invoke_action(&self, id: u32, action_key: &str) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            NotificationService::action_invoked(emitter, id, action_key).await.map_err(|e|
                fdo::Error::Failed(format!("Failed to invoke action: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    /// sends activation token signal on the org.freedesktop.Notifications interface
    async fn send_activation_token(&self, id: u32, activation_token: &str) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            NotificationService::activation_token(emitter, id, activation_token).await.map_err(|e|
                fdo::Error::Failed(format!("Failed to send activation token: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    /// Close a notification with a specific reason
    async fn close_notification_with_reason(&self, id: u32, reason: u32) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            NotificationService::notification_closed(emitter, id, reason).await.map_err(|e|
                fdo::Error::Failed(format!("Failed to close notification: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    #[zbus(signal)]
    async fn notification_received(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        notification: &Notification
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(signal_ctxt: &SignalEmitter<'_>, id: u32) -> zbus::Result<()>;
}
