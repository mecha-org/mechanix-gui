use anyhow::{bail, Result};
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    select,
    sync::{broadcast, mpsc},
};
use tonic::async_trait;
use tracing::info;
use wayland_client::protocol::wl_output::Transform;

use crate::{
    backends::Orientation,
    handlers::{
        rotation::service::{
            change_rotation, dispatch_rotation_status, set_default_rotation, DispatchRotationParams,
        },
        wlroots::service::rotate,
    },
    settings::{RotationConfigs, RotationSettings},
};

#[derive(Debug, Clone)]
pub enum WlrootsHandlerMessage {
    Rotate {
        orientation: Orientation,
        rotation_configs: RotationConfigs,
    },
}

pub struct WlrootsHandler {
    status: ServiceStatus,
}

impl WlrootsHandler {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }
    pub async fn run(&mut self, mut wlroots_handler_rx: mpsc::Receiver<WlrootsHandlerMessage>) {
        // start the service
        let _ = &self.start().await;
        loop {
            select! {
                    msg = wlroots_handler_rx.recv()  => {
                        if msg.is_none() {
                            continue;
                        }
                        info!("WlrootsHandler runner msg received {:?}", msg);
                        match msg.unwrap() {
                            WlrootsHandlerMessage::Rotate{orientation, rotation_configs} => {
                                let _ = rotate(orientation, rotation_configs);
                            }

                        };
                    }


            }
        }
    }
}

#[async_trait]
impl ServiceHandler for WlrootsHandler {
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
