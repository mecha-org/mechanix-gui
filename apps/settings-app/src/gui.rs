use std::any::Any;

use crate::{
    screens::{
        settings_menu::settings_screen::SettingsScreen,
        wireless::{
            handler::WirelessInfoItem, network_details_screen::NetworkDetailsScreen,
            network_screen::NetworkScreen,
        },
    },
    settings::{self, MainSettings},
    AppMessage, AppParams,
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
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;

#[derive(Default, Debug, Clone, Hash)]
pub enum Routes {
    #[default]
    SettingsList,
    NetworkScreen,
    NetworkDetails,
    BluetoothScreen,
    DisplayScreen,
    AppearanceScreen,
    BatteryScreen,
    SoundScreen,
    LockScreen,
}

#[derive(Debug)]
pub struct SettingsAppState {
    settings: MainSettings,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    current_route: Routes,
    connected_network_name: String,
    connected_network_info: Option<WirelessInfoResponse>,
    connected_network_details: Option<WirelessInfoItem>,
    wireless_Status: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute { route: Routes },
    WirelessStatus { status: bool },
    ConnectedNetwork { info: WirelessInfoResponse },
    UpdateWirelessStatus(bool),
    AvailableNetworksList { list: Vec<WirelessInfoItem> },
    ConnectedNetworkDetails { details: Option<WirelessInfoItem> },
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
            settings,
            wireless_Status: false,
            app_channel: None,
            current_route: Routes::default(),
            connected_network_name: String::from(""),
            connected_network_info: None,
            connected_network_details: None,
        });
    }

    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [5., 0., 0., 0.],
            ]
        );

        match self.state_ref().current_route {
            Routes::SettingsList => {
                base = base.push(node!(SettingsScreen {
                    connected_network_name: self.state_ref().connected_network_name.clone()
                }))
            }
            Routes::NetworkScreen => {
                //  Some(WirelessInfoResponse { mac: "12:10:81:ef:3f:2c", frequency: "2412", signal: "-29", flags: "[WPA2-PSK-CCMP][WPS][ESS]", name: "Tejas" })

                base = base.push(node!(NetworkScreen {
                    connected_network: self.state_ref().connected_network_info.clone(),
                    status: self.state_ref().wireless_Status.clone(),
                }))
            }
            Routes::NetworkDetails => {
                base = base.push(node!(NetworkDetailsScreen {
                    connected_network: self.state_ref().connected_network_details.clone(),
                }))
            }
            _ => (),
            // Routes::BluetoothScreen => {
            // Routes::DisplayScreen => todo!(),
            // Routes::SoundScreen => todo!(),
            // Routes::LockScreen => todo!(),
        }

        return Some(base);
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => {
                    match route {
                        _ => (),
                        // Routes::SettingsList => {}
                        // Routes::NetworkScreen => {}
                    }
                    self.state_mut().current_route = route.clone();
                }
                Message::WirelessStatus { status } => {
                    self.state_mut().wireless_Status = status.clone();
                }
                Message::ConnectedNetwork { info } => {
                    self.state_mut().connected_network_name = info.clone().name.to_string();
                    self.state_mut().connected_network_info = Some(info.clone());
                }
                Message::UpdateWirelessStatus(value) => {
                    // println!("CHECK TOGGLE VALUE {:?} ", value.clone());
                    self.state_mut().wireless_Status = value.clone();
                }
                Message::ConnectedNetworkDetails { details } => {
                    self.state_mut().connected_network_details = details.clone();
                }
                Message::AvailableNetworksList { list } => {
                    // println!("Message::AvailableNetworksList  list{:?}", &list);
                }
                _ => (),
            }
        }

        vec![]
    }
}
impl RootComponent<AppParams> for SettingsApp {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
