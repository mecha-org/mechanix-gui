use crate::components::pin_indicators::MAX_PIN_LENGTH;
use crate::pages::permission::Permission;
use crate::pages::pin::Pin;
use crate::settings::{self, PolkitAgentSettings};
use crate::AppMessage;
use clap::Parser;
use command::spawn_command;
use mctk_core::component::RootComponent;
use mctk_core::event::Event;
use mctk_core::layout::{self, Alignment, Dimension};
use mctk_core::reexports::femtovg::{Align, CompositeOperation};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::Styled;
use mctk_core::widgets::Button;
use mctk_core::{component, txt, Color, Point, Pos, Scale, AABB};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, widgets::Div,
    Node,
};
use mechanix_system_dbus_client::security::Security;
use std::any::Any;
use std::ffi::OsString;

#[derive(Default, Debug, Clone, Copy)]
pub enum Routes {
    #[default]
    Permission,
    Pin,
}

#[derive(Debug, Clone)]
pub enum PinKey {
    Close,
    Backspace,
    Text { key: String },
}

pub enum IconType {
    Png,
    Svg,
}

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
    Cancel,
    PinKeyClicked(PinKey),
    Close,
    Next,
    Submit,
    Init(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Default)]
pub struct PolkitAgentState {
    settings: PolkitAgentSettings,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    pin: String,
    current_route: Routes,
    pin_enabled: bool,
    p_message: String,
}

#[component(State = "PolkitAgentState")]
#[derive(Debug, Default)]
pub struct PolkitAgent {}

impl PolkitAgent {}

#[state_component_impl(PolkitAgentState)]
impl Component for PolkitAgent {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => PolkitAgentSettings::default(),
        };

        let pin_enabled = is_pin_enabled();

        self.state = Some(PolkitAgentState {
            settings,
            app_channel: None,
            pin: String::new(),
            current_route: Routes::Permission,
            pin_enabled,
            p_message: String::new(),
        });
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        if let Some(msg) = message.downcast_ref::<Message>() {
            match msg {
                Message::Init(p_message) => {
                    self.state_mut().p_message = p_message.clone();
                }
                Message::Cancel => {
                    println!("Cancel");
                    let _ = self
                        .state_ref()
                        .app_channel
                        .as_ref()
                        .unwrap()
                        .send(AppMessage::Cancel);
                }
                Message::PinKeyClicked(pin_key) => match pin_key {
                    PinKey::Text { key } => {
                        let updated_pin = [self.state_ref().pin.clone(), key.to_string()].join("");
                        self.state_mut().pin = updated_pin.clone();

                        if updated_pin.len() == MAX_PIN_LENGTH {
                            if let Some(app_channel) = self.state_ref().app_channel.as_ref() {
                                let _ = app_channel.send(AppMessage::Authenticate {
                                    password: updated_pin,
                                });
                            }
                        };
                    }
                    PinKey::Close => {
                        self.state_mut().current_route = Routes::Permission;
                        self.state_mut().pin = String::new();
                    }
                    PinKey::Backspace => {
                        let mut pin = self.state_ref().pin.clone();
                        if pin.len() > 0 {
                            pin.pop();
                            self.state_mut().pin = pin;
                        }
                    }
                },
                Message::Close => {
                    // self.state_mut().current_route = Routes::Permission;
                    // self.state_mut().pin = String::new();
                    if let Some(app_channel) = self.state_ref().app_channel.as_ref() {
                        let _ = app_channel.send(AppMessage::Cancel);
                    }
                }
                Message::Next => {
                    self.state_mut().current_route = Routes::Pin;
                    self.state_mut().pin = String::new();
                }
                Message::Submit => {
                    if let Some(app_channel) = self.state_ref().app_channel.as_ref() {
                        let _ = app_channel.send(AppMessage::Authenticate {
                            password: String::new(),
                        });
                    }
                }
            }
        }
        vec![]
    }
    fn view(&self) -> Option<Node> {
        let pin = self.state_ref().pin.clone();
        let current_route = self.state_ref().current_route;
        let pin_enabled = self.state_ref().pin_enabled;
        let mut p_message = self.state_ref().p_message.clone();
        if p_message.is_empty() {
            p_message = "Chromium is requesting permission".to_string();
        }

        let parent_node = node!(
            Div::new()
                .border(Color::TRANSPARENT, 0., (11., 11., 11., 11.))
                .bg(Color::rgb(21., 23., 29.)),
            lay! [
                size: [386, 348],
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
            ]
        );

        let screen = match current_route {
            Routes::Permission => parent_node.push(node!(
                Permission {
                    message: p_message,
                    pin_enabled
                },
                lay![
                    size_pct: [100],
                ]
            )),

            Routes::Pin => parent_node.push(node!(
                Pin {
                    pin_length: pin.len()
                },
                lay![
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )),
        };
        Some(
            node!(
                Div::new().bg(Color::rgba(5., 7., 10., 0.90)),
                lay![
                    // cross_alignment: Alignment::Center,
                    axis_alignment: Alignment::Center,
                    size_pct: [100],
                    padding: [27, 0, 0, 0]
                    // direction: layout::Direction::Column
                ]
            )
            .push(screen),
        )
    }
}

impl RootComponent<AppMessage> for PolkitAgent {
    fn root(&mut self, window: &dyn Any, app_channel: Option<Sender<AppMessage>>) {
        self.state_mut().app_channel = app_channel;
    }
}

fn is_pin_enabled() -> bool {
    let mut password_set = false;

    let password_set_r = Security::is_password_set();

    if let Err(e) = &password_set_r {
        println!("Error while checking password set {:?}", e);
        panic!("Error while checking password set")
    }

    println!("password_set_r {:?}", password_set_r);

    password_set = password_set_r.unwrap_or_default();

    password_set
}
