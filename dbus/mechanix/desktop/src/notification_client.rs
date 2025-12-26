use mechanix_session_services::Notification;
use std::collections::HashMap;
use zbus::proxy;
use mechanix_session_services::interfaces::mechanix::StoredNotification;

#[proxy(
    interface = "org.mechanix.NotificationManager",
    default_service = "org.mechanix.NotificationManager",
    default_path = "/org/mechanix/NotificationManager"
)]
pub trait MechanixNotification {
    /// CloseNotificationWithReason method
    fn close_notification_with_reason(&self, id: u32, reason: u32) -> zbus::Result<()>;

    /// GetAllNotifications method
    fn get_all_notifications(&self) -> zbus::Result<HashMap<u32, StoredNotification>>;

    /// GetVersion method
    fn get_version(&self) -> zbus::Result<String>;

    /// InvokeAction method
    fn invoke_action(&self, id: u32, action_key: &str) -> zbus::Result<()>;

    /// SendActivationToken method
    fn send_activation_token(&self, id: u32, activation_token: &str) -> zbus::Result<()>;

    /// NotificationClosed signal
    #[zbus(signal)]
    fn notification_closed(&self, id: u32) -> zbus::Result<()>;

    /// NotificationReceived signal - using clean Notification struct
    #[zbus(signal)]
    fn notification_received(&self, id: u32, notification: Notification) -> zbus::Result<()>;
}
