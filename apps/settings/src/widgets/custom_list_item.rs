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
}

#[derive(Clone, Debug)]
pub enum InputMessage {
    Pressed,
    Released,
}

/// Configuration for the password key widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomListItemSettings {
    pub start_icon: Option<String>,
    pub text: String,
    pub value: String,
    pub end_icon: Option<String>,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomListItem {
    pub settings: CustomListItemSettings,
    pub is_pressing: bool,
}

#[derive(Debug)]
pub struct CustomListItemWidgets {
    container: gtk::Box,
}

// #[relm4::factory(pub(crate))]
impl SimpleComponent for CustomListItem {
    type Init = CustomListItemSettings;
    type Input = InputMessage;
    type Output = Message;
    type Widgets = CustomListItemWidgets;
    type Root = gtk::Box;

    fn init_root() -> Self::Root {
        let container = gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .css_classes(["custom-list-item-box"])
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
            .label(&init.text)
            .css_classes(["custom-list-item-box-label"])
            .build();


        let value = gtk::Label::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::End)
            .vexpand(true)
            .hexpand(true)
            .label(&init.value)
            .css_classes(["custom-list-item-box-label"])
            .build();


        let action_button = gtk::Box::builder().vexpand(false).build();

        match init.start_icon.clone() {
            Some(icon) => {
                let start_icon_image =
                    get_image_from_path(Some(icon), &["custom-list-item-box-start-icon"]);
                action_button.append(&start_icon_image);
            }
            None => (),
        }

        action_button.append(&label);
        action_button.append(&value);

        match init.end_icon.clone() {
            Some(icon) => {
                let end_icon_image =
                    get_image_from_path(Some(icon), &["custom-list-item-box-end-icon"]);
                action_button.append(&end_icon_image);
            }
            None => (),
        }

        root.append(&action_button);
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            let _ = sender.input_sender().send(InputMessage::Pressed);

        }));

        let key = init.text.to_owned();
        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.input_sender().send(InputMessage::Released);
                let _ = sender.output(Message::WidgetClicked);
        }));
        root.add_controller(left_click_gesture);
        // action_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
        //     sender.output(Message::WidgetClicked(key.to_owned()));
        // }));

        let model = CustomListItem {
            settings: CustomListItemSettings {
                start_icon: init.start_icon,
                text: init.text,
                value: init.value,
                end_icon: init.end_icon,
            },
            is_pressing: false,
        };

        let widgets = CustomListItemWidgets {
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
