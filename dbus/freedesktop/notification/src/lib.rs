pub mod errors;
pub mod interfaces;
pub mod proxies;
pub mod notification;
pub mod database;
// pub mod tests;

//   1. An application calls the Notify method on the standard org.freedesktop.Notifications D-Bus interface.
//   2. FreedesktopNotificationService receives the call, packages the data into a Notification struct, and sends it over an
//       internal channel.
//   3. MechanixNotificationService listens on this channel. It receives the notification, saves it to the database, and emits
//       its own notification_received signal on the org.mechanix.NotificationManager interface.
//   4. A GUI client, listening to the org.mechanix.NotificationManager interface, receives the signal and displays the
//       notification.
//   5. If the user clicks an action button in the GUI, the GUI calls the invoke_action method on the
//       org.mechanix.NotificationManager interface.
//   6. MechanixNotificationService then emits the standard ActionInvoked signal on the org.freedesktop.Notifications         interface, which the original application is listening for to perform the requested action.