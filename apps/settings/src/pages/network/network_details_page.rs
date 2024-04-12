use crate::{
    modules::wireless::service::WirelessService,
    settings::{LayoutSettings, Modules, WidgetConfigs},
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    OutputMessage as IconButtonOutputMessage,
};
use gtk::prelude::*;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk, AsyncComponentSender, Component, ComponentController, Controller,
};
use tracing::info;

#[derive(Debug, Clone, Default)]
pub struct WirelessDetails {
    pub mac: String,
    pub frequency: String,
    pub signal: String,
    pub flags: String,
    pub name: String,
    pub network_id: String,
}

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct NetworkDetailsPage {
    settings: Settings,
    selected_network: WirelessDetails,
}

//Widgets
pub struct NetworkDetailsPageWidgets {
    back_button: Controller<IconButton>,
    remove_button: Controller<IconButton>,
    ssid_value_label: gtk::Label,
    network_id_value_label: gtk::Label,
    passphrase_value_label: gtk::Label,
    frequency_value_label: gtk::Label,
    ip_address_value_label: gtk::Label,
    subnet_mask_value_label: gtk::Label,
    gateway_value_label: gtk::Label,
    header_label: gtk::Label,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    RemovePressed,
    HomeIconPressed,
    ConnectToNetworkChanged(WirelessDetails),
}

pub struct SettingItem {
    name: String,
}
#[async_trait(?Send)]
impl AsyncComponent for NetworkDetailsPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = NetworkDetailsPageWidgets;
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

        let header_label = gtk::Label::builder()
            .label("Actonate 5g")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_label);

        let network_details_box_1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let (ssid_row, ssid_value_label) = get_row("Network SSID", "Actonate 5g");
        let (network_id_row, network_id_value_label) = get_row("Network ID", "1");
        let (passphrase_row, passphrase_value_label) = get_row("Passphrase", "WPA2");
        let (frequency_row, frequency_value_label) = get_row("Frequency", "5GHz");

        network_details_box_1.append(&ssid_row);
        network_details_box_1.append(&network_id_row);

        network_details_box_1.append(&passphrase_row);
        network_details_box_1.append(&frequency_row);

        let network_details_box_2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .css_classes(["settings-item-details-box"])
            .build();

        let (ip_address_row, ip_address_value_label) = get_row("IP Address", "192.168.203.106");
        let (subnet_mask_row, subnet_mask_value_label) = get_row("Subnet Mask", "255.255.255.0");
        let (gateway_row, gateway_value_label) = get_row("Gateway", "192.168.0.1");

        network_details_box_2.append(&ip_address_row);
        network_details_box_2.append(&subnet_mask_row);
        network_details_box_2.append(&gateway_row);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&network_details_box_1);
        scrollable_content.append(&network_details_box_2);

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

        let remove_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: widget_configs.footer.trash_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::RemovePressed,
            });
        let remove_button_widget = remove_button.widget();
        remove_button_widget.set_hexpand(true);
        remove_button_widget.set_halign(gtk::Align::End);

        footer.append(remove_button_widget);
        root.append(&footer);
        root.append(&footer);

        let model = NetworkDetailsPage {
            settings: init,
            selected_network: WirelessDetails::default(),
        };

        let widgets = NetworkDetailsPageWidgets {
            back_button,
            remove_button,
            ssid_value_label,
            network_id_value_label,
            passphrase_value_label,
            frequency_value_label,
            ip_address_value_label,
            subnet_mask_value_label,
            gateway_value_label,
            header_label,
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
            Message::RemovePressed => {
                println!("Disconnect to a wifi {:?}", self.selected_network.name);
                match WirelessService::disconnect(&self.selected_network.network_id.as_str()).await
                {
                    Ok(_) => {
                        let _ = sender.output(Message::BackPressed);
                    }
                    Err(_) => {}
                }
            }
            Message::ConnectToNetworkChanged(value) => self.selected_network = value.clone(),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        widgets.header_label.set_label(&self.selected_network.name);

        widgets
            .ssid_value_label
            .set_label(&self.selected_network.name);

        widgets
            .frequency_value_label
            .set_label(&self.selected_network.frequency);

        widgets
            .network_id_value_label
            .set_label(&self.selected_network.network_id);
        // widgets.passphrase_value_label.set_label(self.selected_network.),
        // widgets.ip_address_value_label.set_label(self.selected_network.),
        // widgets.subnet_mask_value_label.set_label(self.selected_network.),
        // widgets.gateway_value_label.set_label(self.selected_network.),
    }
}

fn get_row(key: &str, value: &str) -> (gtk::Box, gtk::Label) {
    let row_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

    let key_label = gtk::Label::builder()
        .label(key.to_string())
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();
    let value_label = gtk::Label::builder()
        .label(value.to_string())
        .css_classes(["settings-item-details-box-row-value"])
        .build();

    row_box.append(&key_label);
    row_box.append(&value_label);

    return (row_box, value_label);
}
