use gtk::{glib::clone, prelude::*};
use mechanix_zbus_client::wireless::{WirelessInfoResponse, WirelessService};
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self, ffi::GtkSwitch, GestureClick},
    AsyncComponentSender, Component, ComponentController, Controller,
};

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        self,
        custom_list_item::{
            CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
        },
        layout::{Layout, LayoutInit, LayoutMessage},
    },
};
use custom_utils::get_image_from_path;
use tracing::{error, info};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct NetworkPage {
    settings: Settings,
    wifi_status: bool,
    connected_network: WirelessInfoResponse,
}

//Widgets
pub struct NetworkPageWidgets {
    screen_layout: Controller<Layout>,
    connected_network_label: gtk::Label,
    wifi_switch: gtk::Switch,
    connected_network: gtk::Box,
}

//Messages
#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    UpdateView,
    EnableNetworkPressed,
    ManageNetworkPressed,
    IpSettingsPressed,
    EthernetPressed,
    DNSPressed,
    HomeIconPressed,
    ListItemPressed(String),
    WifiStatusChanged(bool),
    WifiStateToggle,
    ConnectedNetworkChanged(WirelessInfoResponse),
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

#[async_trait(?Send)]
impl AsyncComponent for NetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = NetworkPageWidgets;
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

        let wifi_label = gtk::Label::builder()
            .label("Wifi")
            .halign(gtk::Align::Start)
            .build();

        let wifi_list_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let network_details = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-item-details-box"])
            .build();

        let toggle_wifi_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let enable_network_text = gtk::Label::builder()
            .label("Enable Wi-Fi")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["custom-switch-text"])
            .build();

        let switch = gtk::Switch::new();
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");

        toggle_wifi_row.append(&enable_network_text);
        toggle_wifi_row.append(&switch);

        let wifi_click_gesture = GestureClick::builder().button(0).build();
        wifi_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        println!("wifi_click_gesture pressed is {}", this.current_button());
                let _ = sender.input(Message::WifiStateToggle);
        }));

        // wifi_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
        //     println!("wifi_click_gesture released is {}", this.current_button());
        //     let _ = sender.input(Message::WifiStateToggle);
        // }));
        toggle_wifi_row.add_controller(wifi_click_gesture);

        network_details.append(&toggle_wifi_row);

        // when switch is true
        let enabled_network_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["settings-item-details-box-row"])
            .build();

        let enabled_network_text = gtk::Label::builder()
            .label("Loading...")
            .hexpand(true)
            .halign(gtk::Align::Start)
            .css_classes(["settings-item-details-box-row-key"])
            .build();

        let connected_icon_box = gtk::Box::builder()
            .hexpand(true)
            .halign(gtk::Align::End)
            .build();
        let connected_icon = get_image_from_path(widget_configs.network_item.connected_icon, &[]);
        connected_icon_box.append(&connected_icon);

        let see_details_icon = get_image_from_path(widget_configs.menu_item.end_icon.clone(), &[]);

        enabled_network_row.append(&enabled_network_text);
        enabled_network_row.append(&connected_icon_box);
        enabled_network_row.append(&see_details_icon);
        enabled_network_row.set_visible(false);
        network_details.append(&enabled_network_row);

        let network_click_gesture = GestureClick::builder().button(0).build();
        network_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
        }));

        network_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
            info!("gesture button released is {}", this.current_button());
            let _ = sender.output(Message::EnableNetworkPressed);
        }));
        enabled_network_row.add_controller(network_click_gesture);

        let end_icon = widget_configs.menu_item.end_icon.clone();
        let manage_networks = get_list_item(
            "Manage Networks".to_string(),
            "manage-networks".to_string(),
            end_icon.clone(),
            &sender,
        );

        let ip_settings = get_list_item(
            "Ip Settings".to_string(),
            "ip-settings".to_string(),
            end_icon.clone(),
            &sender,
        );

        wifi_list_items.append(manage_networks.widget());
        wifi_list_items.append(ip_settings.widget());

        let others_label = gtk::Label::builder()
            .label("Others")
            .halign(gtk::Align::Start)
            .build();

        let others_list_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let ethernet = get_list_item(
            "Ethernet".to_string(),
            "ethernet".to_string(),
            end_icon.clone(),
            &sender,
        );

        let dns = get_list_item(
            "DNS".to_string(),
            "dns".to_string(),
            end_icon.clone(),
            &sender,
        );

        others_list_items.append(ethernet.widget());
        others_list_items.append(dns.widget());

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&wifi_label);

        scrollable_content.append(&network_details);
        scrollable_content.append(&wifi_list_items);
        scrollable_content.append(&others_label);
        scrollable_content.append(&others_list_items);

        let screen_layout = Layout::builder()
            .launch(LayoutInit {
                title: "Network".to_owned(),
                content: scrollable_content,
                footer_config: widget_configs.footer,
            })
            .forward(sender.input_sender(), |msg| {
                println!("Network callback {:?}", msg);
                match msg {
                    LayoutMessage::BackPressed => Message::BackPressed,
                }
            });

        root.append(screen_layout.widget());

        let model = NetworkPage {
            settings: init,
            wifi_status: false,
            connected_network: WirelessInfoResponse::default(),
        };

        let widgets = NetworkPageWidgets {
            screen_layout,
            wifi_switch: switch,
            connected_network_label: enabled_network_text,
            connected_network: enabled_network_row,
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
        info!("Networks- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::EnableNetworkPressed => {
                let _ = sender.output(Message::EnableNetworkPressed);
            }

            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
            // Message::IpSettingsPressed => {
            //     let _ = sender.output(Message::IpSettingsPressed);
            // }
            // Message::EthernetPressed => {
            //     let _ = sender.output(Message::EthernetPressed);
            // }
            // Message::DNSPressed => {
            //     let _ = sender.output(Message::DNSPressed);
            // }
            // Message::ManageNetworkPressed => {
            //     let _ = sender.output(Message::ManageNetworkPressed);
            // }
            Message::WifiStatusChanged(value) => {
                self.wifi_status = value.clone();
            }
            Message::WifiStateToggle => {
                self.wifi_status = !self.wifi_status;
                println!("wifi status toggle, {:?}", self.wifi_status);

                match self.wifi_status {
                    true => {
                        match WirelessService::enable_wifi().await {
                            Ok(_status) => {
                                let sender: relm4::Sender<Message> = sender.input_sender().clone();
                                get_connected_network(sender).await;
                            }
                            Err(e) => {
                                error!("Error enable_wifi: {}", e);
                            }
                        };
                    }
                    false => {
                        match WirelessService::disable_wifi().await {
                            Ok(_status) => {}
                            Err(e) => {
                                error!("Error disable_wifi: {}", e);
                            }
                        };
                    }
                }
            }
            Message::ListItemPressed(value) => match value.as_str() {
                "ethernet" => {
                    let _ = sender.output(Message::EthernetPressed);
                }
                "dns" => {
                    let _ = sender.output(Message::DNSPressed);
                }
                "ip-settings" => {
                    let _ = sender.output(Message::IpSettingsPressed);
                }
                "manage-networks" => {
                    let _ = sender.output(Message::ManageNetworkPressed);
                }
                _ => {}
            },
            Message::ConnectedNetworkChanged(value) => {
                self.connected_network = value.clone();
            }
            Message::UpdateView => {
                let sender: relm4::Sender<Message> = sender.input_sender().clone();
                get_info(sender).await;
            }

            _ => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        widgets.wifi_switch.set_active(self.wifi_status.clone());

        match self.wifi_status {
            true => {}
            false => {
                widgets.connected_network.set_visible(false);
            }
        };
        widgets
            .connected_network_label
            .set_label(self.connected_network.name.as_str());
        match self.connected_network.name.is_empty() {
            false => {
                widgets.connected_network.set_visible(true);
            }
            true => {
                widgets.connected_network.set_visible(false);
            }
        }

        //
    }
}

fn get_list_item(
    title: String,
    key: String,
    end_icon: Option<String>,
    sender: &AsyncComponentSender<NetworkPage>,
) -> Controller<CustomListItem> {
    let manage_networks = CustomListItem::builder()
        .launch(CustomListItemSettings {
            start_icon: None,
            text: title,
            value: "".to_owned(),
            end_icon: end_icon.clone(),
        })
        .forward(sender.input_sender(), move |msg| {
            info!("manage_networks msg is {:?}", msg);
            match msg {
                CustomListItemMessage::WidgetClicked => Message::ListItemPressed(key.clone()),
            }
        });

    manage_networks
}

async fn get_info(sender: relm4::Sender<Message>) {
    match WirelessService::wifi_status().await {
        Ok(status) => {
            let _ = sender.send(Message::WifiStatusChanged(status));
            match status {
                true => {
                    get_connected_network(sender).await;
                }
                false => {}
            }
        }
        Err(e) => {
            error!("Error getting device oem info: {}", e);
        }
    };
}

async fn get_connected_network(sender: relm4::Sender<Message>) {
    match WirelessService::info().await {
        Ok(value) => {
            let _ = sender.send(Message::ConnectedNetworkChanged(value));
        }
        Err(e) => {
            error!("Error getting device oem info: {}", e);
        }
    };
}
