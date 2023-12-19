use std::{error, process::Command};

use anyhow::{bail, Result};
use echo_client::EchoClient;
use tokio::sync::mpsc;
use tracing::{error, info};
use zbus::{dbus_interface, Connection};

use crate::{
    errors::{BackgroundError, BackgroundErrorCodes},
    process::handler::ChildProcessMessage,
    settings::BackgroundDaemon,
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
    sender: mpsc::Sender<ChildProcessMessage>,
    background_daemon: BackgroundDaemon,
}

pub struct ZbusServiceHandle {
    status: ServiceStatus,
    background_daemon: BackgroundDaemon, // receiver: mpsc::Receiver<ServiceMessage>,
}

const PROCESS_NAME: &str = "background";

#[dbus_interface(name = "org.mechanics.Background")]
impl ZbusHandler {
    async fn bg(&self, path: &str) {
        info!("bg change path is {}", path);
        let mut command = match &self.background_daemon.change_bg_command {
            Some(cmd) => cmd.split(" ").collect(),
            None => vec![],
        };

        command.push(path);

        let main_command = command[0];
        let args = &command[1..main_command.len() - 1];

        match execute_command(main_command, args) {
            Ok(v) => {
                info!("res from  execute_command {}", v);
            }
            Err(e) => {
                error!("error while executing command {}", e);
            }
        };
    }
}

impl ZbusServiceHandle {
    pub fn new(background_daemon: BackgroundDaemon) -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            background_daemon,
        }
    }

    pub async fn run(&mut self, sender: mpsc::Sender<ChildProcessMessage>) {
        let connection = Connection::session().await.unwrap();
        // setup the server
        let _ = connection
            .object_server()
            .at(
                "/org/mechanics/Background",
                ZbusHandler {
                    sender: sender,
                    background_daemon: self.background_daemon.clone(),
                },
            )
            .await;
        // before requesting the name
        let _ = connection.request_name("org.mechanics.Background").await;

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

fn execute_command(command: &str, args: &[&str]) -> Result<bool> {
    info!("execute_command command is {} {:?}", command, args);
    let output = match Command::new(command).args(args).output() {
        Ok(output) => output,
        Err(e) => {
            bail!(BackgroundError::new(
                BackgroundErrorCodes::CommandExecuteError,
                format!("failed to execute command: {}", e),
                true
            ))
        }
    };

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!(BackgroundError::new(
            BackgroundErrorCodes::CommandExecuteError,
            format!("failed to get output from command: {}", error),
            true
        ))
    }

    Ok(true)
}
