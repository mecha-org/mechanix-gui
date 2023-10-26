use std::{env, os::unix::net::UnixStream};

use anyhow::bail;
use anyhow::Result;
use custom_widgets::icon_input::IconPosition;
use gtk::{
    gdk, gio,
    glib::clone,
    prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, WidgetExt},
};
use relm4::Controller;
use relm4::{
    factory::FactoryVecDeque, gtk, Component, ComponentController, ComponentParts, ComponentSender,
    SimpleComponent,
};

use crate::{
    errors::{LockScreenError, LockScreenErrorCodes},
    settings::{LayoutSettings, Modules},
};
use custom_widgets::{
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings,
        InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
    },
    icon_input::{
        IconInput, IconInputCss, IconSettings as IconInputIconSettings,
        InitSettings as IconInputSettings, InputMessage as IconInputInputMessage,
        OutputMessage as IconInputOutputMessage,
    },
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings,
        InputMessage as IconInputPasswordInputMessage,
        OutputMessage as IconInputPasswordOutputMessage,
    },
};
use greetd_ipc::{
    codec::SyncCodec, AuthMessageType, ErrorType, Request as GreetdRequest,
    Response as GreetdResponse,
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
}

//Model
pub struct PasswordAuthentication {
    settings: Settings,
    username: String,
    password: String,
    login_status: Option<LoginResult>,
}

//Widgets
pub struct PasswordAuthenticationWidgets {
    username_input: Controller<IconInput>,
    password_input: Controller<IconInputPassword>,
    login_res_label: gtk::Label,
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    UsernameChange(String),
    PasswordChange(String),
    Submit,
    BackPressed,
}

#[derive(Debug, Copy, Clone)]
enum LoginResult {
    Success,
    Failure,
}

impl SimpleComponent for PasswordAuthentication {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PasswordAuthenticationWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(false)
            .hexpand(false)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();

        let login_label = gtk::Label::builder()
            .label("Login to device")
            .css_classes(["login-label"])
            .halign(gtk::Align::Start)
            .build();

        let login_res_label = gtk::Label::builder().build();

        let username_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: Option::from("Username".to_string()),
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::UsernameChange(text),
            });

        let password_input = IconInputPassword::builder()
            .launch(IconInputPasswordSettings {
                icon: modules.peek_password.icon.default.to_owned(),
                toggle_icon: None,
                placeholder: Option::from("Password".to_string()),
                css: IconInputPasswordCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputPasswordOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        let back_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: modules.back.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        let submit_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: modules.submit.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::Submit,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);
        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();
        footer.append(back_button.widget());
        footer.append(submit_button_widget);

        let form_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["password-auth-container"])
            .vexpand(false)
            .hexpand(false)
            .spacing(16)
            .build();

        // form_box.append(test_input.widget());
        form_box.append(&login_label);
        form_box.append(username_input.widget());
        form_box.append(password_input.widget());
        form_box.append(&login_res_label);

        root.append(&form_box);
        root.append(&footer);
        root.set_focus_child(Option::from(&login_res_label));

        let model = PasswordAuthentication {
            settings: init,
            username: "".to_string(),
            password: "".to_string(),
            login_status: None,
        };

        let widgets = PasswordAuthenticationWidgets {
            username_input,
            password_input,
            login_res_label,
            back_button,
            submit_button,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::UsernameChange(value) => {
                self.username = value;
            }
            Message::PasswordChange(value) => {
                self.password = value;
            }
            Message::Submit => {
                let login_res = login(self.username.clone(), self.password.clone());
                self.login_status = match login_res {
                    Ok(r) => Some(r),
                    Err(_) => Some(LoginResult::Failure),
                };
            }
            Message::BackPressed => {
                sender.output_sender().send(Message::BackPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        match self.login_status {
            Some(login_status) => match login_status {
                LoginResult::Success => {
                    widgets.login_res_label.set_label("Login successfull");
                }
                LoginResult::Failure => {
                    widgets.login_res_label.set_label("Login failed");
                }
            },
            None => (),
        }
    }
}

fn login(username: String, password: String) -> Result<LoginResult> {
    let login_manager_url = match env::var("LOGIN_MANAGER_URL") {
        Ok(v) => v,
        Err(_) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::FindLoginManagerUrlError,
                format!("unable to read LOGIN_MANAGER_URL in env"),
            ));
        }
    };

    let mut stream = match UnixStream::connect(login_manager_url) {
        Ok(v) => v,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::LoginManagerStreamConnectError,
                format!("unable to connect to login manager stream error: {}", e),
            ));
        }
    };

    let username_message = GreetdRequest::CreateSession { username };
    let write_username_message_result = username_message.write_to(&mut stream);
    match write_username_message_result {
        Ok(_) => (),
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamWriteUsernameError,
                format!("unable to write username message in stream  error: {}", e),
            ));
        }
    }

    let read_enter_password_message = GreetdResponse::read_from(&mut stream);
    match read_enter_password_message {
        Ok(r) => r,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamReadEnterPasswordError,
                format!("unable to read next message to enter password error: {}", e),
            ));
        }
    };

    let password_message = GreetdRequest::PostAuthMessageResponse {
        response: Option::from(password),
    };
    let write_password_message_result = password_message.write_to(&mut stream);
    match write_password_message_result {
        Ok(_) => (),
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamWritePasswordError,
                format!("unable to write password message in stream error: {}", e),
            ));
        }
    };

    let read_enter_captcha_message = GreetdResponse::read_from(&mut stream);
    match read_enter_captcha_message {
        Ok(_) => (),
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamReadCaptchaError,
                format!(
                    "unable to read enter captcha message from stream error: {}",
                    e
                ),
            ));
        }
    }

    let captcha_message = GreetdRequest::PostAuthMessageResponse {
        response: Option::from("9".to_string()),
    };
    let write_captcha_message_result = captcha_message.write_to(&mut stream);
    match write_captcha_message_result {
        Ok(_) => (),
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamWriteCaptchaError,
                format!("unable to write captcha message into stream error: {}", e),
            ));
        }
    }

    let read_auth_response = GreetdResponse::read_from(&mut stream);
    let auth_response = match read_auth_response {
        Ok(r) => r,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::StreamReadAuthResponseError,
                format!(
                    "unable to read enter auth response from stream error: {}",
                    e
                ),
            ));
        }
    };

    let login_res = match auth_response {
        GreetdResponse::Success => LoginResult::Success,
        GreetdResponse::Error {
            error_type,
            description,
        } => LoginResult::Failure,
        GreetdResponse::AuthMessage {
            auth_message_type,
            auth_message,
        } => LoginResult::Failure,
    };

    Ok(login_res)
}
