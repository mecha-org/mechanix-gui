use crate::connections::PortalResponse;
use zbus::zvariant;
use zbus::{
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    SignalContext,
};

#[derive(Clone, Copy)]
pub struct Notification {}

#[derive(zvariant::SerializeDict, zvariant::Type)]
#[zvariant(signature = "a{sv}")]
pub struct NotificationResult {
    notification_id: String,
}

#[derive(zvariant::DeserializeDict, zvariant::Type, Clone, Debug)]
#[zvariant(signature = "ssa{sv}a{ss}")]
pub struct NotificationOptions {
    title: Option<String>,
    body: Option<String>,
    icon: Option<Vec<u8>>,
    priority: Option<String>,
    default_action: Option<String>,
    buttons: Option<Vec<(String, String)>>,
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Notification")]
impl Notification {
    async fn add_notification(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        id: &str,
        notification: NotificationOptions,
        options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> PortalResponse<NotificationResult> {
        println!("add notif");
        self.run(handle, app_id, id, notification, options).await
    }

    async fn remove_notification(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        id: &str,
    ) -> PortalResponse<()> {
        dbg!("RemoveNotification called:", handle, app_id, id);
        PortalResponse::Success(())
    }
}

impl Notification {
    async fn run(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        id: &str,
        notification: NotificationOptions,
        _options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> PortalResponse<NotificationResult> {
        dbg!(
            "Notification received - Handle: {:?}, AppID: {}, ID: {}",
            &handle,
            &app_id,
            &id
        );
        dbg!(&notification);

        // Mock response with notification ID
        PortalResponse::Success(NotificationResult {
            notification_id: id.to_string(),
        })
    }
}
