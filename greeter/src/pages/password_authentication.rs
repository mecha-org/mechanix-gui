use gtk::prelude::{BoxExt, WidgetExt};
use relm4::component::AsyncComponent;
use relm4::component::AsyncComponentParts;
use relm4::AsyncComponentSender;
use relm4::Controller;
use relm4::Sender;
use relm4::{async_trait::async_trait, gtk, Component, ComponentController};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::handlers::login::handler::LoginHandlerMessage;
use crate::settings::{LayoutSettings, Modules};
use crate::CommandMsg;
use crate::Prompt;
use custom_widgets::{
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings,
        OutputMessage as IconButtonOutputMessage,
    },
    icon_input::{
        IconInput, IconInputCss, InitSettings as IconInputSettings,
        InputMessage as IconInputInputMessage, OutputMessage as IconInputOutputMessage,
    },
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings,
        OutputMessage as IconInputPasswordOutputMessage,
    },
};
use tracing::{error, info};

#[derive(Debug, PartialEq)]
pub enum LoginSteps {
    UsernameInput,
    CaptchaInput { message: String },
    PasswordInput { message: String },
}

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub login_handler_sender: Option<mpsc::Sender<LoginHandlerMessage>>,
}

//Model
pub struct PasswordAuthentication {
    settings: Settings,
    username: String,
    captcha: String,
    error_message: Option<String>,
    password: String,
    login_step: LoginSteps,
    login_status: Option<LoginResult>,
    auth_message: Option<String>,
    captcha_input_sender: Sender<IconInputInputMessage>,
}

//Widgets
pub struct PasswordAuthenticationWidgets {
    username_input: Controller<IconInput>,
    captcha_input: Controller<IconInput>,
    password_input: Controller<IconInputPassword>,
    login_res_label: gtk::Label,
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
    auth_message_label: gtk::Label,
    error_message_label: gtk::Label,
}

//Messages
#[derive(Debug)]
pub enum Message {
    UsernameChange(String),
    CaptchaChange(String),
    PasswordChange(String),
    Submit,
    BackPressed,
    Command(CommandMsg),
}

#[derive(Debug, Copy, Clone)]
enum LoginResult {
    Success,
    Failure,
}

#[async_trait(?Send)]
impl AsyncComponent for PasswordAuthentication {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PasswordAuthenticationWidgets;
    type CommandOutput = CommandMsg;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(false)
            .hexpand(false)
            .build()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
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

        let auth_message_label = gtk::Label::builder().halign(gtk::Align::Start).build();

        let captcha_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: None,
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::CaptchaChange(text),
            });

        captcha_input.widget().set_visible(false);

        let password_input = IconInputPassword::builder()
            .launch(IconInputPasswordSettings {
                icon: modules.peek_password.icon.default.to_owned(),
                toggle_icon: modules.un_peek_password.icon.default.to_owned(),
                placeholder: Option::from("Password".to_string()),
                css: IconInputPasswordCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputPasswordOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        password_input.widget().set_visible(false);

        let back_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: modules.back.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        let error_message_label = gtk::Label::builder().build();

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
        //form_box.append(&auth_message_label);
        form_box.append(captcha_input.widget());
        form_box.append(password_input.widget());
        form_box.append(&error_message_label);
        form_box.append(&login_res_label);

        root.append(&form_box);
        root.append(&footer);
        root.set_focus_child(Option::from(&login_res_label));

        let model = PasswordAuthentication {
            settings: init,
            username: "".to_string(),
            password: "".to_string(),
            captcha: "".to_string(),
            error_message: None,
            auth_message: None,
            login_status: None,
            login_step: LoginSteps::UsernameInput,
            captcha_input_sender: captcha_input.sender().clone(),
        };

        let widgets = PasswordAuthenticationWidgets {
            username_input,
            captcha_input,
            password_input,
            login_res_label,
            back_button,
            submit_button,
            auth_message_label,
            error_message_label,
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        info!("Update message is {:?}", message);
        match message {
            Message::UsernameChange(value) => {
                self.username = value;
            }
            Message::CaptchaChange(value) => {
                self.captcha = value;
            }
            Message::PasswordChange(value) => {
                self.password = value;
            }
            Message::Submit => {
                let (tx, rx) = oneshot::channel();

                match &self.login_step {
                    LoginSteps::UsernameInput => {
                        let _ = self
                            .settings
                            .login_handler_sender
                            .as_ref()
                            .unwrap()
                            .send(LoginHandlerMessage::Login {
                                username: self.username.clone(),
                                reply_to: tx,
                            })
                            .await;
                    }
                    LoginSteps::CaptchaInput { .. } => {
                        let _ = self
                            .settings
                            .login_handler_sender
                            .as_ref()
                            .unwrap()
                            .send(LoginHandlerMessage::CaptchaInput {
                                captcha: self.captcha.clone(),
                                reply_to: tx,
                            })
                            .await;
                    }
                    LoginSteps::PasswordInput { .. } => {
                        let _ = self
                            .settings
                            .login_handler_sender
                            .as_ref()
                            .unwrap()
                            .send(LoginHandlerMessage::PasswordInput {
                                password: self.password.clone(),
                                reply_to: tx,
                            })
                            .await;
                    }
                }

                let res = rx.await.expect("no reply from service");

                // self.login_status = match res {
                //     Ok(r) => {
                //         info!("login success {:?}", r);
                //         Some(LoginResult::Success)
                //     }
                //     Err(e) => {
                //         error!("login failed {}", e);
                //         Some(LoginResult::Failure)
                //     }
                // };
            }
            Message::BackPressed => {
                let (tx, rx) = oneshot::channel();
                let _ = self
                    .settings
                    .login_handler_sender
                    .as_ref()
                    .unwrap()
                    .send(LoginHandlerMessage::CancelSession { reply_to: tx })
                    .await;
                let res = rx.await.expect("no reply from service");
                sender.output_sender().send(Message::BackPressed);
            }
            Message::Command(command) => {
                info!("command is {:?}", command);
                match command {
                    CommandMsg::ShowErr(message) => self.error_message = Some(message),
                    CommandMsg::ClearErr => self.error_message = None,
                    CommandMsg::HandleGreetdResponse(_) => todo!(),
                    CommandMsg::Prompts(prompt) => {
                        match prompt {
                            Prompt::Captcha { message } => {
                                self.login_step = LoginSteps::CaptchaInput {
                                    message: message.clone(),
                                };
                                let _ = self
                                    .captcha_input_sender
                                    .send(IconInputInputMessage::PlaceHolderChange(message));
                            }
                            Prompt::Password { message } => {
                                self.login_step = LoginSteps::PasswordInput { message }
                            }
                        };
                    }
                    CommandMsg::AuthError => {
                        self.login_step = LoginSteps::UsernameInput;
                        self.captcha = "".to_string();
                        self.password = "".to_string();
                    }
                }
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        match &self.login_status {
            Some(login_status) => match login_status {
                LoginResult::Success => {
                    widgets.login_res_label.set_label("Login successfull");
                }
                LoginResult::Failure => {
                    widgets.login_res_label.set_label("Login failed");
                }
            },
            None => (),
        };

        match &self.login_step {
            LoginSteps::UsernameInput => {
                let _ = widgets
                    .username_input
                    .sender()
                    .send(custom_widgets::icon_input::InputMessage::SetEditable(true));
                widgets.auth_message_label.set_label("");
                widgets.auth_message_label.set_visible(false);

                let _ = widgets
                    .captcha_input
                    .sender()
                    .send(custom_widgets::icon_input::InputMessage::Clear);
                widgets.captcha_input.widget().set_visible(false);

                let _ = widgets
                    .password_input
                    .sender()
                    .send(custom_widgets::icon_input_password::InputMessage::Clear);
                widgets.password_input.widget().set_visible(false);
            }
            LoginSteps::CaptchaInput { message } => {
                widgets.auth_message_label.set_label(&message);
                widgets.auth_message_label.set_visible(true);
                widgets.captcha_input.widget().set_visible(true);
                widgets.password_input.widget().set_visible(false);
                let _ = widgets
                    .username_input
                    .sender()
                    .send(custom_widgets::icon_input::InputMessage::SetEditable(false));
            }
            LoginSteps::PasswordInput { message } => {
                widgets.auth_message_label.set_label(&message);
                widgets.auth_message_label.set_visible(true);
                widgets.captcha_input.widget().set_visible(false);
                widgets.password_input.widget().set_visible(true);
                let _ = widgets
                    .username_input
                    .sender()
                    .send(custom_widgets::icon_input::InputMessage::SetEditable(false));
            }
        }

        match &self.error_message {
            Some(error) => {
                widgets.error_message_label.set_visible(true);
                widgets.error_message_label.set_label(&error);
            }
            None => {
                widgets.error_message_label.set_visible(false);
            }
        }

        // widgets
        //     .auth_message_label
        //     .set_visible(self.login_step == LoginSteps::CaptchaInput);

        // widgets
        //     .password_input
        //     .widget()
        //     .set_visible(self.login_step == LoginSteps::PasswordInput);

        // widgets.password_input.widget().set_vi
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
    }
}
