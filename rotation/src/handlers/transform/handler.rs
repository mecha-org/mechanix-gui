use std::collections::HashMap;

use anyhow::{bail, Result};
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot},
};
use tonic::async_trait;
use tracing::info;
use wayland_client::{backend::ObjectId, protocol::wl_output::Transform};
use wayland_protocols_async::zwlr_output_management_v1::{
    errors::{OutputManagementHandlerError, OutputManagementHandlerErrorCodes},
    handler::{
        OutputHeadMeta, OutputManagementEvent, OutputManagementHandler, OutputManagementMessage,
    },
};

use crate::{
    handlers::rotation::service::{
        change_rotation, dispatch_rotation_status, set_default_rotation, DispatchRotationParams,
    },
    settings::{RotationConfigs, RotationSettings},
};

use super::errors::{TransformHandlerError, TransformHandlerErrorCodes};

#[derive(Debug, Clone)]
pub enum WlrootsHandlerMessage {
    Rotate {
        output_name: String,
        transform: Transform,
    },
}
#[derive(Debug, Clone, Default)]
pub struct OutputState {
    pub name: String,
}

pub struct WlrootsHandler {
    status: ServiceStatus,
    outputs: HashMap<ObjectId, OutputState>,
    output_msg_sender: Option<mpsc::Sender<OutputManagementMessage>>,
}

impl WlrootsHandler {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            outputs: HashMap::new(),
            output_msg_sender: None,
        }
    }
    pub async fn run(&mut self, mut wlroots_handler_rx: mpsc::Receiver<WlrootsHandlerMessage>) {
        // start the service
        let _ = &self.start().await;

        // create mpsc channel for interacting with the output handler
        let (output_msg_tx, mut output_msg_rx) = mpsc::channel(128);
        self.output_msg_sender = Some(output_msg_tx.clone());
        // create mpsc channel for receiving events from the output handler
        let (output_event_tx, mut output_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut output_handler = OutputManagementHandler::new(output_event_tx);

        // start the output handler
        let output_t = tokio::spawn(async move {
            let _ = output_handler.run(output_msg_rx).await;
        });

        loop {
            select! {
                    msg = wlroots_handler_rx.recv()  => {
                        if msg.is_none() {
                            continue;
                        }
                        info!("WlrootsHandler runner msg received {:?}", msg);
                        match msg.unwrap() {
                            WlrootsHandlerMessage::Rotate{output_name, transform} => {
                                let _ = self.rotate_output_by_name(&output_name, transform).await;
                                // let _ = rotate(orientation, rotation_configs);
                            }

                        };
                    }

                    msg = output_event_rx.recv() => {
                        if msg.is_none() {
                            continue;
                        }
                        println!("received output_event event={:?}", msg);

                        match msg.unwrap() {
                            OutputManagementEvent::Head { id } => {
                                let _ = self.add_output(id).await;
                            }
                            _ => {}
                        }
                    }
            }
        }
    }

    pub async fn add_output(&mut self, output_id: ObjectId) -> Result<bool> {
        let mut output_state = OutputState::default();

        match self.get_output_details(output_id.clone()).await {
            Ok(output_details) => {
                output_state.name = output_details.name;
            }
            Err(_) => (),
        };

        self.outputs.insert(output_id, output_state);

        Ok(true)
    }

    pub async fn get_output_details(&self, id: ObjectId) -> Result<OutputHeadMeta> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .output_msg_sender
            .as_ref()
            .unwrap()
            .send(OutputManagementMessage::GetHead { id, reply_to: tx })
            .await;

        let reply = match rx.await {
            Ok(v) => v,
            Err(e) => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::OutputManagemenConnectError,
                format!("unable to connect to output management handler {}", e),
                true
            )),
        };

        let output_meta = match reply {
            Ok(v) => v,
            Err(e) => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::GetOutputMetaError,
                format!("unable to get output head meta {}", e),
                true
            )),
        };

        Ok(output_meta)
    }

    pub async fn rotate_output(&self, output_id: ObjectId, transform: Transform) -> Result<bool> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .output_msg_sender
            .as_ref()
            .unwrap()
            .send(OutputManagementMessage::SetTransform {
                head_id: output_id,
                transform: transform,
                reply_to: tx,
            })
            .await;

        let reply = match rx.await {
            Ok(v) => v,
            Err(e) => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::OutputManagemenConnectError,
                format!("unable to connect to output management handler {}", e),
                true
            )),
        };

        let is_sent = match reply {
            Ok(v) => v,
            Err(e) => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::TransformError,
                format!("unable to rotate output {}", e),
                true
            )),
        };

        Ok(is_sent)
    }

    pub async fn rotate_output_by_name(&self, name: &str, transform: Transform) -> Result<bool> {
        let output_op = self.outputs.iter().find(|(_, state)| state.name == name);

        let output = match output_op {
            Some((key, _)) => key,
            None => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::OutputNotFoundByNameError,
                "output not found".to_string(),
                true
            )),
        };

        let is_rotate_sent = match self.rotate_output(output.clone(), transform).await {
            Ok(v) => v,
            Err(e) => bail!(TransformHandlerError::new(
                TransformHandlerErrorCodes::RotateOutputError,
                format!("unable to rotate output by name {}", e),
                true
            )),
        };

        Ok(is_rotate_sent)
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
