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
pub struct CustomListRadioButtonSettings {
    pub text: String,
    pub description_text: Option<String>,
    pub active_icon: Option<String>,
    pub inactive_icon: Option<String>,
    pub is_active: bool,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CustomListRadioButton {
    pub settings: CustomListRadioButtonSettings,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct CustomListRadioButtonWidgets {
    container: gtk::Box,
}

// #[relm4::factory(pub(crate))]
impl SimpleComponent for CustomListRadioButton {
    type Init = CustomListRadioButtonSettings;
    type Input = InputMessage;
    type Output = Message;
    type Widgets = CustomListRadioButtonWidgets;
    type Root = gtk::Box;

    fn init_root() -> Self::Root {
        let container = gtk::Box::builder()
            .vexpand(false)
            .hexpand(true)
            .css_classes(["custom-list-radio-button"])
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Center)
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
            .css_classes(["custom-list-radio-button-label"])
            .build();

        let action_button = gtk::Box::builder()
            .valign(gtk::Align::Center)
            .vexpand(true)
            .build();

        action_button.append(&label);

        match init.is_active {
            true => match init.active_icon.clone() {
                Some(icon) => {
                    let image =
                        get_image_from_path(Some(icon), &["custom-list-item-box-start-icon"]);
                    action_button.append(&image);
                }
                None => (),
            },
            false => match init.inactive_icon.clone() {
                Some(icon) => {
                    let image = get_image_from_path(Some(icon), &["custom-list-item-box-end-icon"]);
                    action_button.append(&image);
                }
                None => (),
            },
        }

        root.append(&action_button);
        match &init.description_text {
            Some(text) => {
                let description_label = gtk::Label::builder()
                    .valign(gtk::Align::Center)
                    .halign(gtk::Align::Start)
                    .vexpand(true)
                    .hexpand(true)
                    .label(text)
                    .css_classes(["custom-list-radio-button-description"])
                    .build();
                description_label.set_markup(text);
                root.append(&description_label);
            }
            None => (),
        }
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            sender.input_sender().send(InputMessage::Pressed);

        }));

        let key = init.text.to_owned();
        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                sender.input_sender().send(InputMessage::Released);
                sender.output(Message::WidgetClicked);
        }));
        root.add_controller(left_click_gesture);
        // action_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
        //     sender.output(Message::WidgetClicked(key.to_owned()));
        // }));

        let model = CustomListRadioButton {
            settings: CustomListRadioButtonSettings {
                text: init.text,
                active_icon: init.active_icon,
                inactive_icon: init.inactive_icon,
                is_active: init.is_active,
                description_text: init.description_text,
            },
            is_active: init.is_active,
        };

        label.set_class_active("custom-list-radio-button-label-active", init.is_active);

        let widgets = CustomListRadioButtonWidgets {
            container: root.clone(),
        };

        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        info!("password key update message {:?}", message);
        match message {
            InputMessage::Released => {
                self.is_active = true;
            }
            _ => {}
        }
    }

    fn update_view(
        &self,
        widgets: &mut Self::Widgets,
        sender: relm4::prelude::ComponentSender<Self>,
    ) {
        widgets
            .container
            .set_class_active("custom-list-item-box-focus", self.is_active);
    }
}
