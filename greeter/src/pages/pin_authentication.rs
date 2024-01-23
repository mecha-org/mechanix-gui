use custom_utils::get_image_from_path;
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonSettings,
    OutputMessage as IconButtonOutputMessage,
};
use gtk::{glib::clone, prelude::BoxExt};
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    factory::FactoryVecDeque,
    gtk::{self, prelude::WidgetExt, GestureClick},
    AsyncComponentSender, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, SimpleComponent,
};
use tokio::sync::{mpsc, oneshot};

use crate::{
    handlers::login::handler::LoginHandlerMessage,
    settings::{LayoutSettings, Modules},
    widgets::{
        password_key::{
            InputMessage as PasswordKeyInputMessage, Message as PasswordKeyMessage, PasswordKey,
            PasswordKeySettings,
        },
        password_text::{Message as PasswordTextMessage, PasswordText, PasswordTextSettings},
    },
    CommandMsg, Screens,
};
use tracing::{error, info};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub login_handler_sender: Option<mpsc::Sender<LoginHandlerMessage>>,
    pub username: String,
}

//Model
pub struct PinAuthentication {
    settings: Settings,
    password: String,
    is_authentication_failed: bool,
    password_texts: FactoryVecDeque<PasswordText>,
    password_keys: FactoryVecDeque<PasswordKey>,
}

//Widgets
pub struct PinAuthenticationWidgets {
    password_invalid_label: gtk::Label,
    back_button: Controller<IconButton>,
    backspace_btn: gtk::Image,
}

//Messages
#[derive(Debug)]
pub enum Message {
    PasswordKeyPressed(String),
    BackSpacePressed,
    BackPressed,
    Reset,
    Command(CommandMsg),
}

#[async_trait(?Send)]
impl AsyncComponent for PinAuthentication {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PinAuthenticationWidgets;
    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["pin-auth-container"])
            .build()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();

        let mut password_keys: FactoryVecDeque<PasswordKey> = FactoryVecDeque::builder()
            .launch(
                gtk::FlowBox::builder()
                    .valign(gtk::Align::Center)
                    .halign(gtk::Align::Center)
                    .max_children_per_line(4)
                    .min_children_per_line(4)
                    .selection_mode(gtk::SelectionMode::None)
                    .row_spacing(14)
                    .column_spacing(14)
                    .css_classes(["password-keys"])
                    .build(),
            )
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    PasswordKeyMessage::WidgetClicked(key) => {
                        return Message::PasswordKeyPressed(key);

                    }
                }),
            );

        layout.grid.into_iter().for_each(|key| {
            let mut icon: Option<String> = None;
            // info!("key: {} icon: {:?}", key, icon);

            password_keys
                .guard()
                .push_back(PasswordKeySettings { key, icon });
        });

        let input_password_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["input-password-box"])
            .build();

        let mut password_texts = FactoryVecDeque::builder()
            .launch(
                gtk::Box::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .css_classes(["password-text-box"])
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .hexpand(true)
                    .spacing(16)
                    .build(),
            )
            .detach();

        let backspace_btn = get_image_from_path(
            modules.back_space.icon.default.clone(),
            &["password-text-backspace"],
        );
        backspace_btn.set_visible(false);
        backspace_btn.set_vexpand(true);
        backspace_btn.set_valign(gtk::Align::Center);
        let left_click_gesture = GestureClick::builder().build();
        // left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        // info!("gesture button pressed is {}", this.current_button());
        //     sender.input_sender().send(InputMessage::Pressed);

        // }));

        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                sender.input_sender().send(Message::BackSpacePressed);

        }));
        root.add_controller(left_click_gesture);

        for i in 0..modules.password_configs.password_length {
            password_texts
                .guard()
                .insert(i, PasswordTextSettings { is_filled: false });
        }

        let password_invalid_label = gtk::Label::builder()
            .css_classes(["password-invalid-label"])
            .build();

        let back_button = IconButton::builder()
            .launch(IconButtonSettings {
                icon: modules.back.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();

        footer.append(back_button.widget());
        input_password_box.append(password_texts.widget());
        input_password_box.append(&backspace_btn);
        root.append(&input_password_box);
        root.append(&password_invalid_label);
        root.append(password_keys.widget());
        root.append(&footer);

        let model = PinAuthentication {
            settings: init,
            password: "".to_string(),
            is_authentication_failed: false,
            password_texts,
            password_keys,
        };

        let widgets = PinAuthenticationWidgets {
            password_invalid_label,
            back_button,
            backspace_btn,
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
            Message::PasswordKeyPressed(password_key) => {
                info!(task = "key presed", "key Pressed is {:?}", password_key);
                self.is_authentication_failed = false;
                self.password = [self.password.to_owned(), password_key].join("");
                self.password_texts
                    .send(self.password.len() - 1, PasswordTextMessage::ToggleFilled);

                let (tx, rx) = oneshot::channel();
                let _ = self
                    .settings
                    .login_handler_sender
                    .as_ref()
                    .unwrap()
                    .send(LoginHandlerMessage::Login {
                        username: self.settings.username.clone(),
                        reply_to: tx,
                    })
                    .await;
                let res = rx.await.expect("no reply from service");
                let is_password_wrong = false;
                // let is_password_wrong = match res {
                //     Ok(r) => {
                //         info!("login success {:?}", r);
                //         false
                //     }
                //     Err(e) => {
                //         error!("login failed {}", e);
                //         true
                //     }
                // };

                // let is_password_wrong = *"1234" != self.password;
                let is_password_length_reached =
                    self.password.len() == self.settings.modules.password_configs.password_length;
                if is_password_length_reached {
                    info!(task = "auth user", "password entered is {}", self.password);
                }

                if is_password_length_reached && is_password_wrong {
                    self.password = String::from("");
                    self.is_authentication_failed = true;
                    for num in 0..self.settings.modules.password_configs.password_length {
                        self.password_texts
                            .send(num, PasswordTextMessage::ToggleFilled);
                    }
                }
            }
            Message::BackSpacePressed => {
                if self.password.len() <= 0 {
                    return;
                }
                self.password.pop();
                self.password_texts
                    .send(self.password.len(), PasswordTextMessage::ToggleFilled);
            }
            Message::BackPressed => {
                let _ = sender.output_sender().send(Message::BackPressed);
            }
            Message::Reset => {
                // self.password_keys
                //     .broadcast(PasswordKeyInputMessage::UnReveal(false));
                self.password_keys
                    .broadcast(PasswordKeyInputMessage::Reveal);
            }
            Message::Command(command) => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        if self.is_authentication_failed.to_owned() {
            widgets
                .password_invalid_label
                .set_label("Invalid pin, Please try again!")
        } else {
            widgets.password_invalid_label.set_label("")
        };

        widgets.backspace_btn.set_visible(self.password.len() > 0);
    }
}
