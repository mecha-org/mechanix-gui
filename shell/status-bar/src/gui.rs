use std::any::Any;
use std::fmt;

use mctk_core::component::RootComponent;
use mctk_core::layout::{self, Alignment};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::{component, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};

use mctk_core::reexports::glutin::prelude::*;
use mechanix_status_bar_components::get_formatted_battery_level;
use mechanix_status_bar_components::gui::CommonStatusBar;
use mechanix_status_bar_components::types::{
    BatteryLevel, BatteryStatus, BluetoothStatus, WirelessStatus,
};

use crate::AppMessage;
use mechanix_status_bar_components::modules::clock::component::ClockComponent;
use mechanix_status_bar_components::modules::window::component::WindowTitleComponent;
use mechanix_status_bar_components::{
    modules::{
        battery::component::BatteryComponent,
        bluetooth::component::BluetoothComponent,
        wireless::component::WirelessComponent,
        // clock::component::{ClockComponent, ClockMessage, ClockState},
        // window::component::{WindowTitleComponent, WindowTitleMessage},
    },
    settings::{self, StatusBarSettings},
};

use crate::theme::{self, StatusBarTheme};

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    Clock { current_time: String },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
    Window { title: String, activated: bool },
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

#[derive(Debug)]
pub struct StatusBarState {
    battery_level: BatteryLevel,
    wireless_status: WirelessStatus,
    bluetooth_status: BluetoothStatus,
    current_time: String,
    current_window_title: String,
    is_any_window_maximized: bool,
}

#[component(State = "StatusBarState")]
#[derive(Debug, Default)]
pub struct StatusBar {}

#[state_component_impl(StatusBarState)]
impl Component for StatusBar {
    fn init(&mut self) {
        self.state = Some(StatusBarState {
            battery_level: BatteryLevel::default(),
            wireless_status: WirelessStatus::default(),
            bluetooth_status: BluetoothStatus::default(),
            current_time: String::from(""),
            current_window_title: String::from(""),
            is_any_window_maximized: false,
        })
    }

    fn view(&self) -> Option<Node> {
        let bg_color = if self.state_ref().is_any_window_maximized {
            Color::rgba(5., 7., 10., 1.)
        } else {
            Color::TRANSPARENT
        };

        Some(
            node!(
                Div::new().bg(bg_color),
                lay![
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Stretch,
                    size_pct: [100],
                    direction: layout::Direction::Row
                ]
            )
            .push(node!(
                CommonStatusBar {
                    battery_level: self.state_ref().battery_level.clone(),
                    wireless_status: self.state_ref().wireless_status.clone(),
                    bluetooth_status: self.state_ref().bluetooth_status.clone(),
                    current_time: self.state_ref().current_time.clone(),
                },
                lay![
                    size: [Auto, 34],
                ]
            )),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::Clock { current_time }) => {
                self.state_mut().current_time = current_time.clone();
            }
            Some(Message::Wireless { status }) => {
                self.state_mut().wireless_status = status.clone();
            }
            Some(Message::Bluetooth { status }) => {
                self.state_mut().bluetooth_status = status.clone();
            }
            Some(Message::Battery { level, status }) => {
                let battery_level = get_formatted_battery_level(level, status);
                self.state_mut().battery_level = battery_level;
            }
            Some(Message::Window { title, activated }) => {
                self.state_mut().current_window_title = title.clone();
                self.state_mut().is_any_window_maximized = *activated;
            }
            _ => (),
        }
        vec![]
    }
}

impl RootComponent<AppMessage> for StatusBar {
    fn root(&mut self, window: &dyn Any, app_channel: Option<Sender<AppMessage>>) {}
}
