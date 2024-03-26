use gtk::{gdk, gio, prelude::*};

use relm4::gtk::glib::clone;
use relm4::{
    factory::{DynamicIndex, FactoryComponent, FactorySender},
    gtk::GestureClick,
};
use relm4::{gtk, RelmWidgetExt};

use custom_utils::get_image_from_path;
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
}

/// Configuration for the password key widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemSettings {
    pub start_icon: Option<String>,
    pub text: String,
    pub end_icon: Option<String>,
}

/// Password Key component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MenuItem {
    pub settings: MenuItemSettings,
    pub is_pressing: bool,
}

#[derive(Debug)]
pub struct MenuItemWidgets {
    password_key_label: gtk::Label,
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for MenuItem {
    type Init = MenuItemSettings;
    type Input = InputMessage;
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    type Widgets = MenuItemWidgets;
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            .vexpand(false)
            .hexpand(false)
            .css_classes(["password-key-box"])
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Start)
            .build()
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            settings: value,
            is_pressing: false,
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
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let label = gtk::Label::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .vexpand(true)
            .hexpand(true)
            .label(&self.settings.text)
            .css_classes(["password-key-label"])
            .build();

        let action_button = gtk::Box::builder().vexpand(false).build();

        let start_icon_image =
            get_image_from_path(self.settings.start_icon.clone(), &["password-key-icon"]);
        let end_icon_image =
            get_image_from_path(self.settings.end_icon.clone(), &["password-key-icon"]);
        action_button.append(&start_icon_image);
        action_button.append(&label);
        action_button.append(&end_icon_image);
        root.append(&action_button);
        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
            let _ = sender.input_sender().send(InputMessage::Pressed);

        }));

        let key = self.settings.text.to_owned();
        left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button released is {}", this.current_button());
                let _ = sender.input_sender().send(InputMessage::Released);
                let _ = sender.output(Message::WidgetClicked(key.to_owned()));
        }));
        root.add_controller(left_click_gesture);
        // action_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
        //     sender.output(Message::WidgetClicked(key.to_owned()));
        // }));

        let widgets = MenuItemWidgets {
            password_key_label: label,
        };

        widgets
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: FactorySender<Self>) {
        widgets
            .password_key_label
            .set_class_active("password-key-focus", self.is_pressing);
    }
}
