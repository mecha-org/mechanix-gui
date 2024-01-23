use gtk::{gdk, gio, prelude::*};

use relm4::gtk::glib::clone;
use relm4::{
    factory::{DynamicIndex, FactoryComponent, FactorySender},
    gtk::GestureClick,
};
use relm4::{gtk, RelmWidgetExt};

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked(String),
}

#[derive(Clone, Debug)]
pub enum InputMessage {
    Pressed,
    Released,
    Reveal,
}

/// Configuration for the password key widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PasswordKeySettings {
    pub key: String,
    pub icon: Option<String>,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PasswordKey {
    pub settings: PasswordKeySettings,
    pub is_pressing: bool,
    pub reveal: bool,
}

#[derive(Debug)]
pub struct PasswordKeyWidgets {
    password_key_label: gtk::Label,
    revealer: gtk::Revealer,
    child_box: gtk::Box,
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for PasswordKey {
    type Init = PasswordKeySettings;
    type Input = InputMessage;
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;
    type Widgets = PasswordKeyWidgets;
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build()
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            settings: value,
            is_pressing: false,
            reveal: false,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        info!("password key update message {:?}", msg);
        match msg {
            InputMessage::Pressed => {
                self.is_pressing = true;
            }
            InputMessage::Released => {
                self.is_pressing = false;
            }
            InputMessage::Reveal => {
                self.reveal = true;
            }
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let label = gtk::Label::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .vexpand(true)
            .hexpand(true)
            .label(&self.settings.key)
            .css_classes(["password-key-label"])
            .build();

        let action_button = gtk::Box::builder().vexpand(false).build();

        match &self.settings.icon {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .valign(gtk::Align::Center)
                    .halign(gtk::Align::Center)
                    .css_classes(["password-key-icon"])
                    .vexpand(true)
                    .hexpand(true)
                    .build();
                action_button.append(&icon_image);
            }
            None => {
                action_button.append(&label);
            }
        }
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            sender.input_sender().send(InputMessage::Pressed);

        }));

        let key = self.settings.key.to_owned();
        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                sender.input_sender().send(InputMessage::Released);
                sender.output(Message::WidgetClicked(key.to_owned()));
        }));
        let child_box = gtk::Box::builder().build();

        if !self.settings.key.is_empty() {
            child_box.append(&action_button);
            child_box.add_controller(left_click_gesture);
            child_box.set_css_classes(&["password-key-box"]);
        }

        let revealer = gtk::Revealer::builder()
            // .margin_bottom(10)
            .valign(gtk::Align::End)
            .halign(gtk::Align::End)
            .css_classes(["revealer"])
            .build();
        revealer.set_child(Some(&child_box));
        revealer.set_transition_type(gtk::RevealerTransitionType::SwingUp);
        revealer.set_transition_duration(800);
        revealer.set_reveal_child(false);
        root.append(&revealer);
        // action_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
        //     sender.output(Message::WidgetClicked(key.to_owned()));
        // }));

        let widgets = PasswordKeyWidgets {
            password_key_label: label,
            revealer,
            child_box,
        };

        widgets
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: FactorySender<Self>) {
        widgets
            .password_key_label
            .set_class_active("password-key-focus", self.is_pressing);

        info!("self.reveal {}", self.reveal);

        widgets.revealer.set_reveal_child(self.reveal);
    }
}
