use gtk::prelude::*;
use custom_utils::get_image_from_path;
use relm4::{
    gtk::{self},
    Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent, Controller,
};
use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::custom_list_item::{
            CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
        },
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
pub struct DisplayPage {
    settings: Settings,
}

//Widgets
pub struct DisplayPageWidgets {
    back_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    ScreenTimeoutOpted,
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

impl SimpleComponent for DisplayPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = DisplayPageWidgets;

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
            .label("Display")
            .css_classes(["header-title"])
            .build(); 
        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build(); 
        header.append(&header_title);

        let brigntness_label = gtk::Label::builder()
            .label("Brigtness CHECK")
            .halign(gtk::Align::Start)
            .build();

        let brigtness_scale = gtk::Scale::builder()
            .draw_value(true)
            .adjustment(
                &gtk::Adjustment::builder()
                    .lower(0.0)
                    .upper(100.0)
                    .value(50.0)
                    .step_increment(10.0)
                    .page_increment(10.0)
                    .build(),
            )
            .orientation(gtk::Orientation::Horizontal)
            .value_pos(gtk::PositionType::Right)
            .css_classes(["custom-scale"])
            .build();


        let brigtness_items = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let screen_off_timeout = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Screen off timeout".to_string(),
                value: "30s".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                println!("DISPLAY PAGE - SCREEN clicked {:?}", msg);
                match msg { 
                    CustomListItemMessage::WidgetClicked => Message::ScreenTimeoutOpted,
                }
            });

        let screen_off_timeout_widget = screen_off_timeout.widget();
        brigtness_items.append(&brigtness_scale);
        brigtness_items.append(screen_off_timeout_widget);
        // brigtness_items.append(&screen_off_timeout_widget.clone());

        root.append(&header);

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&brigntness_label);
        scrollable_content.append(&brigtness_items);

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

        let model = DisplayPage { settings: init };

        let widgets = DisplayPageWidgets {
            back_button
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("dispay msg - Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::ScreenTimeoutOpted => {
                let _ = sender.output(Message::ScreenTimeoutOpted);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
