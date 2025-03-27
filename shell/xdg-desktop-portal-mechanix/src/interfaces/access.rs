use std::collections::HashMap;
use zbus::zvariant;
use zbus::{fdo, interface};

use crate::connections::PortalResponse;

#[derive(zvariant::DeserializeDict, zvariant::Type, Debug, Clone)]
#[zvariant(signature = "a{sv}")]
pub struct AccessDialogOptions {
    modal: Option<bool>,
    deny_label: Option<String>,
    grant_label: Option<String>,
    icon: Option<String>,
    choices: Option<Vec<(String, String, Vec<(String, String)>, String)>>,
}

#[derive(zvariant::SerializeDict, zvariant::Type, Debug, Clone)]
#[zvariant(signature = "a{sv}")]
pub struct AccessDialogResult {
    choices: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Access {}

#[zbus::interface(name = "org.freedesktop.impl.portal.Access")]
impl Access {
    #[allow(clippy::too_many_arguments)]
    async fn access_dialog(
        &self,
        handle: zvariant::ObjectPath<'_>,
        app_id: &str,
        parent_window: &str,
        title: &str,
        subtitle: &str,
        body: &str,
        options: AccessDialogOptions,
    ) -> PortalResponse<AccessDialogResult> {
        println!("Access dialog called");
        dbg!(
            &handle,
            &app_id,
            &parent_window,
            &title,
            &subtitle,
            &body,
            &options
        );

        let result = AccessDialogResult {
            choices: options
                .choices
                .unwrap_or_default()
                .into_iter()
                .map(|(id, label, _, initial)| (id, initial))
                .collect(),
        };

        PortalResponse::Success(result)
    }
}

// gdbus call --session --dest org.mechanix.services.FileChooser --object-path /org/freedesktop/portal/desktop --method org.freedesktop.impl.portal.Access.AccessDialog "/test/handle"     "test_app"     "test_id"    "title: <Test Title>" "subtitle" "body': <'Test Body'>"    "{}"
