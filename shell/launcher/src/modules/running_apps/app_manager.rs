use anyhow::{bail, Result};
use desktop_entries::{DesktopEntries, DesktopEntry};

use command::spawn_command;
use indexmap::IndexMap;
use mctk_core::{
    reexports::smithay_client_toolkit::reexports::calloop::channel::Sender, widgets::IconType,
};

use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tracing::{debug, error, info};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::{
    errors::{ToplevelHandlerError, ToplevelHandlerErrorCodes},
    handler::{
        ToplevelEvent, ToplevelHandler, ToplevelKey, ToplevelMessage, ToplevelMeta, ToplevelWState,
    },
};

use crate::{modules::applications::model::DesktopEntriesModel, AppMessage};

use super::running_app::{AppDetails, AppInstance};

#[derive(Debug)]
pub enum AppManagerMessage {
    CloseAppInstance {
        instance: ToplevelKey,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    ActivateAppInstance {
        instance: ToplevelKey,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CloseAllApps {
        reply_to: oneshot::Sender<Result<bool>>,
    },
    LaunchApp {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CloseApp {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
}

#[derive(Debug, Clone)]
pub struct AppInstanceState {
    app_id: String,
    title: String,
    state: Option<Vec<ToplevelWState>>,
}

#[derive(Debug, Clone)]
pub struct AppManagerService {
    pub apps: IndexMap<String, IndexMap<ToplevelKey, AppInstanceState>>,
    pub top_level_sender: Option<mpsc::Sender<ToplevelMessage>>,
}

impl AppManagerService {
    pub fn new() -> Self {
        Self {
            apps: IndexMap::new(),
            top_level_sender: None,
        }
    }

    pub async fn run(
        &mut self,
        mut message_rx: mpsc::Receiver<AppManagerMessage>,
        app_switcher_sender: Sender<AppMessage>,
    ) {
        info!("top level handler run called");

        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);

        self.top_level_sender = Some(toplevel_msg_tx.clone());

        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

        // start the toplevel handler
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let _ = runtime.block_on(toplevel_handler.run(toplevel_msg_rx));
        });

        let (tx, rx) = oneshot::channel();
        let _ = toplevel_msg_tx
            .send(ToplevelMessage::GetToplevels { reply_to: tx })
            .await;
        let toplevels = rx.await.unwrap();
        for tl in toplevels.into_iter().rev() {
            let (tx, rx) = oneshot::channel();
            let _ = toplevel_msg_tx
                .send(ToplevelMessage::GetToplevelMeta {
                    key: tl,
                    reply_to: tx,
                })
                .await;
            let tl_meta = rx.await.unwrap();

            if let Some(ToplevelMeta {
                app_id,
                title,
                state,
            }) = tl_meta
            {
                let _ = &self.add_app(app_id, tl, title, state);
            };
        }

        loop {
            select! {
                msg = message_rx.recv() => {
                    if msg.is_none() {
                        continue;
                    }

                    debug!("msg received {:?}", msg);

                    match msg.unwrap() {
                        AppManagerMessage::CloseAppInstance { instance, reply_to } => {
                            let res = self.close_app_instance(instance).await;
                            let _ = reply_to.send(res);
                        }
                        AppManagerMessage::ActivateAppInstance { instance, reply_to } => {
                            let res = self.activate_app_instance(instance).await;
                            let _ = reply_to.send(res);
                        }
                        AppManagerMessage::CloseAllApps { reply_to } => {
                            let res = self.close_all_apps().await;
                            let _ = reply_to.send(res);
                        }
                        AppManagerMessage::LaunchApp { app_id, reply_to }=> {
                            let res = self.launch_app(&app_id).await;
                            let _ = reply_to.send(res);
                        }
                        AppManagerMessage::CloseApp { app_id, reply_to }=> {
                            let res = self.close_app(&app_id).await;
                            let _ = reply_to.send(res);
                        }
                    }
                }

                event = toplevel_event_rx.recv() => {
                    if event.is_none() {
                        continue;
                    }

                    // println!("event received {:?}", event);

                    match event.unwrap() {
                        // ToplevelEvent::Created {
                        //     key,
                        // } => {
                        //     let _ = &self.set_instance_fullscreen(key).await;
                        // }
                        ToplevelEvent::Done {
                            key,
                            title,
                            app_id,
                            state,
                        } => {
                            let _ = &self.add_app(app_id.clone().to_lowercase(), key, title, state);
                            let desktop_entries = DesktopEntriesModel::get().entries.get().to_vec();
                            let formatted_apps = format_apps_from_map_to_vec(self.apps.clone(), desktop_entries.clone());
                            let active_apps_count = self.get_active_apps_count();

                            let mut possible_app_id = "".to_string();
                            if let Some(entry) = find_desktop_entry(&app_id.to_lowercase(), &desktop_entries){
                                possible_app_id = entry.app_id.clone();
                            };
                            println!("possible_app_id {:?}", possible_app_id);
                            let _ = app_switcher_sender.send(AppMessage::AppsUpdated  { app_id: possible_app_id, apps: formatted_apps , active_apps_count });
                        }
                        ToplevelEvent::Closed { key } => {
                            let _ = &self.remove_app_instance(key);
                            let active_apps_count = self.get_active_apps_count();
                            let desktop_entries = DesktopEntriesModel::get().entries.get().to_vec();
                            let _ = app_switcher_sender.send(AppMessage::AppsUpdated { app_id: "".to_string(), apps: format_apps_from_map_to_vec(self.apps.clone(), desktop_entries.clone()), active_apps_count });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn start_app(&self, app_id: &str) -> Result<bool> {
        let desktop_entries = DesktopEntriesModel::get().entries.get().to_vec();
        let app = desktop_entries.iter().find(|entry| entry.app_id == app_id);

        if app.is_none() {
            return Ok(false);
        }

        let app = app.unwrap();
        let exec = app.exec.clone();
        if !exec.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(exec.to_string());
            let res = spawn_command("sh".to_string(), args);
            if let Err(why) = res {
                println!("AppManagerService::start_app() error {:?}", why);
            };
        }

        Ok(true)
    }

    pub async fn launch_app(&self, app_id: &str) -> Result<bool> {
        //check if app is already open, then launch that
        //else spawn app
        let is_app_launched;

        if self.is_app_already_running(app_id) {
            println!("activating old instance {:?}", app_id);
            is_app_launched = match self.activate_app(app_id).await {
                Ok(v) => v,
                Err(e) => bail!(e),
            };
        } else {
            println!("launching new instance {:?}", app_id);
            is_app_launched = match self.start_app(app_id) {
                Ok(v) => v,
                Err(e) => bail!(e),
            }
        }

        Ok(is_app_launched)
    }

    pub async fn close_app(&self, app_id: &str) -> Result<bool> {
        let top_level_keys = match self.get_all_instances(app_id) {
            Some(top_level_keys) => top_level_keys,
            None => IndexMap::new(),
        };

        let mut res = Ok(false);
        for (top_level_key, _) in top_level_keys {
            res = self.close_app_instance(top_level_key).await;
        }

        res
    }

    pub async fn close_app_instance(&self, key: ToplevelKey) -> Result<bool> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .top_level_sender
            .as_ref()
            .unwrap()
            .send(ToplevelMessage::Close {
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
        let is_closed = match reply {
            Ok(v) => v,
            Err(e) => {
                error!("error while closing app instance {}", e);
                false
            }
        };

        Ok(is_closed)
    }

    pub async fn close_all_apps(&self) -> Result<bool> {
        for (app_id, instances) in self.apps.iter() {
            for (&instance, _) in instances.iter() {
                let res = self.close_app_instance(instance).await;
                match res {
                    Ok(is_closed) => {}
                    Err(e) => {
                        error!("error while closing instance of {}", app_id);
                    }
                };
            }
        }

        Ok(true)
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

    pub fn get_all_instances(
        &self,
        app_id: &str,
    ) -> Option<IndexMap<ToplevelKey, AppInstanceState>> {
        let instances = match self.apps.get_key_value(app_id) {
            Some((_, v)) => Some(v.clone()),
            None => None,
        };

        instances
    }

    pub fn add_app(
        &mut self,
        app_id: String,
        new_instance: ToplevelKey,
        title: String,
        state: Option<Vec<ToplevelWState>>,
    ) -> Result<bool> {
        if !(app_id.len() > 0) {
            return Ok(false);
        }

        let mut instances: IndexMap<ToplevelKey, AppInstanceState> =
            match self.apps.get_key_value(&app_id) {
                Some((_, v)) => v.clone(),
                None => IndexMap::new(),
            };
        instances.insert(
            new_instance,
            AppInstanceState {
                app_id: app_id.clone(),
                title,
                state,
            },
        );
        self.apps.insert(app_id, instances);
        Ok(true)
    }

    pub async fn set_instance_fullscreen(&mut self, instance: ToplevelKey) -> Result<bool> {
        let (tx, rx) = oneshot::channel();
        let _ = self
            .top_level_sender
            .as_ref()
            .unwrap()
            .send(ToplevelMessage::SetFullscreen {
                key: instance,
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
        let is_set_app_fullscreen = match reply {
            Ok(v) => v,
            Err(e) => {
                error!("error while setting app fullscreen {}", e);
                false
            }
        };

        info!("is set app fullscreen {}", is_set_app_fullscreen);

        Ok(is_set_app_fullscreen)
    }

    pub fn get_all_apps(
        &self,
    ) -> Result<IndexMap<String, IndexMap<ToplevelKey, AppInstanceState>>> {
        Ok(self.apps.clone())
    }

    pub fn get_active_apps_count(&self) -> i32 {
        let mut active_apps_count = 0;
        for (_, tl_key_map) in self.apps.clone() {
            for (_, app) in tl_key_map {
                if let Some(state) = app.state {
                    if !state.contains(&ToplevelWState::Minimized) {
                        active_apps_count += 1;
                    }
                };
            }
        }
        active_apps_count
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

fn format_apps_from_map_to_vec(
    apps: IndexMap<String, IndexMap<ToplevelKey, AppInstanceState>>,
    desktop_entries: Vec<DesktopEntry>,
) -> Vec<AppDetails> {
    let mut apps_vec: Vec<AppDetails> = Vec::new();

    for (app_id, app_instances) in apps {
        let mut name: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut icon_type: Option<IconType> = None;
        let mut path: Option<String> = None;
        if let Some(entry) = find_desktop_entry(&app_id, &desktop_entries) {
            name = Some(entry.name);
            icon = entry.icon_name;
            if let Some(icon_path) = entry.icon_path {
                if let Some(ext) = icon_path.extension() {
                    if ext == "png" {
                        path = Some(icon_path.clone().into_os_string().into_string().unwrap());
                        icon_type = Some(IconType::Png);
                    } else if ext == "svg" {
                        path = Some(icon_path.clone().into_os_string().into_string().unwrap());
                        icon_type = Some(IconType::Svg);
                    }
                };
            }
        }

        let mut app_instances_vec: Vec<AppInstance> = Vec::new();
        app_instances
            .into_iter()
            .for_each(|(instance_key, instance_state)| {
                let app_instance: AppInstance = AppInstance {
                    title: Some(instance_state.title.clone()),
                    instance_key: instance_key,
                    icon: icon.clone(),
                };
                app_instances_vec.push(app_instance);
            });

        let app_details = AppDetails {
            app_id,
            name: name,
            title: None,
            icon: icon,
            icon_type: icon_type,
            icon_path: path,
            instances: app_instances_vec,
        };
        apps_vec.push(app_details);
    }

    apps_vec
}

fn find_desktop_entry(
    app_id: &String,
    desktop_entries: &Vec<DesktopEntry>,
) -> Option<DesktopEntry> {
    println!("find_desktop_entry {:?}", app_id);
    let entry = desktop_entries.clone().into_iter().find(|entry| {
        app_id.to_lowercase() == entry.app_id.to_lowercase()
            || Some(app_id.clone()) == entry.icon_name
            || entry
                .exec
                .clone()
                .to_lowercase()
                .contains(&app_id.to_lowercase())
    });
    entry
}
