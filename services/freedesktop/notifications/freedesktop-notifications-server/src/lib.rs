pub mod errors;
pub mod interfaces;
pub mod proxies;
pub mod notification;
pub mod database;
pub mod tests;
// Flow

// {Applications by default calls notify/close method on } --> org.freedesktop.Notifications (with Notification Fields)--> FreeDesktopNotificaitonManager (Convert it to Notification Struct) -> Signal ([Notify/Close]) -> org.mechanix.NotificationManager --> Signal <--listerning from UI side --> (Mechanix GUI) --> Show/Remove --> GUI with buttons {Buttons Description provided in Notification Struct Hints} --> yes (GUI  calls method on org.mechanix.NotifiactionManager for the Action Equivalent)--> MechanixNotificationService (Signal Emmiter : FreeDesktop) --> Action Signal Emmited via org.freedesktop.Notification -> { <-- signal listner on org.freedesktop.Notification--> Application}.
