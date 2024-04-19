use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;

use tokio::sync::oneshot;
use zbus::{dbus_interface, Connection};

use crate::AppMessage;

struct ZbusHandler {
    sender: Sender<AppMessage>,
}

pub struct ZbusServiceHandle {
    app_channel: Sender<AppMessage>,
}

#[dbus_interface(name = "org.mechanix.shell.SettingsPanel")]
impl ZbusHandler {
    // Can be `async` as well.
    fn show(&mut self) {
        let _ = self.sender.send(AppMessage::Show);
    }

    fn hide(&mut self) {
        let _ = self.sender.send(AppMessage::Hide);
    }
}

impl ZbusServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        let _ = connection
            .object_server()
            .at(
                "/org/mechanix/shell/SettingsPanel",
                ZbusHandler {
                    sender: self.app_channel.clone(),
                },
            )
            .await;
        // before requesting the name
        let _ = connection
            .request_name("org.mechanix.shell.SettingsPanel")
            .await;

        loop {
            // do something else, wait forever or timeout here:
            // handling D-Bus messages is done in the background
            std::future::pending::<()>().await;
        }
    }
}
