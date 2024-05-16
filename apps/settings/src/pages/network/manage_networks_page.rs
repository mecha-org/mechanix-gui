use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_network_item::{
        CustomNetworkItem, CustomNetworkItemSettings, Message as CustomNetworkItemMessage,
    },
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    OutputMessage as IconButtonOutputMessage,
};
use gtk::prelude::*;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse, WirelessScanListResponse,
    WirelessService,
};
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{
        self,
        glib::{self},
    },
    AsyncComponentSender, Component, ComponentController, Controller, RelmRemoveAllExt,
};
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use super::network_details_page::WirelessDetails;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ManageNetworksPage {
    settings: Settings,
    available_networks: Vec<WirelessInfoResponse>,
    known_networks: Vec<KnownNetworkResponse>,
    selected_network: WirelessDetails,
    handle: Option<glib::JoinHandle<()>>,
}

//Widgets
pub struct ManageNetworksPageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
    known_networks_box: gtk::Box,
    available_networks_box: gtk::Box,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    KnownNetworkPressed,
    AvailableNetworkPressed,
    AddNetworkPressed,
    AvailableNetworkListChanged(WirelessScanListResponse),
    KnownNetworkListChanged(KnownNetworkListResponse),

    SelectedKnownNetworkChanged(WirelessDetails),
    SelectedNetworkChanged(WirelessDetails),
    NetworkDetailClicked(WirelessDetails),
    UpdateView,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

#[async_trait(?Send)]
impl AsyncComponent for ManageNetworksPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ManageNetworksPageWidgets;
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

        let networks_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let known_networks_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        // let known_network_1 = CustomNetworkItem::builder()
        //     .launch(CustomNetworkItemSettings {
        //         name: "Actonate 5g".to_string(),
        //         is_connected: true,
        //         is_private: true,
        //         strength: 80,
        //         connected_icon: widget_configs.network_item.connected_icon.clone(),
        //         private_icon: widget_configs.network_item.private_icon.clone(),
        //         strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
        //         info_icon: widget_configs.network_item.info_icon.clone(),
        //     })
        //     .forward(sender.input_sender(), |msg| {
        //         info!("Actonate 5g - info click- msg is {:?}", msg);
        //         match msg {
        //             CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
        //         }
        //     });

        // let known_network_2 = CustomNetworkItem::builder()
        //     .launch(CustomNetworkItemSettings {
        //         name: "Actonate 2g".to_string(),
        //         is_connected: false,
        //         is_private: true,
        //         strength: 80,
        //         connected_icon: widget_configs.network_item.connected_icon.clone(),
        //         private_icon: widget_configs.network_item.private_icon.clone(),
        //         strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
        //         info_icon: widget_configs.network_item.info_icon.clone(),
        //     })
        //     .forward(sender.input_sender(), |msg| {
        //         info!("msg is {:?}", msg);
        //         match msg {
        //             CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
        //         }
        //     });

        // known_networks_list.append(known_network_1.widget());
        // known_networks_list.append(known_network_2.widget());

        let available_networks_label = gtk::Label::builder()
            .label("Available Networks")
            .css_classes(["list-label"])
            .halign(gtk::Align::Start)
            .build();

        let available_networks_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        // let available_network_1 = CustomNetworkItem::builder()
        //     .launch(CustomNetworkItemSettings {
        //         name: "Mecha 5g".to_string(),
        //         is_connected: false,
        //         is_private: true,
        //         strength: 80,
        //         connected_icon: widget_configs.network_item.connected_icon.clone(),
        //         private_icon: widget_configs.network_item.private_icon.clone(),
        //         strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
        //         info_icon: widget_configs.network_item.info_icon.clone(),
        //     })
        //     .forward(sender.input_sender(), |msg| {
        //         info!("msg is {:?}", msg);
        //         match msg {
        //             CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
        //         }
        //     });

        // let available_network_2 = CustomNetworkItem::builder()
        //     .launch(CustomNetworkItemSettings {
        //         name: "Mecha 5g".to_string(),
        //         is_connected: false,
        //         is_private: false,
        //         strength: 80,
        //         connected_icon: widget_configs.network_item.connected_icon.clone(),
        //         private_icon: widget_configs.network_item.private_icon.clone(),
        //         strength_icon: widget_configs.network_item.wifi_100_icon.clone(),
        //         info_icon: widget_configs.network_item.info_icon.clone(),
        //     })
        //     .forward(sender.input_sender(), |msg| {
        //         info!("msg is {:?}", msg);
        //         match msg {
        //             CustomNetworkItemMessage::WidgetClicked => Message::KnownNetworkPressed,
        //         }
        //     });

        // available_networks_list.append(available_network_1.widget());
        // available_networks_list.append(available_network_2.widget());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&networks_list);
        scrollable_content.append(&known_networks_label);
        scrollable_content.append(&known_networks_box);
        scrollable_content.append(&available_networks_label);
        scrollable_content.append(&available_networks_box);

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

        let model = ManageNetworksPage {
            settings: init,
            known_networks: vec![],
            available_networks: vec![],
            selected_network: WirelessDetails::default(),
            handle: None,
        };

        let widgets = ManageNetworksPageWidgets {
            back_button,
            submit_button,
            known_networks_box,
            available_networks_box,
        };

        let sender: relm4::Sender<Message> = sender.input_sender().clone();
        get_info(sender).await;
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
                self.handle
                    .take()
                    .unwrap()
                    .into_source_id()
                    .unwrap()
                    .remove();
                println!("<<<<<<<<<<<<<<<< Scan info Endded >>>>>>>>>>>>>>>>>");
                println!(" ");
                let _ = sender.output(Message::BackPressed);
            }
            Message::KnownNetworkPressed => {
                let _ = sender.output(Message::SelectedNetworkChanged(
                    self.selected_network.clone(),
                ));
                let _ = sender.output(Message::KnownNetworkPressed);
            }
            Message::NetworkDetailClicked(value) => {
                println!("=> NetworkDetailClicked {:?}", value);
                self.handle
                    .take()
                    .unwrap()
                    .into_source_id()
                    .unwrap()
                    .remove();
                println!("<<<<<<<<<<<<<<<< Scan info Endded >>>>>>>>>>>>>>>>>");
                println!(" ");
                let _ = sender.output(Message::SelectedNetworkChanged(value.clone()));
                let _ = sender.output(Message::KnownNetworkPressed);
            }
            Message::AddNetworkPressed => {
                self.handle
                    .take()
                    .unwrap()
                    .into_source_id()
                    .unwrap()
                    .remove();
                println!("<<<<<<<<<<<<<<<< Scan info Endded >>>>>>>>>>>>>>>>>");
                println!(" ");
                let _ = sender.output(Message::AddNetworkPressed);
            }
            Message::AvailableNetworkListChanged(value) => {
                self.available_networks = value.wireless_network.clone();
            }
            Message::KnownNetworkListChanged(value) => {
                self.known_networks = value.known_network.clone();
            }
            Message::SelectedNetworkChanged(value) => {
                self.handle
                    .take()
                    .unwrap()
                    .into_source_id()
                    .unwrap()
                    .remove();
                println!("<<<<<<<<<<<<<<<< Scan info Endded >>>>>>>>>>>>>>>>>");
                println!(" ");
                self.selected_network = value.clone();

                let _ = sender.output(Message::SelectedNetworkChanged(value));
                let _ = sender.output(Message::AvailableNetworkPressed);
                // self.show_password = true;
            }
            Message::SelectedKnownNetworkChanged(network_details) => {
                if network_details.flags.contains("[CURRENT]") {
                    self.handle
                        .take()
                        .unwrap()
                        .into_source_id()
                        .unwrap()
                        .remove();
                    println!("<<<<<<<<<<<<<<<< Scan info Endded >>>>>>>>>>>>>>>>>");
                    println!(" ");
                    let _ = sender.output(Message::SelectedNetworkChanged(network_details.clone()));
                    let _ = sender.output(Message::KnownNetworkPressed);
                } else {
                    let _ = WirelessService::connect_to_known_network(
                        network_details.network_id.as_str(),
                    )
                    .await;
                }
            }
            Message::UpdateView => {
                self.handle = Some(relm4::spawn_local(async move {
                    loop {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        let sender: relm4::Sender<Message> = sender.input_sender().clone();
                        get_info(sender).await;
                    }
                }));
            }
            _ => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        widgets.known_networks_box.remove_all();
        for network in <Vec<KnownNetworkResponse> as Clone>::clone(&self.known_networks).into_iter()
        {
            let mut network_details = WirelessInfoResponse::default();
            match self
                .available_networks
                .iter()
                .find(|i| i.name == network.ssid)
            {
                Some(value) => {
                    network_details = value.clone();
                }
                None => {}
            }

            let sender_obj = sender.clone();
            let network_item =
                get_known_network_list_item(&self, network, network_details.clone(), sender_obj);
            widgets.known_networks_box.append(network_item.widget());
        }

        widgets.available_networks_box.remove_all();
        for network in
            <Vec<WirelessInfoResponse> as Clone>::clone(&self.available_networks).into_iter()
        {
            if self.known_networks.iter().any(|i| i.ssid == network.name) {
                continue;
            }

            let sender_obj = sender.clone();
            let network_item = get_network_list_item(&self, network, sender_obj);
            widgets.available_networks_box.append(network_item.widget());
        }
    }
}

async fn get_info(sender: relm4::Sender<Message>) {
    println!("<<<<<<<<<<<<<<<< Scan info >>>>>>>>>>>>>>>>>");
    match WirelessService::known_networks().await {
        Ok(value) => {
            let _ = sender.send(Message::KnownNetworkListChanged(value));
        }
        Err(e) => {
            error!("Error getting known_networks info: {}", e);
        }
    };

    match WirelessService::scan().await {
        Ok(value) => {
            let _ = sender.send(Message::AvailableNetworkListChanged(value));
        }
        Err(e) => {
            error!("Error getting scan info: {}", e);
        }
    };
}

fn get_network_list_item(
    network_page: &ManageNetworksPage,
    network: WirelessInfoResponse,
    sender: AsyncComponentSender<ManageNetworksPage>,
) -> Controller<CustomNetworkItem> {
    let details = WirelessDetails {
        network_id: "".to_string(),
        mac: network.mac.to_string(),
        frequency: network.frequency.to_string(),
        signal: network.signal.to_string(),
        flags: network.flags.to_string(),
        name: network.name.to_string(),
    };
    CustomNetworkItem::builder()
        .launch(CustomNetworkItemSettings {
            name: network.name.to_string(),
            is_connected: false,
            is_private: true,
            strength: 80,
            connected_icon: network_page
                .settings
                .widget_configs
                .network_item
                .connected_icon
                .clone(),
            private_icon: network_page
                .settings
                .widget_configs
                .network_item
                .private_icon
                .clone(),
            strength_icon: network_page
                .settings
                .widget_configs
                .network_item
                .wifi_100_icon
                .clone(),
            info_icon: network_page
                .settings
                .widget_configs
                .network_item
                .info_icon
                .clone(),
        })
        .forward(sender.input_sender(), move |msg| {
            info!("msg is {:?}", msg);
            match msg {
                CustomNetworkItemMessage::WidgetClicked => {
                    Message::SelectedNetworkChanged(details.clone())
                }
                CustomNetworkItemMessage::InfoWidgetClicked => {
                    Message::NetworkDetailClicked(details.clone())
                }
            }
        })
}

fn get_known_network_list_item(
    network_page: &ManageNetworksPage,
    network: KnownNetworkResponse,
    network_details: WirelessInfoResponse,
    sender: AsyncComponentSender<ManageNetworksPage>,
) -> Controller<CustomNetworkItem> {
    let is_connected = network.flags.contains("[CURRENT]");
    let details = WirelessDetails {
        network_id: network.network_id,
        mac: network_details.mac.to_string(),
        frequency: network_details.frequency.to_string(),
        signal: network_details.signal.to_string(),
        flags: network_details.flags.to_string(),
        name: network_details.name.to_string(),
    };

    CustomNetworkItem::builder()
        .launch(CustomNetworkItemSettings {
            name: network_details.name.to_string(),
            is_connected: is_connected,
            is_private: true,
            strength: 80,
            connected_icon: network_page
                .settings
                .widget_configs
                .network_item
                .connected_icon
                .clone(),
            private_icon: network_page
                .settings
                .widget_configs
                .network_item
                .private_icon
                .clone(),
            strength_icon: network_page
                .settings
                .widget_configs
                .network_item
                .wifi_100_icon
                .clone(),
            info_icon: network_page
                .settings
                .widget_configs
                .network_item
                .info_icon
                .clone(),
        })
        .forward(sender.input_sender(), move |msg| {
            info!("msg is {:?}", msg);
            match msg {
                CustomNetworkItemMessage::WidgetClicked => {
                    Message::SelectedKnownNetworkChanged(details.clone())
                }
                CustomNetworkItemMessage::InfoWidgetClicked => {
                    Message::NetworkDetailClicked(details.clone())
                }
            }
        })
}
