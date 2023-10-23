// Imports from other crates
use dbus::{arg::Variant, blocking::Connection};
use std::{collections::HashMap, time::Duration};

pub struct DBusClient {
    connection: Connection,
    app_id: String,
    timeout: i32,
    hints: HashMap<String, Variant<String>>,
}

impl DBusClient {
    /// This functions creates a new DBusClient that establishes a connection and then sends all events to feedbackd.
    pub fn new() -> DBusClient {
        // Starting up a connection to the session bus and requesting a name.
        let connection = Connection::new_session().unwrap();

        // This sets up the parameters for the method call on the proxy
        let app_id = "org.fingerboard.Feedback".to_string(); // ID of fingerboard, identifies the app requesting the event from feedbackd
        let timeout = -1; // Never timeout (This is only necessary if you want to end the feedback prematurely)
        let hints: HashMap<String, Variant<String>> = HashMap::new(); // No hints are sent
        info!("DBus client to handle haptic-feedback was spawned in a new thread and is waiting to receive events");

        DBusClient {
            connection,
            app_id,
            timeout,
            hints,
        }
    }

    /// Calls the 'TriggerFeedback' method with the specified event
    pub fn send(&self, event: String) {
        // Creates the proxy for the object the events are sent to
        let proxy = self.connection.with_proxy(
            "org.sigxcpu.Feedback",
            "/org/sigxcpu/Feedback",
            Duration::from_millis(5000),
        );
        info!("Received {} event to send to feedbackd", event);
        // Send the event to feedbackd
        let (_event_id,): (u32,) = proxy
            .method_call(
                "org.sigxcpu.Feedback",
                "TriggerFeedback",
                (&self.app_id, event, self.hints.clone(), self.timeout),
            )
            .unwrap();
    }
}
