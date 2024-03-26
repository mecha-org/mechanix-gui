use std::fmt;

use mctk_core::component::RootComponent;
use mctk_core::layout::Alignment;
use mctk_core::{component, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};

#[cfg(not(target_arch = "wasm32"))]
use mctk_core::reexports::glutin::prelude::*;

use crate::modules::clock::component::ClockComponent;
use crate::modules::window::component::WindowTitleComponent;
use crate::{
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
    TimeTick { current_time: String },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: BatteryLevel },
    Window { title: String },
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessStatus {
    On,
    #[default]
    Off,
    Connected(WirelessConnectedState),
    NotFound,
}

impl fmt::Display for WirelessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessStatus::On => write!(f, "WirelessOn"),
            WirelessStatus::Off => write!(f, "WirelessOff"),
            WirelessStatus::Connected(state) => write!(f, "WirelessConnected({:?})", state),
            WirelessStatus::NotFound => write!(f, "WirelessNotFound"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BluetoothStatus {
    On,
    #[default]
    Off,
    Connected,
    NotFound,
}

impl fmt::Display for BluetoothStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BluetoothStatus::On => write!(f, "BluetoothOn"),
            BluetoothStatus::Off => write!(f, "BluetoothOff"),
            BluetoothStatus::Connected => write!(f, "BluetoothConnected"),
            BluetoothStatus::NotFound => write!(f, "BluetoothNotFound"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BatteryLevel {
    #[default]
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100,
    NotFound,
}

impl fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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
            battery_level: BatteryLevel::Level100,
            wireless_status: WirelessStatus::Connected(WirelessConnectedState::Strong),
            bluetooth_status: BluetoothStatus::Connected,
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
                    wrap: true,
                    padding: [12, 24],
                    size_pct: [100],
                    axis_alignment: Alignment::Start,
                    // cross_alignment: Alignment::SpaceBetween
                ]
            )
            .push(node!(
                ClockComponent {
                    current_time: self.state_ref().current_time.clone(),
                },
                lay![
                    margin: [0,  0],
                ]
            ))
            .push(node!(
                WindowTitleComponent {
                    current_window_title: self.state_ref().current_window_title.clone(),
                },
                lay![
                    margin: [0, 14],
                ]
            ))
            .push(node!(
                WirelessComponent {
                    status: self.state_ref().wireless_status,
                },
                lay![margin: [0, 0]]
            ))
            .push(node!(
                BluetoothComponent {
                    status: self.state_ref().bluetooth_status,
                },
                lay![margin: [0, 14]]
            ))
            .push(node!(
                BatteryComponent {
                    level: self.state_ref().battery_level,
                },
                lay![margin: [0, 0]]
            )),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        //println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::TimeTick { current_time }) => {
                self.state_mut().current_time = current_time.clone();
            }
            Some(Message::Wireless { status }) => {
                self.state_mut().wireless_status = status.clone();
            }
            Some(Message::Bluetooth { status }) => {
                self.state_mut().bluetooth_status = status.clone();
            }
            Some(Message::Battery { level }) => {
                self.state_mut().battery_level = level.clone();
            }
            Some(Message::Window { title }) => {
                self.state_mut().current_window_title = title.clone();
            }
            _ => (),
        }
        vec![]
    }
}

impl RootComponent for StatusBar {}
