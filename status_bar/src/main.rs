use std::time::SystemTime;

use chrono::Local;
use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, GtkWindowExt},
};
use relm4::gtk::prelude::ObjectExt;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

mod settings;
mod theme;
use tracing::{error, info};
pub mod errors;

use crate::settings::StatusBarSettings;
use crate::theme::StatusBarTheme;
// #[allow(non_snake_case)]
// pub mod networkmanager {
//     tonic::include_proto!("networkmanager");
// }

/// # Status Bar state
///
/// This struct is the state definition of the entire application
struct StatusBar {
    settings: StatusBarSettings,
    current_time: String,
    custom_theme: StatusBarTheme,
    wifi_state: WifiState,
    bluetooth_state: BluetoothState,
    battery_state: BatteryState,
}

#[derive(Debug, Clone, Copy)]
pub enum WifiConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Debug, Clone, Copy)]
pub enum WifiState {
    On,
    Off,
    Connected(WifiConnectedState),
}

#[derive(Debug, Clone, Copy)]
pub enum BluetoothState {
    On,
    Off,
    Connected,
}

#[derive(Debug, Clone, Copy)]
pub enum BatteryState {
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    TimeTick(String),
    WifiStateUpdate(WifiState),
    BluetoothStateUpdate(BluetoothState),
    BatteryStatusUpdate(BatteryState),
}

struct AppWidgets {
    clock_label: gtk::Label,
    wifi_image: gtk::Image,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: StatusBarSettings) -> gtk::Window {
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
fn init_window(settings: StatusBarSettings) -> gtk::Window {
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

    // Push other windows out of the way
    gtk4_layer_shell::auto_exclusive_zone_enable(&window);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);

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

impl SimpleComponent for StatusBar {
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
            Err(_) => StatusBarSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for status bar: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => StatusBarTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for status bar: {:?}", custom_theme
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
            Err(_) => StatusBarSettings::default(),
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => StatusBarTheme::default(),
        };

        let modules = settings.modules.clone();

        let model = StatusBar {
            settings: settings.clone(),
            custom_theme,
            current_time: current_time(modules.clock.format.as_str()),
            wifi_state: WifiState::Off,
            bluetooth_state: BluetoothState::Off,
            battery_state: BatteryState::Level0,
        };

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["container"])
            .build();

        let left_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["left"])
            .build();

        let center_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .css_classes(["center"])
            .build();

        let right_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .halign(gtk::Align::End)
            .css_classes(["right"])
            .spacing(16)
            .build();

        let formatted_time = current_time(modules.clock.format.as_str());
        let clock_label = gtk::Label::new(Some(&formatted_time));
        clock_label.set_class_active("clock", true);

        let wifi_file = gio::File::for_path("src/assets/pngs/wifi_strong.png");
        let wifi_asset_paintable = gdk::Texture::from_file(&wifi_file).unwrap();
        let wifi_image = gtk::Image::builder()
            .paintable(&wifi_asset_paintable)
            .build();

        let bluetooth_file = gio::File::for_path("src/assets/pngs/bluetooth_connected.png");
        let bluetooth_asset_paintable = gdk::Texture::from_file(&bluetooth_file).unwrap();
        let bluetooth_image = gtk::Image::builder()
            .paintable(&bluetooth_asset_paintable)
            .build();

        let battery_file = gio::File::for_path("src/assets/pngs/battery_70.png");
        let battery_asset_paintable = gdk::Texture::from_file(&battery_file).unwrap();
        let battery_image = gtk::Image::builder()
            .paintable(&battery_asset_paintable)
            .build();

        let layout = settings.clone().layout;

        let window_title_label = gtk::Label::new(Some(&""));

        //Generate left layout
        for layout_item in layout.left.iter() {
            match layout_item.as_str() {
                "clock" => {
                    left_box.append(&clock_label);
                }
                "wifi" => {
                    left_box.append(&wifi_image);
                }
                "bluetooth" => {
                    left_box.append(&bluetooth_image);
                }
                "battery" => {
                    left_box.append(&battery_image);
                }
                "window_title" => {
                    left_box.append(&window_title_label);
                }
                _ => println!("Component not found for: {}", layout_item),
            }
        }

        //Generate center layout
        for layout_item in layout.center.iter() {
            match layout_item.as_str() {
                "clock" => {
                    center_box.append(&clock_label);
                }
                "wifi" => {
                    center_box.append(&wifi_image);
                }
                "bluetooth" => {
                    center_box.append(&bluetooth_image);
                }
                "battery" => {
                    center_box.append(&battery_image);
                }
                "window_title" => {
                    center_box.append(&window_title_label);
                }
                _ => println!("Component not found for: {}", layout_item),
            }
        }

        //Generate right layout
        for layout_item in layout.right.iter() {
            match layout_item.as_str() {
                "clock" => {
                    right_box.append(&clock_label);
                }
                "wifi" => {
                    right_box.append(&wifi_image);
                }
                "bluetooth" => {
                    right_box.append(&bluetooth_image);
                }
                "battery" => {
                    right_box.append(&battery_image);
                }
                "window_title" => {
                    right_box.append(&window_title_label);
                }
                _ => println!("Component not found for: {}", layout_item),
            }
        }

        main_box.append(&left_box);
        main_box.append(&center_box);
        main_box.append(&right_box);

        window.set_child(Some(&main_box));

        let widgets = AppWidgets {
            clock_label,
            wifi_image,
        };

        // we are using a closure to capture the label (else we could also use a normal function)
        let tick = move || {
            sender.input_sender().send(Message::TimeTick(current_time(
                modules.clock.format.as_str(),
            )));

            let wifi_states = vec![
                WifiState::Off,
                WifiState::On,
                WifiState::Connected(WifiConnectedState::Low),
                WifiState::Connected(WifiConnectedState::Weak),
                WifiState::Connected(WifiConnectedState::Good),
                WifiState::Connected(WifiConnectedState::Strong),
            ];
            let x = get_sys_time_in_secs() as usize % wifi_states.len();
            sender
                .input_sender()
                .send(Message::WifiStateUpdate(wifi_states[x]));
            // we could return glib::ControlFlow::Break to stop our clock after this tick
            glib::ControlFlow::Continue
        };

        // executes the closure once every second
        glib::timeout_add_seconds_local(1, tick);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::TimeTick(local_time) => {
                self.current_time = local_time;
            }
            Message::WifiStateUpdate(strength) => {
                self.wifi_state = strength;
            }
            Message::BluetoothStateUpdate(state) => {
                self.bluetooth_state = state;
            }
            Message::BatteryStatusUpdate(state) => {
                self.battery_state = state;
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets
            .clock_label
            .set_label(&format!("{}", self.current_time));

        let modules = self.settings.modules.clone();

        match self.wifi_state {
            WifiState::Off => {
                if let Some(icon) = modules.wifi.icon.off.clone() {
                    info!("wifi icon path is {}", icon);
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::On => {
                if let Some(icon) = modules.wifi.icon.off.clone() {
                    info!("wifi icon path is {}", icon);
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::Connected(WifiConnectedState::Low) => {
                if let Some(icon) = modules.wifi.icon.low.clone() {
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::Connected(WifiConnectedState::Weak) => {
                if let Some(icon) = modules.wifi.icon.weak.clone() {
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::Connected(WifiConnectedState::Good) => {
                if let Some(icon) = modules.wifi.icon.good.clone() {
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::Connected(WifiConnectedState::Strong) => {
                if let Some(icon) = modules.wifi.icon.strong.clone() {
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
        }
    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_status_bar=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("status.bar");
    relm4::set_global_css_from_file("src/assets/css/style.css");
    app.run::<StatusBar>(());
}

fn current_time(format_string: &str) -> String {
    format!("{}", Local::now().format(format_string))
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 1,
    }
}
