use crate::{
    modules::device_info::service::DeviceInfo,
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::header::Header
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    OutputMessage as IconButtonOutputMessage,
};
use relm4::{async_trait::async_trait, gtk::Label};
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk, AsyncComponentSender, Component, ComponentController, Controller,
};
use gtk::prelude::*;

use tracing::{error, info};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct AboutPage {
    settings: Settings,
    device_info: DeviceInfo,
}
//Widgets
pub struct AboutPageWidgets {
    back_button: Controller<IconButton>,
    os_name: gtk::Label,
    os_version: gtk::Label,
    ethernet_mac_address: gtk::Label,
    wifi_mac_adddress: gtk::Label,
    serial_no: gtk::Label,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    DeviceInfoChanged(DeviceInfo),
}

#[async_trait(?Send)]
impl AsyncComponent for AboutPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = AboutPageWidgets;
    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["page-container"])
            .build()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();
        let widget_configs = init.widget_configs.clone();

        let about_details_list1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let about_details_list2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let (os_row, os_label) = get_row("OS");
        let (version_row, version_label) = get_row("Version");

        let (serial_row, serial_label) = get_row("Serial Number");
        let (wifi_address_row, wifi_address_label) = get_row("Wi-Fi MAC Address");
        let (ethernet_address_row, ethernet_address_label) = get_row("Ethernet MAC Address");

        about_details_list1.append(&os_row);
        about_details_list1.append(&version_row);

        about_details_list2.append(&serial_row);
        about_details_list2.append(&wifi_address_row);
        about_details_list2.append(&ethernet_address_row);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        scrollable_content.append(&about_details_list1);
        scrollable_content.append(&about_details_list2);

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .child(&scrollable_content)
            .build();

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

        let header = Header::builder()
            .launch("About".to_owned());
        root.append(header.widget());
        root.append(&scrolled_window);
        root.append(&footer);

        let model = AboutPage {
            settings: init,
            device_info: DeviceInfo::default(),
        };

        let widgets = AboutPageWidgets {
            back_button,
            os_name: os_label,
            os_version: version_label,
            ethernet_mac_address: ethernet_address_label,
            wifi_mac_adddress: wifi_address_label,
            serial_no: serial_label,
        };
        let sender: relm4::Sender<Message> = sender.input_sender().clone();
        get_device_info(sender).await;
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        info!("About- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::DeviceInfoChanged(device_info) => {
                self.device_info = device_info.clone();
                let _ = sender.output(Message::DeviceInfoChanged(device_info));
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        info!("About- Update view");
        widgets.os_name.set_label(&self.device_info.os_name);
        widgets.os_version.set_label(&self.device_info.os_version);
        widgets
            .ethernet_mac_address
            .set_label(&self.device_info.ethernet_mac_address);
        widgets
            .wifi_mac_adddress
            .set_label(&self.device_info.wifi_mac_address);
        widgets
            .serial_no
            .set_label(&self.device_info.serial_number);
    }
}

async fn get_device_info(sender: relm4::Sender<Message>) {
    match DeviceInfo::get_device_info_service().await {
        Ok(device_info) => {
            info!("Distro info: {:?}", device_info);
            let _ = sender.send(Message::DeviceInfoChanged(device_info));
        }
        Err(e) => {
            error!("Error getting device oem info: {}", e);
        }
    };
}

fn get_label(text: String, align: gtk::Align) -> Label {
    let label = gtk::Label::builder()
        .label(text)
        .hexpand(true)
        .halign(align)
        .css_classes(["settings-item-details-box-row-key"])
        .build();

    return label;
}

fn get_row(key: &str) -> (gtk::Box, Label) {
    let about_details_row_1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

    let os_label = get_label(key.to_string(), gtk::Align::Start);

    let os_value = get_label("".to_owned(), gtk::Align::End);

    about_details_row_1.append(&os_label);
    about_details_row_1.append(&os_value);
    return (about_details_row_1, os_value);
}
