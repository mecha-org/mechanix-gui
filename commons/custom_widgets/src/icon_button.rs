use gtk::{gdk, gio, glib::clone, prelude::*, subclass::*};
use relm4::{
    gtk::{self, GestureClick},
    ComponentParts, RelmWidgetExt, SimpleComponent,
};
use tracing::info;

#[derive(Debug, Clone)]
pub struct IconButtonCss {
    root_container: Option<Vec<String>>,
    container: Option<Vec<String>>,
    container_pressing: Option<String>,
    icon: Option<Vec<String>>,
}

impl Default for IconButtonCss {
    fn default() -> Self {
        Self {
            root_container: Option::from(vec!["icon-button-root-default".to_string()]),
            container: Option::from(vec!["icon-button-container-default".to_string()]),
            container_pressing: Option::from("icon-button-container-pressing-default".to_string()),
            icon: Option::from(vec!["icon-button-icon-default".to_string()]),
        }
    }
}

#[derive(Debug)]
pub struct InitSettings {
    pub icon: Option<String>,
    pub css: IconButtonCss,
}

#[derive(Debug)]
pub enum InputMessage {
    Pressed,
    Released,
}

#[derive(Debug)]
pub enum OutputMessage {
    Clicked,
}

pub struct IconButton {
    settings: InitSettings,
    is_in_pressing_state: bool,
}

pub struct ComponentWidgets {
    container_box: gtk::Box,
}

impl SimpleComponent for IconButton {
    type Input = InputMessage;

    type Output = OutputMessage;

    type Init = InitSettings;

    type Root = gtk::Box;

    type Widgets = ComponentWidgets;

    fn init_root() -> Self::Root {
        let root_box = gtk::Box::builder()
            .valign(gtk::Align::Center)
            .hexpand(false)
            .vexpand(false)
            .build();
        root_box
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        info!("icon button init called");

        match init.css.root_container.to_owned() {
            Some(css) => root.set_css_classes(&[css.join(",").as_str()]),
            None => (),
        }

        let container_box = gtk::Box::builder()
            .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        match init.css.container.to_owned() {
            Some(css) => container_box.set_css_classes(&[css.join(",").as_str()]),
            None => (),
        }

        let icon = init.icon.clone();
        match icon.to_owned() {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                let image = gtk::Image::builder()
                    .paintable(&asset_paintable)
                    .hexpand(true)
                    .vexpand(true)
                    .build();

                match init.css.icon.to_owned() {
                    Some(css) => image.set_css_classes(&[css.join(",").as_str()]),
                    None => (),
                };
                container_box.append(&image);
                let left_click_gesture = GestureClick::builder().button(0).build();
                left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
                info!("gesture button pressed is {}", this.current_button());
                    sender.input_sender().send(InputMessage::Pressed);

                }));

                left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                        info!("gesture button released is {}", this.current_button());
                        sender.input_sender().send(InputMessage::Released);

                }));
                root.add_controller(left_click_gesture);
            }
            None => (),
        }

        root.append(&container_box);

        let model = IconButton {
            settings: init,
            is_in_pressing_state: false,
        };

        let widgets = ComponentWidgets { container_box };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        info!("icon button update message {:?}", message);
        match message {
            InputMessage::Pressed => {
                self.is_in_pressing_state = true;
            }
            InputMessage::Released => {
                self.is_in_pressing_state = false;
                let _ = sender.output_sender().send(OutputMessage::Clicked);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: relm4::ComponentSender<Self>) {
        match self.settings.css.container_pressing.to_owned() {
            Some(css) => {
                widgets.container_box.set_class_active(
                    &css.as_str(),
                    self.is_in_pressing_state,
                );
            },
            None => (),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {
        info!("icon button sutdown called");
    }
}
