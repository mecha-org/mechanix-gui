use std::borrow::Borrow;

use gtk::prelude::*;

use relm4::gtk::glib::clone;
use relm4::{gtk, RelmRemoveAllExt, RelmWidgetExt, SimpleComponent};
use relm4::{gtk::GestureClick, prelude::ComponentParts};

use custom_utils::get_image_from_path;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::pages::bluetooth_details_page::SettingItem;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked,
}

#[derive(Clone, Debug)]
pub enum InputMessage {
    Pressed,
    Released,
    ChangeActiveValue(bool),
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
    action_button: gtk::Box,
    image: gtk::Image,
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

        let (action_button, image) = get_action_button(&init, init.is_active);

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
        let left_click_gesture = GestureClick::builder().build();
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

        

        let widgets = CustomListRadioButtonWidgets {
            container: root.clone(),
            action_button,
            image
        };

        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        info!("password key update message {:?}", message);
        match message {
            InputMessage::Released => {
                self.is_active = !self.is_active;
            }
            InputMessage::ChangeActiveValue(value) => {
                self.is_active = value;
            }
            _ => {
                println!("WidgetClicked is {}", self.is_active );
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
            .set_class_active("custom-list-item-box-focus", self.is_active);

        match self.is_active {
            true => match self.settings.active_icon.clone() {
                Some(icon) => {
                    let image = get_image_from_path(Some(icon), &["custom-list-item-box-start-icon"]);
                    // action_button.append(&image);
                    match image.paintable() {
                        None=>{

                        }
                        Some(paint) => {
                            widgets.image.set_paintable(Some(&paint));
                        }
                    }
                    
                }
                None => (),
            },
            false => match self.settings.inactive_icon.clone() {
                Some(icon) => {
                    let image = get_image_from_path(Some(icon), &["custom-list-item-box-end-icon"]);
                    // action_button.append(&image);
                    // widgets.image = image;
                    match image.paintable() {
                        None=>{

                        }
                        Some(paint) => {
                            widgets.image.set_paintable(Some(&paint));
                        }
                    }
                }
                None => (),
            },
        }
        // match widgets.container.last_child() {
        //     None => {

        //     },
        //     Some(widget) => {
        //         widgets.container.remove(&widget);
        //     }   
        // }
        // ;

        // widgets.container.append(&get_action_button(&self.settings, self.is_active));
        
    }
    
}
fn get_action_button(settings: &CustomListRadioButtonSettings, is_active: bool) -> (gtk::Box, gtk::Image) {
    let label = gtk::Label::builder()
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Start)
        .vexpand(true)
        .hexpand(true)
        .label(&settings.text)
        .css_classes(["custom-list-radio-button-label"])
        .build();

    let action_button = gtk::Box::builder()
        .valign(gtk::Align::Center)
        .vexpand(true)
        .build();

    action_button.append(&label);
    let mut image = gtk::Image::new();
    match is_active {
        true => match settings.active_icon.clone() {
            Some(icon) => {
                image = get_image_from_path(Some(icon), &["custom-list-item-box-start-icon"]);
                action_button.append(&image);
            }
            None => (),
        },
        false => match settings.inactive_icon.clone() {
            Some(icon) => {
                image = get_image_from_path(Some(icon), &["custom-list-item-box-end-icon"]);
                action_button.append(&image);
            }
            None => (),
        },
    }
    label.set_class_active("custom-list-radio-button-label-active", is_active);
    (action_button, image)
}
