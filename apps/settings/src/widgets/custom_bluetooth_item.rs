use gtk::{gdk, gio, prelude::*};

use relm4::gtk::glib::clone;
use relm4::{gtk, RelmWidgetExt, SimpleComponent};
use relm4::{gtk::GestureClick, prelude::ComponentParts};

use custom_utils::get_image_from_path;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked,
}

#[derive(Clone, Debug)]
pub enum InputMessage {
    Pressed,
    Released,
}

/// Configuration for the password key widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomBluetoothItemSettings {
    pub name: String,
    pub is_connected: bool,
    pub connected_icon: Option<String>,
    pub info_i_icon: Option<String>,
    pub info_arrow_icon: Option<String>,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomBluetoothItem {
    pub settings: CustomBluetoothItemSettings,
    pub is_pressing: bool,
}

#[derive(Debug)]
pub struct CustomBluetoothItemWidgets {
    container: gtk::Box,
}

// #[relm4::factory(pub(crate))]
impl SimpleComponent for CustomBluetoothItem {
    type Init = CustomBluetoothItemSettings;
    type Input = InputMessage;
    type Output = Message;
    type Widgets = CustomBluetoothItemWidgets;
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

        let bluetooth_item_button = gtk::Box::builder().vexpand(false).build();
        bluetooth_item_button.append(&label);

        if init.is_connected {
            match init.connected_icon.clone() {
                Some(icon) => {
                    let connected_icon_image =
                        get_image_from_path(Some(icon), &["custom-connected-icon"]);
                    bluetooth_item_button.append(&connected_icon_image);
                }
                None => (),
            }
        }

        match init.info_i_icon.clone() {
            Some(icon) => {
                let info_icon_image =
                    get_image_from_path(Some(icon), &["custom-info-icon"]);
                bluetooth_item_button.append(&info_icon_image);
            }
            None => (),
        }

        match init.info_arrow_icon.clone() {
            Some(icon) => {
                let end_icon_image =
                    get_image_from_path(Some(icon), &["custom-list-item-box-end-icon"]);
                    bluetooth_item_button.append(&end_icon_image);
            }
            None => (),
        }

        root.append(&bluetooth_item_button);
        let button_click_gesture = GestureClick::builder().button(0).build();
        button_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("Bluetooth gesture button pressed is {}", this.current_button());
            sender.input_sender().send(InputMessage::Pressed);

        }));

        let key = init.name.to_owned();
        button_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("Bluetooth gesture button released is {}", this.current_button());
                sender.input_sender().send(InputMessage::Released);
                sender.output(Message::WidgetClicked);
        }));
        root.add_controller(button_click_gesture);
        
        // bluetooth_item_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
        //     sender.output(Message::WidgetClicked(key.to_owned()));
        // }));

        let model = CustomBluetoothItem {
            settings: CustomBluetoothItemSettings {
                name: init.name,
                is_connected: init.is_connected,
                connected_icon: init.connected_icon,
                info_i_icon: init.info_i_icon,
                info_arrow_icon: init.info_arrow_icon,
            },
            is_pressing: false,
        };

        let widgets = CustomBluetoothItemWidgets {
            container: root.clone(),
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
