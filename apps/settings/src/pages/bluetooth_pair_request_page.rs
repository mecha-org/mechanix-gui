
use gtk::prelude::*;
use relm4::{
    gtk::{self},
    ComponentParts, ComponentSender, SimpleComponent, Controller, Component, ComponentController,
};
use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct BluetoothPairRequestPage {
    settings: Settings,
}

//Widgets
pub struct BluetoothPairRequestPageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
    SubmitPressed
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for BluetoothPairRequestPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BluetoothPairRequestPageWidgets;

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

        let header_title = gtk::Label::builder()
            .label("Pairing Request")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let pairing_request_label = gtk::Label::builder()
            .label("'Macbook Pro' has requested to pair with your device. \nConfirm this code on their device to connect.")
            .css_classes(["bluetooth-pair-request-list-label"])
            .halign(gtk::Align::Start)
            .build();

        let pairing_request_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["bluetooth-pair-request-box"])
            .build();

        let pairing_request_value = gtk::Label::builder()
            .label("243 562")
            .css_classes(["bluetooth-pair-request-value"])
            .halign(gtk::Align::Center)
            .build();
        pairing_request_box.append(&pairing_request_value);

        root.append(&header);
        root.append(&pairing_request_label);
        root.append(&pairing_request_box);

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

        let model = BluetoothPairRequestPage { settings: init };

        let widgets = BluetoothPairRequestPageWidgets {
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
            }
            Message::SubmitPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
