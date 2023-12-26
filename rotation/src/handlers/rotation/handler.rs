use anyhow::{bail, Result};
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    select,
    sync::{broadcast, mpsc},
};
use tonic::async_trait;
use tracing::info;

use crate::{
    handlers::{
        rotation::service::{
            change_rotation, dispatch_rotation_status, set_default_rotation, DispatchRotationParams,
        },
        wlroots::handler::WlrootsHandlerMessage,
    },
    settings::{RotationConfigs, RotationSettings},
};

pub struct RotationHandler {
    pub wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>,
    status: ServiceStatus,
}

#[derive(Debug, Clone)]
pub enum RotationHandlerMessage {
    Rotation { mode: String, persist: bool },
    ZbusStarted,
}

impl RotationHandler {
    pub fn new(wlroots_sender_tx: mpsc::Sender<WlrootsHandlerMessage>) -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            wlroots_sender_tx,
        }
    }
    pub async fn run(
        &mut self,
        mut rotation_handler_rx: broadcast::Receiver<RotationHandlerMessage>,
        mut rotation_settings: RotationSettings,
    ) {
        // start the service
        let _ = &self.start().await;
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(3));
        loop {
            select! {
                        msg = rotation_handler_rx.recv() => {
                            if !msg.is_ok() {
                                continue;
                            }
                            info!("RotationHandler runner msg received {:?}", msg);
                            match msg.unwrap() {
                                RotationHandlerMessage::Rotation{ mode, persist } => {
                                    let _ = change_rotation(&mode, persist, &mut rotation_settings);
                                }
                                RotationHandlerMessage::ZbusStarted => {
                                    let _ = set_default_rotation(&mut rotation_settings);
                                }
                            };
                        }

                        _ = timer.tick() => {
                            if rotation_settings.rotation.enabled {
                               let _ = dispatch_rotation_status(DispatchRotationParams {
                                    wlroots_sender_tx: self.wlroots_sender_tx.clone(),
                                    rotation_configs: rotation_settings.rotation.configs.clone()
                                }).await;
                        } else {
                            info!("Rotation is not enabled");
                        }
                }
            }
        }
    }
}

#[async_trait]
impl ServiceHandler for RotationHandler {
    async fn start(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STARTED;
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
