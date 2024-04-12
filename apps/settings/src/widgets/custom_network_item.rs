use gtk::prelude::*;
use relm4::gtk::glib::clone;
use relm4::{gtk, RelmWidgetExt, SimpleComponent};
use relm4::{gtk::GestureClick, prelude::ComponentParts};

use custom_utils::get_image_from_path;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked,
    InfoWidgetClicked
}

#[derive(Clone, Debug)]
pub enum InputMessage {
    Pressed,
    Released,
}

/// Configuration for the password key widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomNetworkItemSettings {
    pub name: String,
    pub is_connected: bool,
    pub is_private: bool,
    pub strength: i32,
    pub connected_icon: Option<String>,
    pub private_icon: Option<String>,
    pub strength_icon: Option<String>,
    pub info_icon: Option<String>,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomNetworkItem {
    pub settings: CustomNetworkItemSettings,
    pub is_pressing: bool,
}

#[derive(Debug)]
pub struct CustomNetworkItemWidgets {
    container: gtk::Box,
    network_info_button: gtk::Box,
}

// #[relm4::factory(pub(crate))]
impl SimpleComponent for CustomNetworkItem {
    type Init = CustomNetworkItemSettings;
    type Input = InputMessage;
    type Output = Message;
    type Widgets = CustomNetworkItemWidgets;
    type Root = gtk::Box;

    fn init_root() -> Self::Root {
        let container = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .css_classes(["custom-item-box"])
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .build();
        container
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let label = gtk::Label::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .vexpand(true)
            .hexpand(true)
            .label(&init.name)
            .css_classes(["custom-item-name"])
            .build();

        let network_item_button = gtk::Box::builder().vexpand(false).build();
        network_item_button.append(&label);

        if init.is_connected {
            match init.connected_icon.clone() {
                Some(icon) => {
                    let connected_icon_image =
                        get_image_from_path(Some(icon), &["custom-connected-icon"]);
                    network_item_button.append(&connected_icon_image);
                }
                None => (),
            }
        }

        if init.is_private {
            match init.private_icon.clone() {
                Some(icon) => {
                    let private_icon_image =
                        get_image_from_path(Some(icon), &["custom-private-icon"]);
                    network_item_button.append(&private_icon_image);
                }
                None => (),
            }
        }

        match init.strength_icon.clone() {
            Some(icon) => {
                let strength_icon_image =
                    get_image_from_path(Some(icon), &["custom-strength-icon"]);
                network_item_button.append(&strength_icon_image);
            }
            None => (),
        }

        let network_info_button = gtk::Box::builder().vexpand(false).build();
        match init.info_icon.clone() {
            Some(icon) => {
                let info_icon_image =
                    get_image_from_path(Some(icon), &["custom-info-icon"]);
                    network_info_button.append(&info_icon_image);
            }
            None => (),
        }

        root.append(&network_item_button);
        root.append(&network_info_button);

        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
           let _ = sender.input_sender().send(InputMessage::Pressed);

        }));


        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ =  sender.input_sender().send(InputMessage::Released);
                let _ =  sender.output(Message::WidgetClicked);
        }));
        network_item_button.add_controller(left_click_gesture);




        let right_click_gesture = GestureClick::builder().button(0).build();
        right_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                println!("=> right_click_gesture button released is {}", this.current_button());
                let _ =  sender.output(Message::InfoWidgetClicked);
        }));
        network_info_button.add_controller(right_click_gesture);


        let model = CustomNetworkItem {
            settings: CustomNetworkItemSettings {
                name: init.name,
                is_connected: init.is_connected,
                is_private: init.is_private,
                strength: init.strength,
                connected_icon: init.connected_icon,
                private_icon: init.private_icon,
                strength_icon: init.strength_icon,
                info_icon: init.info_icon,
            },
            is_pressing: false,
        };

        let widgets = CustomNetworkItemWidgets {
            container: root.clone(),
            network_info_button

        };

        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        info!("password key update message {:?}", message);
        match message {
            InputMessage::Pressed => {
                self.is_pressing = true;
            }
            InputMessage::Released => {
                self.is_pressing = false;
            }
        }
    }

    fn update_view(
        &self,
        widgets: &mut Self::Widgets,
        sender: relm4::prelude::ComponentSender<Self>,
    ) {
        widgets
            .container
            .set_class_active("custom-list-item-box-focus", self.is_pressing);
    }
}
