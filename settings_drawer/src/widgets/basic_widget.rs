use gtk::{
    gdk, gio,
    prelude::*,
};

use relm4::{factory::{DynamicIndex, FactoryComponent, FactorySender}, gtk::GestureClick};
use relm4::gtk::glib::clone;
use relm4::{gtk, RelmWidgetExt};

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BasicWidgetType {
    #[default]
    Normal,
    Slider,
}

#[derive(Clone, Debug)]
pub enum MessageOutput {
    WidgetClicked(usize, String),
}

#[derive(Clone, Debug)]
pub enum MessageInput {
    ValueChanged(Option<i8>),
    IconChanged(Option<String>),
    TitleChanged(String),
    Pressed,
    Released
}

/// Configuration for the password key widget
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BasicWidgetSettings {
    pub widget_type: BasicWidgetType,
    pub title: String,
    pub icon: Option<String>,
    pub value: Option<i8>,
    pub value_subscript: Option<String>,
}

/// Password Key component.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BasicWidget {
    pub settings: BasicWidgetSettings,
    pub is_pressing: bool
}

pub struct BasicWidgetWidgets {
    title_label: gtk::Label,
    value_label: gtk::Label,
    settings_button_box: gtk::Box
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for BasicWidget {
    type Init = BasicWidgetSettings;
    type Input = MessageInput;
    type Output = MessageOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;
    type Widgets = BasicWidgetWidgets;
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            // .vexpand(false)
            .hexpand(false)
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["grid-item"])
            // .halign(gtk::Align::Start)
            // .valign(gtk::Align::Start)
            .build()
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { settings: value, is_pressing: false }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        info!("password key update message {:?}", msg);
        match msg {
            MessageInput::TitleChanged(title) => {
                self.settings.title = title;
            }
            MessageInput::Pressed => {
                self.is_pressing = true;
            },
            MessageInput::Released => {
                self.is_pressing = false;
            },
            MessageInput::ValueChanged(_) => (),
            MessageInput::IconChanged(_) => (),
        }
    }

    fn init_widgets(
        &mut self,
        parent_index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let title_label = gtk::Label::builder()
            .label(&self.settings.title)
            .css_classes(["settings-title"])
            .halign(gtk::Align::Start)
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();

        // let settings_button = gtk::Button::builder()
        //     .css_classes(["settings-button"])
        //     .vexpand(false)
        //     .build();

        let settings_button_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["settings-button-box"])
            .build();

        let left_click_gesture = GestureClick::builder().button(0).build();
        left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
        info!("gesture button pressed is {}", this.current_button());
                sender.input_sender().send(MessageInput::Pressed);
    
        }));
    
        let title = self.settings.title.to_owned();
            left_click_gesture.connect_released(clone!(@strong sender, @strong title, @strong parent_index => move |this, _, _,_| {
                    info!("gesture button released is {}", this.current_button());
                    sender.input_sender().send(MessageInput::Released);
                    sender.output(MessageOutput::WidgetClicked(parent_index.current_index(), title.to_owned()));
            }));

        settings_button_box.add_controller(left_click_gesture);    

        match &self.settings.widget_type {
            BasicWidgetType::Slider => {
                settings_button_box.set_class_active("settings-button-box-slider", true);
            }
            _ => {}
        };

        match &self.settings.icon {
            Some(icon) => {
                info!("icon is {}", icon);
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .css_classes(["settings-icon"])
                    .halign(gtk::Align::Start)
                    .build();
                settings_button_box.append(&icon_image);
            }
            None => (),
        }

        let value_row = gtk::Box::builder()
            .halign(gtk::Align::Start)
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();

        let value_label = gtk::Label::builder()
            .css_classes(["settings-value"])
            .build();

        let value_subscript_label = gtk::Label::builder()
            .css_classes(["settings-value-subscript"])
            .valign(gtk::Align::End)
            .build();

        match &self.settings.value {
            Some(value) => {
                value_label.set_label(&format!("{}", value));
                value_row.append(&value_label);
                settings_button_box.append(&value_row);
            }
            None => (),
        }

        match &self.settings.value_subscript {
            Some(value) => {
                value_subscript_label.set_label(&value);
                value_row.append(&value_subscript_label);
            }
            None => (),
        }

        settings_button_box.append(&title_label);

        root.append(&settings_button_box);
        let title = self.settings.title.to_owned();
 //       settings_button.connect_clicked(
        //     clone!(@strong sender, @strong title, @strong parent_index  => move |_| {
        //         sender.output(MessageOutput::WidgetClicked(parent_index.current_index(), title.to_owned()));
        //     }),
        // );

        BasicWidgetWidgets {
            title_label,
            value_label,
            settings_button_box
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: FactorySender<Self>) {
        widgets.title_label.set_label(&self.settings.title);
        widgets.settings_button_box.set_class_active("settings-button-box-pressing",self.is_pressing);
    }
}
