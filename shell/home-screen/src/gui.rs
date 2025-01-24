use crate::components::pinned_app::PinnedApp;
use crate::settings::{self, HomescreenSettings};
use crate::theme::{self, HomescreenTheme};
use crate::{AppMessage, AppParams};
use command::spawn_command;
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension};
use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::{Image, Renderable};
use mctk_core::widgets::Carousel;
use mctk_core::{component, layout, Color, Scale, AABB};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, widgets::Div,
    Node,
};
use std::any::Any;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum SettingNames {
    Wireless,
    Bluetooth,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppClicked { app_id: String },
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Default)]
pub struct HomescreenState {
    settings: HomescreenSettings,
    custom_theme: HomescreenTheme,
}

#[component(State = "HomescreenState")]
#[derive(Debug, Default)]
pub struct Homescreen {}

#[state_component_impl(HomescreenState)]
impl Component for Homescreen {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => HomescreenSettings::default(),
        };

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => HomescreenTheme::default(),
        };
        self.state = Some(HomescreenState {
            settings,
            custom_theme,
        });
    }

    fn view(&self) -> Option<Node> {
        let mut pinned_apps_list_node = node!(
            // Carousel::new().scroll_x(),
            Div::new(),
            lay![
                padding: [10, 16, 10, 0],
                size_pct: [100, Auto],
                axis_alignment: Alignment::Start,
                direction: Row,
            ]
        );

        for (i, app) in self
            .state_ref()
            .settings
            .modules
            .apps
            .clone()
            .into_iter()
            .enumerate()
        {
            pinned_apps_list_node = pinned_apps_list_node.push(
                node!(
                    PinnedApp::new(app.app_id.clone(), app.icon.unwrap()).on_click(Box::new(
                        move || msg!(Message::AppClicked {
                            app_id: app.app_id.clone()
                        })
                    ))
                )
                .key(i as u64),
            );
        }

        Some(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [100],
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::End
                ]
            )
            .push(
                node!(
                    Div::new().bg(Color::rgba(5., 7., 10., 0.45)),
                    lay![
                        size: [Auto, 88]
                    ]
                )
                .push(pinned_apps_list_node),
            ),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::AppClicked { app_id }) => {
                println!("app clicked {:?}", app_id);
                let apps = self.state_ref().settings.modules.apps.clone();
                let app = apps.into_iter().find(|app| app.app_id == *app_id).unwrap();
                let run_command = app.run_command;
                if !run_command.is_empty() {
                    let command = run_command[0].clone();
                    let args: Vec<String> = run_command.clone()[1..].to_vec();
                    let _ = spawn_command(command, args);
                }
            }
            _ => (),
        }
        vec![]
    }

    fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
        let width = context.aabb.width();
        let height = context.aabb.height();
        let AABB { pos, .. } = context.aabb;
        let mut rs = vec![];

        let image = Image::new(pos, Scale { width, height }, "background")
            .composite_operation(CompositeOperation::DestinationOver);

        rs.push(Renderable::Image(image));

        Some(rs)
    }
}

impl RootComponent<AppParams> for Homescreen {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
