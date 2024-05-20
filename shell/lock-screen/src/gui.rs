use crate::components::overlay::Overlay;
use crate::components::pin_indicators::MAX_PIN_LENGTH;
use crate::components::unlock_button::UnlockButton;
use crate::pages::pin::Pin;
use crate::settings::{self, LockScreenSettings};
use crate::theme::{self, LockScreenTheme};
use crate::AppMessage;
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension, PositionType};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::widgets::{Carousel, Image};
use mctk_core::{component, layout, Color};
use mctk_core::{
    component::Component,
    event::{Event, Tick},
    lay, msg, node, rect, size, size_pct, state_component_impl,
    widgets::Div,
    Node,
};
use mctk_smithay::session_lock::lock_window::{SessionLockMessage, SessionLockWindow};
use mechanix_status_bar_components::get_formatted_battery_level;
use mechanix_status_bar_components::gui::CommonStatusBar;
use mechanix_status_bar_components::types::{
    BatteryLevel, BatteryStatus, BluetoothStatus, WirelessStatus,
};
use std::any::Any;
use std::time::{Duration, Instant};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum PinKey {
    Home,
    Backspace,
    Text { key: String },
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    Show,
    Hide,
    UnlockPressed,
    UnlockReleased,
    PinKeyClicked(PinKey),
    BackspaceClicked,
    BackClicked,
    ChangeRoute(Routes),
    Clock { current_time: String },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Routes {
    #[default]
    Unlock,
    Pin,
}

#[derive(Debug, Default)]
pub struct LockScreenState {
    settings: LockScreenSettings,
    // custom_theme: LockScreenTheme,
    unlock_pressing_time: u128,
    unlock_pressing: bool,
    unlock_pressed_at: Option<Instant>,
    current_route: Routes,
    pin: String,
    session_lock_sender: Option<Sender<SessionLockMessage>>,
    battery_level: BatteryLevel,
    wireless_status: WirelessStatus,
    bluetooth_status: BluetoothStatus,
    current_time: String,
}

#[component(State = "LockScreenState")]
#[derive(Debug, Default)]
pub struct LockScreen {}

#[state_component_impl(LockScreenState)]
impl Component for LockScreen {
    fn on_tick(&mut self, _event: &mut Event<Tick>) {
        if self.state_ref().unlock_pressing {
            let unlock_pressed_duration = match self.state_ref().unlock_pressed_at {
                Some(start_time) => Instant::now().duration_since(start_time),
                None => Duration::from_secs(0),
            };
            let t_in_ms = unlock_pressed_duration.as_millis();
            self.state_mut().unlock_pressing_time = t_in_ms;
        }
    }

    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        // let custom_theme = match theme::read_theme_yml() {
        //     Ok(theme) => theme,
        //     Err(_) => LockScreenTheme::default(),
        // };
        self.state = Some(LockScreenState {
            settings,
            // custom_theme,
            unlock_pressing_time: 0,
            unlock_pressed_at: None,
            unlock_pressing: false,
            current_route: Routes::default(),
            pin: String::new(),
            session_lock_sender: None,
            battery_level: BatteryLevel::default(),
            wireless_status: WirelessStatus::default(),
            bluetooth_status: BluetoothStatus::default(),
            current_time: String::from(""),
        });
    }

    fn view(&self) -> Option<Node> {
        let unlock_pressing_time = self.state_ref().unlock_pressing_time;
        let pin = self.state_ref().pin.clone();
        let current_route = self.state_ref().current_route;

        let overlay_node = node!(
            Overlay::new(unlock_pressing_time),
            lay! [
                size_pct: [100],
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
            ]
        );

        let screen = match current_route {
            Routes::Unlock => overlay_node.push(node!(UnlockButton::new(unlock_pressing_time)
                .on_press(Box::new(|| msg!(Message::UnlockPressed)))
                .on_release(Box::new(|| msg!(Message::UnlockReleased))),)),

            Routes::Pin => overlay_node.push(node!(
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
                Div::new(),
                lay![
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Stretch,
                    size_pct: [100],
                    direction: layout::Direction::Column
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
                    position_type: mctk_core::layout::PositionType::Absolute,
                    position: [0.0, 0.0, Auto, 0.0],
                ]
            ))
            .push(screen),
        )
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::UnlockPressed) => {
                println!("unlock pressed");
                if !self.state_ref().unlock_pressing {
                    self.state_mut().unlock_pressing = true;
                    self.state_mut().unlock_pressed_at = Some(Instant::now());
                }
            }
            Some(Message::UnlockReleased) => {
                println!("unlock released");

                if self.state_ref().unlock_pressing {
                    let unlock_pressing_time = self.state_mut().unlock_pressing_time;
                    self.state_mut().unlock_pressing = false;
                    self.state_mut().unlock_pressing_time = 0;
                    self.state_mut().unlock_pressed_at = None;
                    if unlock_pressing_time > 750 {
                        let is_pin_enabled = true;
                        if !is_pin_enabled {
                            let _ = unlock(self.state_ref().session_lock_sender.clone());
                        } else {
                            self.state_mut().current_route = Routes::Pin;
                        }
                    }
                }
            }
            Some(Message::PinKeyClicked(pin_key)) => {
                println!("pin key clicked {:?}", pin_key);
                match pin_key {
                    PinKey::Text { key } => {
                        let updated_pin = [self.state_ref().pin.clone(), key.to_string()].join("");
                        self.state_mut().pin = updated_pin.clone();

                        if updated_pin.len() == MAX_PIN_LENGTH {
                            let is_pin_correct = updated_pin == "0000";
                            if is_pin_correct {
                                let _ = unlock(self.state_ref().session_lock_sender.clone());
                            }
                        };
                    }
                    PinKey::Home => {
                        self.state_mut().current_route = Routes::Unlock;
                        self.state_mut().pin = String::new();
                    }
                    PinKey::Backspace => {
                        let mut pin = self.state_ref().pin.clone();
                        if pin.len() > 0 {
                            pin.pop();
                            self.state_mut().pin = pin;
                        }
                    }
                }
            }
            Some(Message::ChangeRoute(route)) => {
                println!("change route ");
                self.state_mut().current_route = *route;
            }
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
            _ => (),
        }
        vec![]
    }
}

impl RootComponent<AppMessage> for LockScreen {
    fn root(&mut self, w: &dyn Any, app_channel: Option<Sender<AppMessage>>) {
        let session_lock_window = w.downcast_ref::<SessionLockWindow>();
        if session_lock_window.is_some() {
            self.state_mut().session_lock_sender = Some(session_lock_window.unwrap().sender());
        }
    }
}

fn unlock(session_lock_sender_op: Option<Sender<SessionLockMessage>>) -> anyhow::Result<bool> {
    if let Some(session_lock_sender) = session_lock_sender_op {
        let _ = session_lock_sender.send(SessionLockMessage::Unlock);
        // std::process::exit(0);
        return Ok(true);
    }

    Ok(false)
}
