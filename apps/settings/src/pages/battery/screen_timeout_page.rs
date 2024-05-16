use crate::{
    settings::{LayoutSettings, Modules, WidgetConfigs},
    widgets::{
        custom_list_radio_button::{
            CustomListRadioButton, CustomListRadioButtonSettings, InputMessage,
            Message as CustomListRadioButtonMessage,
        },
        header::Header,
        scrolled_box::ScrolledBox,
    },
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    OutputMessage as IconButtonOutputMessage,
};
use gtk::prelude::*;
use mechanix_system_dbus_client::power::Power;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk, AsyncComponentSender, Component, ComponentController, Controller,
};
use std::collections::HashMap;
use tracing::{error, info};

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct ScreenTimeoutPage {
    settings: Settings,
    selected_value: String,
}

//Widgets
pub struct ScreenTimeoutPageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
    radi_button_list: HashMap<String, Controller<CustomListRadioButton>>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    SubmitPressed,
    SelectedValueChanged(String),
    UpdateView,
}

#[async_trait(?Send)]
impl AsyncComponent for ScreenTimeoutPage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = ScreenTimeoutPageWidgets;
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

        let radio_button_group = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let names = vec!["10s", "30s", "60s", "5m", "15m", "30m"];
        // let mut radio_buttons  = Vec::new();

        let mut radio_buttons = HashMap::new();

        for name in names.iter() {
            let radio_button = get_radio_button(name.to_string(), &widget_configs, &sender);
            let radio_button_widget = radio_button.widget();
            radio_button_group.append(radio_button_widget);
            // radio_buttons.push(radio_button);
            radio_buttons.insert(name.to_string(), radio_button);
        }

        let scrollable_content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        scrollable_content.append(&radio_button_group);

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

        let header = Header::builder().launch("Screen Off Timeout".to_owned());
        let scrolled_box = ScrolledBox::builder().launch(scrollable_content);

        root.append(header.widget());
        root.append(scrolled_box.widget());
        root.append(&footer);

        let model = ScreenTimeoutPage {
            settings: init,
            selected_value: "".to_owned(),
        };

        let widgets = ScreenTimeoutPageWidgets {
            back_button,
            submit_button,
            radi_button_list: radio_buttons,
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
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::SubmitPressed => {
                let input_sender: relm4::Sender<Message> = sender.input_sender().clone();
                set_screen_timeout(input_sender, __self.selected_value.clone()).await;
                let _ = sender.output(Message::SubmitPressed);
            }
            Message::SelectedValueChanged(value) => {
                __self.selected_value = value.clone();
            }
            Message::UpdateView => {
                let sender: relm4::Sender<Message> = sender.input_sender().clone();
                get_info(sender).await;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        for (_key, radio_button) in widgets.radi_button_list.iter() {
            radio_button.emit(InputMessage::ChangeActiveValue(false))
        }

        match widgets.radi_button_list.get(&self.selected_value) {
            None => {}
            Some(selected_radio) => {
                selected_radio.emit(InputMessage::ChangeActiveValue(true));
            }
        };
    }
}

async fn get_info(sender: relm4::Sender<Message>) {
    // match Power::get_screen_timeout().await {
    //     Ok(value) => {

    //         let _ = sender.send(Message::SelectedValueChanged(value));
    //     }
    //     Err(e) => {
    //         error!("Error getting device get_screen_timeout: {}", e);
    //     }
    // };
}

async fn set_screen_timeout(sender: relm4::Sender<Message>, value: String) {
    // match Power::set_screen_timeout(get_screen_timeout_in_seconds(&value)).await {
    //     Ok(value) => {

    //         let _ = sender.send(Message::BackPressed);
    //     }
    //     Err(e) => {
    //         error!("Error getting device set_screen_timeout: {}", e);
    //     }
    // };
}

fn get_radio_button(
    text: String,
    widget_configs: &WidgetConfigs,
    sender: &AsyncComponentSender<ScreenTimeoutPage>,
) -> Controller<CustomListRadioButton> {
    CustomListRadioButton::builder()
        .launch(CustomListRadioButtonSettings {
            text: text.to_string(),
            active_icon: widget_configs.radio_item.active_icon.clone(),
            inactive_icon: widget_configs.radio_item.inactive_icon.clone(),
            is_active: false,
            ..Default::default()
        })
        .forward(sender.input_sender(), move |msg| {
            info!("msg is {:?}", msg);
            match msg {
                CustomListRadioButtonMessage::WidgetClicked => {
                    Message::SelectedValueChanged(text.clone())
                }
            }
        })
}

fn get_screen_timeout_in_seconds(value: &str) -> u32 {
    let result = match value {
        "30s" => 30,
        "10s" => 10,
        "60s" => 60,
        "5m" => 300,
        "15m" => 900,
        "30m" => 1800,
        _ => 0,
    };
    result
}
