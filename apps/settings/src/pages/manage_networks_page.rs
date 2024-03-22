use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_network_item::{
        CustomNetworkItem, CustomNetworkItemSettings, Message as CustomNetworkItemMessage,
    },
};
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
pub struct ManageNetworksPage {
    settings: Settings,
}

//Widgets
pub struct ManageNetworksPageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    KnownNetworkPressed,
    AvailableNetworkPressed,
    AddNetworkPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for ManageNetworksPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ManageNetworksPageWidgets;

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
            .label("Manage Networks")
            .css_classes(["header-title"])
            .build();
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();
        header.append(&header_title);

        let known_networks_label = gtk::Label::builder()
            .label("Known Networks")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        let known_networks_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let known_network_1 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Actonate 5g".to_string(),
                is_connected: true,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("Actonate 5g - info click- msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
                }
            });

        let known_network_2 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Actonate 2g".to_string(),
                is_connected: false,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
                }
            });

        known_networks_list.append(known_network_1.widget());
        known_networks_list.append(known_network_2.widget());

        let available_networks_label = gtk::Label::builder()
            .label("Available Networks")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        let available_networks_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let available_network_1 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Mecha 5g".to_string(),
                is_connected: false,
                is_private: true,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::AvailableNetworkPressed,
                }
            });

        let available_network_2 = CustomNetworkItem::builder()
            .launch(CustomNetworkItemSettings {
                name: "Mecha 5g".to_string(),
                is_connected: false,
                is_private: false,
                strength: 80,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                private_icon: widget_configs.network_item.private_icon.clone(),
                strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
                info_icon: widget_configs.network_item.info_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomNetworkItemMessage::WidgetClicked => Message::AvailableNetworkPressed,
                }
            });

        available_networks_list.append(available_network_1.widget());
        available_networks_list.append(available_network_2.widget());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&known_networks_label);
        scrollable_content.append(&known_networks_list);
        scrollable_content.append(&available_networks_label);
        scrollable_content.append(&available_networks_list);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .child(&scrollable_content)
            .build();
        root.append(&scrolled_window);

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
                icon: widget_configs.footer.add_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::AddNetworkPressed,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);

        footer.append(submit_button_widget);

        root.append(&footer);

        let model = ManageNetworksPage { settings: init };

        let widgets = ManageNetworksPageWidgets {
            back_button,
            submit_button,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::KnownNetworkPressed => {
                let _ = sender.output(Message::KnownNetworkPressed);
            }
            Message::AvailableNetworkPressed => {
                let _ = sender.output(Message::AvailableNetworkPressed);
            }
            Message::AddNetworkPressed => {
                let _ = sender.output(Message::AddNetworkPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
