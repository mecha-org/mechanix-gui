use gtk::{
    gdk, gio,
    prelude::{BoxExt, ButtonExt, EventControllerExt, GestureSingleExt, WidgetExt},
};

use relm4::gtk::glib::clone;
use relm4::{
    factory::{DynamicIndex, FactoryComponent, FactorySender},
    gtk::GestureClick,
};
use relm4::{gtk, RelmWidgetExt};

use custom_utils::get_image_from_path;
use serde::{Deserialize, Serialize};
use tracing::info;
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;

#[derive(Clone, Debug)]
pub enum Message {
    AppInstanceClicked(ToplevelKey),
    AppInstanceCloseClicked(ToplevelKey),
}

#[derive(PartialEq, Eq, Hash, Default, Debug, Clone)]
pub struct AppInstance {
    pub title: Option<String>,
    pub instance_key: ToplevelKey,
    pub icon: Option<String>,
}

/// Configuration for the App widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone)]
pub struct AppDetails {
    pub app_id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub instances: Vec<AppInstance>,
}

/// App component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone)]
pub(crate) struct RunningApp {
    pub app_details: AppDetails,
    pub close_icon: Option<String>,
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for RunningApp {
    type Init = RunningApp;
    type Input = ();
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    type Widgets = ();
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .build()
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            app_details: init.app_details,
            close_icon: init.close_icon,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        info!("App update message {:?}", msg);
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let first_instance_op = self.app_details.instances.get(0);
        match first_instance_op {
            Some(instance) => {
                let icon_box_header = gtk::Box::builder()
                    .css_classes(["app-instance-icon-box-header"])
                    .build();

                let close_button = gtk::Box::builder()
                    .css_classes(["app-instance-close-btn"])
                    .halign(gtk::Align::End)
                    .build();
                let close_button_left_click_gesture = GestureClick::builder().build();
                close_button_left_click_gesture.connect_released(
                    clone!(@strong sender, @strong instance => move |_, _, _,_| {
                        let _ = sender.output(Message::AppInstanceCloseClicked(instance.instance_key));
                    }),
                );
                close_button.add_controller(close_button_left_click_gesture);

                info!("close icon is {:?}", self.close_icon);

                let close_icon = get_image_from_path(self.close_icon.clone(), &[""]);
                close_icon.set_hexpand(true);
                close_icon.set_vexpand(true);
                close_button.append(&close_icon);

                icon_box_header.append(&close_button);

                let icon_box = gtk::Box::builder()
                    .css_classes(["app-instance-icon-box"])
                    .orientation(gtk::Orientation::Horizontal)
                    .hexpand(true)
                    .vexpand(true)
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .build();

                let app_instance_icon = gtk::Image::builder()
                    .icon_name(match &instance.icon {
                        Some(icon) => icon,
                        None => "",
                    })
                    .css_classes(["app-instance-icon"])
                    .icon_size(gtk::IconSize::Large)
                    .pixel_size(88)
                    .hexpand(true)
                    .vexpand(true)
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .build();

                icon_box.append(&app_instance_icon);

                let instance_title = gtk::Label::builder()
                    .halign(gtk::Align::Start)
                    .label(match &instance.title {
                        Some(v) => v,
                        None => "",
                    })
                    .css_classes(["app-instance-title"])
                    .build();

                let app_name = gtk::Label::builder()
                    .label(match &self.app_details.name {
                        Some(v) => v,
                        None => "",
                    })
                    .halign(gtk::Align::Start)
                    .css_classes(["app-name"])
                    .build();

                root.append(&icon_box_header);
                root.append(&icon_box);
                root.append(&instance_title);
                root.append(&app_name);
                let left_click_gesture = GestureClick::builder().build();
                left_click_gesture.connect_released(
                    clone!(@strong sender, @strong instance => move |_, _, _,_| {
                        let _ = sender.output(Message::AppInstanceClicked(instance.instance_key));
                    }),
                );
                icon_box.add_controller(left_click_gesture.clone());
                // instance_title.add_controller(left_click_gesture.clone());
                // app_name.add_controller(left_click_gesture);
            }
            None => (),
        }
    }
}
