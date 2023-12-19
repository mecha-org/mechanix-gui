use echo_client::EchoClient;
use libc::{SIGRTMIN, SIGUSR1, SIGUSR2};
use tokio::sync::mpsc;
use tracing::{error, info};
use zbus::{dbus_interface, Connection};

use crate::process::handler::ChildProcessMessage;

#[derive(Debug, Clone, Copy)]
pub enum EventMessage {
    Show,
    Hide,
    Toggle,
}

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: mpsc::Sender<u32> },
    Stop { respond_to: mpsc::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

struct ZbusHandler {
    sender: mpsc::Sender<ChildProcessMessage>,
}

pub struct ZbusServiceHandle {
    status: ServiceStatus,
    // receiver: mpsc::Receiver<ServiceMessage>,
}

const PROCESS_NAME: &str = "osk";

#[dbus_interface(name = "org.mechanics.Osk")]
impl ZbusHandler {
    // Can be `async` as well.
    async fn show(&self) {
        // let _ = self
        //     .sender
        //     .send(ChildProcessMessage::Signal {
        //         process_name: String::from(PROCESS_NAME),
        //         code: SIGUSR2,
        //     })
        //     .await;
        let b = 'b';
        match EchoClient::echo(
            "sm.puri.OSK0",
            "/sm/puri/OSK0",
            "sm.puri.OSK0",
            "SetVisible",
            (true),
        )
        .await
        {
            Ok(r) => {
                info!("squeeboard res {}", r);
            }
            Err(e) => {
                error!("squeboard error {}", e);
            }
        };
    }

    async fn hide(&mut self) {
        // let _ = self
        //     .sender
        //     .send(ChildProcessMessage::Signal {
        //         process_name: String::from(PROCESS_NAME),
        //         code: SIGUSR1,
        //     })
        //     .await;
        let _ = EchoClient::echo(
            "sm.puri.OSK0",
            "/sm/puri/OSK0",
            "sm.puri.OSK0",
            "SetVisible",
            (false),
        )
        .await;
    }

    async fn toggle(&mut self) {
        // let _ = self
        //     .sender
        //     .send(ChildProcessMessage::Signal {
        //         process_name: String::from(PROCESS_NAME),
        //         code: SIGRTMIN(),
        //     })
        //     .await;
        let is_keyboard_visible_op: Option<bool> = match EchoClient::echo_property(
            "sm.puri.OSK0",
            "/sm/puri/OSK0",
            "sm.puri.OSK0",
            "Visible",
        )
        .await
        {
            Ok(r) => Some(r),
            Err(e) => {
                error!("error while getting visible status {}", e);
                None
            }
        };

        match is_keyboard_visible_op {
            Some(is_keyboard_visible) => {
                let _ = EchoClient::echo(
                    "sm.puri.OSK0",
                    "/sm/puri/OSK0",
                    "sm.puri.OSK0",
                    "SetVisible",
                    (!is_keyboard_visible),
                )
                .await;
            }
            None => (),
        }
    }
}

impl ZbusServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: mpsc::Sender<ChildProcessMessage>) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        let _ = connection
            .object_server()
            .at("/org/mechanics/Osk", ZbusHandler { sender: sender })
            .await;
        // before requesting the name
        let _ = connection.request_name("org.mechanics.Osk").await;

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
