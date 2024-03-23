use crate::modules::battery::component::{BatteryComponent, BatteryLevel};
use crate::modules::bluetooth::component::{BluetoothComponent, BluetoothStatus};
use crate::modules::brightness::component::BrightnessComponent;
use crate::modules::cpu::component::CpuComponent;
use crate::modules::memory::component::MemoryComponent;
use crate::modules::rotation::component::{RotationComponent, RotationStatus};
use crate::modules::running_apps::component::RunningAppsComponent;
use crate::modules::settings::component::SettingsComponent;
use crate::modules::sound::component::SoundComponent;
use crate::modules::wireless::component::{
    WirelessComponent, WirelessConnectedState, WirelessStatus,
};
use crate::settings::SettingsPanelSettings;
use crate::theme::{self, SettingsPanelTheme};
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension};
use mctk_core::{component, layout, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, size_pct, state_component_impl, widgets::Div, Node,
};
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
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: BatteryLevel },
    Rotation { status: RotationStatus },
    RunningApps { count: i32 },
    Cpu { usage: i32 },
    Memory { usage: i32 },
    Sound { value: i32 },
    Brightness { value: i32 },
    Window { title: String },
    Show,
    Hide,
    SettingClicked(SettingNames),
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct SettingsPanelState {
    settings: SettingsPanelSettings,
    custom_theme: SettingsPanelTheme,
    battery_level: BatteryLevel,
    wireless_status: WirelessStatus,
    bluetooth_status: BluetoothStatus,
    rotation_status: RotationStatus,
    running_apps_count: i32,
    cpu_usage: i32,
    memory_usage: i32,
    sound_value: i32,
    brightness_value: i32,
}

#[component(State = "SettingsPanelState")]
#[derive(Debug, Default)]
pub struct SettingsPanel {}

#[state_component_impl(SettingsPanelState)]
impl Component for SettingsPanel {
    fn init(&mut self) {
        self.state = Some(SettingsPanelState {
            settings: SettingsPanelSettings::default(),
            custom_theme: SettingsPanelTheme::default(),
            battery_level: BatteryLevel::Level70,
            wireless_status: WirelessStatus::Connected(WirelessConnectedState::Strong),
            bluetooth_status: BluetoothStatus::Connected,
            rotation_status: RotationStatus::Portrait,
            running_apps_count: 10,
            cpu_usage: 30,
            memory_usage: 50,
            sound_value: 30,
            brightness_value: 50,
        })
    }

    fn view(&self) -> Option<Node> {
        let bg_color = Color::BLACK;

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
                    status: self.state_ref().wireless_status,
                },
                lay![margin: rect!(0., 0., 5.5, 7.)]
            ))
            .push(node!(
                BluetoothComponent {
                    status: self.state_ref().bluetooth_status,
                },
                lay![margin: rect!(0., 7., 5.5, 7.)]
            ))
            .push(node!(
                BatteryComponent {
                    level: self.state_ref().battery_level,
                },
                lay![margin: rect!(0., 7., 0., 7.)]
            ))
            .push(node!(
                RotationComponent {
                    status: self.state_ref().rotation_status,
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
            )),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::Wireless { status }) => {
                self.state_mut().wireless_status = status.clone();
            }
            Some(Message::Bluetooth { status }) => {
                self.state_mut().bluetooth_status = status.clone();
            }
            Some(Message::Battery { level }) => {
                self.state_mut().battery_level = level.clone();
            }
            Some(Message::Sound { value }) => {
                self.state_mut().sound_value = *value;
            }
            Some(Message::Brightness { value }) => {
                self.state_mut().brightness_value = *value;
            }
            Some(Message::SettingClicked(settings_name)) => {
                println!("setting clicked: {:?}", settings_name);
                match settings_name {
                    SettingNames::Wireless => {}
                    SettingNames::Bluetooth => {
                        self.state_mut().bluetooth_status = BluetoothStatus::Off;
                    }
                }
            }
            _ => (),
        }
        vec![]
    }
}
impl RootComponent for SettingsPanel {}
