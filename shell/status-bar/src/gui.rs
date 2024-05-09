use std::any::Any;
use std::fmt;

use mctk_core::component::RootComponent;
use mctk_core::layout::Alignment;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::{component, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};

use mctk_core::reexports::glutin::prelude::*;

use crate::modules::clock::component::ClockComponent;
use crate::modules::window::component::WindowTitleComponent;
use crate::types::{
    BatteryLevel, BatteryStatus, BluetoothStatus, WirelessConnectedState, WirelessStatus,
};
use crate::AppMessage;
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
                    padding: [6, 14, 5, 10],
                    size_pct: [100],
                    axis_alignment: Alignment::Stretch,
                    //  cross_alignment: Alignment::SpaceBetween
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::Start
                    ],
                )
                .push(node!(
                    ClockComponent {
                        current_time: self.state_ref().current_time.clone(),
                    },
                    lay![
                        margin: [0,  0],
                    ]
                )),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::End
                    ]
                )
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
            ),
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
                let battery_level = if *status == BatteryStatus::Unknown {
                    BatteryLevel::NotFound
                } else if *status == BatteryStatus::Charging {
                    match level {
                        0..=9 => BatteryLevel::ChargingLevel10,
                        10..=19 => BatteryLevel::ChargingLevel20,
                        20..=34 => BatteryLevel::ChargingLevel30,
                        35..=49 => BatteryLevel::ChargingLevel40,
                        50..=59 => BatteryLevel::ChargingLevel50,
                        60..=69 => BatteryLevel::ChargingLevel60,
                        70..=79 => BatteryLevel::ChargingLevel70,
                        80..=89 => BatteryLevel::ChargingLevel80,
                        90..=94 => BatteryLevel::ChargingLevel90,
                        95..=100 => BatteryLevel::ChargingLevel100,
                        _ => BatteryLevel::NotFound,
                    }
                } else {
                    match level {
                        0..=9 => BatteryLevel::Level10,
                        10..=19 => BatteryLevel::Level20,
                        20..=34 => BatteryLevel::Level30,
                        35..=49 => BatteryLevel::Level40,
                        50..=59 => BatteryLevel::Level50,
                        60..=69 => BatteryLevel::Level60,
                        70..=79 => BatteryLevel::Level70,
                        80..=89 => BatteryLevel::Level80,
                        90..=94 => BatteryLevel::Level90,
                        95..=100 => BatteryLevel::Level100,
                        _ => BatteryLevel::NotFound,
                    }
                };

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
