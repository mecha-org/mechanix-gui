use gtk::prelude::*;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::Screens;
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
};
use tracing::info;

pub struct Settings {
    pub lock_icon: Option<String>,
    pub unlock_icon: Option<String>,
    pub password_icon: Option<String>,
}

pub struct HomePage {}

pub struct HomePageWidgets {
    password_screen_btn: Controller<IconButton>,
    unlock_btn: Controller<IconButton>,
}

#[derive(Debug)]
pub enum Message {
    ChangeScreen(Screens),
}

impl SimpleComponent for HomePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = HomePageWidgets;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["footer"])
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::End)
            .build();

        let password_screen_btn = IconButton::builder()
            .launch(IconButtonStetings {
                icon: init.password_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    IconButtonOutputMessage::Clicked => {
                        Message::ChangeScreen(Screens::PasswordScreen)
                    }
                }
            });

        let unlock_btn = IconButton::builder()
            .launch(IconButtonStetings {
                icon: init.lock_icon.to_owned(),
                toggle_icon: init.unlock_icon.to_owned(),
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    IconButtonOutputMessage::Clicked => Message::ChangeScreen(Screens::PinScreen),
                }
            });
        let unlock_btn_widget = unlock_btn.widget();
        unlock_btn_widget.set_hexpand(true);
        unlock_btn_widget.set_halign(gtk::Align::End);

        footer.append(password_screen_btn.widget());
        footer.append(unlock_btn_widget);
        root.append(&footer);

        let model = HomePage {};
        let widgets = HomePageWidgets {
            password_screen_btn,
            unlock_btn,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Message::ChangeScreen(screen) => {
                sender.output_sender().send(Message::ChangeScreen(screen));
            }
        }
    }

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["home-container"])
            .build()
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        info!("home shutdown called")
    }
}
