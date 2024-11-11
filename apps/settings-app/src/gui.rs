// CLEAN CODE

use std::any::Any;

use crate::{
    screens::{
        settings_menu::settings_screen::SettingsScreen, sound::sound_screen::SoundScreen,
        wireless::network_details_screen::NetworkDetailsScreen,
        wireless::network_screen::NetworkScreen,
    },
    settings::{self, MainSettings},
    AppMessage, AppParams,
};
use mctk_core::{
    component::{self, Component, RootComponent},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect,
    reexports::smithay_client_toolkit::reexports::calloop,
    size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, Text},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

#[derive(Debug)]
pub struct NetworkState {
    isConnected: bool,
    connectedValue: String,
}

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
}

// // NOTE : this kind of state - to be maintained in main.rs
// #[derive(Debug, Clone)]
// pub enum Message {
//     Network { value: String },
// }

#[derive(Debug, Clone)]
pub enum Message {
    ChangeRoute { route: Routes },
    // ShowSettingsList,
    // ShowNetworkOptions,
    // ShowBluetoothOptions,
    // ShowDisplayOptions,
    // ShowAppearanceOptions,
    // ShowBatteryOptions,
    // ShowSoundOptions,
    // ShowLockOptions,
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
            app_channel: None,
            current_route: Routes::default(),
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
            Routes::SettingsList => base = base.push(node!(SettingsScreen {})),
            Routes::NetworkScreen => base = base.push(node!(NetworkScreen {})),
            Routes::NetworkDetails => base = base.push(node!(NetworkDetailsScreen {})),
            Routes::BluetoothScreen => todo!(),
            Routes::DisplayScreen => todo!(),
            Routes::AppearanceScreen => todo!(),
            Routes::BatteryScreen => todo!(),
            Routes::SoundScreen => base = base.push(node!(SoundScreen {})),
            Routes::LockScreen => todo!(),
        }

        return Some(base);
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::ChangeRoute { route } => {
                    match route {
                        Routes::SettingsList => {
                            println!("GET DEFAULT VALUES");
                        }
                        Routes::NetworkScreen => {
                            println!("------------GET CONNECTED NETWORK VALUE--------------");
                        }
                        _ => (),
                        // Routes::BluetoothScreen => {
                        //     println!("------------GET CONNECTED BLUETOOTH VALUE--------------");
                        // },
                        // Routes::DisplayScreen => todo!(),
                        // Routes::AppearanceScreen => todo!(),
                        // Routes::BatteryScreen => todo!(),
                        // Routes::SoundScreen => todo!(),
                        // Routes::LockScreen => todo!(),
                    }
                    self.state_mut().current_route = route.clone();
                }
            }
        }

        vec![]
    }
}
impl RootComponent<AppParams> for SettingsApp {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {}
}
