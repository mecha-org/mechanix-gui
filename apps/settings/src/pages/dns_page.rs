use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_list_item::{
            CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
        },
};
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
pub struct DNSPage {
    settings: Settings,
}

//Widgets
pub struct DNSPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    DNSModePressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for DNSPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = DNSPageWidgets;

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
            .label("DNS")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let items_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let dns_mode = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Mode".to_string(),
                value: "Auto".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("DNS PAGE msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::DNSModePressed,
                }
            });

        let dns_mode_widget = dns_mode.widget();

        //
        let dns_server_1 = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "DNS Server 1".to_string(),
                value: "192.168.1.1".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("DNS PAGE msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::DNSModePressed,
                }
            });

        let dns_server_1_widget = dns_server_1.widget();

        items_box.append(dns_mode_widget);
        items_box.append(dns_server_1_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&items_box);

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

        let model = DNSPage { settings: init };

        let widgets = DNSPageWidgets {
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
            }
            Message::DNSModePressed => {
                let _ = sender.output(Message::DNSModePressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
