use desktop_dbus::Notification;

#[derive(Debug)]
pub enum AppEvents {
    NotificationReceived {id: u32, notification: Notification },
    CloseNotification { id: u32 },
    // Sent from UI to backend when a user clicks a notification action button
    ActionInvoked { id: u32, action_id: String },
}
