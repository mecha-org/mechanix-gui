use gtk::gdk::Display;
use gtk::{
    gdk, gio,
    glib::{clone, object::ObjectExt},
    prelude::*,
    subclass::*,
};
use relm4::gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
use relm4::{
    gtk::{self, CssProvider, GestureClick},
    ComponentParts, RelmWidgetExt, SimpleComponent,
};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct IconInputCss {
    root_container: Option<Vec<String>>,
    root_container_focused: Option<Vec<String>>,
    container: Option<Vec<String>>,
    icon: Option<Vec<String>>,
}

impl Default for IconInputCss {
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

#[derive(Default, Debug, Clone)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct IconSettings {
    pub path: String,
    pub position: IconPosition,
}

#[derive(Debug, Clone)]
pub struct InitSettings {
    pub placeholder: Option<String>,
    pub icon: Option<IconSettings>,
    pub clear_icon: Option<IconSettings>,
    pub css: IconInputCss,
}

#[derive(Debug)]
pub enum InputMessage {
    InputChange(String),
    InputFocusEnter,
    InputFocusLeave,
    Clear,
}

#[derive(Debug)]
pub enum OutputMessage {
    InputChange(String),
}

pub struct IconInput {
    settings: InitSettings,
    view_password: bool,
    is_focused: bool,
    input: gtk::Entry,
}

pub struct ComponentWidgets {
    container_box: gtk::Box,
    icon_image: gtk::Image,
    root: gtk::Box,
    clear_icon_image: gtk::Image
}

impl SimpleComponent for IconInput {
    type Input = InputMessage;

    type Output = OutputMessage;

    type Init = InitSettings;

    type Root = gtk::Box;

    type Widgets = ComponentWidgets;

    fn init_root() -> Self::Root {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("assets/css/style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let root_box = gtk::Box::builder()
            .hexpand(true)
            .vexpand(false)
            .orientation(gtk::Orientation::Horizontal)
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

        let input = gtk::Entry::builder().hexpand(true).build();

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

        match init.icon.clone() {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon.path);
                let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                icon_image.set_paintable(Option::from(&asset_paintable));
                match init.css.icon.to_owned() {
                    Some(css) => icon_image.set_css_classes(&[css.join(",").as_str()]),
                    None => (),
                };
                match icon.position {
                    IconPosition::Left => root.prepend(&icon_image),
                    IconPosition::Right => root.append(&icon_image),
                };
            }
            None => (),
        }

        let clear_icon_image = gtk::Image::builder()
        .visible(false)
        .hexpand(false)
        .vexpand(false)
        .build();
        match init.clear_icon.clone() {
            Some(icon) => {
                let icon_file = gio::File::for_path(icon.path);
                let asset_paintable = gdk::Texture::from_file(&icon_file).unwrap();
                clear_icon_image.set_paintable(Option::from(&asset_paintable));
                match init.css.icon.to_owned() {
                    Some(css) => clear_icon_image.set_css_classes(&[css.join(",").as_str()]),
                    None => (),
                };
                match icon.position {
                    IconPosition::Left => root.prepend(&clear_icon_image),
                    IconPosition::Right => root.append(&clear_icon_image),
                };
                let left_click_gesture = GestureClick::builder().button(0).build();

                left_click_gesture.connect_released(clone!(@strong sender => move |this, _, _,_| {
                        sender.input_sender().send(InputMessage::Clear);

                }));
                clear_icon_image.add_controller(left_click_gesture);
            }
            None => (),
        }

        let model = IconInput {
            settings: init,
            view_password: false,
            is_focused: false,
            input,
        };

        let widgets = ComponentWidgets {
            container_box,
            icon_image,
            root: root.clone(),
            clear_icon_image
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        info!("icon button update message {:?}", message);
        match message {
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
            InputMessage::Clear => {
                self.input.set_text("");
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: relm4::ComponentSender<Self>) {
        match self.settings.css.root_container_focused.to_owned() {
            Some(css) => widgets
                .root
                .set_class_active(&css.join(",").as_str(), self.is_focused),
            None => (),
        }
        widgets.clear_icon_image.set_visible(self.is_focused);
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {
        info!("icon button sutdown called");
    }
}
