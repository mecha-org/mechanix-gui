use gtk::{
    gdk, gio,
    glib::clone,
    prelude::{BoxExt, ButtonExt},
};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::Screens;
use tracing::info;

pub struct Settings {
    pub lock_icon: Option<String>,
}

pub struct HomePage {}

pub struct HomePageWidgets {}

#[derive(Debug)]
pub enum Message {
    ChangeScreen(Screens),
}

impl SimpleComponent for HomePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = HomePageWidgets;

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        match init.lock_icon.to_owned() {
            Some(icon) => {
                let lock_screen_icon_file = gio::File::for_path(icon);
                let lock_screen_asset_paintable =
                    gdk::Texture::from_file(&lock_screen_icon_file).unwrap();
                let lock_screen_image = gtk::Image::builder()
                    .paintable(&lock_screen_asset_paintable)
                    .build();

                let lock_screen_button = gtk::Button::builder()
                    .vexpand(false)
                    .css_classes(["lock-button"])
                    .build();
                lock_screen_button.set_child(Some(&lock_screen_image));
                lock_screen_button.connect_clicked(clone!(@strong sender => move |_| {
                    sender.output(Message::ChangeScreen(Screens::PasswordScreen));
                    sender.input(Message::ChangeScreen(Screens::LockScreen));
                }));
                root.append(&lock_screen_button);
            }
            None => (),
        }

        let model = HomePage {};
        let widgets = HomePageWidgets {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: ComponentSender<Self>) {}

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["home-container"])
            .build()
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        info!("home shutdown called")
    }
}
