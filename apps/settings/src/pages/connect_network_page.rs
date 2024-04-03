use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};

use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::{
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings, OutputMessage as IconInputPasswordOutputMessage,
    },
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
    } 
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ConnectNetworkPage {
    settings: Settings,
}

//Widgets
pub struct ConnectNetworkPageWidgets {
    password_input: Controller<IconInputPassword>,
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for ConnectNetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ConnectNetworkPageWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["page-container"])
            .build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();
        let widget_configs = init.widget_configs.clone();

        let enter_password_label = gtk::Label::builder()
            .label("Enter password for 'Mecha 5g'")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&enter_password_label);

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

        root.append(&header);
        root.append(password_input.widget());

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .hexpand(true)
        .vexpand(true)
        .valign(gtk::Align::End)
        .build();

        let back_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: widget_configs.footer.back_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        footer.append(back_button.widget());
        root.append(&footer);

        let model = ConnectNetworkPage { settings: init };

        let widgets = ConnectNetworkPageWidgets { 
            password_input,
            back_button
         };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
            Message::PasswordChange(text) => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
