use anyhow::{bail, Result};
use services::{ServiceHandler, ServiceStatus};
use tokio::{select, sync::broadcast};
use tonic::async_trait;
use tracing::info;

use crate::{
    handlers::background::service::{change_background, set_default_background},
    settings::BackgroundSettings,
};

pub struct BackgroundHandler {
    status: ServiceStatus,
}

#[derive(Debug, Clone)]
pub enum BackgroundHandlerMessage {
    ChangeBackground {
        path: String,
        mode: String,
        persist: bool,
    },
    ZbusStarted,
}

impl BackgroundHandler {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }
    pub async fn run(
        &mut self,
        mut background_handler_rx: broadcast::Receiver<BackgroundHandlerMessage>,
        background_settings: BackgroundSettings,
    ) {
        // start the service
        let _ = &self.start().await;

        loop {
            select! {
                    msg = background_handler_rx.recv() => {
                        if !msg.is_ok() {
                            continue;
                        }
                        info!("BackgroundHandler runner msg received {:?}", msg);
                        match msg.unwrap() {
                            BackgroundHandlerMessage::ChangeBackground{ path, mode, persist } => {
                                let _ = change_background(&path, &mode, persist, background_settings.clone());
                            }
                            BackgroundHandlerMessage::ZbusStarted => {
                                let _ = set_default_background(background_settings.clone());
                            }
                        };
                    }
            }
        }
    }
}

#[async_trait]
impl ServiceHandler for BackgroundHandler {
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
