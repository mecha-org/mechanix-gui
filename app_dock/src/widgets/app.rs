use gtk::{
    gdk, gio,
    prelude::{BoxExt, ButtonExt, EventControllerExt, GestureSingleExt, WidgetExt},
};

use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender};
use relm4::gtk::glib::clone;
use relm4::{gtk, RelmWidgetExt};

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetPressed(String),
    WidgetLongPressed(String),
}

/// Configuration for the App widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub app_id: String,
    pub title: String,
    pub alias: String,
    pub icon: Option<String>,
}

/// App component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct App {
    pub settings: AppSettings,
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for App {
    type Init = AppSettings;
    type Input = ();
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    type Widgets = ();
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            .vexpand(true)
            .hexpand(true)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build()
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { settings: value }
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
        let button = gtk::Button::builder()
            .css_classes(["app-button"])
            .vexpand(false)
            .build();

        match &self.settings.icon {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .build();
                button.set_child(Some(&icon_image));
                button.set_class_active("app-button-icon", true);
            }
            None => (),
        }

        root.append(&button);
        let app_id = self.settings.app_id.to_owned();
        button.connect_clicked(clone!(@strong sender, @strong app_id => move |_| {
            sender.output(Message::WidgetPressed(app_id.to_owned()));
        }));

        let press_gesture = gtk::GestureLongPress::new();
        press_gesture.set_touch_only(false);
        press_gesture.set_propagation_phase(gtk::PropagationPhase::Capture);
        press_gesture.connect_pressed(clone!(@strong sender, @strong app_id => move |_, _ ,_| {
            sender.output(Message::WidgetLongPressed(app_id.to_owned()));
        }));
        button.add_controller(press_gesture);
    }
}
