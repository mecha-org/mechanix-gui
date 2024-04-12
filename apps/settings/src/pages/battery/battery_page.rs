use gtk::prelude::*;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::{self},
    AsyncComponentSender, Component, ComponentController, Controller,
};

use mechanix_zbus_client::power::Power;

use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        custom_list_item::{
            CustomListItem, CustomListItemSettings, InputMessage as ListItemInputMessage,
            Message as CustomListItemMessage,
        },
        layout::{Layout, LayoutInit, LayoutMessage},
    },
};

use tracing::{error, info};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct BatteryPage {
    settings: Settings,
    battery_level: f32,
    screen_off_timeout: String,
    performance_mode: String,
}

//Widgets
pub struct BatteryPageWidgets {
    // back_button: Controller<IconButton>,
    battery_percentage_level: gtk::LevelBar,
    battery_performance_mode: Controller<CustomListItem>,
    screen_off_timeout: Controller<CustomListItem>,
    screen_layout: Controller<Layout>,
}

//Messages
#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    ScreenTimeoutOpted,
    PerformanceOpted,
    BatteryLevelChanged(f32),
    PerformanceModeChanged(String),
    ScreenTimeoutChanged(String),
    UpdateView,
}

#[async_trait(?Send)]
impl AsyncComponent for BatteryPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = BatteryPageWidgets;
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
        // let modules = init.modules.clone();
        // let layout = init.layout.clone();
        let widget_configs = init.widget_configs.clone();

        let battery_label = gtk::Label::builder()
            .label("Battery Percentage")
            .halign(gtk::Align::Start)
            .build();

        let battery_percentage_level = gtk::LevelBar::builder()
            .min_value(0.0)
            .max_value(100.0)
            .value(70.0)
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["custom-levelbar"])
            .build();

        let battery_items = gtk::Box::builder()
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
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::ScreenTimeoutOpted,
                }
            });

        let battery_performance_mode = CustomListItem::builder()
            .launch(CustomListItemSettings {
                start_icon: None,
                text: "Performance Mode".to_string(),
                value: "Balenced".to_owned(),
                end_icon: widget_configs.menu_item.end_icon.clone(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    CustomListItemMessage::WidgetClicked => Message::PerformanceOpted,
                }
            });

        battery_items.append(&battery_percentage_level);
        battery_items.append(screen_off_timeout.widget());
        battery_items.append(battery_performance_mode.widget());
        // battery_items.append(&screen_off_timeout_widget.clone());

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&battery_label);
        scrollable_content.append(&battery_items);

        let screen_layout = Layout::builder()
            .launch(LayoutInit {
                title: "Battery".to_owned(),
                content: scrollable_content,
                footer_config: widget_configs.footer,
            })
            .forward(sender.input_sender(), |msg| {
                println!("Batery callback {:?}", msg);
                match msg {
                    LayoutMessage::BackPressed => Message::BackPressed,
                }
            });

        root.append(screen_layout.widget());

        let model = BatteryPage {
            settings: init,
            battery_level: 0.0,
            performance_mode: "".to_owned(),
            screen_off_timeout: "".to_owned(),
        };

        let widgets = BatteryPageWidgets {
            // back_button,
            screen_layout,
            battery_percentage_level,
            battery_performance_mode,
            screen_off_timeout,
        };

        let sender: relm4::Sender<Message> = sender.input_sender().clone();
        get_info(sender).await;
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        // info!("battery page - msg - Update message is {:?}", message);
        // let sender_1: relm4::Sender<Message> = sender.input_sender().clone();
        // get_info(sender_1).await;
        match message {
            Message::BackPressed => {
                println!("Batery BackPressed");
                let _ = sender.output(Message::BackPressed);
            }
            Message::ScreenTimeoutOpted => {
                let _ = sender.output(Message::ScreenTimeoutOpted);
            }
            Message::PerformanceOpted => {
                let _ = sender.output(Message::PerformanceOpted);
            }
            Message::BatteryLevelChanged(value) => {
                __self.battery_level = value.clone();
                let _ = sender.output(Message::BatteryLevelChanged(value));
            }
            Message::PerformanceModeChanged(value) => {
                __self.performance_mode = value.clone();
                let _ = sender.output(Message::PerformanceModeChanged(value));
            }
            Message::ScreenTimeoutChanged(value) => {
                __self.screen_off_timeout = value.clone();
                let _ = sender.output(Message::ScreenTimeoutChanged(value));
            }
            Message::UpdateView => {
                let sender: relm4::Sender<Message> = sender.input_sender().clone();
                get_info(sender).await;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        info!("About- Update view");
        widgets
            .battery_percentage_level
            .set_value(self.battery_level as f64);

        widgets
            .battery_performance_mode
            .emit(ListItemInputMessage::ValueChanged(
                self.performance_mode.to_owned(),
            ));
        widgets
            .screen_off_timeout
            .emit(ListItemInputMessage::ValueChanged(
                self.screen_off_timeout.to_string(),
            ))
    }
}

async fn get_info(sender: relm4::Sender<Message>) {
    match Power::get_battery_percentage().await {
        Ok(status) => {
            let _ = sender.send(Message::BatteryLevelChanged(status));
        }
        Err(e) => {
            error!("Error getting device oem info: {}", e);
        }
    };

    // match Power::get_screen_timeout().await {
    //     Ok(value) => {

    //         let _ = sender.send(Message::ScreenTimeoutChanged(value));
    //     }
    //     Err(e) => {
    //         error!("Error getting device oem info: {}", e);
    //     }
    // };

    match Power::get_performance_mode().await {
        Ok(value) => {
            let _ = sender.send(Message::PerformanceModeChanged(value));
        }
        Err(e) => {
            error!("Error getting device oem info: {}", e);
        }
    };
}
