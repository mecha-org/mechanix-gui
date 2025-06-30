use zbus::{ interface };
use std::collections::HashMap;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
use std::sync::atomic::{ AtomicU32, Ordering };
use zbus::object_server::SignalEmitter;
use std::sync::Arc;
use tokio::{ sync::mpsc::{ Receiver, Sender, channel }, sync::RwLock };
use crate::notification::Notification;

pub enum Event {
    Show(u32, Notification),
    Close(u32),
    Replace(u32, Notification),
}

#[derive(Debug, Clone)]
pub struct NotificationService {
    next_id: Arc<AtomicU32>,
    notifications: Arc<RwLock<HashMap<u32, Notification>>>,
    sender: Option<Sender<Event>>,
}

impl NotificationService {
    pub fn new() -> (Self, Receiver<Event>) {
        let (sender, reciever) = channel(100);
        let service = Self {
            next_id: Arc::new(AtomicU32::new(1)),
            notifications: Arc::new(RwLock::new(HashMap::new())),
            sender: Some(sender),
        };
        (service, reciever)
    }

    pub fn set_sender(&mut self, sender: Sender<Event>) {
        self.sender = Some(sender);
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationService {
    /// org.freedesktop.Notifications.CloseNotification
    /// Causes a notification to be forcefully closed and removed from the user's view.
    /// The NotificationClosed signal is emitted by this method.
    /// If the notification no longer exists, an empty D-BUS Error message is sent back.
    async fn close_notification(&self, id: u32) {
        let mut notifications = self.notifications.write().await;
        let notification_existed = notifications.remove(&id).is_some();
        drop(notifications);

        if notification_existed {
            if let Some(sender) = &self.sender {
                let _ = sender.send(Event::Close(id));
            }
            //#todo Signal Emmiter Implementation
            // if let Ok(signl_ext) = SignalEmitter::new()
            // {

            // }
        }
    }

    /// org.freedesktop.Notifications.Notify
    ///
    /// Sends a notification to the notification server.
    ///
    /// Parameters:
    /// - app_name: The optional name of the application sending the notification. Can be blank.
    /// - replaces_id: The optional notification ID that this notification replaces.
    ///   The server must atomically replace the given notification with this one.
    ///   A value of 0 means that this notification won't replace any existing notifications.
    /// - app_icon: The optional program icon of the calling application. Can be empty.
    /// - summary: The summary text briefly describing the notification.
    /// - body: The optional detailed body text. Can be empty.
    /// - actions: Actions sent as a list of pairs. Each even element represents the identifier
    ///   for the action. Each odd element is the localized string displayed to the user.
    /// - hints: Optional hints that can be passed from client to server. Can be empty.
    /// - expire_timeout: The timeout time in milliseconds. If -1, server-dependent default.
    ///   If 0, never expire.
    ///
    /// Returns: UINT32 notification ID. If replaces_id is 0, returns a unique new ID.
    /// If replaces_id is not 0, returns the same value as replaces_id.
    /// The returned ID is always greater than zero.
    async fn notify(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32
    ) -> u32 {
        let id = if replaces_id == 0 {
            self.next_id.fetch_add(1, Ordering::SeqCst)
        } else {
            replaces_id
        };

        let notification = Notification {
            app_name: app_name.to_string(),
            replaces_id,
            app_icon: app_icon.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            actions: actions
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            hints: hints
                .into_iter()
                .filter_map(|(k, v)| {
                    v.try_to_owned()
                        .ok()
                        .map(|owned_v| (k.to_string(), owned_v))
                })
                .collect(),
            expire_timeout,
        };

        let mut notifications = self.notifications.write().await;
        notifications.insert(id, notification.clone());
        drop(notifications);

        if let Some(sender) = &self.sender {
            let event = if replaces_id == 0 {
                Event::Show(id, notification)
            } else {
                Event::Replace(id, notification)
            };
            let _ = sender.send(event).await;
        }

        id
    }

    /// org.freedesktop.Notifications.GetCapabilities
    ///
    /// This message takes no parameters and returns an array of strings.
    /// Each string describes an optional capability implemented by the server.
    ///
    /// Defined capabilities:
    /// - "action-icons": Supports using icons instead of text for displaying actions
    /// - "actions": The server will provide the specified actions to the user
    /// - "body": Supports body text
    /// - "body-hyperlinks": The server supports hyperlinks in the notifications
    /// - "body-images": The server supports images in the notifications
    /// - "body-markup": Supports markup in the body text
    /// - "icon-multi": The server will render an animation of all frames in an image array
    /// - "icon-static": Supports display of exactly 1 frame of any given image array
    /// - "persistence": The server supports persistence of notifications
    /// - "sound": The server supports sounds on notifications

    async fn get_capabilities(&self) -> Vec<&'static str> {
        vec![
            "body",
            "icon-static",
            "persistence",
            //#todo support these
            "actions",
            "action-icons",
            "body-markup",
            "body-hyperlinks",
            "sound"
        ]
    }

    /// org.freedesktop.Notifications.GetServerInformation
    ///
    /// This message returns the information on the server.
    /// Specifically, the server name, vendor, and version number.
    ///
    /// Returns:
    /// - name: The product name of the server
    /// - vendor: The vendor name (e.g., "KDE," "GNOME," "freedesktop.org," "Microsoft")
    /// - version: The server's version number
    /// - spec_version: The specification version the server is compliant with
    #[zbus(out_args("name", "vendor", "version", "spec_version"))]
    async fn get_server_information(
        &self
    ) -> (&'static str, &'static str, &'static str, &'static str) {
        ("mechanix-notifications", "Mecha", VERSION, "1.2")
    }

    /// org.freedesktop.Notifications.ActionInvoked Signal
    ///
    /// This signal is emitted when one of the following occurs:
    /// 1. The user performs some global "invoking" action upon a notification
    ///    (e.g., clicking somewhere on the notification itself)
    /// 2. The user invokes a specific action as specified in the original Notify request
    ///    (e.g., clicking on an action button)
    ///
    /// Parameters:
    /// - id: The ID of the notification emitting the ActionInvoked signal
    /// - action_key: The key of the action invoked. These match the keys sent in the actions list
    ///
    /// Note: Clients should not assume the server will generate this signal.
    /// Some servers may not support user interaction at all.
    #[zbus(signal)]
    async fn action_invoked(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        action_key: &str
    ) -> zbus::Result<()>;

    /// org.freedesktop.Notifications.ActivationToken Signal
    ///
    /// This signal can be emitted before an ActionInvoked signal.
    /// It carries an activation token that can be used to activate a toplevel.
    ///
    /// Parameters:
    /// - id: The ID of the notification emitting the ActionInvoked signal
    /// - activation_token: An activation token. This can be either an X11-style startup ID
    ///   (see Startup notification protocol) or a Wayland xdg-activation token
    ///
    /// Note: Clients should not assume the server will generate this signal.
    /// Some servers may not support user interaction or activation token generation.

    #[zbus(signal)]
    async fn activation_token(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        activation_token: &str
    ) -> zbus::Result<()>;

    /// id	UINT32	The ID of the notification that was closed.
    /// reason	UINT32
    /// The reason the notification was closed.
    /// 1 - The notification expired.
    /// 2 - The notification was dismissed by the user.
    /// 3 - The notification was closed by a call to CloseNotification.
    /// 4 - Undefined/reserved reasons.
    #[zbus(signal)]
    async fn notification_closed(
        signal_ctxt: &SignalEmitter<'_>,
        id: u32,
        reason: u32
    ) -> zbus::Result<()>;
}
