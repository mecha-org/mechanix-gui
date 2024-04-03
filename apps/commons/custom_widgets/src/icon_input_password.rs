use gtk::{
    gdk, gio,
    glib::{clone, object::ObjectExt},
    prelude::*,
    subclass::*,
};
use relm4::{
    gtk::{self, GestureClick},
    ComponentParts, RelmWidgetExt, SimpleComponent,
};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct IconInputPasswordCss {
    root_container: Option<Vec<String>>,
    root_container_focused: Option<Vec<String>>,
    container: Option<Vec<String>>,
    icon: Option<Vec<String>>,
}

impl Default for IconInputPasswordCss {
    fn default() -> Self {
        Self {
            root_container: Option::from(vec!["icon-input-root-default".to_string()]),
            root_container_focused: Option::from(vec![
                "icon-input-root-focused-default".to_string()
            ]),
            container: Option::from(vec!["icon-input-container-default".to_string()]),
            icon: Option::from(vec!["icon-input-icon-default".to_string()]),
        }
    }
}

#[derive(Debug)]
pub struct InitSettings {
    pub placeholder: Option<String>,
    pub icon: Option<String>,
    pub toggle_icon: Option<String>,
    pub css: IconInputPasswordCss,
}

#[derive(Debug)]
pub enum InputMessage {
    ToggleViewPassword,
    InputChange(String),
    InputFocusEnter,
    InputFocusLeave,
}

#[derive(Debug)]
pub enum OutputMessage {
    InputChange(String),
}

pub struct IconInputPassword {
    settings: InitSettings,
    is_text_visible: bool,
    is_focused: bool,
}

pub struct ComponentWidgets {
    container_box: gtk::Box,
    icon_image: gtk::Image,
    root: gtk::Box,
    input: gtk::Entry,
}

impl SimpleComponent for IconInputPassword {
    type Input = InputMessage;

    type Output = OutputMessage;

    type Init = InitSettings;

    type Root = gtk::Box;

    type Widgets = ComponentWidgets;

    fn init_root() -> Self::Root {
        let root_box = gtk::Box::builder()
            .hexpand(true)
            .vexpand(false)
            .orientation(gtk::Orientation::Horizontal)
            .build();
        root_box
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
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

        let input = gtk::Entry::builder()
            .hexpand(true)
            .visibility(false)
            .build();

        let event_controller = gtk::EventControllerFocus::builder().build();

        event_controller.connect_enter(clone!(@strong sender => move |_| {
            sender.input(InputMessage::InputFocusEnter);
        }));

        event_controller.connect_leave(clone!(@strong sender => move |_| {
            sender.input(InputMessage::InputFocusLeave);
        }));

        input.add_controller(event_controller);

        input.connect_changed(clone!(@strong sender => move |entry| {
            sender.input(InputMessage::InputChange(entry.text().into()));
        }));

        let icon_image = gtk::Image::builder().hexpand(false).vexpand(false).build();

        match init.placeholder.clone() {
            Some(placeholder) => {
                input.set_placeholder_text(Option::from(placeholder.as_str()));
            }
            None => (),
        }

        root.append(&input);

        let icon = init.icon.clone();
        match icon.to_owned() {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon);
                let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                // let image = gtk::Image::builder()
                //     .paintable(&asset_paintable)
                //     .hexpand(true)
                //     .vexpand(true)
                //     .build();
                icon_image.set_paintable(Option::from(&asset_paintable));

                match init.css.icon.to_owned() {
                    Some(css) => icon_image.set_css_classes(&[css.join(",").as_str()]),
                    None => (),
                };
                root.append(&icon_image);
                let left_click_gesture = GestureClick::builder().button(0).build();
                // left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
                // info!("gesture button pressed is {}", this.current_button());
                //     sender.input_sender().send(InputMessage::Pressed);

                // }));

                left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                        info!("gesture button released is {}", this.current_button());
                        sender.input_sender().send(InputMessage::ToggleViewPassword);

                }));
                icon_image.add_controller(left_click_gesture);
            }
            None => (),
        }

        let model = IconInputPassword {
            settings: init,
            is_text_visible: false,
            is_focused: false,
        };

        let widgets = ComponentWidgets {
            container_box,
            icon_image,
            root: root.clone(),
            input,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        info!("icon button update message {:?}", message);
        match message {
            InputMessage::ToggleViewPassword => {
                self.is_text_visible = !self.is_text_visible;
            }
            InputMessage::InputChange(text) => {
                let _ = sender
                    .output_sender()
                    .send(OutputMessage::InputChange(text));
            }
            InputMessage::InputFocusEnter => {
                self.is_focused = true;
            }
            InputMessage::InputFocusLeave => {
                self.is_focused = false;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: relm4::ComponentSender<Self>) {
        widgets.input.set_visibility(self.is_text_visible);

        match self.is_text_visible {
            true => match self.settings.toggle_icon.to_owned() {
                Some(icon) => {
                    let icon_file = gio::File::for_path(icon);
                    let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                    widgets
                        .icon_image
                        .set_paintable(Option::from(&asset_paintable));
                }
                None => (),
            },
            false => match self.settings.icon.to_owned() {
                Some(icon) => {
                    let icon_file = gio::File::for_path(icon);
                    let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                    widgets
                        .icon_image
                        .set_paintable(Option::from(&asset_paintable));
                }
                None => (),
            },
        }
        match self.settings.css.root_container_focused.to_owned() {
            Some(css) => widgets
                .root
                .set_class_active(&css.join(",").as_str(), self.is_focused),
            None => (),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {
        info!("icon button sutdown called");
    }
}
