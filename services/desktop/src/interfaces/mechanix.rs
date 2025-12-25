use crate::handlers::notification::{Notification, SerializableNotification};
use crate::interfaces::database::{
    add_notification_to_db, get_all_notifications_from_db, remove_notification_from_db,
};
use crate::interfaces::freedesktop::FreedesktopNotificationEvent;
use crate::interfaces::freedesktop::FreedesktopNotificationService;
use futures::channel::mpsc::Receiver;
use futures::executor::ThreadPool;
use futures_util::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use zbus::{fdo, interface, object_server::SignalEmitter, Connection};
use zvariant::{ObjectPath, Type, Value};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct StoredNotification {
    pub notification: Notification,
    pub received_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct DatabaseNotification {
    pub notification: SerializableNotification,
    pub received_at: u64,
}
static THREAD_POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPool::new().expect("Failed to build pool"));
#[derive(Debug, Clone)]
pub struct MechanixNotificationService {
    pub freedesktop_signal_emitter: Option<SignalEmitter<'static>>,
    // Add shared storage for notifications as HashMap
    pub notifications: Arc<RwLock<HashMap<u32, StoredNotification>>>,
}

impl MechanixNotificationService {
    pub fn new() -> Self {
        Self {
            freedesktop_signal_emitter: None,
            notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Helper method to check if notification is resident
    async fn is_notification_resident(&self, id: u32) -> bool {
        let notifications = self.notifications.read().await;
        if let Some(stored_notification) = notifications.get(&id) {
            stored_notification.notification.is_resident();
        }
        false
    }

    pub async fn new_from_database() -> Result<Self, Box<dyn std::error::Error>> {
        let existing_db_notifications = get_all_notifications_from_db()
            .await
            .expect("Failed to get notifications from database - load issue");
        let mut existing_notifications = HashMap::new();

        for (id, db_notif) in existing_db_notifications {
            let notification = Notification {
                app_name: db_notif.notification.app_name,
                replaces_id: db_notif.notification.replaces_id,
                app_icon: db_notif.notification.app_icon,
                summary: db_notif.notification.summary,
                body: db_notif.notification.body,
                actions: db_notif.notification.actions,
                hints: db_notif
                    .notification
                    .hints
                    .into_iter()
                    .map(|(k, v)| (k, Value::from(v).try_to_owned().unwrap().into()))
                    .collect(),
                expire_timeout: db_notif.notification.expire_timeout,
            };
            existing_notifications.insert(
                id,
                StoredNotification {
                    notification,
                    received_at: db_notif.received_at,
                },
            );
        }

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
        notifications: Arc<RwLock<HashMap<u32, StoredNotification>>>,
    ) {
        THREAD_POOL.spawn_ok(async move {
            while let Some(event) = event_receiver.next().await {
                match event {
                    FreedesktopNotificationEvent::Notify(id, notification) => {
                        let store_notification = StoredNotification {
                            notification: notification.clone(),
                            received_at: epoch_seconds(),
                        };
                        // Store the notification
                        {
                            let mut notifs = notifications.write().await;
                            notifs.insert(id, store_notification.clone());
                        }

                        // Store in database
                        // "transient": BOOLEAN	=> When set the server will treat the notification as transient and by-pass the server's persistence capability, if it should exist.
                        if !notification.is_transient() {
                            let db_notification = DatabaseNotification {
                                notification: notification.to_serializable(),
                                received_at: store_notification.received_at,
                            };
                            if let Err(e) = add_notification_to_db(id, &db_notification).await {
                                eprintln!("Failed to add notification to database: {}", e);
                            } else {
                                println!("Notification {}, added to database", &id);
                            }
                        } else {
                            println!(
                                "Notification {}, is transient, will not be stored in databse",
                                &id
                            );
                        }
                        info!("Notification:app_name {:?}", notification.app_name);
                        let _ =
                            Self::notification_received(&signal_emitter, id, &notification).await;
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
        let mut notificationbus = Self::new_from_database()
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
            FreedesktopNotificationService::action_invoked(emitter, id, action_key)
                .await
                .map_err(|e| fdo::Error::Failed(format!("Failed to invoke action: {}", e)))
        } else {
            Err(fdo::Error::Failed(
                "Freedesktop signal emitter not available".to_string(),
            ))
        }
    }

    /// sends activation token signal on the org.freedesktop.Notifications interface
    async fn send_activation_token(&self, id: u32, activation_token: &str) -> fdo::Result<()> {
        if let Some(emitter) = &self.freedesktop_signal_emitter {
            FreedesktopNotificationService::activation_token(emitter, id, activation_token)
                .await
                .map_err(|e| fdo::Error::Failed(format!("Failed to send activation token: {}", e)))
        } else {
            Err(fdo::Error::Failed(
                "Freedesktop signal emitter not available".to_string(),
            ))
        }
    }

    /// Close a notification with a specific reason
    async fn close_notification_with_reason(&self, id: u32, reason: u32) -> fdo::Result<()> {
        if self.is_notification_resident(id).await {
            return Err(fdo::Error::Failed(
                "Resident Notification can only be closed by Sender".to_string(),
            ));
        }

        // Remove the notification
        {
            let mut notifs = self.notifications.write().await;
            notifs.remove(&id);
        }

        if let Err(e) = remove_notification_from_db(id).await {
            eprintln!("Failed to remove notification from database: {}", e);
        } else {
            println!("Notification {} removed from database", id);
        }

        if let Some(emitter) = &self.freedesktop_signal_emitter {
            FreedesktopNotificationService::notification_closed(emitter, id, reason)
                .await
                .map_err(|e| fdo::Error::Failed(format!("Failed to close notification: {}", e)))
        } else {
            Err(fdo::Error::Failed(
                "Freedesktop signal emitter not available".to_string(),
            ))
        }
    }

    /// Get all active notifications
    async fn get_all_notifications(&self) -> HashMap<u32, StoredNotification> {
        let notifications = self.notifications.read().await;
        notifications.clone()
    }

    #[zbus(signal)]
    async fn notification_received(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        notification: &Notification,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(signal_ctxt: &SignalEmitter<'_>, id: u32) -> zbus::Result<()>;
}

/// helper returning epoch seconds
fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
