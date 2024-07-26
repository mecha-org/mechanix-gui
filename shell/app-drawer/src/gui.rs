use std::any::Any;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::{fs::read_dir, path::Path};

use crate::components::app::App as AppComponent;
use crate::components::filter_app::FilterApp;
use crate::errors::{AppDrawerError, AppDrawerErrorCodes};
use crate::pages::app_list::AppList;
use crate::pages::search::Search;
use crate::settings::{self, AppDrawerSettings};
use crate::{AppMessage, AppParams};
use anyhow::{bail, Result};
use command::spawn_command;
use desktop_entries::{self, DesktopEntries, DesktopEntry};
use mctk_core::component::{Component, RootComponent};
use mctk_core::layout::{Alignment, Direction};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::Styled;
use mctk_core::widgets::Div;
use mctk_core::{lay, node, size_pct, Color};
use mctk_macros::{component, state_component_impl};

#[derive(Debug)]
pub struct AppDrawerState {
    apps: Vec<DesktopEntry>,
    filtered_apps: Vec<DesktopEntry>,
    current_route: Routes,
    search_text: String,
    settings: AppDrawerSettings,
}

impl Default for AppDrawerState {
    fn default() -> Self {
        Self {
            apps: vec![],
            filtered_apps: vec![],
            search_text: "".to_string(),
            settings: AppDrawerSettings::default(),
            current_route: Routes::default(),
        }
    }
}

/// # AppDrawer State
///
/// This struct is the state definition of the entire application
#[component(State = "AppDrawerState")]
#[derive(Debug, Default)]
pub struct AppDrawer {}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SearchTextChanged(String),
    RunApp { name: String, exec: String },
    ChangeRoute { route: Routes },
}

#[derive(Default, Debug, Clone, Hash)]
pub enum Routes {
    #[default]
    Apps,
    Search,
}

#[state_component_impl(AppDrawerState)]
impl Component for AppDrawer {
    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        if self.state.is_some() {
            self.state_ref().apps.len().hash(hasher);
            self.state_ref().filtered_apps.len().hash(hasher);
            self.state_ref().search_text.hash(hasher);
            self.state_ref().current_route.hash(hasher);
        }
    }

    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => AppDrawerSettings::default(),
        };

        let mut apps = vec![];

        if let Ok(desktop_entries) = DesktopEntries::new() {
            apps = desktop_entries.entries.to_vec();
        };

        println!("apps are {:?}", apps);

        self.state = Some(AppDrawerState {
            settings,
            apps,
            ..Default::default()
        });
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        println!("App was sent: {:?}", msg.downcast_ref::<Message>());
        match msg.downcast_ref::<Message>() {
            Some(Message::ChangeRoute { route }) => {
                match route {
                    Routes::Apps => {
                        self.state_mut().search_text = "".to_string();
                        self.state_mut().filtered_apps = vec![];
                    }
                    _ => (),
                };
                self.state_mut().current_route = route.clone();
            }
            Some(Message::SearchTextChanged(search_text)) => {
                self.state_mut().filtered_apps = self
                    .state_ref()
                    .apps
                    .clone()
                    .into_iter()
                    .filter(|app| {
                        app.name
                            .to_lowercase()
                            .starts_with(&search_text.to_lowercase())
                    })
                    .collect();
            }
            Some(Message::RunApp { name, exec }) => {
                println!("Runing app {:?}", name);
                if !exec.is_empty() {
                    let mut args: Vec<String> = vec!["-c".to_string()];
                    args.push(exec.clone());
                    let _ = spawn_command("sh".to_string(), args);
                }
            }
            _ => (),
        }

        vec![]
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let mut c_node = node!(
            Div::new().bg(Color::rgb(5., 7., 10.)),
            lay![
                size_pct: [100]
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
            ]
        );

        match self.state_ref().current_route {
            Routes::Apps => {
                c_node = c_node.push(node!(AppList {
                    apps: self.state_ref().apps.clone(),
                },));
            }
            Routes::Search => {
                c_node = c_node.push(node!(Search {
                    apps: self.state_ref().filtered_apps.clone(),
                }));
            }
        }

        Some(c_node)
    }
}

impl RootComponent<AppParams> for AppDrawer {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
