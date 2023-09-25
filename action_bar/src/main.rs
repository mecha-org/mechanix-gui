use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
};
use relm4::{gtk::prelude::ObjectExt, RelmSetChildExt};
use relm4::{
    gtk::{self, glib::clone},
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

mod settings;
mod theme;
use tracing::{error, info};
pub mod errors;

use crate::settings::ActionBarSettings;
use crate::theme::ActionBarTheme;

/// # Action bar state
///
/// This struct is the state definition of the entire application
struct ActionBar {
    settings: ActionBarSettings,
    custom_theme: ActionBarTheme,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SettingsPressed,
    HomePressed,
    KeyBoardPressed,
}

struct AppWidgets {}

impl SimpleComponent for ActionBar {
    /// The type of the messages that this component can receive.
    type Input = Message;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ActionBarSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for action bar: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => ActionBarTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for action bar: {:?}", custom_theme
        );

        let window_settings = settings.window;
        let window = gtk::Window::builder()
            .title(settings.title)
            .default_width(window_settings.size.0)
            .default_height(window_settings.size.1)
            .css_classes(["window"])
            .build();
        window
    }

    /// Initialize the UI and model.
    fn init(
        _: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ActionBarSettings::default(),
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => ActionBarTheme::default(),
        };

        let modules = settings.modules.clone();

        let model = ActionBar {
            settings: settings.clone(),
            custom_theme,
        };

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["container"])
            .build();

        let layout = settings.clone().layout;
        let layout_items_merged = [
            layout.left.clone(),
            layout.center.clone(),
            layout.right.clone(),
        ];

        layout_items_merged
            .concat()
            .into_iter()
            .enumerate()
            .for_each(|(index, item)| {
                let mut icon_path: Option<String> = None;
                let mut message: Option<Message> = None;

                if item.to_lowercase() == modules.settings.title.to_lowercase() {
                    match modules.settings.icon.clone() {
                        Some(icon) => {
                            icon_path = Some(icon);
                            message = Some(Message::SettingsPressed);
                        }
                        None => (),
                    }
                } else if item == modules.home.title {
                    match modules.home.icon.clone() {
                        Some(icon) => {
                            icon_path = Some(icon);
                            message = Some(Message::HomePressed);
                        }
                        None => (),
                    }
                } else if item == modules.keyboard.title {
                    match modules.keyboard.icon.clone() {
                        Some(icon) => {
                            icon_path = Some(icon);
                            message = Some(Message::KeyBoardPressed);
                        }
                        None => (),
                    }
                }

                match icon_path {
                    Some(icon) => {
                        let is_hexpand = index != layout_items_merged.len() - 1;

                        info!("is_hexpand {}", is_hexpand);
                        let c_box = gtk::Box::builder()
                            .hexpand(is_hexpand)
                            .orientation(gtk::Orientation::Horizontal)
                            .build();

                        let image = generate_image(icon);
                        let action_button = gtk::Button::builder()
                            .child(&image)
                            .css_classes(["action-button"])
                            .build();

                        match message {
                            Some(m) => {
                                action_button.connect_clicked(clone!(@strong sender => move |_| {
                                    sender.input(m.clone());
                                }));
                            }
                            None => (),
                        }
                        c_box.append(&action_button);
                        main_box.append(&c_box)
                    }
                    None => todo!(),
                }
            });

        window.set_child(Some(&main_box));

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        info!("update message is {:?}", message);
        match message {
            Message::SettingsPressed => {}
            Message::HomePressed => {}
            Message::KeyBoardPressed => {}
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_action_bar=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("action.bar");
    relm4::set_global_css_from_file("src/assets/css/style.css");
    app.run::<ActionBar>(());
}

pub fn generate_image(path: String) -> gtk::Image {
    let file = gio::File::for_path(path);
    let asset_paintable = gdk::Texture::from_file(&file).unwrap();
    let image = gtk::Image::builder()
        .paintable(&asset_paintable)
        .css_classes(["action-img"])
        .build();
    image
}
