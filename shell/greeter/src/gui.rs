use crate::components::pin_indicators::MAX_PIN_LENGTH;
use crate::pages::password::{Captcha, Password, Username};
use crate::pages::pin::Pin;
use crate::pages::power_options::PowerOptions;
use crate::pages::users::Users;
use crate::settings::{self, GreeterSettings};
use crate::theme::{self, GreeterTheme};
use crate::users::{self, UsersSettings};
use crate::{AppMessage, AuthSubmit, LoginHandlerEvents, Prompt};
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension, PositionType};
use mctk_core::widgets::{Carousel, Image};
use mctk_core::{component, layout, Color};
use mctk_core::{
    component::Component,
    event::{Event, Tick},
    lay, msg, node, rect, size, size_pct, state_component_impl,
    widgets::Div,
    Node,
};
use mctk_smithay::lock_window::{SessionLockMessage, SessionLockWindow};
use smithay_client_toolkit::reexports::calloop;
use smithay_client_toolkit::reexports::calloop::channel::Sender;
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
    PinKeyClicked(PinKey),
    BackspaceClicked,
    BackClicked,
    ChangeRoute(Routes),
    UserClicked { username: String },
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Default, Debug, Clone)]
pub enum Routes {
    #[default]
    Users,
    Pin,
    Password(PasswordAuthRoute),
    PowerOptions,
}

#[derive(Default, Debug, Clone)]
pub enum PasswordAuthRoute {
    #[default]
    Username,
    Password {
        message: String,
    },
    Captcha {
        message: String,
    },
}

#[derive(Debug)]
pub enum PasswordAuthMessage {
    UsernameChange(String),
    CaptchaChange(String),
    PasswordChange(String),
    Submit,
    BackPressed,
    // Command(CommandMsg),
}

#[derive(Debug, Copy, Clone)]
enum AuthResult {
    Success,
    Failure,
}

#[derive(Debug)]
pub struct GreeterState {
    settings: GreeterSettings,
    custom_theme: GreeterTheme,
    users_settings: UsersSettings,
    current_route: Routes,
    pin: String,
    app_channel: Option<calloop::channel::Sender<AppMessage>>,

    //Password pages state
    username: String,
    captcha: String,
    error_message: Option<String>,
    password: String,
    auth_status: Option<AuthResult>,
    auth_message: Option<String>,
}

impl Default for GreeterState {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            custom_theme: Default::default(),
            users_settings: Default::default(),
            current_route: Default::default(),
            pin: Default::default(),
            app_channel: Default::default(),
            username: "mecha".to_string(),
            captcha: "9".to_string(),
            error_message: Default::default(),
            password: "mecha".to_string(),
            auth_status: Default::default(),
            auth_message: Default::default(),
        }
    }
}

#[component(State = "GreeterState")]
#[derive(Debug, Default)]
pub struct Greeter {}

#[state_component_impl(GreeterState)]
impl Component for Greeter {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => GreeterSettings::default(),
        };

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => GreeterTheme::default(),
        };

        let users_settings = match users::read_users_yml() {
            Ok(users) => users,
            Err(_) => UsersSettings::default(),
        };

        self.state = Some(GreeterState {
            settings,
            users_settings,
            custom_theme,
            ..Default::default()
        });
    }

    fn view(&self) -> Option<Node> {
        let pin = self.state_ref().pin.clone();
        let current_route = self.state_ref().current_route.clone();
        let users = self.state_ref().users_settings.users.clone();
        let error_message = self.state_ref().error_message.clone();
        let node = match current_route {
            Routes::Users => node!(Users { users }, lay![size_pct: [100]]),

            Routes::Password(route) => match route {
                PasswordAuthRoute::Username => node!(
                    Username {},
                    lay![
                        size_pct: [100]
                    ]
                ),
                PasswordAuthRoute::Password { .. } => node!(
                    Password {
                        default_value: self.state_ref().password.clone()
                    },
                    lay![
                        size_pct: [100]
                    ]
                ),
                PasswordAuthRoute::Captcha { message } => node!(
                    Captcha {
                        message,
                        error_message,
                        default_value: self.state_ref().captcha.clone()
                    },
                    lay![
                        size_pct: [100]
                    ]
                ),
            },

            Routes::Pin => node!(
                Pin {
                    pin_length: pin.len()
                },
                lay![size_pct: [100]]
            ),

            Routes::PowerOptions => node!(PowerOptions {}, lay![size_pct: [100]]),
        };
        Some(node)
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::PinKeyClicked(pin_key)) => {
                println!("pin key clicked {:?}", pin_key);
                match pin_key {
                    PinKey::Text { key } => {
                        let updated_pin = [self.state_ref().pin.clone(), key.to_string()].join("");
                        self.state_mut().pin = updated_pin.clone();

                        if updated_pin.len() == MAX_PIN_LENGTH {
                            let is_pin_correct = updated_pin == "0000";
                            if is_pin_correct {}
                        };
                    }
                    PinKey::Home => self.state_mut().current_route = Routes::Users,
                    PinKey::Backspace => {
                        let mut pin = self.state_ref().pin.clone();
                        if pin.len() > 0 {
                            pin.pop();
                            self.state_mut().pin = pin;
                        }
                    }
                }
            }
            Some(Message::BackClicked) => {
                println!("back clicked ");
                self.state_mut().pin = String::new();
                // self.state_mut().current_route = Routes::Unlock;
            }
            Some(Message::ChangeRoute(route)) => {
                println!("change route ");
                self.state_mut().current_route = route.clone();
            }
            _ => (),
        }

        match message.downcast_ref::<LoginHandlerEvents>() {
            Some(LoginHandlerEvents::ShowErr(message)) => {
                self.state_mut().error_message = Some(message.clone())
            }
            Some(LoginHandlerEvents::ClearErr) => {
                self.state_mut().error_message = None;
            }
            Some(LoginHandlerEvents::HandleGreetdResponse(_)) => todo!(),
            Some(LoginHandlerEvents::Prompts(prompt)) => {
                match prompt {
                    Prompt::Captcha { message } => {
                        self.state_mut().current_route =
                            Routes::Password(PasswordAuthRoute::Captcha {
                                message: message.clone(),
                            });
                    }
                    Prompt::Password { message } => {
                        self.state_mut().current_route =
                            Routes::Password(PasswordAuthRoute::Password {
                                message: message.clone(),
                            });
                    }
                };
            }
            Some(LoginHandlerEvents::AuthError) => {
                self.state_mut().current_route = Routes::Password(PasswordAuthRoute::Username);
                // self.state_mut().captcha = "".to_string();
                // self.state_mut().password = "".to_string();
            }
            _ => (),
        }

        match message.downcast_ref::<PasswordAuthMessage>() {
            Some(PasswordAuthMessage::UsernameChange(username)) => {
                self.state_mut().username = username.clone().to_lowercase();
            }
            Some(PasswordAuthMessage::PasswordChange(password)) => {
                self.state_mut().password = password.clone().to_lowercase();
            }
            Some(PasswordAuthMessage::CaptchaChange(captcha)) => {
                self.state_mut().captcha = captcha.clone();
            }
            Some(PasswordAuthMessage::BackPressed) => {
                let current_route = self.state_ref().current_route.clone();
                match current_route {
                    Routes::Password(route) => match route {
                        PasswordAuthRoute::Username => {
                            self.state_mut().current_route = Routes::Users;
                        }
                        PasswordAuthRoute::Password { .. } => {
                            self.state_mut().current_route = Routes::Users;
                            // self.state_mut().current_route =
                            //     Routes::Password(PasswordAuthRoute::Username);
                        }
                        PasswordAuthRoute::Captcha { .. } => {
                            self.state_mut().current_route =
                                Routes::Password(PasswordAuthRoute::Username);
                        }
                    },
                    _ => (),
                }
            }
            Some(PasswordAuthMessage::Submit) => {
                let username = self.state_ref().username.clone();
                let password = self.state_ref().password.clone();
                let captcha = self.state_ref().captcha.clone();
                let current_route = self.state_ref().current_route.clone();

                let app_channel = self.state_ref().app_channel.clone().unwrap();
                match current_route {
                    Routes::Password(route) => match route {
                        PasswordAuthRoute::Username => {
                            let _ = app_channel
                                .send(AppMessage::AuthSubmit(AuthSubmit::Username(username)));
                        }
                        PasswordAuthRoute::Password { .. } => {
                            let _ = app_channel
                                .send(AppMessage::AuthSubmit(AuthSubmit::Password(password)));
                        }
                        PasswordAuthRoute::Captcha { .. } => {
                            let _ = app_channel
                                .send(AppMessage::AuthSubmit(AuthSubmit::Captcha(captcha)));
                        }
                    },
                    Routes::Users => {
                        let _ = app_channel
                            .send(AppMessage::AuthSubmit(AuthSubmit::Username(username)));
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        vec![]
    }
}

impl RootComponent<AppMessage> for Greeter {
    fn root(
        &mut self,
        window: &dyn Any,
        app_channel: Option<calloop::channel::Sender<AppMessage>>,
    ) {
        self.state_mut().app_channel = app_channel;
    }
}
