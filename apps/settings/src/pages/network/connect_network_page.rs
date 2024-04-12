use gtk::prelude::*;
use mechanix_zbus_client::wireless::WirelessService;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk, AsyncComponentSender, Component, ComponentController, Controller,
};

use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::{
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings,
        OutputMessage as IconButtonOutputMessage,
    },
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings,
        OutputMessage as IconInputPasswordOutputMessage,
    },
};

use tracing::{error, info};

use super::network_details_page::WirelessDetails;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ConnectNetworkPage {
    settings: Settings,
    selected_network: WirelessDetails,
    password: String,
}

//Widgets
pub struct ConnectNetworkPageWidgets {
    password_input: Controller<IconInputPassword>,
    back_button: Controller<IconButton>,
    network_name: gtk::Label,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
    ConnectToNetworkChanged(WirelessDetails),
    SubmitChanged,
    NetworkConnected,
}

pub struct SettingItem {
    name: String,
}

#[async_trait(?Send)]
impl AsyncComponent for ConnectNetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ConnectNetworkPageWidgets;
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

        let enter_password_label = gtk::Label::builder()
            .label("Enter password for 'Mecha 5g'")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&enter_password_label);

        let password_input = IconInputPassword::builder()
            .launch(IconInputPasswordSettings {
                icon: modules.peek_password.icon.default.to_owned(),
                toggle_icon: None,
                placeholder: Option::from("Password".to_string()),
                css: IconInputPasswordCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputPasswordOutputMessage::InputChange(text) => Message::PasswordChange(text),
            });

        root.append(&header);
        root.append(password_input.widget());

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

        let submit_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: widget_configs.footer.next_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::SubmitChanged,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);

        footer.append(submit_button_widget);

        root.append(&footer);

        let model = ConnectNetworkPage {
            settings: init,
            selected_network: WirelessDetails::default(),
            password: "".to_string(),
        };

        let widgets = ConnectNetworkPageWidgets {
            password_input,
            back_button,
            network_name: enter_password_label,
            submit_button,
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
            Message::MenuItemPressed(_key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::HomeIconPressed => {
                let _ = sender.output(Message::HomeIconPressed);
            }
            Message::PasswordChange(text) => {
                self.password = text.clone();
            }
            Message::ConnectToNetworkChanged(value) => {
                // if value.name.eq(&self.selected_network.name){

                // } else {
                self.selected_network = value.clone();
                self.password = "".to_string();
                // }
            }
            Message::SubmitChanged => {
                println!(
                    "connect to network {:?} {:?}",
                    self.selected_network.name.as_str(),
                    self.password.as_str()
                );
                match WirelessService::connect_to_network(
                    self.selected_network.name.as_str(),
                    self.password.as_str(),
                )
                .await
                {
                    Ok(()) => {
                        let _ = sender.output(Message::BackPressed);
                    }
                    Err(e) => {
                        error!("Error getting known_networks info: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        widgets
            .network_name
            .set_label(self.selected_network.name.as_str());
    }
}
