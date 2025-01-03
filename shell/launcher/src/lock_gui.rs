use crate::gui::Message;
use crate::pages::home_ui::HomeUi;
use crate::settings::{self, LauncherSettings};
use crate::theme::{self, LauncherTheme};
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use crate::utils::get_formatted_battery_level;
use crate::{AppMessage, AppParams};
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Direction};

use mctk_core::reexports::femtovg::CompositeOperation;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::rect::InstanceBuilder;
use mctk_core::renderables::{Image, Rect, Renderable};
use mctk_core::{component, Color, Pos, Scale, AABB};
use mctk_core::{
    component::Component, lay, node, size_pct, state_component_impl, widgets::Div, Node,
};
use mctk_smithay::session_lock::lock_window::{SessionLockMessage, SessionLockWindow};
use std::any::Any;

#[derive(Debug, Default)]
pub struct LockscreenState {
    settings: LauncherSettings,
    custom_theme: LauncherTheme,
    bluetooth_status: BluetoothStatus,
    app_channel: Option<Sender<AppMessage>>,
    // session_lock_sender: Option<Sender<SessionLockMessage>>,
}

#[component(State = "LockscreenState")]
#[derive(Debug, Default)]
pub struct Lockscreen {}

#[state_component_impl(LockscreenState)]
impl Component for Lockscreen {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LauncherSettings::default(),
        };

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LauncherTheme::default(),
        };
        self.state = Some(LockscreenState {
            settings,
            custom_theme,
            bluetooth_status: BluetoothStatus::default(),
            app_channel: None,
            // session_lock_sender: None,
        });
    }

    fn view(&self) -> Option<Node> {
        let bluetooth_status = self.state_ref().bluetooth_status.clone();
        let settings = self.state_ref().settings.clone();

        let mut start_node = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.64)),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                direction: Direction::Column,
                // padding: [22]
            ]
        );

        start_node = start_node.push(node!(
            HomeUi {
                settings,
                bluetooth_status,
                is_lock_screen: true,
                ..Default::default()
            },
            lay![size_pct: [100, Auto],]
        ));

        Some(start_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message);
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::Bluetooth { status } => {
                    self.state_mut().bluetooth_status = status.clone();
                }
                Message::Unlock => {
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::Unlock);
                    }
                }

                _ => (),
            }
        }
        vec![]
    }

    fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
        // if self.state_ref().running_apps_count > 0 {
        //     return None;
        // }

        let mut rs = vec![];
        if self
            .state_ref()
            .settings
            .modules
            .background
            .icon
            .default
            .len()
            > 0
        {
            let width = context.aabb.width();
            let height = context.aabb.height();
            let AABB { pos, .. } = context.aabb;

            let image = Image::new(pos, Scale { width, height }, "background")
                .composite_operation(CompositeOperation::DestinationOver);

            rs.push(Renderable::Image(image));
        } else {
            let mut rect_instance = InstanceBuilder::default()
                .pos(Pos {
                    x: context.aabb.pos.x,
                    y: context.aabb.pos.y,
                    z: 0.1,
                })
                .scale(context.aabb.size())
                .color(Color::BLACK)
                .build()
                .unwrap();

            rs.push(Renderable::Rect(Rect::from_instance_data(rect_instance)))
        }

        Some(rs)
    }
}

impl RootComponent<AppParams> for Lockscreen {
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
