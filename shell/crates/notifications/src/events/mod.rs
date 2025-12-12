use desktop_dbus::Notification;

#[derive(Debug)]
pub enum AppEvents {
    NotificationReceived {id: u32, notification: Notification },
    CloseNotification { id: u32 },
}
