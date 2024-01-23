use custom_utils::get_image_from_path;
use gtk::prelude::*;
use relm4::{
    factory::{widgets, FactoryVecDeque},
    gtk::{self, gdk::BUTTON_PRIMARY, glib, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::{
    users::{read_users_yml, User, UsersSettings},
    widgets::user_card::{Message as UserCardMessage, UserCard},
    Screens,
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
};
use tracing::{error, info};

pub struct Settings {
    pub close_icon: Option<String>,
    pub shutdown_icon: Option<String>,
    pub restart_icon: Option<String>,
    pub sleep_icon: Option<String>,
}

pub struct PowerOptions {
    reveal: bool,
}

pub struct PowerOptionsWidgets {
    close_btn: Controller<IconButton>,
    shutdown_btn: gtk::Box,
    restart_btn: gtk::Box,
    sleep_btn: gtk::Box,
    revealer: gtk::Revealer,
}

#[derive(Debug)]
pub enum Message {
    ChangeScreen(Screens),
    Reset,
}

impl SimpleComponent for PowerOptions {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = PowerOptionsWidgets;

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let power_options_list = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Center)
            .css_classes(["power-options-list"])
            .build();

        let shutdown_btn = create_power_option_btn(init.shutdown_icon.clone(), "Shut down");
        let restart_btn = create_power_option_btn(init.restart_icon.clone(), "Restart");
        let sleep_btn = create_power_option_btn(init.sleep_icon.clone(), "Sleep");

        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["footer"])
            .valign(gtk::Align::End)
            // .vexpand(true)
            .build();

        let close_btn = IconButton::builder()
            .launch(IconButtonStetings {
                icon: init.close_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    IconButtonOutputMessage::Clicked => Message::ChangeScreen(Screens::HomeScreen),
                }
            });
        footer.append(close_btn.widget());

        power_options_list.append(&shutdown_btn);
        power_options_list.append(&restart_btn);
        power_options_list.append(&sleep_btn);

        let revealer = gtk::Revealer::builder()
            // .margin_bottom(10)
            .css_classes(["power-option-revealer"])
            .build();
        revealer.set_child(Some(&power_options_list));
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
        revealer.set_transition_duration(600);
        revealer.set_reveal_child(false);

        root.append(&revealer);
        root.append(&footer);

        let model = PowerOptions { reveal: false };
        let widgets = PowerOptionsWidgets {
            close_btn,
            shutdown_btn,
            restart_btn,
            sleep_btn,
            revealer,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Message::ChangeScreen(screen) => {
                sender.output_sender().send(Message::ChangeScreen(screen));
            }
            Message::Reset => {
                // self.password_keys
                //     .broadcast(PasswordKeyInputMessage::UnReveal(false));
                self.reveal = true;
            }
        }
    }

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["power-options-container"])
            .build()
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.revealer.set_reveal_child(self.reveal);
        // widgets.restart_btn.set_reveal_child(self.reveal);
        // widgets.sleep_btn.set_reveal_child(self.reveal);
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        info!("home shutdown called")
    }
}

fn create_power_option_btn(icon_path: Option<String>, text: &str) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .css_classes(["power-option-btn"])
        .build();

    let icon = get_image_from_path(icon_path, &["power-option-btn-icon"]);

    container.append(&icon);

    let label = gtk::Label::builder()
        .label(text)
        .css_classes(["power-option-btn-label"])
        .build();

    container.append(&label);
    container
}
