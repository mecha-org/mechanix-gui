use gtk::{gdk, gio, glib::clone, prelude::*, subclass::*};
use relm4::{
    adw,
    factory::FactoryVecDeque,
    gtk::{self, gdk::BUTTON_PRIMARY, glib, GestureClick},
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent,
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
    pub lock_icon: Option<String>,
    pub unlock_icon: Option<String>,
    pub password_icon: Option<String>,
}

pub struct HomePage {}

pub struct HomePageWidgets {
    password_screen_btn: Controller<IconButton>,
    user_cards: FactoryVecDeque<UserCard>,
}

#[derive(Debug)]
pub enum Message {
    ChangeScreen(Screens),
    UserClicked(User),
}

impl SimpleComponent for HomePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = HomePageWidgets;

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["footer"])
            .valign(gtk::Align::End)
            // .vexpand(true)
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
                        Message::ChangeScreen(Screens::PowerOptions)
                    }
                }
            });

        let carousel = adw::Carousel::builder()
            // .valign(gtk::Align::Center)
            // .halign(gtk::Align::Center)
            .vexpand(true)
            .hexpand(true)
            .spacing(14)
            .css_classes(["user-cards"])
            .build();

        carousel.connect_page_changed(|carousel, active_index| {
            info!(
                "connect_page_changed change called, active page is {}",
                active_index
            );

            for index in 0..carousel.n_pages() {
                let widget = carousel.nth_page(index);
                widget.set_class_active("user-card-active", index == active_index);
            }
        });

        let mut user_cards: FactoryVecDeque<UserCard> = FactoryVecDeque::builder()
            .launch(carousel.clone())
            .forward(sender.input_sender(), |msg| {
                info!("app widget forwarded message is {:?}", msg);
                return match msg {
                    UserCardMessage::UserClicked(username) => Message::UserClicked(username),
                };
            });

        let user_settings = match read_users_yml("users.yml") {
            Ok(v) => v,
            Err(e) => {
                error!("error while reading users.yml {}", e);
                UsersSettings::default()
            }
        };

        info!("user settings is {:?}", user_settings);

        user_settings.users.into_iter().for_each(|user| {
            user_cards.guard().push_back(UserCard { user });
        });

        if carousel.n_pages() > 0 {
            let first_child = carousel.nth_page(0);
            first_child.set_class_active("user-card-active", true);
        }

        // let scrollable = gtk::ScrolledWindow::builder()
        //     .vscrollbar_policy(gtk::PolicyType::Never) // Disable vertical scrolling
        //     .min_content_width(360)
        //     .min_content_height(360)
        //     .css_classes(["scrollable"])
        //     .child(user_cards.widget())
        //     .build();

        // let carousel = adw::Carousel::builder()
        //     .orientation(gtk::Orientation::Horizontal)
        //     .build();

        // carousel.append(&gtk::Label::builder().label("hello world 1").build());
        // carousel.append(&gtk::Label::builder().label("hello world 2").build());
        // carousel.append(&gtk::Label::builder().label("hello world 3").build());
        // carousel.append(&gtk::Label::builder().label("hello world 4").build());
        // carousel.append(&gtk::Label::builder().label("hello world 5").build());
        // carousel.append(user_cards.widget());

        root.append(user_cards.widget());
        footer.append(password_screen_btn.widget());
        root.append(&footer);

        let model = HomePage {};
        let widgets: HomePageWidgets = HomePageWidgets {
            password_screen_btn,
            user_cards,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Message::ChangeScreen(screen) => {
                sender.output_sender().send(Message::ChangeScreen(screen));
            }
            Message::UserClicked(user) => {
                info!("user clicked is {:?}", user);
                if user.username == "custom" {
                    let _ = sender
                        .output_sender()
                        .send(Message::ChangeScreen(Screens::PasswordScreen));
                } else {
                    match user.pin_enabled {
                        Some(is_pin_enabled) => {
                            if is_pin_enabled {
                                let _ = sender
                                    .output_sender()
                                    .send(Message::ChangeScreen(Screens::PinScreen));
                            }
                        }
                        None => (),
                    }
                }
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
