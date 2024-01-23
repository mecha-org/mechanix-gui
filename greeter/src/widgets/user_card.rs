use gtk::{
    gdk, gio,
    prelude::{BoxExt, ButtonExt, EventControllerExt, GestureSingleExt, WidgetExt},
};

use relm4::gtk::glib::clone;
use relm4::{
    adw,
    factory::{DynamicIndex, FactoryComponent, FactorySender},
    gtk::GestureClick,
};
use relm4::{gtk, RelmWidgetExt};

use custom_utils::get_image_from_path;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::users::User;

#[derive(Clone, Debug)]
pub enum Message {
    UserClicked(User),
}

// #[derive(Default, Debug, Clone)]
// pub struct AppInstance {
//     pub title: Option<String>,
//     pub instance_key: String,
//     pub icon: Option<String>,
// }

/// Configuration for the App widget
/// App component.
#[derive(Default, Debug, Clone)]
pub(crate) struct UserCard {
    pub user: User,
}

// #[relm4::factory(pub(crate))]
impl FactoryComponent for UserCard {
    type Init = UserCard;
    type Input = ();
    type Output = Message;
    type CommandOutput = ();
    type ParentWidget = adw::Carousel;
    type Widgets = ();
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["user-card"])
            .build()
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { user: init.user }
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
        info!("user card user is {:?}", self.user);

        let user_avatar = get_image_from_path(self.user.avatar.clone(), &["user-card-avatar"]);
        user_avatar.set_hexpand(true);
        user_avatar.set_vexpand(true);
        user_avatar.set_valign(gtk::Align::Center);
        user_avatar.set_halign(gtk::Align::Center);
        let user_name = gtk::Label::builder()
            .vexpand(true)
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Center)
            .label(match &self.user.name {
                Some(v) => v,
                None => &self.user.username,
            })
            .css_classes(["user-name-label"])
            .build();

        root.append(&user_avatar);
        root.append(&user_name);
        let left_click_gesture = GestureClick::builder().build();
        let user = self.user.clone();
        left_click_gesture.connect_released(
            clone!(@strong sender, @strong user => move |_, _, _,_| {
                let _ = sender.output(Message::UserClicked(user.clone()));
            }),
        );
        root.add_controller(left_click_gesture.clone());
        // instance_title.add_controller(left_click_gesture.clone());
        // app_name.add_controller(left_click_gesture);
    }
}
