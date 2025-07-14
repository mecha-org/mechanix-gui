use zbus::{ object_server::SignalEmitter, Connection, interface, fdo };
use zvariant::ObjectPath;
use tokio::sync::mpsc::Receiver;
use crate::interfaces::freedesktop::FreedesktopNotificationService;
use crate::interfaces::freedesktop::{ FreedesktopNotificationEvent };
use crate::notification::{ self, Notification };
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::database::{
    add_notification_to_db,
    remove_notification_from_db,
    get_all_notifications_from_db,
};
use serde::{ Serialize, Deserialize };
#[derive(Debug, Clone)]
pub struct MechanixNotificationService {
    freedesktop_signal_emitter: Option<SignalEmitter<'static>>,
    // Add shared storage for notifications as HashMap
    notifications: Arc<RwLock<HashMap<u32, Notification>>>,
}

impl MechanixNotificationService {
    pub fn new() -> Self {
        Self {
            freedesktop_signal_emitter: None,
            notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn new_from_database() -> Result<Self, Box<dyn std::error::Error>> {
        let existing_notifications = get_all_notifications_from_db().await?;
        Ok(Self {
            freedesktop_signal_emitter: None,
            notifications: Arc::new(RwLock::new(existing_notifications)),
        })
    }

    pub fn set_signal_emmiter(&mut self, signal_emitter: SignalEmitter<'static>) {
        self.freedesktop_signal_emitter = Some(signal_emitter);
    }

    // Start processing events in a background task
    pub async fn handle_event(
        mut event_receiver: Receiver<FreedesktopNotificationEvent>,
        signal_emitter: SignalEmitter<'static>,
        notifications: Arc<RwLock<HashMap<u32, Notification>>>
    ) {
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                match event {
                    FreedesktopNotificationEvent::Notify(id, notification) => {
                        /// Store the notification
                        {
                            let mut notifs = notifications.write().await;
                            notifs.insert(id, notification.clone());
                        }

                        /// Store in database
                        /// "resident" :BOOLEAN --> When set the server will not automatically remove the notification when an action has been invoked. The notification will remain resident in the server until it is explicitly removed by the user or by the sender. This hint is likely only useful when the server has the "persistence" capability. 
                        if let Err(e) = add_notification_to_db(id, &notification).await {
                            eprintln!("Failed to add notification to database: {}", e);
                        }

                        let _ = Self::notification_received(
                            &signal_emitter,
                            id,
                            &notification
                        ).await;
                    }
                    FreedesktopNotificationEvent::Close(id) => {
                        // Remove the notification
                        {
                            let mut notifs = notifications.write().await;
                            notifs.remove(&id);
                        }

                        if let Err(e) = remove_notification_from_db(id).await {
                            eprintln!("Failed to remove notification from database: {}", e);
                        }

                        let _ = Self::notification_closed(&signal_emitter, id).await;
                    }
                }
            }
        });
    }

    /// Start the complete notification service including freedesktop and mechanix interfaces
    pub async fn start_service() -> zbus::Result<()> {
        // Setup freedesktop notification service
        let (freedesktop_connection, _service, receiver) =
            FreedesktopNotificationService::create_connection().await.map_err(|e|
                zbus::Error::Failure(format!("Failed to create freedesktop connection: {}", e))
            )?;

        let freedesktop_signal_emitter = SignalEmitter::from_parts(
            freedesktop_connection.clone(),
            ObjectPath::try_from("/org/freedesktop/Notifications").map_err(|e|
                zbus::Error::Failure(format!("Invalid object path: {}", e))
            )?
        );

        // Setup mechanix notification service
        let mut notificationbus = Self::new_from_database().await.expect("Unable to initialize notificaion Service");
        let notifications_clone = notificationbus.notifications.clone();

        notificationbus.set_signal_emmiter(freedesktop_signal_emitter);
        let connection = Connection::session().await?;

        connection
            .object_server()
            .at("/org/mechanix/NotificationManager", notificationbus.clone()).await?;

        connection.request_name("org.mechanix.NotificationManager").await?;

        // gets the signal emitter for org.mechanix.NotificationManager
        let mechanix_signal_emitter = SignalEmitter::from_parts(
            connection.clone(),
            ObjectPath::try_from("/org/mechanix/NotificationManager").map_err(|e|
                zbus::Error::Failure(format!("Invalid object path: {}", e))
            )?
        );

        // starts event handling
        Self::handle_event(receiver, mechanix_signal_emitter, notifications_clone).await;

        std::future::pending::<()>().await;
        Ok(())
    }
}

#[interface(name = "org.mechanix.NotificationManager")]
impl MechanixNotificationService {
    async fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Invoke an action on a notification, triggering the ActionInvoked signal
    /// on the org.freedesktop.Notifications interface
    async fn invoke_action(&self, id: u32, action_key: &str) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            FreedesktopNotificationService::action_invoked(emitter, id, action_key).await.map_err(
                |e| fdo::Error::Failed(format!("Failed to invoke action: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    /// sends activation token signal on the org.freedesktop.Notifications interface
    async fn send_activation_token(&self, id: u32, activation_token: &str) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            FreedesktopNotificationService::activation_token(
                emitter,
                id,
                activation_token
            ).await.map_err(|e|
                fdo::Error::Failed(format!("Failed to send activation token: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    /// Close a notification with a specific reason
    async fn close_notification_with_reason(&self, id: u32, reason: u32) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            FreedesktopNotificationService::notification_closed(emitter, id, reason).await.map_err(
                |e| fdo::Error::Failed(format!("Failed to close notification: {}", e))
            )
        } else {
            Err(fdo::Error::Failed("Freedesktop signal emitter not available".to_string()))
        }
    }

    /// Get all active notifications
    async fn get_all_notifications(&self) -> HashMap<u32, Notification> {
        let notifications = self.notifications.read().await;
        notifications.clone()
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
