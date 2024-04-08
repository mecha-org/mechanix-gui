
use gtk::{prelude::*};
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use custom_widgets::{
    icon_input::{
        IconInput, IconInputCss, InitSettings as IconInputSettings, OutputMessage as IconInputOutputMessage,
    },
    icon_input_password::{
        IconInputPassword, IconInputPasswordCss, InitSettings as IconInputPasswordSettings, OutputMessage as IconInputPasswordOutputMessage,
    },
    icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
    }
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct AddNetworkPage {
    settings: Settings,
}

//Widgets
pub struct AddNetworkPageWidgets {
    network_name_input: Controller<IconInput>,
    password_input: Controller<IconInputPassword>,
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    HomeIconPressed,
    PasswordChange(String),
    NetworkNameChange(String),
    SubmitPressed
}

impl SimpleComponent for AddNetworkPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = AddNetworkPageWidgets;

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

        let new_network_label = gtk::Label::builder()
            .label("New Network")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&new_network_label);

        let network_name_label = gtk::Label::builder()
            .label("Network Name")
            .halign(gtk::Align::Start)
            .css_classes(["add-network-name-label"])
            .build();

        let network_name_input = IconInput::builder()
            .launch(IconInputSettings {
                clear_icon: None,
                icon: None,
                placeholder: Option::from("Network Name".to_string()),
                css: IconInputCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconInputOutputMessage::InputChange(text) => Message::NetworkNameChange(text),
            });

        let password_label = gtk::Label::builder()
            .label("Password")
            .halign(gtk::Align::Start)
            .css_classes(["add-network-password-label"])
            .build();

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
        root.append(&network_name_label);
        root.append(network_name_input.widget());
        root.append(&password_label);
        root.append(password_input.widget());

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
                IconButtonOutputMessage::Clicked => Message::SubmitPressed,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);

        footer.append(submit_button_widget);

        root.append(&footer);


        let model = AddNetworkPage { settings: init };

        let widgets = AddNetworkPageWidgets {
            password_input,
            network_name_input,
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
            },
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            },
            Message::PasswordChange(text) => {},
            Message::NetworkNameChange(text) => {},
            Message::SubmitPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
