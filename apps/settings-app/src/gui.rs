use crate::{
    screens::{
        display::display_screen::DisplayScreen,
        display::screen_off_time::ScreenOffTime,
        language::language_screen::LanguageScreen,
        language::language_select::LanguageSelect,
        settings_menu::settings_screen::SettingsScreen,
        sound::sound_screen::SoundScreen,
        wireless::{
            handler::WirelessInfoItem, network_details_screen::NetworkDetailsScreen,
            network_screen::NetworkScreen,
        },
    },
    settings::{self, MainSettings},
    shared::h_divider::HDivider,
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
use std::any::Any;

#[derive(Default, Debug, Clone, Hash, Copy)]
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
    LanguageScreen,
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
                padding: [5.0, 10.0, 5.0, 10.0],
            ],
        ));
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
            Routes::LanguageScreen => base = base.push(node!(LanguageScreen {})),
            Routes::DisplayScreen => base = base.push(node!(DisplayScreen {})),
            Routes::SoundScreen => base = base.push(node!(SoundScreen {})),
            _ => (),
            // Routes::BluetoothScreen => {
            // Routes::DisplayScreen => todo!(),
            // Routes::LockScreen => todo!(),
        }

        app_node = app_node.push(base);
        return Some(app_node);
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => {
                    // match route {
                    //     _ => (),
                    // }
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
