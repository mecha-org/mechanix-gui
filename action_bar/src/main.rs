use echo_client::EchoClient;
use event_handler::zbus::ZbusServiceHandle;
use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, ButtonExt, GtkWindowExt},
};
use modules::input::handler::InputServiceHandle;
use process::ChildProcessManager;
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    gtk::prelude::{ObjectExt, WidgetExt},
    AsyncComponentSender, RelmSetChildExt,
};
use relm4::{
    gtk::{self, glib::clone},
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt,
};

mod settings;
mod theme;
use tracing::{error, info};
pub mod errors;
mod event_handler;
mod modules;

use crate::settings::ActionBarSettings;
use crate::theme::ActionBarTheme;

/// # Action bar state
///
/// This struct is the state definition of the entire application
struct ActionBar {
    settings: ActionBarSettings,
    custom_theme: ActionBarTheme,
    window: gtk::Window,
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
    Show,
    Hide,
    ShowKeyboard,
    HideKeyboard,
}

struct AppWidgets {}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: ActionBarSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

#[cfg(feature = "layer-shell")]
fn init_window(settings: ActionBarSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Overlay);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    match window_settings.layer_shell.margin.top {
        Some(v) => {
            gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, v);
        }
        None => (),
    }
    match window_settings.layer_shell.margin.right {
        Some(v) => {
            gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, v);
        }
        None => (),
    }
    match window_settings.layer_shell.margin.bottom {
        Some(v) => {
            gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, v);
        }
        None => (),
    }
    match window_settings.layer_shell.margin.left {
        Some(v) => {
            gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, v);
        }
        None => (),
    }

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    match window_settings.layer_shell.anchor.top {
        Some(v) => {
            gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Top, v);
        }
        None => (),
    }
    match window_settings.layer_shell.anchor.right {
        Some(v) => {
            gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Right, v);
        }
        None => (),
    }
    match window_settings.layer_shell.anchor.bottom {
        Some(v) => {
            gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Bottom, v);
        }
        None => (),
    }
    match window_settings.layer_shell.anchor.left {
        Some(v) => {
            gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Left, v);
        }
        None => (),
    }

    window
}

#[async_trait(?Send)]
impl AsyncComponent for ActionBar {
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

    type CommandOutput = Message;

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

        let window = init_window(settings);
        window
    }

    /// Initialize the UI and model.
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => ActionBarSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => ActionBarTheme::default(),
        };

        let modules = settings.modules.clone();

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

        let model = ActionBar {
            settings: settings.clone(),
            custom_theme,
            window,
        };

        let widgets = AppWidgets {};

        let sender: relm4::Sender<Message> = sender.input_sender().clone();

        init_services(settings, sender).await;

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        info!("update message is {:?}", message);
        match message {
            Message::SettingsPressed => {}
            Message::HomePressed => {}
            Message::KeyBoardPressed => {}
            Message::Show => {
                self.window.set_visible(true);
            }
            Message::Hide => {
                self.window.set_visible(false);
            }
            Message::ShowKeyboard => {}
            Message::HideKeyboard => {}
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {}
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command_index = args.iter().position(|arg| arg == "-cmd");
    match command_index {
        Some(index) => match args.get(index + 1) {
            Some(cmd) => {
                match EchoClient::echo(
                    "org.mechanics.ActionBar",
                    "/org/mechanics/ActionBar",
                    "org.mechanics.ActionBar",
                    cmd,
                )
                .await
                {
                    Ok(r) => {
                        println!("echo success");
                    }
                    Err(e) => {
                        println!("echo failed {}", e);
                    }
                };
                return;
            }
            None => (),
        },
        None => (),
    }

    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_action_bar=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("action.bar").with_args(vec![]);
    app.run_async::<ActionBar>(());
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

async fn init_services(settings: ActionBarSettings, sender: relm4::Sender<Message>) {
    let mut zbus_service_handle = ZbusServiceHandle::new();
    let sender_clone_1 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting zbus service");
        zbus_service_handle.run(sender_clone_1).await;
    });

    //this service is listening for keyboard events
    let mut window_manager_service_handle = InputServiceHandle::new();
    let sender_clone_2 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting input service");
        window_manager_service_handle.run(sender_clone_2).await;
    });

    let mut child_process_manager_service_handle = ChildProcessManager::new();
    let sender_clone_3 = sender;
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting input service");
        child_process_manager_service_handle
            .spawn("wvkbd-mobintl", &[])
            .await;
    });
}
