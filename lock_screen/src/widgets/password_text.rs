use gtk::prelude::BoxExt;

use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender};

use relm4::{gtk, RelmWidgetExt};

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug)]
pub enum Message {
    ToggleFilled,
}

/// Configuration for the password text widget
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PasswordTextSettings {
    pub is_filled: bool,
}

/// Password text widget component.
#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PasswordText {
    pub settings: PasswordTextSettings,
}

pub struct PasswordTextWidgets {
    pub password_text_box: gtk::Box,
}

impl FactoryComponent for PasswordText {
    type Init = PasswordTextSettings;
    type Input = Message;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    type Widgets = PasswordTextWidgets;
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder().build()
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { settings: value }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        info!("password text update message {:?}", msg);
        match msg {
            Message::ToggleFilled => {
                self.settings.is_filled = !self.settings.is_filled;
            }
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let password_text_box = gtk::Box::builder()
            .css_classes(["password-text"])
            .vexpand(false)
            .hexpand(false)
            .build();

        root.append(&password_text_box);

        PasswordTextWidgets { password_text_box }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets
            .password_text_box
            .set_class_active("password-text-filled", self.settings.is_filled);
    }
}
