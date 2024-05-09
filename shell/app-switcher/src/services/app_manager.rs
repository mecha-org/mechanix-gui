use std::path::PathBuf;

use crate::{
    components::running_app::{AppDetails, AppInstance},
    gui::AppMessage,
    Message,
};
use anyhow::{bail, Result};
use desktop_entries::{DesktopEntries, DesktopEntry};

use command::spawn_command;
use indexmap::IndexMap;
use kmeans_colors::{get_kmeans, Calculate, Kmeans, MapColor, Sort};
use mctk_core::{widgets::IconType, Color};
use palette::{
    cast::{from_component_slice, into_component_slice},
    Srgb, Srgba,
};
use palette::{FromColor, IntoColor, Lab};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tracing::{debug, error, info};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::{
    errors::{ToplevelHandlerError, ToplevelHandlerErrorCodes},
    handler::{ToplevelEvent, ToplevelHandler, ToplevelKey, ToplevelMessage, ToplevelWState},
};

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
}

#[derive(Debug, Clone)]
pub struct AppInstanceState {
    app_id: String,
    title: String,
}

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
                    }
                }

                event = toplevel_event_rx.recv() => {
                    if event.is_none() {
                        continue;
                    }

                    println!("event received {:?}", event);

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
                            let _ = &self.add_app(app_id, key, title);
                            info!("all apps are {:?}", self.get_all_apps());

                            let _ = app_switcher_sender.send(AppMessage::AppsUpdated { apps: format_apps_from_map_to_vec(self.apps.clone()) });
                        }
                        ToplevelEvent::Closed { key } => {
                            let _ = &self.remove_app_instance(key);

                            let _ = app_switcher_sender.send(AppMessage::AppsUpdated { apps: format_apps_from_map_to_vec(self.apps.clone()) });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn start_app(&self, app_id: &str, exec: &str) -> Result<bool> {
        if !exec.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(exec.to_string());
            let _ = spawn_command("sh".to_string(), args);
        }

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
) -> Vec<AppDetails> {
    let desktop_entries_r = DesktopEntries::new();

    if let Err(e) = desktop_entries_r {
        println!("Error while getting desktop entries {:?}", e);
        return vec![];
    };

    let mut apps_vec: Vec<AppDetails> = Vec::new();
    let desktop_entries = desktop_entries_r.unwrap();

    for (app_id, app_instances) in apps {
        let mut name: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut icon_type: Option<IconType> = None;
        let mut path: Option<String> = None;
        let mut dominant_color: Option<Color> = None;
        if let Some(entry) = desktop_entries.entries.clone().into_iter().find(|entry| {
            entry
                .exec
                .clone()
                .to_lowercase()
                .contains(&app_id.to_lowercase())
        }) {
            name = Some(entry.name);
            icon = entry.icon_name;
            if let Some(icon_path) = entry.icon_path {
                println!("icon path {:?}", icon_path);
                if let Some(ext) = icon_path.extension() {
                    if ext == "png" {
                        path = Some(icon_path.clone().into_os_string().into_string().unwrap());
                        icon_type = Some(IconType::Png);
                    } else if ext == "svg" {
                        path = Some(icon_path.clone().into_os_string().into_string().unwrap());
                        icon_type = Some(IconType::Svg);
                    }
                };

                if let Ok(color) = find_dominant_color(icon_path) {
                    dominant_color = Some(color);
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
            icon_dominant_color: dominant_color,
        };
        apps_vec.push(app_details);
    }

    apps_vec
}

fn find_dominant_color(path: PathBuf) -> Result<Color> {
    let ext_o = path.extension();
    if ext_o.is_none() {
        bail!("unable to get extension of {:?}", path);
    }

    let ext = ext_o.unwrap();

    //Check unsupported ext
    if ext != "png" && ext != "jpg" && ext != "jpeg" {
        bail!("unsupported extension {:?}", ext);
    }

    let img_r = image::open(path);
    if let Err(e) = img_r {
        bail!("error while reading image {:?}", e);
    };

    let img_vec = img_r.unwrap().into_rgb8().to_vec();
    let runs = 1;
    let k = 3;
    let max_iter = 20;
    let converge = 8.0;
    let verbose = true;
    let seed = 0;
    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(&img_vec)
        .iter()
        .map(|x| x.into_format().into_color())
        .collect();
    let mut result = Kmeans::new();
    for i in 0..runs {
        let run_result = get_kmeans(k, max_iter, converge, verbose, &lab, seed + i as u64);
        if run_result.score < result.score {
            result = run_result;
        }
    }
    // Using the results from the previous example, process the centroid data
    let mut res = Lab::sort_indexed_colors(&result.centroids, &result.indices);

    // We can find the dominant color directly
    let color_lab_r = Lab::get_dominant_color(&res);

    if color_lab_r.is_none() {
        bail!("error while getting color lab");
    }

    let color_lab = color_lab_r.unwrap();

    let dominant_color = Srgb::from_color(color_lab).into_format::<u8>();

    println!("dominant_color {:?}", dominant_color);
    use colors_transform::Color;
    let lighten_color;
    {
        lighten_color = colors_transform::Rgb::from(
            dominant_color.red as f32,
            dominant_color.green as f32,
            dominant_color.blue as f32,
        )
        .lighten(35.0)
        .saturate(83.)
        .adjust_hue(10.);
    }

    println!("lighten_color {:?}", lighten_color);

    Ok(mctk_core::Color::rgb(
        lighten_color.get_red(),
        lighten_color.get_green(),
        lighten_color.get_blue(),
    ))
}
