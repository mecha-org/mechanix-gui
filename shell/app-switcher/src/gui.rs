use std::any::Any;
use std::{fmt, process};

use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{IconButton, Text};
use mctk_core::{component, msg, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, txt, widgets::Div,
    Node,
};

use mctk_core::reexports::glutin::prelude::*;
use smithay_client_toolkit::reexports::calloop;
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;

use crate::components::running_app::{AppDetails, AppInstance, RunningApp};
use crate::components::running_apps_carousel::Carousel;
use crate::services::app_manager::AppManagerMessage;
use crate::settings::{self, AppSwitcherSettings};

use crate::theme::{self, AppSwitcherTheme};

#[derive(Debug, Clone)]
pub enum AppMessage {
    AppsUpdated { apps: Vec<AppDetails> },
    AppInstanceClicked(ToplevelKey),
    AppInstanceCloseClicked(ToplevelKey),
    CloseAllApps,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppsUpdated { apps: Vec<AppDetails> },
    AppInstanceClicked(ToplevelKey),
    AppInstanceCloseClicked(ToplevelKey),
    CloseAllApps,
    BackPressed,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct AppSwitcherState {
    running_apps: Vec<RunningApp>,
    // app_manager_sender: mpsc::Sender<AppManagerMessage>,
    cpu_usage: String,
    memory_usage: String,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[component(State = "AppSwitcherState")]
#[derive(Debug, Default)]
pub struct AppSwitcher {}

#[state_component_impl(AppSwitcherState)]
impl Component for AppSwitcher {
    fn init(&mut self) {
        let running_apps = vec![
            // RunningApp::new(AppDetails {
            //     app_id: "1".to_string(),
            //     name: Some("Chromium".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Systems - Home".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("chromium".to_string()),
            //     }],
            // }),
            // RunningApp::new(AppDetails {
            //     app_id: "2".to_string(),
            //     name: Some("Mecha Connect".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Connect Settings".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("mecha_connect".to_string()),
            //     }],
            // }),
            // RunningApp::new(AppDetails {
            //     app_id: "1".to_string(),
            //     name: Some("Chromium".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Systems - Home".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("chromium".to_string()),
            //     }],
            // }),
            // RunningApp::new(AppDetails {
            //     app_id: "2".to_string(),
            //     name: Some("Mecha Connect".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Connect Settings".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("mecha_connect".to_string()),
            //     }],
            // }),
            // RunningApp::new(AppDetails {
            //     app_id: "1".to_string(),
            //     name: Some("Chromium".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Systems - Home".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("chromium".to_string()),
            //     }],
            // }),
            // RunningApp::new(AppDetails {
            //     app_id: "2".to_string(),
            //     name: Some("Mecha Connect".to_string()),
            //     title: None,
            //     icon: None,
            //     icon_dominant_color: None,
            //     instances: vec![AppInstance {
            //         title: Some("Mecha Connect Settings".to_string()),
            //         instance_key: ToplevelKey::default(),
            //         icon: Some("mecha_connect".to_string()),
            //     }],
            // }),
        ];

        self.state = Some(AppSwitcherState {
            running_apps,
            cpu_usage: "".to_string(),
            memory_usage: "".to_string(),
            app_channel: None,
        })
    }

    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 0.45)),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::BackPressed)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
            .push(node!(
                IconButton::new("close_all_icon")
                    .on_click(Box::new(|| msg!(Message::CloseAllApps)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        let mut running_apps_list_node = if self.state_ref().running_apps.len() > 0 {
            node!(
                Carousel::new(),
                lay![
                    padding: [14, 24, 18, 0],
                    margin: [20, 0, 0, 0],
                    size_pct: [100, Auto],
                    direction: Row,
                ]
            )
        } else {
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )
            .push(node!(
                Text::new(txt!("No apps running".to_string()))
                    .style("color", Color::rgb(197., 200., 207.))
                    .style("size", 12.0)
                    .style("font_weight", FontWeight::Normal) // .style("v_alignment", VerticalPosition::Center)
            ))
        };

        for (i, app) in self
            .state_ref()
            .running_apps
            .clone()
            .into_iter()
            .enumerate()
        {
            running_apps_list_node = running_apps_list_node.push(
                node!(
                    app,
                    lay![
                        size: [208, 208],
                        margin: [10]
                    ]
                )
                .key(i as u64),
            );
        }

        Some(
            node!(
                Div::new().bg(Color::rgba(5., 7., 10., 0.75)),
                lay![
                    size_pct: [100]
                    direction: Direction::Column,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(running_apps_list_node)
            .push(footer),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message);
        match message.downcast_ref::<Message>() {
            Some(Message::AppsUpdated { apps }) => {
                println!("apps updated are {:?}", apps);
                self.state_mut().running_apps = apps
                    .iter()
                    .map(|app| {
                        RunningApp::new(AppDetails {
                            icon: Some("chromium".to_string()),
                            ..app.clone()
                        })
                    })
                    .collect();
            }
            Some(Message::AppInstanceClicked(instance)) => {
                if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    let _ = app_channel.send(AppMessage::AppInstanceClicked(*instance));
                    process::exit(0)
                };
            }
            Some(Message::AppInstanceCloseClicked(instance)) => {
                if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    let _ = app_channel.send(AppMessage::AppInstanceCloseClicked(*instance));
                };
            }
            Some(Message::BackPressed) => {
                process::exit(0);
            }
            Some(Message::CloseAllApps) => {
                if let Some(app_channel) = self.state_ref().app_channel.clone() {
                    let _ = app_channel.send(AppMessage::CloseAllApps);
                };
            }
            _ => (),
        }
        vec![]
    }
}

impl RootComponent<AppMessage> for AppSwitcher {
    fn root(
        &mut self,
        window: &dyn Any,
        app_channel: Option<calloop::channel::Sender<AppMessage>>,
    ) {
        self.state_mut().app_channel = app_channel;
    }
}
