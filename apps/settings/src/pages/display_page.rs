use gtk::prelude::*;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self},
    AsyncComponentSender, Component, ComponentController, Controller,
};
use custom_utils::get_image_from_path;

use crate::{
    modules::display::service::Display, settings::{LayoutSettings, Modules, WidgetConfigs}, widgets::{custom_list_item::{
            CustomListItem, CustomListItemSettings, Message as CustomListItemMessage,
        }, layout::{Layout, LayoutInit, LayoutMessage}}
};

use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings, OutputMessage as IconButtonOutputMessage,
};

use tracing::{info, error};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct DisplayPage {
    settings: Settings,
    brightness_percentage: u8
}

//Widgets
pub struct DisplayPageWidgets {
    screen_layout: Controller<Layout>,
    brightness_scale: gtk::Scale
}

//Messages
#[derive(Debug)]
pub enum Message {
    MenuItemPressed(String),
    BackPressed,
    ScreenTimeoutOpted,
    DisplayBrightnessChanged(u8),
    DisplayBrightnessUpdated(u8),
    UpdateView
}

pub struct SettingItem {
    text: String,
    start_icon: Option<String>,
    end_icon: Option<String>,
}

#[async_trait(?Send)]
impl AsyncComponent for DisplayPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = DisplayPageWidgets;
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


        let brigntness_label = gtk::Label::builder()
            .label("Brigtness CHECK")
            .halign(gtk::Align::Start)
            .build();

        let brightness_scale = gtk::Scale::builder()
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

        let input_sender  = sender.input_sender().clone();
             // Connect to the value_changed signal of the scale widget
        brightness_scale.connect_value_changed(move |scale| {
            let value = scale.value();
            // Send a message with the new value to update function
            input_sender.emit(Message::DisplayBrightnessUpdated(value as u8));
        });


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
        brigtness_items.append(&brightness_scale);
        brigtness_items.append(screen_off_timeout_widget);
        // brigtness_items.append(&screen_off_timeout_widget.clone());


        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&brigntness_label);
        scrollable_content.append(&brigtness_items);


        let screen_layout = Layout::builder().launch(LayoutInit {
            title: "Display".to_owned(),
            content: scrollable_content,
            footer_config: widget_configs.footer,
        }).forward(sender.input_sender(), |msg| {
            println!("Batery callback {:?}", msg);
            match msg {
        
            LayoutMessage::BackPressed => Message::BackPressed,
        }});

        root.append(screen_layout.widget());


        let model = DisplayPage { settings: init, brightness_percentage: 0 };

        let widgets = DisplayPageWidgets {
            screen_layout,
            brightness_scale
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>,  _root: &Self::Root,) {
        info!("dispay msg - Update message is {:?}", message);
        match message {
            Message::MenuItemPressed(key) => {}
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::ScreenTimeoutOpted => {
                let _ = sender.output(Message::ScreenTimeoutOpted);
            }
            Message::DisplayBrightnessChanged(value) => {
                __self.brightness_percentage = value.clone();
            }
            Message::DisplayBrightnessUpdated(value) => {
                set_birghtness_percentage(value.clone());
            }
            Message::UpdateView => {
                let sender: relm4::Sender<Message> = sender.input_sender().clone();
                get_info(sender).await;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {

        widgets
        .brightness_scale
        .set_value(self.brightness_percentage as f64);
    }
}


async fn get_info(sender: relm4::Sender<Message>) {
    println!("in display page");
    match Display::get_brightness_percentage().await {
        Ok(status) => {
            let _ = sender.send(Message::DisplayBrightnessChanged(status));
        }
        Err(e) => {
            error!("Error get_brightness_percentage: {}", e);
        }
    };
}


async fn set_birghtness_percentage(value: u8) { 
    
}
