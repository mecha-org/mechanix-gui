use gtk::prelude::*;
use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use relm4::{
    gtk::{self},
    Component, ComponentParts, ComponentSender, SimpleComponent, Controller, ComponentController,
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct SetDatePage {
    settings: Settings,
}

//Widgets
pub struct SetDatePageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    HomeIconPressed,
    SubmitPressed
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for SetDatePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = SetDatePageWidgets;

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

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        let header_title = gtk::Label::builder()
            .label("Set Date")
            .css_classes(["header-title"])
            .build();

        header.append(&header_title);

        let calendar = gtk::Calendar::builder()
            .vexpand_set(true)
            .hexpand_set(true)
            .height_request(modules.pages_settings.dateandtime.window_size.1)
            .width_request(modules.pages_settings.dateandtime.window_size.0)
            .build();
        calendar.style_context().add_class("custom-calendar");

        calendar.connect_day_selected(|cal| {
            let year = cal.year();
            let month = cal.month();
            let day = cal.day();

            info!("Selected Date: {:?}-{:?}-{:?}", &year, &month, &day);
        });

        root.append(&header);
        root.append(&calendar);

        
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
                icon: modules.submit.icon.default.to_owned(),
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

        let model = SetDatePage { settings: init };

        let widgets = SetDatePageWidgets {
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
            }
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
            Message::SubmitPressed => {},
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
