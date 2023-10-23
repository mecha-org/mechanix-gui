use std::{default, time::SystemTime};

use anyhow::bail;
use chrono::Local;
use custom_utils::get_image_from_path;
use grpc::battery_client::BatteryManagerClient;
use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
};
use relm4::{
    async_trait::async_trait, gtk, tokio, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt,
    SimpleComponent,
};
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    gtk::prelude::ObjectExt,
    AsyncComponentSender,
};

mod settings;
mod theme;
use tracing::{error, info};
pub mod errors;

mod grpc;
use crate::errors::{StatusBarError, StatusBarErrorCodes};
use crate::grpc::bluetooth_client::BluetoothManagerClient;
use crate::grpc::network_client::NetworkManagerClient;
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
    window: gtk::Window,
}

#[derive(Debug, Clone, Copy)]
pub enum WifiConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum WifiState {
    On,
    #[default]
    Off,
    Connected(WifiConnectedState),
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BluetoothState {
    On,
    #[default]
    Off,
    Connected,
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
    bluetooth_image: gtk::Image,
    battery_image: gtk::Image,
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
    // gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Overlay);

    // Push other windows out of the way
    gtk4_layer_shell::set_exclusive_zone(&window, window_settings.size.1);

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
        (gtk4_layer_shell::Edge::Top, true),
        (gtk4_layer_shell::Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
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
    type CommandOutput = ();

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
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => StatusBarSettings::default(),
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => StatusBarTheme::default(),
        };

        let modules = settings.modules.clone();

        let init_state_values_response = get_init_data().await;

        let init_state_values = match init_state_values_response {
            Ok(r) => r,
            Err(e) => InitValues::default(),
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

        let wifi_image = get_image_from_path(modules.wifi.icon.strong, &["icon"]);
        let bluetooth_image = get_image_from_path(modules.bluetooth.icon.connected, &["icon"]);
        let battery_image = get_image_from_path(modules.battery.icon.level_70, &["icon"]);

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

        let model = StatusBar {
            settings: settings.clone(),
            custom_theme,
            current_time: current_time(modules.clock.format.as_str()),
            wifi_state: init_state_values.wifi_state,
            bluetooth_state: init_state_values.bluetooth_state,
            battery_state: init_state_values.battery_state,
            window,
        };

        let widgets = AppWidgets {
            clock_label,
            wifi_image,
            bluetooth_image,
            battery_image,
        };

        let tick = move || {
            let sender_clone = sender.clone();
            let format = modules.clock.format.clone();
            tokio::spawn(async move {
                let _ = sender_clone
                    .input_sender()
                    .send(Message::TimeTick(current_time(format.as_str())));

                let init_state_values_response = get_init_data().await;

                let init_state_values = match init_state_values_response {
                    Ok(r) => r,
                    Err(_) => InitValues::default(),
                };

                let _ = sender_clone
                    .input_sender()
                    .send(Message::WifiStateUpdate(init_state_values.wifi_state));

                let _ = sender_clone
                    .input_sender()
                    .send(Message::BluetoothStateUpdate(
                        init_state_values.bluetooth_state,
                    ));

                let _ = sender_clone
                    .input_sender()
                    .send(Message::BatteryStatusUpdate(
                        init_state_values.battery_state,
                    ));
            });

            glib::ControlFlow::Continue
        };

        //glib::timeout_add_seconds_local(1, tick);

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
            Message::WifiStateUpdate(state) => {
                self.wifi_state = state;
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
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        widgets
            .clock_label
            .set_label(&format!("{}", self.current_time));

        let modules = self.settings.modules.clone();
        info!("wifi state is {:?}", self.wifi_state);
        match self.wifi_state {
            WifiState::Off => {
                if let Some(icon) = modules.wifi.icon.off.clone() {
                    widgets.wifi_image.set_file(Some(&icon));
                }
            }
            WifiState::On => {
                if let Some(icon) = modules.wifi.icon.on.clone() {
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

        match self.bluetooth_state {
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
            BatteryState::Level0 => {
                if let Some(icon) = modules.battery.icon.level_0.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level10 => {
                if let Some(icon) = modules.battery.icon.level_10.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level20 => {
                if let Some(icon) = modules.battery.icon.level_20.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level30 => {
                if let Some(icon) = modules.battery.icon.level_30.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level40 => {
                if let Some(icon) = modules.battery.icon.level_40.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level50 => {
                if let Some(icon) = modules.battery.icon.level_50.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level60 => {
                if let Some(icon) = modules.battery.icon.level_60.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level70 => {
                if let Some(icon) = modules.battery.icon.level_70.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level80 => {
                if let Some(icon) = modules.battery.icon.level_80.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level90 => {
                if let Some(icon) = modules.battery.icon.level_90.clone() {
                    widgets.battery_image.set_file(Some(&icon));
                }
            }
            BatteryState::Level100 => {
                if let Some(icon) = modules.battery.icon.level_100.clone() {
                    widgets.battery_image.set_file(Some(&icon));
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
    let app = RelmApp::new("mecha.status.bar");
    let assets_base_path =
        std::env::var("MECHA_STATUS_BAR_ASSETS_PATH").unwrap_or(String::from(""));
    relm4::set_global_css_from_file(assets_base_path + "/css/style.css");
    app.run_async::<StatusBar>(());
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

#[derive(Default, Debug, Clone)]
struct InitValues {
    wifi_state: WifiState,
    battery_state: BatteryState,
    bluetooth_state: BluetoothState,
}

async fn get_init_data() -> anyhow::Result<InitValues> {
    let wifi_state = match get_wifi_data().await {
        Ok(r) => r,
        Err(_) => WifiState::Off,
    };

    let bluetooth_state = match get_bluetooth_data().await {
        Ok(r) => r,
        Err(_) => BluetoothState::Off,
    };

    let battery_state = match get_battery_data().await {
        Ok(r) => r,
        Err(_) => BatteryState::Level0,
    };

    Ok(InitValues {
        wifi_state,
        bluetooth_state,
        battery_state,
        ..Default::default()
    })
}

async fn get_network_client() -> anyhow::Result<NetworkManagerClient> {
    let url = match std::env::var("GRPC_SERVER_URL") {
        Ok(r) => r,
        Err(_) => "".to_string(),
    };
    let network_manager_client_response = NetworkManagerClient::new(url).await;
    let network_manager_client = match network_manager_client_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitNetworkManagerClient,
                format!("unable to create network manager client error - {}", e),
                true
            ));
        }
    };
    Ok(network_manager_client)
}

async fn get_bluetooth_client() -> anyhow::Result<BluetoothManagerClient> {
    let url = match std::env::var("GRPC_SERVER_URL") {
        Ok(r) => r,
        Err(_) => "".to_string(),
    };
    let bluetooth_manager_client_response = BluetoothManagerClient::new(url).await;
    let bluetooth_manager_client = match bluetooth_manager_client_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitBluetoothManagerClient,
                format!("unable to create bluetooth manager client error - {}", e),
                true
            ));
        }
    };
    Ok(bluetooth_manager_client)
}

async fn get_battery_client() -> anyhow::Result<BatteryManagerClient> {
    let url = match std::env::var("GRPC_SERVER_URL") {
        Ok(r) => r,
        Err(_) => "".to_string(),
    };
    let battery_manager_client_response = BatteryManagerClient::new(url).await;
    let battery_manager_client = match battery_manager_client_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitBatteryManagerClient,
                format!("unable to create battery manager client error - {}", e),
                true
            ));
        }
    };
    Ok(battery_manager_client)
}

async fn get_wifi_data() -> anyhow::Result<WifiState> {
    let mut network_manager_client = match get_network_client().await {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitNetworkManagerClient,
                format!("unable to create network manager client error - {}", e),
                true
            ));
        }
    };

    let wifi_status_response = network_manager_client.get_wireless_network_status().await;
    let wifi_status = match wifi_status_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::GetWifiStatusError,
                format!("unable to get wireless network status error - {}", e),
                true
            ));
        }
    };

    let mut wifi_state = match wifi_status.wifi_on {
        true => WifiState::On,
        false => WifiState::Off,
    };

    match wifi_status.current_network {
        Some(current_network) => {
            if current_network.signal <= -80 {
                wifi_state = WifiState::Connected(WifiConnectedState::Low);
            } else if current_network.signal <= -60 {
                wifi_state = WifiState::Connected(WifiConnectedState::Weak);
            } else if current_network.signal <= -40 {
                wifi_state = WifiState::Connected(WifiConnectedState::Good);
            } else {
                wifi_state = WifiState::Connected(WifiConnectedState::Strong);
            }
        }
        None => {}
    }

    // let current_wireless_response = network_manager_client.get_current_wireless_network().await;
    // let current_wireless_network = match current_wireless_response {
    //     Ok(r) => Option::from(r),
    //     Err(e) => {
    //         bail!(StatusBarError::new(
    //             StatusBarErrorCodes::GetWifiStatusError,
    //             format!("unable to get wireless network status error - {}", e),
    //             true
    //         ));
    //     }
    // };

    // match current_wireless_network {
    //     Some(_) => {
    //         wifi_state = WifiState::Connected(WifiConnectedState::Strong);
    //     }
    //     None => {}
    // }

    Ok(wifi_state)
}

async fn get_bluetooth_data() -> anyhow::Result<BluetoothState> {
    let mut bluetooth_manager_client = match get_bluetooth_client().await {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitBluetoothManagerClient,
                format!("unable to create bluetooth manager client error - {}", e),
                true
            ));
        }
    };
    let bluetooth_status_response = bluetooth_manager_client.get_bluetooth_status().await;
    let bluetooth_status = match bluetooth_status_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::GetBluetoothStatusError,
                format!("unable to get bluetooth status error - {}", e),
                true
            ));
        }
    };

    let bluetooth_state = match bluetooth_status.enabled {
        true => BluetoothState::On,
        false => BluetoothState::Off,
    };

    Ok(bluetooth_state)
}

async fn get_battery_data() -> anyhow::Result<BatteryState> {
    let mut battery_manager_client = match get_battery_client().await {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::InitBatteryManagerClient,
                format!("unable to create battery manager client error - {}", e),
                true
            ));
        }
    };
    let battery_status_response = battery_manager_client.get_battery_status().await;
    let battery_status = match battery_status_response {
        Ok(r) => r,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::GetBatteryStatusError,
                format!("unable to get battery status error - {}", e),
                true
            ));
        }
    };

    let battery_capacity = battery_status.capacity.parse::<u8>().unwrap();

    let battery_state = match battery_capacity {
        0..=9 => BatteryState::Level0,
        10..=19 => BatteryState::Level10,
        20..=29 => BatteryState::Level20,
        30..=39 => BatteryState::Level30,
        40..=49 => BatteryState::Level40,
        50..=59 => BatteryState::Level50,
        60..=69 => BatteryState::Level60,
        70..=79 => BatteryState::Level70,
        80..=89 => BatteryState::Level80,
        90..=99 => BatteryState::Level90,
        100 => BatteryState::Level100,
        _ => BatteryState::Level100,
    };

    Ok(battery_state)
}
