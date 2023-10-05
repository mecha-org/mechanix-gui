use gtk::{
    gdk, gio,
    prelude::{BoxExt, ButtonExt},
};

use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender};
use relm4::gtk::glib::clone;
use relm4::{gtk, RelmWidgetExt};

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    WidgetClicked(String),
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
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for PasswordKey {
    type Init = PasswordKeySettings;
    type Input = ();
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;
    type Widgets = ();
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
        Self { settings: value }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        info!("password key update message {:?}", msg);
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let label = gtk::Label::builder()
            .label(&self.settings.key)
            .css_classes(["password-key-label"])
            .build();

        let action_button = gtk::Button::builder()
            .css_classes(["password-key-button"])
            .vexpand(false)
            .build();

        match &self.settings.icon {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let icon_asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let icon_image = gtk::Image::builder()
                    .paintable(&icon_asset_paintable)
                    .build();
                action_button.set_child(Some(&icon_image));
                action_button.set_class_active("password-key-icon-button", true);
            }
            None => {
                action_button.set_child(Some(&label));
                action_button.set_class_active("password-key-label-button", true);
                root.set_class_active("password-key-box", true);
            }
        }

        root.append(&action_button);
        let key = self.settings.key.to_owned();
        action_button.connect_clicked(clone!(@strong sender, @strong key => move |_| {
            sender.output(Message::WidgetClicked(key.to_owned()));
        }));
    }
}
