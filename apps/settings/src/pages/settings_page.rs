use gtk::{glib::clone, prelude::*};
use custom_utils::get_image_from_path;
use relm4::{
    gtk::{self},
    Component, ComponentParts, ComponentSender, SimpleComponent, ComponentController, Controller, factory::FactoryVecDeque,
};
use custom_widgets::icon_button::{
        IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
    };

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::menu_item::{MenuItem, MenuItemSettings, Message as MenuItemMessage},
    Screens,
};

use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct SettingsPage {
    settings: Settings,
    settings_menu: FactoryVecDeque<MenuItem>,
}

//Widgets
pub struct SettingsPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum OutputMessage {
    ChangeScreen(Screens),
}

#[derive(Debug)]
pub enum InputMessage {
    MenuItemPressed(String),
    BackPressed,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for SettingsPage {
    type Init = Settings;
    type Input = InputMessage;
    type Output = OutputMessage;
    type Root = gtk::Box;
    type Widgets = SettingsPageWidgets;

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
            .label(modules.settings.title.clone())
            .css_classes(["header-title"])
            .build(); 
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build(); 
        header.append(&header_title);

        let mut settings_menu_items: FactoryVecDeque<MenuItem> = FactoryVecDeque::builder()
            .launch(
                gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(12)
                    .valign(gtk::Align::Start)
                    .build(),
            )
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    MenuItemMessage::WidgetClicked(key) => {
                        if key == modules.back_space.title {
                            return InputMessage::BackPressed
                        } else {
                            return InputMessage::MenuItemPressed(key);
                        }
                    }
                }),
            );

        modules.settings.items.into_iter().for_each(|item| {
            // info!("key: {} icon: {:?}", key, icon);

            settings_menu_items.guard().push_back(MenuItemSettings {
                start_icon: item.icon,
                text: item.title,
                end_icon: widget_configs.menu_item.end_icon.clone(),
            });
        });
        root.append(&header);

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
                IconButtonOutputMessage::Clicked => todo!(),
                // IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        footer.append(back_button.widget());

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .child(settings_menu_items.widget())
            .build();
        root.append(&scrolled_window);
        root.append(&footer);

        let model = SettingsPage {
            settings: init,
            settings_menu: settings_menu_items,
        };

        let widgets = SettingsPageWidgets {
            back_button,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Settings Update message is {:?}", message);

        match message {
            InputMessage::MenuItemPressed(menu_item) => {
                info!(task = "key presed", "key Pressed is {:?}", menu_item);

                if menu_item.to_lowercase() == "network" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::Network));
                } else if menu_item.to_lowercase() == "bluetooth" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::ManageBluetooth));
                } else if menu_item.to_lowercase() == "display" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::Display));
                } else if menu_item.to_lowercase() == "sound" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::Sound));
                } else if menu_item.to_lowercase() == "battery" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::Battery));
                } else if menu_item.to_lowercase() == "security" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::Security));
                } else if menu_item.to_lowercase() == "date, time" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::DateTime));
                } else if menu_item.to_lowercase() == "about" {
                    let _ = sender
                        .output_sender()
                        .send(OutputMessage::ChangeScreen(Screens::About));
                }

            }
            InputMessage::BackPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
