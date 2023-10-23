// Imports from other crates
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

// Imports from other modules
use crate::user_interface;

// Modules
mod dbus_client;
mod dbus_server;
use dbus_client::DBusClient;
use dbus_server::DBusServer;

/// The DBusService starts the DBusClient and DBusServer and saves the state of the visibility of the keyboard.
/// It also can forward events that need to be sent to feedbackd to the DBusClient. This is used to give a haptic feedback when buttons are pressed and released
pub struct DBusService {
    client: DBusClient,
    visibility: Arc<AtomicBool>,
}

impl DBusService {
    /// Starts the DBusClient and DBusServer and returns an DBusService to handle them.
    pub fn new(sender: relm::Sender<user_interface::Msg>) -> DBusService {
        let visibility = Arc::new(AtomicBool::new(false));
        let visibility_clone = Arc::clone(&visibility); // Gets moved to DBusServer

        DBusServer::spawn_and_detach(Mutex::new(sender), visibility_clone);
        let client = DBusClient::new();
        DBusService { client, visibility }
    }

    /// Changes the value of the visibility of the keyboard. It does not cause the keyboard to show or hide. That has to be done by the UI manager
    pub fn change_visibility(&mut self, visible: bool) {
        self.visibility.store(visible, Ordering::SeqCst);
        info!("Keyboard visibility changed to {}", visible);
    }

    /// Tell the DBusClient to send the event to feedbackd (used for haptic feedback)
    pub fn haptic_feedback(&self, event: String) {
        info!("'{}' event is handed to the DBusClient", event);
        self.client.send(event);
    }
}
