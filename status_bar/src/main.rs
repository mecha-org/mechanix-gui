use chrono::Local;
use custom_utils::get_image_from_path;
use echo_client::EchoClient;
use event_handler::zbus::ZbusServiceHandle;

use gtk::gdk;
use gtk::{
    ffi,
    gdk::{
        prelude::{DisplayExt, MonitorExt, SurfaceExt},
        Display,
    },
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
};
use modules::{
    battery::handler::BatteryServiceHandle, bluetooth::handler::BluetoothServiceHandle,
    clock::handler::ClockServiceHandle, window::handler::WindowServiceHandle,
    wireless::handler::WirelessServiceHandle,
};
use process::is_app_already_running;
use relm4::{async_trait::async_trait, gtk, tokio, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    AsyncComponentSender,
};

mod settings;
mod theme;
use tracing::info;
pub mod errors;
mod event_handler;
mod modules;

use crate::settings::StatusBarSettings;
use crate::theme::StatusBarTheme;
// #[allow(non_snake_case)]
// pub mod networkmanager {
//     tonic::include_proto!("networkmanager");
// }

/// # Status Bar state
///
/// This struct is the state definition of the entire application
pub struct StatusBar {
    pub settings: StatusBarSettings,
    pub current_time: String,
    pub custom_theme: StatusBarTheme,
    pub wireless_state: WirelessState,
    pub bluetooth_state: BluetoothState,
    pub battery_state: BatteryState,
    pub window: gtk::Window,
    pub current_window_title: String,
}

#[derive(Debug, Clone, Copy)]
pub enum WirelessConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum WirelessState {
    On,
    #[default]
    Off,
    Connected(WirelessConnectedState),
    NotFound,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BluetoothState {
    On,
    #[default]
    Off,
    Connected,
    NotFound,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BatteryState {
    #[default]
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
    NotFound,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    TimeTick(String),
    WirelessStateUpdate(WirelessState),
    BluetoothStateUpdate(BluetoothState),
    BatteryStatusUpdate(BatteryState),
    CurrentWindowTitleUpdate(String),
    TopLevelActiveUpdate(bool),
    Show,
    Hide,
}

pub struct AppWidgets {
    pub clock_label: gtk::Label,
    pub wireless_image: gtk::Image,
    pub bluetooth_image: gtk::Image,
    pub battery_image: gtk::Image,
    pub current_window_title_label: gtk::Label,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: StatusBarSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .css_classes(["window"])
        .build();

    match window_settings.width {
        Some(v) => {
            window.set_default_width(v);
        }
        None => {
            window.set_hexpand(true);
        }
    }

    match window_settings.height {
        Some(v) => {
            window.set_default_height(v);
        }
        None => {
            window.set_vexpand(true);
        }
    }
    window
}

#[cfg(feature = "layer-shell")]
fn init_window(settings: StatusBarSettings) -> gtk::Window {
    use relm4::gtk::{
        gdk::DisplayManager,
        prelude::{NativeExt, SurfaceExt},
    };

    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .css_classes(["window"])
        .build();

    match window_settings.width {
        Some(v) => {
            window.set_default_width(v);
        }
        None => {}
    }

    match window_settings.height {
        Some(v) => {
            window.set_default_height(v);
            // Push other windows out of the way
            gtk4_layer_shell::set_exclusive_zone(&window, v);
        }
        None => {}
    }

    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    // gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Overlay);

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
impl AsyncComponent for StatusBar {
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
        info!("init_root started");
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

        init_window(settings)
    }

    /// Initialize the UI and model.
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => StatusBarSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => StatusBarTheme::default(),
        };

        let modules = settings.modules.clone();

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
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

        let formatted_time = String::from("");
        let clock_label = gtk::Label::new(Some(&formatted_time));
        clock_label.set_class_active("clock", true);

        let wireless_image = get_image_from_path(modules.wireless.icon.strong, &["icon"]);
        let bluetooth_image = get_image_from_path(modules.bluetooth.icon.connected, &["icon"]);
        let battery_image = get_image_from_path(modules.battery.icon.level_70, &["icon"]);

        let layout = settings.clone().layout;

        let current_window_title = String::from("");
        let current_window_title_label = gtk::Label::builder()
            .label(&current_window_title)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        //Generate left layout
        for layout_item in layout.left.iter() {
            match layout_item.as_str() {
                "clock" => {
                    left_box.append(&clock_label);
                }
                "wireless" => {
                    left_box.append(&wireless_image);
                }
                "bluetooth" => {
                    left_box.append(&bluetooth_image);
                }
                "battery" => {
                    left_box.append(&battery_image);
                }
                "window_title" => {
                    left_box.append(&current_window_title_label);
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
                "wireless" => {
                    center_box.append(&wireless_image);
                }
                "bluetooth" => {
                    center_box.append(&bluetooth_image);
                }
                "battery" => {
                    center_box.append(&battery_image);
                }
                "window_title" => {
                    center_box.append(&current_window_title_label);
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
                "wireless" => {
                    right_box.append(&wireless_image);
                }
                "bluetooth" => {
                    right_box.append(&bluetooth_image);
                }
                "battery" => {
                    right_box.append(&battery_image);
                }
                "window_title" => {
                    right_box.append(&current_window_title_label);
                }
                _ => println!("Component not found for: {}", layout_item),
            }
        }

        main_box.append(&left_box);
        main_box.append(&center_box);
        main_box.append(&right_box);

        window.set_child(Some(&main_box));

        let model = StatusBar {
            settings: settings.clone(),
            custom_theme,
            current_time: formatted_time,
            wireless_state: WirelessState::default(),
            bluetooth_state: BluetoothState::default(),
            battery_state: BatteryState::default(),
            window,
            current_window_title,
        };

        let widgets = AppWidgets {
            clock_label,
            wireless_image,
            bluetooth_image,
            battery_image,
            current_window_title_label,
        };

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
        info!("Update message is {:?}", message);
        match message {
            Message::TimeTick(time) => {
                self.current_time = time;
            }
            Message::WirelessStateUpdate(state) => {
                self.wireless_state = state;
            }
            Message::BluetoothStateUpdate(state) => {
                self.bluetooth_state = state;
            }
            Message::BatteryStatusUpdate(state) => {
                self.battery_state = state;
            }
            Message::Show => {
                self.window.set_visible(true);
            }
            Message::Hide => {
                self.window.set_visible(false);
            }
            Message::CurrentWindowTitleUpdate(window_title) => {
                self.current_window_title = window_title;
            }
            Message::TopLevelActiveUpdate(is_active) => {
                self.window.set_class_active("window-active", is_active);
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        widgets
            .clock_label
            .set_label(&self.current_time.to_string());

        let modules = self.settings.modules.clone();
        info!("wireless state is {:?}", self.wireless_state);
        match self.wireless_state {
            WirelessState::NotFound => {
                if let Some(icon) = modules.wireless.icon.not_found.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::Off => {
                if let Some(icon) = modules.wireless.icon.off.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::On => {
                if let Some(icon) = modules.wireless.icon.on.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::Connected(WirelessConnectedState::Low) => {
                if let Some(icon) = modules.wireless.icon.low.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::Connected(WirelessConnectedState::Weak) => {
                if let Some(icon) = modules.wireless.icon.weak.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::Connected(WirelessConnectedState::Good) => {
                if let Some(icon) = modules.wireless.icon.good.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
            WirelessState::Connected(WirelessConnectedState::Strong) => {
                if let Some(icon) = modules.wireless.icon.strong.clone() {
                    widgets.wireless_image.set_file(Some(&icon));
                }
            }
        }

        match self.bluetooth_state {
            BluetoothState::NotFound => {
                if let Some(icon) = modules.bluetooth.icon.not_found.clone() {
                    widgets.bluetooth_image.set_file(Some(&icon));
                }
            }
            BluetoothState::Off => {
                if let Some(icon) = modules.bluetooth.icon.off.clone() {
                    widgets.bluetooth_image.set_file(Some(&icon));
                }
            }
            BluetoothState::On => {
                if let Some(icon) = modules.bluetooth.icon.on.clone() {
                    widgets.bluetooth_image.set_file(Some(&icon));
                }
            }
            BluetoothState::Connected => {
                if let Some(icon) = modules.bluetooth.icon.connected.clone() {
                    widgets.bluetooth_image.set_file(Some(&icon));
                }
            }
        }

        match self.battery_state {
            BatteryState::NotFound => {
                if let Some(icon) = modules.battery.icon.not_found {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level0 => {
                if let Some(icon) = modules.battery.icon.level_0 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level10 => {
                if let Some(icon) = modules.battery.icon.level_10 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level20 => {
                if let Some(icon) = modules.battery.icon.level_20 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level30 => {
                if let Some(icon) = modules.battery.icon.level_30 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level40 => {
                if let Some(icon) = modules.battery.icon.level_40 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level50 => {
                if let Some(icon) = modules.battery.icon.level_50 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level60 => {
                if let Some(icon) = modules.battery.icon.level_60 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level70 => {
                if let Some(icon) = modules.battery.icon.level_70 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level80 => {
                if let Some(icon) = modules.battery.icon.level_80 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level90 => {
                if let Some(icon) = modules.battery.icon.level_90 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level100 => {
                if let Some(icon) = modules.battery.icon.level_100 {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
        }

        widgets
            .current_window_title_label
            .set_label(&self.current_window_title);
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        println!("update_cmd message {:?}", message);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command_index = args.iter().position(|arg| arg == "-cmd");
    match command_index {
        Some(index) => match args.get(index + 1) {
            Some(cmd) => {
                match EchoClient::echo(
                    "org.mechanics.StatusBar",
                    "/org/mechanics/StatusBar",
                    "org.mechanics.StatusBar",
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
        .with_env_filter("mecha_status_bar=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("mecha.status.bar").with_args(vec![]);
    app.run_async::<StatusBar>(());
}

async fn init_services(settings: StatusBarSettings, sender: relm4::Sender<Message>) {
    let mut zbus_service_handle = ZbusServiceHandle::new();
    let sender_clone_1 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting zbus service");
        zbus_service_handle.run(sender_clone_1).await;
    });

    let mut wireless_service_handle = WirelessServiceHandle::new();
    let sender_clone_2 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting wireless service");
        wireless_service_handle.run(sender_clone_2).await;
    });

    let mut bluetooth_service_handle = BluetoothServiceHandle::new();
    let sender_clone_3 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting bluetooth service");
        bluetooth_service_handle.run(sender_clone_3).await;
    });

    let mut battery_service_handle = BatteryServiceHandle::new();
    let sender_clone_4 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting battery service");
        battery_service_handle.run(sender_clone_4).await;
    });

    let mut clock_service_handle = ClockServiceHandle::new();
    let sender_clone_4 = sender.clone();
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting clock");
        clock_service_handle
            .run(settings.modules.clock.format, sender_clone_4)
            .await;
    });

    let mut window_manager_service_handle = WindowServiceHandle::new();
    let sender_clone_5 = sender;
    let _ = relm4::spawn_local(async move {
        info!(task = "init_services", "Starting window manager service");
        window_manager_service_handle.run(sender_clone_5).await;
    });
}
