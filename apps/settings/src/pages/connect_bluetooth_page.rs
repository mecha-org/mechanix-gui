use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
};
use custom_widgets::{
    icon_input::{
        IconInput, IconInputCss, InitSettings as IconInputSettings, OutputMessage as IconInputOutputMessage,
    },
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings,
        InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
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
pub struct ConnectBluetoothPage {
    settings: Settings,
}

//Widgets
pub struct ConnectBluetoothPageWidgets {
    code_input: Controller<IconInput>,
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
    SubmitPressed
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for ConnectBluetoothPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ConnectBluetoothPageWidgets;

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
            .label("Pair with 'Macbook Pro'")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&enter_password_label);

        let code_input_label = gtk::Label::builder()
            .label("Enter code shared by the device here")
            .halign(gtk::Align::Start)
            .css_classes(["text-14-label"])
            .build();

        let code_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: Option::from("".to_string()),
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        root.append(&header);
        root.append(&code_input_label);
        root.append(code_input.widget());

        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .vexpand(true)
        .hexpand(true)
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

        let submit_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: modules.submit.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::SubmitPressed,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);

        footer.append(submit_button_widget);
        
        root.append(&footer);

        let model = ConnectBluetoothPage { settings: init };

        let widgets = ConnectBluetoothPageWidgets { 
            code_input,
            back_button,
            submit_button,
         };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {},
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            },
            Message::PasswordChange(text) => {}
            Message::SubmitPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
