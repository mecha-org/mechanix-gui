use crate::{
    screens::{
        about::about_device::AboutDevice,
        battery::battery_screen::BatteryScreen,
        bluetooth::{
            bluetooth_pairing_enter_code::BluetoothPairingEnterCode,
            bluetooth_screen::BluetoothScreen,
        },
        display::display_screen::{DisplayScreen, DisplayScreenRoute},
        network::{
            add_network::AddNetwork, network_details::NetworkDetails,
            network_settings::NetworkSettings, networking::NetworkingScreen,
            saved_network_details::SavedNetworkDetails,
            unknown_network_details::UnknownNetworkDetails,
        },
        settings_menu::settings_screen::SettingsScreen,
        sound::sound_screen::{SoundScreen, SoundScreenRoute},
    },
    settings::{self, MainSettings},
    shared::h_divider::HDivider,
    AppMessage, AppParams, WirelessMessage,
};
use mctk_core::{
    component::{self, Component, RootComponent},
    lay,
    layout::{Alignment, Direction},
    node, rect,
    reexports::smithay_client_toolkit::reexports::calloop,
    size_pct,
    style::Styled,
    widgets::Div,
    Color, Node,
};
use mctk_macros::{component, state_component_impl};
use mechanix_system_dbus_client::wireless::KnownNetworkResponse;
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

// #[derive(Default, Debug, Clone)]
// pub enum NetworkScreenRoutes {
//     #[default]
//     NetworkScreen,
//     NetworkDetailsScreen, // available or connected/manage/known
//     ManageNetworksScreen,
//     AvailableNetworksScreen,
//     ConnectNetworkEnterCode,
// }
#[derive(Default, Debug, Clone)]
pub enum NetworkScreenRoutes {
    #[default]
    Networking,
    UnknownNetworkDetails {
        mac: String,
    },
    AddNetwork {
        ssid: String,
    },
    NetworkSettings,
    SavedNetworkDetails {
        mac: String,
    },
    NetworkDetails,
}

#[derive(Default, Debug, Clone)]
pub enum Routes {
    #[default]
    SettingsList,
    Network {
        screen: NetworkScreenRoutes,
    },
    BluetoothScreen,
    BluetoothPairingVerifyCode,
    BluetoothPairingEnterCode,
    BluetoothDeviceInfo,
    ScreenOffTime,
    DisplayScreen,
    AppearanceScreen,
    BatteryScreen,
    PerformanceModes,
    SoundScreen,
    AboutScreen,
    LockScreen,
    LanguageScreen,
    LanguageSelect,
}

#[derive(Debug)]
pub struct SettingsAppState {
    settings: Arc<RwLock<MainSettings>>,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    current_route: Routes,
    connected_network_name: String,
    known_networks_list: Vec<KnownNetworkResponse>,
    wireless_Status: bool,
    add_network_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute { route: Routes },
    ChangeSoundScreenRoute { route: SoundScreenRoute },
    ChangeDisplayScreenRoute { route: DisplayScreenRoute },
}

pub enum NetworkMessage {
    WirelessStatus { status: bool },
    ConnectedNetworkName { name: String },
    KnownNetworksList { list: Vec<KnownNetworkResponse> },
    Toggle(bool),
}

/// # SettingsApp State
///
/// This struct is the state definition of the entire application
#[component(State = "SettingsAppState")]
#[derive(Debug, Default)]
pub struct SettingsApp {}

#[state_component_impl(SettingsAppState)]
impl Component for SettingsApp {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => MainSettings::default(),
        };

        self.state = Some(SettingsAppState {
            settings: Arc::new(RwLock::new(MainSettings::default())),
            wireless_Status: false,
            app_channel: None,
            current_route: Routes::default(),
            connected_network_name: String::from(""),
            // connected_network_details: None,
            // available_networks_list: vec![],
            known_networks_list: vec![],
            add_network_name: String::from(""),
        });
    }

    fn view(&self) -> Option<Node> {
        let mut app_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );
        app_node = app_node.push(node!(
            HDivider { size: 1. },
            lay![
                padding: [2.0, 20.0, 2.0, 20.0],
            ],
        ));
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [0., 20., 0., 20.],
            ]
        );

        match &self.state_ref().current_route {
            Routes::SettingsList => {
                base = base.push(node!(SettingsScreen {
                    connected_network_name: self.state_ref().connected_network_name.clone()
                }))
            }
            Routes::SoundScreen => base = base.push(node!(SoundScreen::new())),
            Routes::Network { screen } => match screen {
                NetworkScreenRoutes::Networking => {
                    base = base.push(node!(NetworkingScreen::new(
                        self.state_ref().wireless_Status.clone(),
                        self.state_ref().connected_network_name.clone() // self.state_ref().connected_network_details.clone()
                    )))
                }
                NetworkScreenRoutes::AddNetwork { ssid } => {
                    base = base.push(node!(AddNetwork::new(ssid.to_string())))
                }
                NetworkScreenRoutes::NetworkSettings => {
                    base = base.push(node!(NetworkSettings::new()))
                }
                NetworkScreenRoutes::NetworkDetails => {
                    base = base.push(node!(NetworkDetails::new()))
                }
                NetworkScreenRoutes::UnknownNetworkDetails { mac } => {
                    base = base.push(node!(UnknownNetworkDetails::new(mac.to_string())))
                }
                NetworkScreenRoutes::SavedNetworkDetails { mac } => {
                    base = base.push(node!(SavedNetworkDetails::new(mac.to_string())))
                }
            },
            Routes::DisplayScreen => base = base.push(node!(DisplayScreen::new())),
            Routes::BatteryScreen => base = base.push(node!(BatteryScreen {})),
            Routes::AboutScreen => base = base.push(node!(AboutDevice {})),
            Routes::BluetoothScreen => base = base.push(node!(BluetoothScreen {})),
            Routes::BluetoothPairingEnterCode => {
                base = base.push(node!(BluetoothPairingEnterCode {}))
            }
            _ => (),
        }

        app_node = app_node.push(base);
        Some(app_node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => match route {
                    Routes::SettingsList => {
                        self.state_mut().current_route = route.clone();
                    }
                    Routes::Network { .. } => {
                        self.state_mut().current_route = route.clone();
                    }
                    _ => {
                        self.state_mut().current_route = route.clone();
                    }
                },
                _ => (),
            }
        }

        if let Some(msg) = message.downcast_ref::<NetworkMessage>() {
            match msg {
                NetworkMessage::WirelessStatus { status } => {
                    self.state_mut().wireless_Status = status.clone();
                }
                NetworkMessage::ConnectedNetworkName { name } => {
                    self.state_mut().connected_network_name = name.to_string();
                }
                NetworkMessage::KnownNetworksList { list } => {
                    self.state_mut().known_networks_list = list.clone();
                }
                NetworkMessage::Toggle(value) => {
                    if let Some(app_channel) = self.state_ref().app_channel.clone() {
                        let _ = app_channel.send(AppMessage::Wireless {
                            message: WirelessMessage::Toggle {
                                value: Some(value.clone()),
                            },
                        });
                    }
                }
            }
        }
        vec![]
    }
}
impl RootComponent<AppParams> for SettingsApp {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
        self.state_mut().settings = app_params.settings.clone();
    }
}
