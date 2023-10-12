use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, GtkWindowExt},
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::{
    gtk::{
        glib::clone,
        prelude::{EditableExt, EditableExtManual, EntryExt, ObjectExt},
    },
    RelmRemoveAllExt,
};

mod settings;
mod theme;
use settings::App;
use tracing::{error, info};
pub mod errors;

use crate::settings::AppDrawerSettings;
use crate::theme::AppDrawerTheme;

/// # AppDrawer State
///
/// This struct is the state definition of the entire application
struct AppDrawer {
    settings: AppDrawerSettings,
    custom_theme: AppDrawerTheme,
    search_text: String,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SearchTextChanged(String),
}

struct AppWidgets {
    apps_grid: gtk::FlowBox,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: AppDrawerSettings) -> gtk::Window {
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
fn init_window(settings: AppDrawerSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, true),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window
}

impl SimpleComponent for AppDrawer {
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
            Err(_) => AppDrawerSettings::default(),
        };

        info!(
            task = "init_settings",
            "settings initialized for app drawer {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppDrawerTheme::default(),
        };

        info!(
            task = "init_theme",
            "theme initialized for app drawer {:?}", custom_theme
        );

        let window = init_window(settings);

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
            Err(_) => AppDrawerSettings::default(),
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppDrawerTheme::default(),
        };

        let modules = settings.modules.clone();

        let model = AppDrawer {
            settings: settings.clone(),
            custom_theme,
            search_text: String::from(""),
        };

        let container_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["container"])
            .build();

        let search_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["search-box"])
            .build();

        let search_input = gtk::Entry::builder()
            .placeholder_text("Seach Application")
            .hexpand(true)
            .css_classes(["search-input", "search-input:focus"])
            .focusable(false)
            .build();

        search_input.connect_changed(clone!(@strong sender => move |entry| {
            entry.grab_focus_without_selecting();
            let new_name: String = entry.text().into();
            sender.input(Message::SearchTextChanged(new_name));
        }));

        let search_icon_file = gio::File::for_path("src/assets/pngs/search_icon.png");
        let search_icon_paintable = gdk::Texture::from_file(&search_icon_file).unwrap();
        let search_icon = gtk::Image::builder()
            .paintable(&search_icon_paintable)
            .css_classes(["search-icon"])
            .build();

        search_box.append(&search_icon);
        search_box.append(&search_input);

        container_box.append(&search_box);

        let apps_grid = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(30)
            .min_children_per_line(4)
            .selection_mode(gtk::SelectionMode::None)
            .row_spacing(5)
            .build();

        modules.apps.into_iter().for_each(|app| {
            let widget = generate_apps_ui(app);
            apps_grid.insert(&widget, -1);
        });

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .child(&apps_grid)
            .build();

        container_box.append(&scrolled_window);

        window.set_child(Some(&container_box));

        let widgets = AppWidgets { apps_grid };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::SearchTextChanged(term) => {
                self.search_text = term;
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.apps_grid.remove_all();
        self.settings
            .clone()
            .modules
            .apps
            .into_iter()
            .filter(|app| app.name.to_lowercase().starts_with(&self.search_text))
            .for_each(|app| {
                let widget = generate_apps_ui(app);
                widgets.apps_grid.insert(&widget, -1);
            });
    }
}

/// Initialize the application with settings, and starts
fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_app_drawer=trace")
        .with_thread_names(true)
        .init();

    let app = RelmApp::new("app.drawer");
    relm4::set_global_css_from_file("src/assets/css/style.css");
    app.run::<AppDrawer>(());
}

fn generate_apps_ui(app: App) -> gtk::Box {
    let app_name_label = gtk::Label::new(Some(&app.name));
    let app_icon_file = gio::File::for_path(app.icon);
    let app_icon_paintable = gdk::Texture::from_file(&app_icon_file).unwrap();
    let app_icon = gtk::Image::builder()
        .paintable(&app_icon_paintable)
        .css_classes(["app-image"])
        .build();
    let app_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["app"])
        .build();
    app_box.append(&app_icon);
    app_box.append(&app_name_label);
    app_box
}
