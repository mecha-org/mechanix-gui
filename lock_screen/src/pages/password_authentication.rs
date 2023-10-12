use std::{env, os::unix::net::UnixStream};

use anyhow::bail;
use anyhow::Result;
use gtk::{
    gdk, gio,
    glib::clone,
    prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, WidgetExt},
};
use relm4::{
    factory::FactoryVecDeque, gtk, Component, ComponentController, ComponentParts, ComponentSender,
    SimpleComponent,
};

use crate::{
    errors::{LockScreenError, LockScreenErrorCodes},
    settings::{LayoutSettings, Modules},
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
    username_input: gtk::Entry,
    password_input: gtk::PasswordEntry,
    login_res_label: gtk::Label,
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
            .spacing(10)
            .css_classes(["password-auth-container"])
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
            .label("Login using your device credentials")
            .css_classes(["login-label"])
            .halign(gtk::Align::Start)
            .build();

        let username_input = gtk::Entry::builder().placeholder_text("Username").build();

        let password_input = gtk::PasswordEntry::builder()
            .placeholder_text("Password")
            .show_peek_icon(true)
            .build();
        let login_res_label = gtk::Label::builder().build();
        let submit_button = gtk::Button::builder()
            .css_classes(["submit-button"])
            .build();
        match modules.submit.icon.default {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .build();
                submit_button.set_child(Some(&icon_image));
            }
            None => (),
        };
        let submit_button_box = gtk::Box::builder().build();
        submit_button_box.append(&submit_button);
        let back_button = gtk::Button::builder().css_classes(["back-button"]).build();
        match modules.back.icon.default {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .build();
                back_button.set_child(Some(&icon_image));
            }
            None => (),
        };
        let back_button_box = gtk::Box::builder().hexpand(true).build();
        back_button_box.append(&back_button);
        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();
        footer.append(&back_button_box);
        footer.append(&submit_button_box);

        username_input.connect_changed(clone!(@strong sender => move |entry| {
            info!("username change called {}", entry.text());
            sender.input(Message::UsernameChange(entry.text().into()));
        }));

        password_input.connect_changed(clone!(@strong sender => move |entry| {
            sender.input(Message::PasswordChange(entry.text().into()));

        }));

        back_button.connect_clicked(clone!(@strong sender => move |_| {
            let _ = sender.output(Message::BackPressed);
        }));

        submit_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(Message::Submit);
        }));

        root.append(&login_label);
        root.append(&username_input);
        root.append(&password_input);
        root.append(&login_res_label);
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
            _ => (),
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
