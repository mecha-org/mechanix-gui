use gtk::{glib::clone, prelude::*};
use custom_utils::get_image_from_path;
use relm4::{
    gtk::{self, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_list_item::{
        CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
    }
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
pub struct NetworksPage {
    settings: Settings,
}

//Widgets
pub struct NetworksPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message { 
BackPressed,
    EnableNetworkPressed,
    ManageNetworkPressed,
    IpSettingsPressed,
    EthernetPressed,
    DNSPressed,
    HomeIconPressed,
}


pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for NetworksPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = NetworksPageWidgets;

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
            .label("Network")
            .css_classes(["header-title"])
            .build();
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();
        header.append(&header_title);

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

        let enable_network_row = gtk::Box::builder()
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
        switch.set_active(true);
        let style_context = switch.style_context();
        style_context.add_class("custom-switch");

        enable_network_row.append(&enable_network_text);
        enable_network_row.append(&switch);
        network_details.append(&enable_network_row);

        // when switch is true 
        let enabled_network_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["settings-item-details-box-row"])
        .build();

        let enabled_network_text = gtk::Label::builder()
        .label("Actonate Office net1")
        .hexpand(true)
        .halign(gtk::Align::Start)
        .css_classes(["settings-item-details-box-row-key"])
        .build();


        let connected_icon_box = gtk::Box::builder().hexpand(true).halign(gtk::Align::End).build();
        let connected_icon = get_image_from_path(widget_configs.network_item.connected_icon, &[]);
        connected_icon_box.append(&connected_icon);

        let see_details_icon = get_image_from_path(widget_configs.menu_item.end_icon.clone(), &[]);

        enabled_network_row.append(&enabled_network_text);
        enabled_network_row.append(&connected_icon_box);
        enabled_network_row.append(&see_details_icon);
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


        let manage_networks = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Manage Networks".to_string(),
                value: "".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("manage_networks msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::ManageNetworkPressed,
                }
            });

        let ip_settings = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Ip Settings".to_string(),
                value: "".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::IpSettingsPressed,
                }
            });

        let manage_networks_widget = manage_networks.widget();
        let ip_settings_widget = ip_settings.widget();
        wifi_list_items.append(manage_networks_widget);
        wifi_list_items.append(ip_settings_widget);

        let others_label = gtk::Label::builder()
            .label("Others")
            .halign(gtk::Align::Start)
            .build();

        let others_list_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let ethernet = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Ethernet".to_string(),
                value: "".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::EthernetPressed,
                }
            });
        let ethernet_widget = ethernet.widget();

        let dns = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "DNS".to_string(),
                value: "".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::DNSPressed,
                }
            });
        let dns_widget = dns.widget();

        others_list_items.append(ethernet_widget);
        others_list_items.append(dns_widget);

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&wifi_label);

        scrollable_content.append(&network_details);
        scrollable_content.append(&wifi_list_items);
        scrollable_content.append(&others_label);
        scrollable_content.append(&others_list_items);

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

        let model = NetworksPage { settings: init };

        let widgets = NetworksPageWidgets {
            back_button
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Networks- Update message is {:?}", message);
        match message {
            Message::BackPressed => {
            let _ = sender.output(Message::BackPressed);
            },
            Message::EnableNetworkPressed => {
                let _ = sender.output(Message::EnableNetworkPressed);
            },
            Message::ManageNetworkPressed => {
                let _ = sender.output(Message::ManageNetworkPressed);
                            },
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
            Message::IpSettingsPressed => {
                let _ = sender.output(Message::IpSettingsPressed);
            }
            Message::EthernetPressed => {
                let _ = sender.output(Message::EthernetPressed);
            }
            Message::DNSPressed => {
                let _ = sender.output(Message::DNSPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
