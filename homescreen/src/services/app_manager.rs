use crate::Message;
use anyhow::{bail, Result};
use command::spawn_command;
use relm4::Sender;
use services::{ServiceHandler, ServiceStatus};
use std::collections::HashMap;
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
pub enum AppManagerMessage {
    LaunchApp {
        app_id: String,
        start_command: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
}

pub struct AppManagerService {
    status: ServiceStatus,
    pub apps: HashMap<String, HashMap<ToplevelKey, ()>>,
    pub top_level_sender: Option<mpsc::Sender<ToplevelMessage>>,
}

impl AppManagerService {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
            apps: HashMap::new(),
            top_level_sender: None,
        }
    }

    pub async fn run(&mut self, mut message_rx: mpsc::Receiver<AppManagerMessage>) {
        info!("top level handler run called");

        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);

        self.top_level_sender = Some(toplevel_msg_tx.clone());

        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

        // start the toplevel handler
        tokio::spawn(async move {
            let _ = toplevel_handler.run(toplevel_msg_rx).await;
        });

        loop {
            select! {
                msg = message_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    debug!("msg received {:?}", msg);

                    match msg.unwrap() {
                        AppManagerMessage::LaunchApp { app_id, start_command, reply_to } => {
                            let res = self.launch_app(&app_id, &start_command).await;
                            let _ = reply_to.send(res);
                        }
                    }
                }

                event = toplevel_event_rx.recv() => {
                    if event.is_none() {
                        continue;
                    }

                    debug!("event received {:?}", event);

                    match event.unwrap() {
                        ToplevelEvent::Done {
                            key,
                            title,
                            app_id,
                            state,
                        } => {
                            let _ = &self.add_app(app_id, key);
                            info!("all apps are {:?}", self.get_all_apps());
                        }
                        ToplevelEvent::Closed { key } => {
                            let _ = &self.remove_app_instance(key);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn start_app(&self, app_id: &str, start_command: &str) -> Result<bool> {
        let main_command: Vec<&str> = start_command.split(" ").collect();
        let args: Vec<&str> = main_command.clone()[1..]
            .iter()
            .filter(|&&arg| arg != "%u" && arg != "%U" && arg != "%F")
            .cloned()
            .collect();
        match spawn_command(main_command[0], &args) {
            Ok(_) => {
                info!("app started successfully {}", app_id);
            }
            Err(e) => {
                error!(
                    "error while starting app app_id {} command {} error {}",
                    app_id, start_command, e
                );
                bail!(e)
            }
        };

        Ok(true)
    }

    pub async fn launch_app(&self, app_id: &str, exec: &str) -> Result<bool> {
        //check if app is already open, then launch that
        //else spawn app
        let is_app_launched;

        if self.is_app_already_running(app_id) {
            is_app_launched = match self.activate_app(app_id).await {
                Ok(v) => v,
                Err(e) => bail!(e),
            };
        } else {
            is_app_launched = match self.start_app(app_id, exec) {
                Ok(v) => v,
                Err(e) => bail!(e),
            }
        }

        Ok(is_app_launched)
    }

    pub async fn activate_app_instance(&self, key: ToplevelKey) -> Result<bool> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .top_level_sender
            .as_ref()
            .unwrap()
            .send(ToplevelMessage::Activate {
                key: key,
                reply_to: tx,
            })
            .await;

        let reply = match rx.await {
            Ok(v) => v,
            Err(e) => Err(ToplevelHandlerError::new(
                ToplevelHandlerErrorCodes::UnknownError,
                "unable to connect to top level hanler".to_string(),
            )),
        };
        let is_activated = match reply {
            Ok(v) => v,
            Err(e) => {
                error!("error while activating app instance {}", e);
                false
            }
        };

        Ok(is_activated)
    }

    pub async fn activate_app(&self, app_id: &str) -> Result<bool> {
        let instance_op = match self.get_all_instances(app_id) {
            Some(top_level_keys) => top_level_keys.keys().copied().next(),
            None => None,
        };

        let instance = match instance_op {
            Some(v) => v,
            None => {
                bail!("app is not having any top level key")
            }
        };

        let is_activated = match self.activate_app_instance(instance).await {
            Ok(v) => v,
            Err(e) => bail!(e),
        };

        Ok(is_activated)
    }

    pub fn is_app_already_running(&self, app_id: &str) -> bool {
        self.apps.contains_key(app_id)
    }

    pub fn get_all_instances(&self, app_id: &str) -> Option<HashMap<ToplevelKey, ()>> {
        let instances = match self.apps.get_key_value(app_id) {
            Some((_, v)) => Some(v.clone()),
            None => None,
        };

        instances
    }

    pub fn add_app(&mut self, app_id: String, new_instance: ToplevelKey) -> Result<bool> {
        if !(app_id.len() > 0) {
            return Ok(false);
        }

        let mut instances: HashMap<ToplevelKey, ()> = match self.apps.get_key_value(&app_id) {
            Some((_, v)) => v.clone(),
            None => HashMap::new(),
        };
        instances.insert(new_instance, ());
        self.apps.insert(app_id, instances);
        Ok(true)
    }

    pub fn get_all_apps(&self) -> Result<HashMap<String, HashMap<ToplevelKey, ()>>> {
        Ok(self.apps.clone())
    }

    pub fn remove_app_instance(&mut self, instance_to_remove: ToplevelKey) -> Result<bool> {
        let app_op = self
            .apps
            .clone()
            .into_iter()
            .find(|(_, value)| value.contains_key(&instance_to_remove));

        match app_op {
            Some((app_id, mut instances)) => {
                instances.remove_entry(&instance_to_remove);

                if instances.is_empty() {
                    self.apps.remove_entry(&app_id);
                } else {
                    self.apps.insert(app_id, instances);
                }
            }
            None => (),
        }

        Ok(true)
    }
}

#[async_trait]
impl ServiceHandler for AppManagerService {
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
