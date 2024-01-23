use std::process::Stdio;

use anyhow::Result;
use tokio::{
    process::{Child, Command},
    signal::unix::{signal, SignalKind},
};
use tracing::{error, info};
pub struct ChildProcessManager {
    child_process: Option<Child>,
}

impl ChildProcessManager {
    pub fn new() -> Self {
        Self {
            child_process: None,
        }
    }

    pub async fn spawn(&mut self, command: &str, args: &[&str]) {
        let mut child = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start child process");

        info!("child process started with id {:?}", child.id());

        self.child_process = Some(child);
    }

    pub async fn signal(&self) -> Result<bool> {
        info!("sending signal to child");

        Ok(true)
    }

    pub async fn stop(&mut self) {
        match &mut self.child_process {
            Some(child) => match child.kill().await {
                Ok(r) => {
                    info!("child process killed successfully {:?}", r)
                }
                Err(e) => {
                    error!("error while killing child process {}", e)
                }
            },
            None => {
                info!("no child process are spawned")
            }
        }
        self.child_process = None;
    }
}
