use gtk::prelude::*;
use custom_utils::get_image_from_path;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_bluetooth_item::{
            CustomBluetoothItem, CustomBluetoothItemSettings, Message as CustomBluetoothItemMessage,
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
pub struct ManageBluetoothPage {
    settings: Settings,
}

//Widgets
pub struct ManageBluetoothPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    AvaiableDevicePressed,
    OtherDevicePressed,
    HomeIconPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for ManageBluetoothPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ManageBluetoothPageWidgets;

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
            .label("Bluetooth")
            .css_classes(["header-title"])
            .build(); 
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build(); 
        header.append(&header_title);
        
        let bluetooth_status_label = gtk::Label::builder()
        .label("Bluetooth")
        .css_classes(["list-label"])
        .halign(gtk::Align::Start)
        .build();

        let bluetooth_status_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["settings-item-details-box"])
        .build();

        let enable_bluetooth_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let enable_network_text = gtk::Label::builder()
            .label("Enable Bluetooth")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["custom-switch-text"])
            .build();

        let switch = gtk::Switch::new();
        switch.set_active(true);
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");

        enable_bluetooth_row.append(&enable_network_text);
        enable_bluetooth_row.append(&switch);
        bluetooth_status_box.append(&enable_bluetooth_row);

        let available_devices_label = gtk::Label::builder()
        .label("Available Devices")
        .css_classes(["list-label"])
        .halign(gtk::Align::Start)
        .build();

        let available_devices_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        // for not/connected device -> click on info/forward icon -> show BluetoothDetails (having forgot connection option)
        // for connected device -> show connected icon
        // for not connected device ( without connected_icon ) -> integrate connection flow (gtk spinner)
        let available_device_1 = CustomBluetoothItem::builder()
            .launch(CustomBluetoothItemSettings {
                name: "Macbook Pro".to_string(),
                is_connected: true,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                info_i_icon: None,
                info_arrow_icon: widget_configs.menu_item.end_icon.clone(), // forword arrow
            })
            .forward(sender.input_sender(), |msg| {
                info!("available_device_1 msg is {:?}", msg);
                match msg {
                    CustomBluetoothItemMessage::WidgetClicked => Message::AvaiableDevicePressed,
                }
            });

        let available_device_2 = CustomBluetoothItem::builder()
            .launch(CustomBluetoothItemSettings {
                name: "Macbook Air".to_string(),
                is_connected: false,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                info_i_icon: None,
                info_arrow_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomBluetoothItemMessage::WidgetClicked => Message::AvaiableDevicePressed,
                }
            });

        available_devices_list.append(available_device_1.widget());
        available_devices_list.append(available_device_2.widget());

        let other_devices_label = gtk::Label::builder()
            .label("Other Devices")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        // here, device will pair or have pairing request 
        let other_devices_list = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

        let other_device_1 = CustomBluetoothItem::builder()
            .launch(CustomBluetoothItemSettings {
                name: "Shoaib's Iphone".to_string(),
                is_connected: false,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                info_i_icon: None,
                info_arrow_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomBluetoothItemMessage::WidgetClicked => Message::OtherDevicePressed,
                }
            });

        let other_device_2 = CustomBluetoothItem::builder()
            .launch(CustomBluetoothItemSettings {
                name: "Infinity Glide".to_string(),
                is_connected: false,
                connected_icon: widget_configs.network_item.connected_icon.clone(),
                info_i_icon: None,
                info_arrow_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomBluetoothItemMessage::WidgetClicked => Message::OtherDevicePressed,
                }
            });

        other_devices_list.append(other_device_1.widget());
        other_devices_list.append(other_device_2.widget());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&bluetooth_status_label);
        scrollable_content.append(&bluetooth_status_box);
        scrollable_content.append(&available_devices_label);
        scrollable_content.append(&available_devices_list);
        scrollable_content.append(&other_devices_label);
        scrollable_content.append(&other_devices_list);

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

        let model = ManageBluetoothPage { settings: init };

        let widgets = ManageBluetoothPageWidgets {
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
            Message::AvaiableDevicePressed => {
                let _ = sender.output(Message::AvaiableDevicePressed);
            },
            Message::OtherDevicePressed => {
                let _ = sender.output(Message::OtherDevicePressed);
            },
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
