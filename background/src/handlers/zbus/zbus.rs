use std::{error, process::Command};

use anyhow::{bail, Result};
use echo_client::EchoClient;
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};
use zbus::{dbus_interface, Connection};

use crate::{
    errors::{BackgroundError, BackgroundErrorCodes},
    settings::{update_settings, BackgroundSettings},
    BackgroundHandlerMessage,
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
    background_handler_tx: broadcast::Sender<BackgroundHandlerMessage>,
}

pub struct ZbusServiceHandle {
    status: ServiceStatus,
    background_handler_tx: broadcast::Sender<BackgroundHandlerMessage>,
}

const PROCESS_NAME: &str = "background";

#[dbus_interface(name = "org.mechanics.Background")]
impl ZbusHandler {
    async fn change_bg(&self, path: &str, mode: &str, persist: bool) {
        info!(
            "change bg path: {} mode: {} persist: {}",
            path, mode, persist
        );

        let _ = self
            .background_handler_tx
            .send(BackgroundHandlerMessage::ChangeBackground {
                path: String::from(path),
                mode: String::from(mode),
                persist,
            });
    }
}

impl ZbusServiceHandle {
    pub fn new(background_handler_tx: broadcast::Sender<BackgroundHandlerMessage>) -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            background_handler_tx,
        }
    }

    pub async fn run(&mut self) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        let _ = connection
            .object_server()
            .at(
                "/org/mechanics/Background",
                ZbusHandler {
                    background_handler_tx: self.background_handler_tx.clone(),
                },
            )
            .await;
        // before requesting the name
        let _ = connection.request_name("org.mechanics.Background").await;

        loop {
            // do something else, wait forever or timeout here:
            // handling D-Bus messages is done in the background

            let _ = self
                .background_handler_tx
                .send(BackgroundHandlerMessage::ZbusStarted);

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
