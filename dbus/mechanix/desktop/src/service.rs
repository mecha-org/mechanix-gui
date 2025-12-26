use crate::errors::NotificationError;
use crate::notification_client::NotificationProxy;
use anyhow::Result;
use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::StreamExt;
use log::{error, info};
use session_service::interfaces::mechanix::StoredNotification;
use session_service::Notification;
use std::collections::HashMap;
use std::sync::LazyLock;
use zbus::Connection;

static THREAD_POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPool::new().expect("Failed to build pool"));

const CHANNEL_SIZE: usize = 10;

/// A service wrapper for interacting with a NetworkManager implementation.
///
/// This generic struct provides high-level methods for managing WiFi connections,
/// such as enabling/disabling WiFi, listing available networks, and connecting to a network.
/// The implementation is generic over any type that implements `NetworkManagerInterface`.
#[derive(Clone)]
pub struct NotificationService {
    proxy: NotificationProxy<'static>,
}

impl NotificationService {
    /// Creates a new `NetworkManagerService` with the given NetworkManager interface.
    ///
    /// # Arguments
    ///
    /// * `nm` - An object implementing the `NetworkManagerInterface` trait.
    /// Async constructor: handles connection and proxy creation internally.
    pub async fn new() -> Result<Self, NotificationError> {
        let conn = Connection::session()
            .await
            .map_err(|e| NotificationError::CreateProxyError(e.to_string()))?;
        let proxy = NotificationProxy::new(&conn)
            .await
            .map_err(|e| NotificationError::CreateProxyError(e.to_string()))?;
        info!("notification service created successfully");
        Ok(Self { proxy })
    }
    /// Returns a stream of notifications received from the dbus service.
    pub async fn stream_receive_notification(&self) -> mpsc::Receiver<(u32, Notification)> {
        let proxy = self.proxy.clone();
        let (mut sender, receiver) = mpsc::channel(CHANNEL_SIZE);
        THREAD_POOL.spawn_ok(async move {
            match proxy.receive_notification_received().await {
                Ok(mut stream) => {
                    while let Some(event) = stream.next().await {
                        if let Ok(args) = event.args() {
                            if let Err(e) = sender.try_send((args.id, args.notification)) {
                                error!("failed to send notification to receiver: {}", e);
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get the stream of notification: {}", e);
                }
            }
        });
        receiver
    }
    /// Fetch all notifications from the service (unread list for Notification Center)
    pub async fn fetch_all(
        &self,
    ) -> Result<HashMap<u32, StoredNotification>, NotificationError> {
        self.proxy
            .get_all_notifications()
            .await
            .map_err(|e| NotificationError::CreateProxyError(e.to_string()))
    }

    /// Returns a stream of notifications closed from the dbus service.
    pub async fn stream_close_notification(&self) -> mpsc::Receiver<u32> {
        let proxy = self.proxy.clone();
        let (mut sender, receiver) = mpsc::channel(CHANNEL_SIZE);
        THREAD_POOL.spawn_ok(async move {
            match proxy.receive_notification_closed().await {
                Ok(mut stream) => {
                    while let Some(event) = stream.next().await {
                        if let Ok(args) = event.args() {
                            if let Err(e) = sender.try_send(args.id) {
                                error!("failed to send a close signal to receiver: {}", e);
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get the stream of a close signal: {}", e);
                }
            }
        });
        receiver
    }
    pub async fn send_action_invoke(
        &self,
        id: u32,
        action_key: &str,
    ) -> Result<(), NotificationError> {
        let proxy = self.proxy.clone();
        proxy
            .invoke_action(id, action_key)
            .await
            .map_err(|e| NotificationError::CreateProxyError(e.to_string()))
    }
    pub async fn close_notification(
        &self,
        id: u32,
        reason_id: u32,
    ) -> Result<(), NotificationError> {
        let proxy = self.proxy.clone();
        proxy
            .close_notification_with_reason(id, reason_id)
            .await
            .map_err(|e| NotificationError::CloseNotificationActionFailed(e.to_string()))
    }
}
