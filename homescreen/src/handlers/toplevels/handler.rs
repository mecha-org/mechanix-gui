use super::service::ToplevelService;
use crate::Message;
use anyhow::Result;
use relm4::Sender;
use services::{ServiceHandler, ServiceStatus};
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tonic::{async_trait, Status};
use tracing::{debug, error, info};
use zwlr_foreign_toplevel_v1_async::{
    errors::{ToplevelHandlerError, ToplevelHandlerErrorCodes},
    handler::{ToplevelEvent, ToplevelHandler, ToplevelKey, ToplevelMessage, ToplevelWState},
};

#[derive(Debug)]
pub enum ServiceMessage {
    IsAppOnTopLevel {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    ActivateApp {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },

    AddTopLevelsByAppId {
        app_id: String,
        key: ToplevelKey,
    },
    RemoveTopLevelByKey {
        key: ToplevelKey,
    },
}

pub struct ToplevelServiceHandle {
    status: ServiceStatus,
    top_level_service: ToplevelService,
}

impl ToplevelServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            top_level_service: ToplevelService::new(),
        }
    }

    pub async fn run(
        &mut self,
        message_tx: mpsc::Sender<ServiceMessage>,
        mut message_rx: mpsc::Receiver<ServiceMessage>,
    ) {
        info!("top level handler run called");

        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);

        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

        // start the toplevel handler
        let toplevel_t = tokio::spawn(async move {
            let _ = toplevel_handler.run(toplevel_msg_rx).await;
        });

        let mut top_level_service = ToplevelService::new();
        let top_level_service_handle_t = tokio::spawn(async move {
            loop {
                select! {
                    msg = message_rx.recv() => {
                        if msg.is_none() {
                            continue;
                        }

                        info!("msg received in top level service handler {:?}", msg);

                        match msg.unwrap() {
                            ServiceMessage::IsAppOnTopLevel{app_id, reply_to} => {
                                info!(
                                    "current top levels reply {:?}",
                                    top_level_service.get_all_top_levels().unwrap()
                                );
                                let _ = reply_to.send(top_level_service.top_levels_by_app_id_exists(app_id));
                            }
                            ServiceMessage::AddTopLevelsByAppId{app_id, key} => {
                                let _ = top_level_service.add_top_levels_by_app_id(app_id, key);
                            }
                            ServiceMessage::RemoveTopLevelByKey{key} => {
                                let _ = top_level_service.remove_top_level_by_key(key);
                            }
                            ServiceMessage::ActivateApp{app_id, reply_to} => {
                                let top_level_keys_op =
                                match top_level_service.get_top_levels_keys_by_app_id(app_id) {
                                    Ok(top_level_keys_op) => top_level_keys_op,
                                    Err(_) => None,
                                };

                                let first_top_level_op = match top_level_keys_op {
                                    Some(top_level_keys) => top_level_keys.keys().copied().next(),
                                    None => None,
                                };

                            match first_top_level_op {
                                Some(first_top_level) => {
                                    let (tx, rx) = oneshot::channel();

                                    let _ = toplevel_msg_tx
                                        .send(ToplevelMessage::Activate {
                                            key: first_top_level,
                                            reply_to: tx,
                                        })
                                        .await;

                                    let reply = match rx.await {
                                        Ok(v) => v,
                                        Err(e) => Err(ToplevelHandlerError::new(
                                            ToplevelHandlerErrorCodes::UnknownError,
                                            "".to_string(),
                                        )),
                                    };
                                    let is_app_activated = match reply {
                                        Ok(v) => v,
                                        Err(e) => {
                                            error!("error while activating app {}", e);
                                            false
                                        }
                                    };

                                    info!("app open is  {}", is_app_activated);
                                }
                                None => (),
                            }

                            }

                        }
                    }

                    msg = toplevel_event_rx.recv() => {
                        if msg.is_none() {
                            continue;
                        }
                        match msg.unwrap() {
                            ToplevelEvent::Done {
                                key,
                                title,
                                app_id,
                                state,
                            } => {
                                let _ = message_tx
                                    .send(ServiceMessage::AddTopLevelsByAppId { app_id, key })
                                    .await;
                            }
                            ToplevelEvent::Closed { key } => {
                                let _ = message_tx
                                    .send(ServiceMessage::RemoveTopLevelByKey { key })
                                    .await;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        let _ = toplevel_t.await.unwrap();
        let _ = top_level_service_handle_t.await.unwrap();
    }
}

#[async_trait]
impl ServiceHandler for ToplevelServiceHandle {
    async fn start(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STARTED;
        Ok(true)
    }

    async fn stop(&mut self) -> Result<bool> {
        self.status = ServiceStatus::STOPPED;
        Ok(true)
    }

    fn get_status(&self) -> Result<ServiceStatus> {
        Ok(self.status)
    }

    fn is_stopped(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STOPPED)
    }

    fn is_started(&self) -> Result<bool> {
        Ok(self.status == ServiceStatus::STARTED)
    }
}
