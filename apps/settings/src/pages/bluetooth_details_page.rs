use gtk::prelude::*;
use relm4::{
    gtk::{self},
    ComponentParts, ComponentSender, SimpleComponent, Controller, Component, ComponentController,
};

use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct BluetoothDetailsPage {
    settings: Settings,
}

//Widgets
pub struct BluetoothDetailsPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for BluetoothDetailsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BluetoothDetailsPageWidgets;

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

        let network_name = gtk::Label::builder()
            .label("Macbook Pro")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&network_name);

        let device_type_label = gtk::Label::builder()
            .label("Device Type")
            .css_classes(["bluetooth-details-list-label"])
            .halign(gtk::Align::Start)
            .build();

        let device_type_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-details-device-type-box"])
            .build();

        let device_type_value = gtk::Label::builder()
            .label("Mecha MX")
            .css_classes(["bluetooth-details-device-type-value"])
            .halign(gtk::Align::Start)
            .build();
        device_type_box.append(&device_type_value);

        let forget_network_button = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-details-forget-btn-box"])
            .build();

        let forget_network_text = gtk::Label::builder()
            .label("Forget this network")
            .css_classes(["bluetooth-details-forget-btn-text"])
            .halign(gtk::Align::Center)
            .build();
        forget_network_button.append(&forget_network_text);

        root.append(&header);
        root.append(&device_type_label);
        root.append(&device_type_box);
        root.append(&forget_network_button);

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

        let model = BluetoothDetailsPage { settings: init };

        let widgets = BluetoothDetailsPageWidgets {
            back_button
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {  
                let _ = sender.output(Message::BackPressed);}
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
