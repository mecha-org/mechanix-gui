use relm4::Sender;
use std::future::pending;
use std::{error::Error, time::Duration};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    time,
};
use zbus::{dbus_interface, Connection};

use crate::Message;

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: oneshot::Sender<u32> },
    Stop { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

struct ZbusHandler {
    sender: Sender<Message>,
}

pub struct ZbusServiceHandle {
    status: ServiceStatus,
    // receiver: mpsc::Receiver<ServiceMessage>,
}

#[dbus_interface(name = "org.zbus.StatusBar")]
impl ZbusHandler {
    // Can be `async` as well.
    fn show(&mut self) {
        println!("show called");
        self.sender.send(Message::Show);
    }

    fn hide(&mut self) {
        println!("hide called");
        self.sender.send(Message::Hide);
    }
}

impl ZbusServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        connection
            .object_server()
            .at(
                "/org/zbus/StatusBar",
                ZbusHandler {
                    sender: sender.clone(),
                },
            )
            .await;
        // before requesting the name
        connection.request_name("org.zbus.StatusBar").await;

        loop {
            // do something else, wait forever or timeout here:
            // handling D-Bus messages is done in the background
            std::future::pending::<()>().await;
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
