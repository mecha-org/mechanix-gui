use crate::modules::battery::component::BatteryComponent;
use crate::modules::bluetooth::component::BluetoothComponent;
use crate::modules::brightness::component::BrightnessComponent;
use crate::modules::cpu::component::CpuComponent;
use crate::modules::memory::component::MemoryComponent;
use crate::modules::rotation::component::{RotationComponent, RotationStatus};
use crate::modules::running_apps::component::RunningAppsComponent;
use crate::modules::settings::component::SettingsComponent;
use crate::modules::sound::component::SoundComponent;
use crate::modules::wireless::component::WirelessComponent;
use crate::settings::{self, SettingsPanelSettings};
use crate::theme::{self, SettingsPanelTheme};
use crate::types::{
    BatteryLevel, BatteryStatus, BluetoothStatus, WirelessConnectedState, WirelessStatus,
};
use crate::{AppMessage, BluetoothMessage, BrightnessMessage, SoundMessage, WirelessMessage};
use command::spawn_command;
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, IconButton};
use mctk_core::{component, layout, Color};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, txt,
    widgets::Div, Node,
};
use std::any::Any;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum SettingNames {
    Wireless,
    Bluetooth,
    Rotation,
    Settings,
}
#[derive(Debug, Clone)]
pub enum SliderSettingsNames {
    Brightness { value: i32 },
    Sound { value: i32 },
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
    Rotation { status: RotationStatus },
    RunningApps { count: i32 },
    Cpu { usage: i32 },
    Memory { usage: u64 },
    Sound { value: i32 },
    Brightness { value: i32 },
    Window { title: String },
    Show,
    Hide,
    SettingClicked(SettingNames),
    SliderChanged(SliderSettingsNames),
    SlideUp,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Loading {
    wireless: bool,
    bluetooth: bool,
    rotation: bool,
}

#[derive(Debug)]
pub struct SettingsPanelState {
    settings: SettingsPanelSettings,
    custom_theme: SettingsPanelTheme,
    battery_level: BatteryLevel,
    battery_percentage: u8,
    wireless_status: WirelessStatus,
    bluetooth_status: BluetoothStatus,
    rotation_status: RotationStatus,
    running_apps_count: i32,
    cpu_usage: i32,
    memory_usage: u64,
    sound_value: i32,
    brightness_value: i32,
    loading: Loading,
    app_channel: Option<Sender<AppMessage>>,
    visible: bool,
}

#[component(State = "SettingsPanelState")]
#[derive(Debug, Default)]
pub struct SettingsPanel {}

#[state_component_impl(SettingsPanelState)]
impl Component for SettingsPanel {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => SettingsPanelSettings::default(),
        };
        self.state = Some(SettingsPanelState {
            settings,
            custom_theme: SettingsPanelTheme::default(),
            battery_percentage: 0,
            battery_level: BatteryLevel::Level0,
            wireless_status: WirelessStatus::default(),
            bluetooth_status: BluetoothStatus::default(),
            rotation_status: RotationStatus::Portrait,
            running_apps_count: 0,
            cpu_usage: 0,
            memory_usage: 0,
            sound_value: 0,
            brightness_value: 0,
            loading: Loading::default(),
            app_channel: None,
            visible: true,
        })
    }

    fn view(&self) -> Option<Node> {
        let bg_color = Color::rgba(5., 7., 10., 0.85);
        if !self.state_ref().visible {
            return Some(node!(Div::new(), lay![size: [0, 0]]));
        };

        Some(
            node!(
                Div::new().bg(bg_color),
                lay![
                    wrap: true,
                    padding: [12, 18],
                    size_pct: [100],
                    axis_alignment: Alignment::Start,
                    // cross_alignment: Alignment::SpaceBetween
                ]
            )
            .push(node!(
                WirelessComponent {
                    status: self.state_ref().wireless_status.clone(),
                    loading: self.state_ref().loading.wireless
                },
                lay![margin: rect!(0., 0., 5.5, 7.)]
            ))
            .push(node!(
                BluetoothComponent {
                    status: self.state_ref().bluetooth_status,
                    loading: self.state_ref().loading.bluetooth
                },
                lay![margin: rect!(0., 7., 5.5, 7.)]
            ))
            .push(node!(
                BatteryComponent {
                    level: self.state_ref().battery_level,
                    percentage: self.state_ref().battery_percentage
                },
                lay![margin: rect!(0., 7., 0., 7.)]
            ))
            .push(node!(
                RotationComponent {
                    status: self.state_ref().rotation_status,
                    loading: self.state_ref().loading.rotation
                },
                lay![margin: rect!(0., 7., 0., 0.)]
            ))
            .push(node!(
                SettingsComponent {},
                lay![margin: rect!(5.5, 0., 5.5, 7.)]
            ))
            .push(node!(
                RunningAppsComponent {
                    count: self.state_ref().running_apps_count,
                },
                lay![margin: rect!(5.5, 7., 5.5, 7.)]
            ))
            .push(node!(
                CpuComponent {
                    usage: self.state_ref().cpu_usage,
                },
                lay![margin: rect!(5.5, 7., 5.5, 7.)]
            ))
            .push(node!(
                MemoryComponent {
                    usage: self.state_ref().memory_usage,
                },
                lay![margin: rect!(5.5, 7., 5.5, 0.)]
            ))
            .push(node!(
                SoundComponent {
                    value: self.state_ref().sound_value,
                },
                lay![margin: rect!(5.5, 0., 5.5, 7.)]
            ))
            .push(node!(
                BrightnessComponent {
                    value: self.state_ref().brightness_value,
                },
                lay![margin: rect!(5.5, 7., 5.5, 0.)]
            ))
            .push(node!(
                SlideLine {},
                lay![
                    position_type: Absolute,
                    position: [Auto, Auto, 20.0, 184.0],
                ]
            )),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        // println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
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
                self.state_mut().battery_percentage = *level;
                self.state_mut().battery_level = battery_level;
            }
            Some(Message::Sound { value }) => {
                self.state_mut().sound_value = *value;
            }
            Some(Message::Brightness { value }) => {
                self.state_mut().brightness_value = *value as i32;
            }
            Some(Message::SettingClicked(settings_name)) => {
                println!("setting clicked: {:?}", settings_name);
                match settings_name {
                    SettingNames::Wireless => {
                        let wireless_status = self.state_ref().wireless_status.clone();
                        let value = match wireless_status {
                            WirelessStatus::Off => true,
                            WirelessStatus::On => false,
                            WirelessStatus::Connected(_, _) => false,
                            WirelessStatus::NotFound => false,
                        };
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            self.state_mut().loading.wireless = true;
                            let _ = app_channel.send(AppMessage::Wireless {
                                message: WirelessMessage::Toggle { value: Some(value) },
                            });
                            self.state_mut().loading.wireless = false;
                        }
                    }
                    SettingNames::Bluetooth => {
                        let bluetooth_status = self.state_ref().bluetooth_status.clone();
                        let value = match bluetooth_status {
                            BluetoothStatus::Off => true,
                            BluetoothStatus::On => false,
                            BluetoothStatus::Connected => false,
                            BluetoothStatus::NotFound => false,
                        };
                        if let Some(app_channel) = self.state_ref().app_channel.clone() {
                            self.state_mut().loading.bluetooth = true;
                            let _ = app_channel.send(AppMessage::Bluetooth {
                                message: BluetoothMessage::Toggle { value: Some(value) },
                            });
                            self.state_mut().loading.bluetooth = false;
                        }
                    }
                    SettingNames::Rotation => {}
                    SettingNames::Settings => {
                        let run_command = self
                            .state_ref()
                            .settings
                            .modules
                            .settings
                            .run_command
                            .clone();
                        println!("run_command {:?}", run_command);
                        if !run_command.is_empty() {
                            let command = run_command[0].clone();
                            let args: Vec<String> = run_command.clone()[1..].to_vec();
                            println!("command {:?} args {:?}", command, args);
                            let _ = spawn_command(command, args);
                        }
                    }
                }
            }
            Some(Message::SliderChanged(settings_name)) => match settings_name {
                SliderSettingsNames::Brightness { value } => {
                    self.state_mut().brightness_value = *value;
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::Brightness {
                            message: BrightnessMessage::Change {
                                value: *value as u8,
                            },
                        });
                    }
                }
                SliderSettingsNames::Sound { value } => {
                    self.state_mut().sound_value = *value;
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::Sound {
                            message: SoundMessage::Change {
                                value: *value as u8,
                            },
                        });
                    }
                }
            },
            Some(Message::Cpu { usage }) => {
                self.state_mut().cpu_usage = *usage;
            }
            Some(Message::Memory { usage }) => {
                self.state_mut().memory_usage = *usage;
            }
            Some(Message::RunningApps { count }) => {
                self.state_mut().running_apps_count = *count;
            }
            Some(Message::Show) => {
                self.state_mut().visible = true;
            }
            Some(Message::Hide) => {
                self.state_mut().visible = true;
            }
            Some(Message::SlideUp) => {
                std::process::exit(0);
            }
            _ => (),
        };
        vec![]
    }
}
impl RootComponent<AppMessage> for SettingsPanel {
    fn root(&mut self, window: &dyn Any, app_channel: Option<Sender<AppMessage>>) {
        self.state_mut().app_channel = app_channel;
    }
}

#[derive(Debug)]
pub struct SlideLine {}

impl Component for SlideLine {
    fn on_click(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Click>) {
        event.emit(msg!(Message::SlideUp));
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center,
                    size: [110, 30]
                ],
            )
            .push(node!(
                Button::new(txt!(""))
                    .style("background_color", Color::rgb(129., 129., 129.))
                    .style("active_color", Color::rgb(129., 129., 129.))
                    .style("radius", 2.)
                    .on_click(Box::new(|| msg!(Message::SlideUp))),
                lay![
                    size: [80, 6],
                ]
            )),
        )
    }
}
