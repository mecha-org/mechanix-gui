use std::process::Stdio;

use anyhow::Result;
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    process::{Child, Command},
    select,
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};
use tonic::async_trait;
use tracing::{error, info};

use crate::ChildProcessMessage;
pub struct ChildProcessHandle {
    status: ServiceStatus,
}

impl ChildProcessHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&self, mut message_rx: oneshot::Receiver<ChildProcessMessage>) {
        // start the service
        let _ = &self.start().await;
        loop {
            select! {
                msg = message_rx.try_recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    match msg.unwrap() {
                        ChildProcessMessage::Show  => {
                            let _ = temp().await;
                        }

                        ChildProcessMessage::Hide => {
                            let _ = temp().await;
                        }
                        ChildProcessMessage::Toggle => {
                            let _ = temp().await;
                        }
                    };
                }

            }
        }
    }
}

#[async_trait]
impl ServiceHandler for ChildProcessHandle {
    async fn start(&mut self) -> Result<bool> {
        // Start if device is provisioned
        Ok(true)
    }

    async fn stop(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STOPPED;
        Ok(true)
    }

    fn get_status(&self) -> anyhow::Result<ServiceStatus> {
        Ok(self.status)
    }

    fn is_stopped(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STOPPED)
    }

    fn is_started(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STARTED)
    }
}

pub async fn temp() -> Result<bool> {
    Ok(true)
}
