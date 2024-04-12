use std::time::Duration;

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        self,
        custom_list_item::{
            CustomListItem, CustomListItemSettings, InputMessage as CustomListItemInputMessage,
            Message as CustomListItemMessage,
        },
        layout::{Layout, LayoutInit, LayoutMessage},
    },
};
use custom_utils::get_image_from_path;
use gtk::{glib::clone, prelude::*};
use mechanix_zbus_client::wireless::{WirelessInfoResponse, WirelessService};
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self, ffi::GtkSwitch, GestureClick},
    AsyncComponentSender, Component, ComponentController, ComponentParts, ComponentSender,
    Controller,
};
use tracing::{error, info};

use super::{manage_networks_page::ManageNetworksPage, network_details_page::WirelessDetails};

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
    wifi_status_loading: bool,
    show_toast: bool,
    toast_message: String,
}

//Widgets
pub struct NetworkPageWidgets {
    screen_layout: Controller<Layout>,
    connected_network_label: gtk::Label,
    wifi_switch: gtk::Switch,
    connected_network: gtk::Box,
    enable_network_text: gtk::Label,
    manage_networks: Controller<CustomListItem>,
    toggle_wifi_spinner: gtk::Spinner,
    toggle_wifi_row: gtk::Box,
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

    WifiStateToggle,
    ConnectedNetworkClicked,
    GetConnectedNewtwork,

    SelectedNetworkChanged(WirelessDetails),
}

#[derive(Debug)]
pub enum CommandOutputMessage {
    WifiStatusLoadingChanged(bool),
    WifiStatusChanged(bool),
    ConnectedNetworkChanged(WirelessInfoResponse),
    ToastStatusChanged(bool, String),
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl Component for NetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = NetworkPageWidgets;
    type CommandOutput = CommandOutputMessage;

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
            .css_classes(["custom-list-item-box-label", "disabled"])
            .build();

        let toggle_wifi_spinner = gtk::Spinner::builder()
            .css_classes(["blue", "hide"])
            .height_request(25)
            .width_request(25)
            .spinning(true)
            .build();

        let switch = gtk::Switch::new();
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");
        switch.set_visible(false);

        toggle_wifi_row.append(&enable_network_text);
        toggle_wifi_row.append(&switch);
        toggle_wifi_row.append(&toggle_wifi_spinner);

        let wifi_click_gesture = GestureClick::builder().button(0).build();
        wifi_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
                let _ = sender.input(Message::WifiStateToggle);
        }));
        switch.add_controller(wifi_click_gesture);

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
            let _ = sender.input(Message::ConnectedNetworkClicked);
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
                // println!("Network callback {:?}", msg);
                match msg {
                    LayoutMessage::BackPressed => Message::BackPressed,
                }
            });

        root.append(screen_layout.widget());

        let model = NetworkPage {
            settings: init,
            wifi_status: false,
            connected_network: WirelessInfoResponse::default(),
            wifi_status_loading: true,
            show_toast: false,
            toast_message: "".to_owned(),
        };

        let widgets = NetworkPageWidgets {
            screen_layout,
            wifi_switch: switch,
            connected_network_label: enabled_network_text,
            connected_network: enabled_network_row,
            enable_network_text,
            manage_networks,
            toggle_wifi_spinner,
            toggle_wifi_row,
        };

        // let sender: relm4::Sender<Message> = sender.input_sender().clone();
        // get_info(sender).await;

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        info!("Networks- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::EnableNetworkPressed => {
                let _ = sender.output(Message::EnableNetworkPressed);
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
            Message::WifiStateToggle => {
                self.wifi_status_loading = true;

                let wifi_status = !self.wifi_status;
                match wifi_status {
                    true => {
                        sender.oneshot_command(async {
                            // Run async background task
                            match WirelessService::enable_wifi().await {
                                Ok(status) => CommandOutputMessage::WifiStatusChanged(status),
                                Err(e) => {
                                    error!("Error enable_wifi: {}", e);
                                    CommandOutputMessage::ToastStatusChanged(
                                        true,
                                        "Some error".to_string(),
                                    )
                                }
                            }
                        });
                    }
                    false => {
                        sender.oneshot_command(async {
                            match WirelessService::disable_wifi().await {
                                Ok(status) => CommandOutputMessage::WifiStatusChanged(!status),
                                Err(e) => {
                                    error!("Error disable_wifi: {}", e);
                                    CommandOutputMessage::ToastStatusChanged(
                                        true,
                                        "Some error".to_string(),
                                    )
                                }
                            }
                        });
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

            Message::UpdateView => {
                println!("==> Message::UpdateView ");
                println!(" ");
                sender.oneshot_command(async {
                    // Run async background task
                    match WirelessService::wifi_status().await {
                        Ok(status) => CommandOutputMessage::WifiStatusChanged(status),
                        Err(e) => {
                            error!("Error getting device oem info: {}", e);
                            CommandOutputMessage::ToastStatusChanged(true, "Some error".to_string())
                        }
                    }
                });
            }
            Message::GetConnectedNewtwork => {
                sender.oneshot_command(async {
                    // Run async background task
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    let network_info = match WirelessService::info().await {
                        Ok(value) => value,
                        Err(e) => {
                            error!("Error getting device oem info: {}", e);
                            WirelessInfoResponse::default()
                        }
                    };

                    CommandOutputMessage::ConnectedNetworkChanged(network_info)
                });
            }
            Message::ConnectedNetworkClicked => {
                let details = WirelessDetails {
                    network_id: "".to_string(),
                    mac: self.connected_network.mac.to_string(),
                    frequency: self.connected_network.frequency.to_string(),
                    signal: self.connected_network.signal.to_string(),
                    flags: self.connected_network.flags.to_string(),
                    name: self.connected_network.name.to_string(),
                };

                let _ = sender.output(Message::SelectedNetworkChanged(details));
                let _ = sender.output(Message::EnableNetworkPressed);
            }

            _ => {}
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        println!("==> update_cmd {:?}", message);
        println!(" ");
        match message {
            CommandOutputMessage::WifiStatusLoadingChanged(value) => {
                self.wifi_status_loading = value;
            }
            CommandOutputMessage::WifiStatusChanged(value) => {
                self.wifi_status = value.clone();
                self.wifi_status_loading = false;
                match value {
                    true => {
                        let _ = sender.input_sender().send(Message::GetConnectedNewtwork);
                    }
                    false => {
                        self.connected_network = WirelessInfoResponse::default();
                    }
                }
            }
            CommandOutputMessage::ConnectedNetworkChanged(value) => {
                self.connected_network = value.clone();
            }
            CommandOutputMessage::ToastStatusChanged(flag, msg) => {
                self.show_toast = flag.clone();
                self.toast_message = msg.clone();
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        println!("==> Message::update_view ");
        println!(" ");
        println!("===>> Network update view");
        println!(
            "=====>> wifi_status_loading :: {:?}",
            self.wifi_status_loading
        );
        println!("=====>> wifi_status :: {:?}", self.wifi_status);
        println!("=====>> connected_network :: {:?}", self.connected_network);
        println!(" ");

        widgets
            .manage_networks
            .emit(CustomListItemInputMessage::StatusChanged(
                self.wifi_status_loading.clone(),
            ));

        match self.wifi_status_loading {
            true => {
                widgets.enable_network_text.add_css_class("disabled");
                widgets.toggle_wifi_spinner.show();
                widgets.wifi_switch.set_visible(false);
            }
            false => {
                widgets.enable_network_text.remove_css_class("disabled");
                widgets.toggle_wifi_spinner.hide();
                widgets.wifi_switch.set_visible(true);
            }
        };

        widgets.wifi_switch.set_active(self.wifi_status.clone());
        widgets
            .connected_network_label
            .set_label(self.connected_network.name.as_str());

        match self.wifi_status {
            true => match self.connected_network.name.is_empty() {
                false => {
                    widgets.connected_network.set_visible(true);
                }
                true => {
                    widgets.connected_network.set_visible(false);
                }
            },
            false => {
                widgets.connected_network.set_visible(false);
            }
        };

        //
    }
}

fn get_list_item(
    title: String,
    key: String,
    end_icon: Option<String>,
    sender: &ComponentSender<NetworkPage>,
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

// async fn get_info(sender: relm4::Sender<Message>) {
//     println!("==> get_info ");
//     println!(" ");
//     match WirelessService::wifi_status().await {
//         Ok(status) => {
//             let _ = sender.send(Message::WifiStatusChanged(status));
//             match status {
//                 true => {get_connected_network(sender).await;},
//                 false => {},
//             }

//         }
//         Err(e) => {
//             error!("Error getting device oem info: {}", e);
//         }
//     };

// }

// async fn get_connected_network(sender: relm4::Sender<Message>){
//     match WirelessService::info().await {
//         Ok(value) => {
//             let _ = sender.send(Message::ConnectedNetworkChanged(value));
//         }
//         Err(e) => {
//             error!("Error getting device oem info: {}", e);
//         }
//     };
// }
