use crate::service::PowerOptionsService;
use crate::settings::{self, PowerOptionsSettings};
use crate::{AppMessage, AppParams};

use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension};
use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::{Image, Renderable};
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, IconButton};
use mctk_core::{component, layout, Color, Scale, AABB};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, txt,
    widgets::Div, Node,
};
use std::any::Any;
use std::process;
use std::{collections::HashMap, fmt};

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    ShutdownClicked,
    RestartClicked,
    LogoutClicked,
}

#[derive(Debug)]
pub struct PowerOptionsState {
    settings: PowerOptionsSettings,
    app_channel: Option<Sender<AppMessage>>,
}

#[component(State = "PowerOptionsState")]
#[derive(Debug, Default)]
pub struct PowerOptions {}

#[state_component_impl(PowerOptionsState)]
impl Component for PowerOptions {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => PowerOptionsSettings::default(),
        };
        self.state = Some(PowerOptionsState {
            settings,
            app_channel: None,
        })
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::rgba(0., 0., 0., 0.80)),
                lay![
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )
            .push(node!(
                IconButton::new("shutdown_icon")
                    .on_click(Box::new(|| msg!(Message::ShutdownClicked)))
                    .style("background_color", Color::rgba(42., 42., 44., 0.90))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 20.)
                    .style("radius", 20.),
                lay![
                    size: [100, 100],
                ]
            ))
            .push(node!(
                IconButton::new("restart_icon")
                    .on_click(Box::new(|| msg!(Message::RestartClicked)))
                    .style("background_color", Color::rgba(42., 42., 44., 0.90))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 20.)
                    .style("radius", 20.),
                lay![
                    size: [100, 100],
                    margin: [0, 34]
                ]
            ))
            .push(node!(
                IconButton::new("logout_icon")
                    .on_click(Box::new(|| msg!(Message::LogoutClicked)))
                    .style("background_color", Color::rgba(42., 42., 44., 0.90))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 20.)
                    .style("radius", 20.),
                lay![
                    size: [100, 100],
                ]
            )),
        )
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

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::ShutdownClicked) => {
                let _ = PowerOptionsService::lock();
            }
            Some(Message::RestartClicked) => {
                let _ = PowerOptionsService::restart();
            }
            Some(Message::LogoutClicked) => {
                let _ = PowerOptionsService::suspend();
            }
            _ => (),
        };
        vec![]
    }
}
impl RootComponent<AppParams> for PowerOptions {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
