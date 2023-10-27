use gtk::{glib::clone, prelude::BoxExt};
use relm4::{
    factory::FactoryVecDeque, gtk, Component, ComponentController, ComponentParts, ComponentSender,
    SimpleComponent,
};

use crate::{
    settings::{LayoutSettings, Modules},
    widgets::{
        password_key::{Message as PasswordKeyMessage, PasswordKey, PasswordKeySettings},
        password_text::{Message as PasswordTextMessage, PasswordText, PasswordTextSettings},
    },
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
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
}

//Messages
#[derive(Debug)]
pub enum Message {
    PasswordKeyPressed(String),
    BackSpacePressed,
    HomeIconPressed,
}

impl SimpleComponent for PinAuthentication {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PinAuthenticationWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["pin-auth-container"])
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();

        let mut password_keys: FactoryVecDeque<PasswordKey> = FactoryVecDeque::builder()
            .launch(
                gtk::FlowBox::builder()
                    .valign(gtk::Align::Start)
                    .max_children_per_line(30)
                    .min_children_per_line(4)
                    .selection_mode(gtk::SelectionMode::None)
                    .row_spacing(5)
                    .column_spacing(5)
                    .build(),
            )
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    PasswordKeyMessage::WidgetClicked(key) => {
                        if key == modules.home.title {
                            Message::HomeIconPressed
                        } else if key == modules.back_space.title {
                            return Message::BackSpacePressed
                        } else {
                            return Message::PasswordKeyPressed(key);
                        }
                    }
                }),
            );

        layout.grid.into_iter().for_each(|key| {
            let mut icon: Option<String> = None;

            if key == modules.home.title {
                icon = modules.home.icon.default.to_owned()
            } else if key == modules.back_space.title {
                icon = modules.back_space.icon.default.to_owned()
            }

            // info!("key: {} icon: {:?}", key, icon);

            password_keys
                .guard()
                .push_back(PasswordKeySettings { key, icon });
        });

        let mut password_texts = FactoryVecDeque::builder()
            .launch(
                gtk::Box::builder()
                    .orientation(gtk::Orientation::Horizontal)
                    .css_classes(["password-text-box"])
                    .halign(gtk::Align::Center)
                    .hexpand(true)
                    .spacing(16)
                    .build(),
            )
            .detach();

        for i in 0..modules.password_configs.password_length {
            password_texts
                .guard()
                .insert(i, PasswordTextSettings { is_filled: false });
        }

        let password_invalid_label = gtk::Label::builder()
            .css_classes(["password-invalid-label"])
            .build();

        root.append(password_texts.widget());
        root.append(&password_invalid_label);
        root.append(password_keys.widget());

        let model = PinAuthentication {
            settings: init,
            password: "".to_string(),
            is_authentication_failed: false,
            password_texts,
            password_keys,
        };

        let widgets = PinAuthenticationWidgets {
            password_invalid_label,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::PasswordKeyPressed(password_key) => {
                info!(task = "key presed", "key Pressed is {:?}", password_key);
                self.is_authentication_failed = false;
                self.password = [self.password.to_owned(), password_key].join("");
                self.password_texts
                    .send(self.password.len() - 1, PasswordTextMessage::ToggleFilled);
                let is_password_wrong = *"1234" != self.password;
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
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        if self.is_authentication_failed.to_owned() {
            widgets
                .password_invalid_label
                .set_label("Invalid pin, Please try again!")
        };
    }
}
