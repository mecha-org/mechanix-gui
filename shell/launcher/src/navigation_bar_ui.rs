use crate::gui::Message;
use crate::pages::home_ui::HomeUi;
use crate::settings::{self, LauncherSettings};
use crate::theme::{self, LauncherTheme};
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use crate::utils::get_formatted_battery_level;
use crate::{AppMessage, AppParams};
use mctk_core::component::RootComponent;
use mctk_core::event::{self, Event};
use mctk_core::layout::{Alignment, Direction, PositionType};

use mctk_core::prelude::ComponentHasher;
use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::rect::InstanceBuilder;
use mctk_core::renderables::{Image, Rect, Renderable};
use mctk_core::{component, msg, Color, Pos, Scale, AABB};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};
use std::any::Any;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct NavigationBarState {
    app_channel: Option<Sender<AppMessage>>,
    swipe_up: f32,
}

#[component(State = "NavigationBarState")]
#[derive(Debug, Default)]
pub struct NavigationBar {}

#[state_component_impl(NavigationBarState)]
impl Component for NavigationBar {
    fn init(&mut self) {
        self.state = Some(NavigationBarState {
            app_channel: None,
            swipe_up: 10., // session_lock_sender: None,
        });
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        if self.state.is_some() {
            (self.state_ref().swipe_up as u64).hash(hasher);
        }
    }

    fn view(&self) -> Option<Node> {
        let mut start_node = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                size_pct: [100],
                // padding: [22]
            ]
        );

        start_node = start_node.push(
            node!(
                Div::new().bg(Color::rgb(77., 77., 77.)),
                lay![
                    size: [120, 4],
                    position_type: PositionType::Absolute,
                    position: [Auto, Auto, self.state_ref().swipe_up, 210.],
                ]
            )
            .key(self.state_ref().swipe_up as u64),
        );

        Some(start_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message);
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::MinimizeAll => {
                    println!(" Message::MinimizeAll");
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::MinimizeAll);
                    }
                }

                _ => (),
            }
        }
        vec![]
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) {
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut Event<event::Drag>) {
        event.stop_bubbling();
        println!("on_drag() {:?}", event.physical_delta().y.abs());

        self.state_mut().swipe_up = 10. + (event.physical_delta().y.abs() - 15.);
    }

    fn on_drag_end(&mut self, event: &mut Event<event::DragEnd>) {
        event.stop_bubbling();
        println!("on_drag_end()");
        if let Some(app_channel) = self.state_ref().app_channel.clone() {
            let _ = app_channel.send(AppMessage::MinimizeAll);
        }
        self.state_mut().swipe_up = 10.;
    }

    fn on_touch_drag_start(&mut self, event: &mut Event<event::TouchDragStart>) {
        event.stop_bubbling();
    }

    fn on_touch_drag(&mut self, event: &mut Event<event::TouchDrag>) {
        event.stop_bubbling();
        println!("on_drag() {:?}", event.physical_delta().y.abs());

        self.state_mut().swipe_up = 10. + (event.physical_delta().y.abs() - 15.);
    }

    fn on_touch_drag_end(&mut self, event: &mut Event<event::TouchDragEnd>) {
        event.stop_bubbling();
        println!("on_drag_end()");
        if let Some(app_channel) = self.state_ref().app_channel.clone() {
            let _ = app_channel.send(AppMessage::MinimizeAll);
        }
        self.state_mut().swipe_up = 10.;
    }
}

impl RootComponent<AppParams> for NavigationBar {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        let app_channel = app_params.app_channel.clone();
        self.state_mut().app_channel = app_channel;

        // let session_lock_window = window.downcast_ref::<SessionLockWindow>();
        // if session_lock_window.is_some() {
        //     self.state_mut().session_lock_sender = Some(session_lock_window.unwrap().sender());
        // }
    }
}
