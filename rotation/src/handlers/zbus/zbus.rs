use std::{error, process::Command};

use anyhow::{bail, Result};
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};
use zbus::{dbus_interface, Connection};

use crate::{
    errors::{RotationError, RotationErrorCodes},
    handlers::rotation::handler::RotationHandlerMessage,
};

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
    rotation_handler_tx: broadcast::Sender<RotationHandlerMessage>,
}

pub struct ZbusServiceHandle {
    status: ServiceStatus,
    rotation_handler_tx: broadcast::Sender<RotationHandlerMessage>,
}

const PROCESS_NAME: &str = "rotation";

#[dbus_interface(name = "org.mechanics.Rotation")]
impl ZbusHandler {
    async fn rotation(&self, mode: &str, persist: bool) {
        info!("rotation mode {} persist {}", mode, persist);
        let _ = self
            .rotation_handler_tx
            .send(RotationHandlerMessage::Rotation {
                mode: String::from(mode),
                persist,
            });
    }
}

impl ZbusServiceHandle {
    pub fn new(rotation_handler_tx: broadcast::Sender<RotationHandlerMessage>) -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            rotation_handler_tx,
        }
    }

    pub async fn run(&mut self) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        let _ = connection
            .object_server()
            .at(
                "/org/mechanics/Rotation",
                ZbusHandler {
                    rotation_handler_tx: self.rotation_handler_tx.clone(),
                },
            )
            .await;
        // before requesting the name
        let _ = connection.request_name("org.mechanics.Rotation").await;

        loop {
            // do something else, wait forever or timeout here:
            // handling D-Bus messages is done in the Rotation
            let _ = self
                .rotation_handler_tx
                .send(RotationHandlerMessage::ZbusStarted);
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
