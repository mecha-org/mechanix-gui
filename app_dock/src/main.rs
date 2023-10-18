use gtk::{
    gdk, gio,
    glib::clone,
    prelude::{BoxExt, GtkWindowExt},
};
use relm4::{factory::FactoryVecDeque, gtk::prelude::ObjectExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

mod settings;
mod theme;
mod widgets;
use tracing::{error, info};
use widgets::app::{App, AppSettings, Message as AppMessage};
pub mod errors;

use crate::settings::AppDockSettings;
use crate::theme::AppDockTheme;

/// # App Dock state
///
/// This struct is the state definition of the entire application
struct AppDock {
    settings: AppDockSettings,
    custom_theme: AppDockTheme,
    pinned_apps: FactoryVecDeque<App>,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppClicked(String),
    AppClickedReleased(String),
    HomePressed,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: AppDockSettings) -> gtk::Window {
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
fn init_window(settings: AppDockSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .decorated(false)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Bottom);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 24);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, false),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window
}

struct AppWidgets {}

impl SimpleComponent for AppDock {
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
            Err(_) => AppDockSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for App Dock: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppDockTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for App Dock: {:?}", custom_theme
        );

        let window = init_window(settings);

        window.set_resizable(false);

        info!("window height is {}", window.default_height());
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
            Err(_) => AppDockSettings::default(),
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppDockTheme::default(),
        };

        let modules = settings.modules.clone();

        let mut pinned_apps: FactoryVecDeque<App> = FactoryVecDeque::builder(
            gtk::Box::builder()
                .valign(gtk::Align::Start)
                .halign(gtk::Align::Start)
                .spacing(14)
                .css_classes(["apps-list"])
                .build(),
        )
        .launch()
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {

                info!("app widget forwarded message is {:?}", msg);
                return match msg {
                AppMessage::WidgetPressed(key) => {
                    if key == modules.home.title {
                        Message::HomePressed
                    }  else {
                        return Message::AppClicked(key);
                    }
                },
                AppMessage::WidgetLongPressed(key) => {
                    if key == modules.home.title {
                        Message::HomePressed
                    }  else {
                        return Message::AppClicked(key);
                    }
                },
            }}),
        );

        modules.pinned_apps.into_iter().for_each(|pinned_app| {
            pinned_apps.guard().push_back(AppSettings {
                app_id: pinned_app.app_id,
                title: pinned_app.title,
                alias: pinned_app.alias,
                icon: pinned_app.icon,
            });
        });

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["container"])
            .build();

        container.append(pinned_apps.widget());

        window.set_child(Some(&container));

        let model = AppDock {
            settings,
            custom_theme,
            pinned_apps,
        };

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        //info!("Update message is {:?}", message);
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_app_dock=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => AppDockSettings::default(),
    };
    let app = RelmApp::new("app.dock");
    relm4::set_global_css_from_file("src/assets/css/style.css");
    app.run::<AppDock>(());
}
