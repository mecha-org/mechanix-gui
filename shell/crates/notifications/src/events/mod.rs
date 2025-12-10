use desktop_dbus::Notification;

#[derive(Debug)]
pub enum AppEvents {
    NotificationReceived { notification: Notification },
}
