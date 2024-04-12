use std::fmt;
use gtk::{glib::clone, prelude::GtkWindowExt};
use mechanix_zbus_client::wireless::WirelessInfoResponse;
use relm4::component::{AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController };
use relm4::async_trait::async_trait;
use relm4::{gtk, AsyncComponentSender, RelmApp};
use relm4::{Component, ComponentController, Controller};


mod pages;
mod modules;
mod settings;
mod theme;
mod widgets;
use pages::{
   
    bluetooth_details_page::{
        BluetoothDetailsPage, Message as BluetoothDetailsPageMessage,
        Settings as BluetoothDetailsPageSettings,
    },
    bluetooth_pair_request_page::{
        BluetoothPairRequestPage, Message as BluetoothPairRequestPageMessage,
        Settings as BluetoothPairRequestPageSettings,
    },
    connect_bluetooth_page::{
        ConnectBluetoothPage, Message as ConnectBluetoothPageMessage,
        Settings as ConnectBluetoothPageSettings,
    },
    
    display_page::{DisplayPage, Message as DisplayPageMessage, Settings as DisplayPageSettings},
    battery_page::{BatteryPage, Message as BatteryPageMessage, Settings as BatteryPageSettings},
    home::{HomePage, Message as HomePageMessage, Settings as HomePageSettings},
    lock_timeout_page::{
        LockTimeoutPage, Message as LockTimeoutPageMessage, Settings as LockTimeoutPageSettings,
    },
    reset_pin_page::{
        ResetPinPage, Message as ResetPinPageMessage, Settings as ResetPinPageSettings,
    },
    manage_bluetooth_page::{
        ManageBluetoothPage, Message as ManageBluetoothPageMessage,
        Settings as ManageBluetoothPageSettings,
    },
    
    network::{
        network_page::{
            Message as NetworkPageMessage, NetworkPage, Settings as NetworkPageSettings,
        },
        ethernet_page::{
            Message as EthernetPageMessage, EthernetPage, Settings as EthernetPageSettings,
        },
        dns_page::{
            Message as DNSPageMessage, DNSPage, Settings as DNSPageSettings,
        },
        manage_networks_page::{
            ManageNetworksPage, Message as ManageNetworkPageMessage,
            Settings as ManageNetworkPageSettings,
        },
        network_details_page::{
            Message as NetworkDetailsPageMessage, NetworkDetailsPage,
            Settings as NetworkDetailsPageSettings,
        },
        add_network_page::{
            AddNetworkPage, Message as AddNetworkPageMessage, Settings as AddNetworkPageSettings,
        },
        connect_network_page::{
            ConnectNetworkPage, Message as ConnectNetworkPageMessage,
            Settings as ConnectNetworkPageSettings,
        },
    },
    ip_settings_page::{
        Message as IPSettingsPageMessage, IPSettingsPage, Settings as IPSettingsPageSettings,
    },
    
   
    protocol_modes_page::{
        Message as ProtocolModesPageMessage, ProtocolModesPage, Settings as ProtocolModesPageSettings,
    },
    protocol_details_page::{
        Message as ProtocolDetailsPageMessage, ProtocolDetailsPage, Settings as ProtocolDetailsPageSettings,
    },
    password_authentication::{
        Message as PasswordAuthenticationMessage, PasswordAuthentication,
        Settings as PasswordAuthenticationSettings,
    },
    performance_mode_page::{
        Message as PerformanceModePageMessage, PerformanceModePage,
        Settings as PerformanceModePageSettings,
    },
    pin_authentication::{
        Message as PinAuthenticationMessage, PinAuthentication,
        Settings as PinAuthenticationSettings,
    },
    screen_timeout_page::{
        Message as ScreenTimeoutPageMessage, ScreenTimeoutPage,
        Settings as ScreenTimeoutPageSettings,
    },
    security_page::{
        Message as SecurityPageMessage, SecurityPage, Settings as SecurityPageSettings,
    },
    settings_page::{
        OutputMessage as SettingsPageMessage, Settings as SettingsPageSettings, SettingsPage,
    },
    sound_page::{Message as SoundPageMessage, Settings as SoundPageSettings, SoundPage},
    date_time_page::{
        Message as DateTimePageMessage, DateTimePage, Settings as DateTimePageSettings,
    },
    set_time_page::{
        Message as SetTimePageMessage, SetTimePage, Settings as SetTimePageSettings,
    },
    set_date_page::{
        Message as SetDatePageMessage, SetDatePage, Settings as SetDatePageSettings,
    },
    about_page::{
        Message as AboutPageMessage, AboutPage, Settings as AboutPageSettings,
    }
};
use settings::LockScreenSettings;
use tracing::info;
pub mod errors; 
use crate::theme::LockScreenTheme;

/// # LockScreen State
///
/// This struct is the state definition of the entire application
struct LockScreen {
    current_screen: Screens,
    previous_screen: Vec<Screens>,
    settings: LockScreenSettings,
    custom_theme: LockScreenTheme,
    home_page: Controller<HomePage>,
    settings_page: Controller<SettingsPage>,
    network_page: AsyncController<NetworkPage>,
    manage_networks_page: AsyncController<ManageNetworksPage>,
    network_details_page: Controller<NetworkDetailsPage>,
    connect_network_page: AsyncController<ConnectNetworkPage>,
    add_network_page: Controller<AddNetworkPage>,
    manage_bluetooth_page: Controller<ManageBluetoothPage>,
    bluetooth_details_page: Controller<BluetoothDetailsPage>,
    connect_bluetooth_page: Controller<ConnectBluetoothPage>,
    bluetooth_pair_request_page: Controller<BluetoothPairRequestPage>,
    display_page: Controller<DisplayPage>,
    screen_timeout_page: AsyncController<ScreenTimeoutPage>,
    sound_page: Controller<SoundPage>,
    performance_mode_page: AsyncController<PerformanceModePage>,
    security_page: Controller<SecurityPage>,
    lock_timeout_page: Controller<LockTimeoutPage>,
    battery_page: AsyncController<BatteryPage>,
    reset_pin_page: Controller<ResetPinPage>,
    date_time_page: Controller<DateTimePage>,
    set_time_page: Controller<SetTimePage>,
    set_date_page: Controller<SetDatePage>,
    about_page: AsyncController<AboutPage>,
    ip_settings_page: Controller<IPSettingsPage>,
    protocol_modes_page: Controller<ProtocolModesPage>,
    protocol_details_page: Controller<ProtocolDetailsPage>,
    ethernet_page: Controller<EthernetPage>,
    dns_page: Controller<DNSPage>,

    selected_network: WirelessInfoResponse,
}

#[derive(Debug, Clone)]
pub enum Screens {
    LockScreen,
    PasswordScreen,
    PinScreen,
    Home,
    Settings,
    Network,
    ManageNetworks,
    NetworkDetails,
    ConnectNetwork,
    AddNetwork,
    ManageBluetooth,
    BluetoothDetails,
    ConnectBluetooth,
    BluetoothPairRequest,
    Display,
    ScreenTimeout,
    Sound,
    PerformanceMode,
    Security,
    LockTimeout,
    Battery,
    ResetPin,
    DateTime,
    SetTime,
    SetDate,
    About,
    IPSettings,
    Ethernet,
    DNSPage,
    ProtocolModes,
    ProtocolDetails,
}

impl fmt::Display for Screens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Screens::LockScreen => write!(f, "lock_screen"),
            Screens::PasswordScreen => write!(f, "password_screen"),
            Screens::PinScreen => write!(f, "pin_screen"),
            Screens::Home => write!(f, "home"),
            Screens::Network => write!(f, "network"),
            Screens::ManageNetworks => write!(f, "manage_networks"),
            Screens::NetworkDetails => write!(f, "network_details"),
            Screens::ConnectNetwork => write!(f, "connect_network"),
            Screens::AddNetwork => write!(f, "add_network"),
            Screens::ManageBluetooth => write!(f, "manage_bluetooth"),
            Screens::BluetoothDetails => write!(f, "bluetooth_details"),
            Screens::ConnectBluetooth => write!(f, "connect_bluetooth"),
            Screens::BluetoothPairRequest => write!(f, "bluetooth_pair_request"),
            Screens::Display => write!(f, "display"),
            Screens::ScreenTimeout => write!(f, "screen_timeout"),
            Screens::Sound => write!(f, "sound"),
            Screens::PerformanceMode => write!(f, "performance_mode"),
            Screens::Settings => write!(f, "settings"),
            Screens::Security => write!(f, "security"),
            Screens::LockTimeout => write!(f, "lock_timeout"),
            Screens::Battery => write!(f, "battery"),
            Screens::ResetPin => write!(f, "reset_pin"),
            Screens::DateTime => write!(f, "date_time"),
            Screens::SetTime => write!(f, "set_time"), 
            Screens::SetDate => write!(f, "set_date"), 
            Screens::About => write!(f, "about"),
            Screens::IPSettings => write!(f, "ip_settings"),
            Screens::ProtocolModes => write!(f, "protocol_modes"),
            Screens::ProtocolDetails => write!(f, "protocol_details"),
            Screens::Ethernet => write!(f, "ethernet_page"),
            Screens::DNSPage => write!(f, "dns_page"),
        }
    }
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    GoBack,
    Dummy,
    UpdateView,
    SelectedNetworkChanged(WirelessInfoResponse)

}

struct AppWidgets {
    screens_stack: gtk::Stack,
}

// #[cfg(not(feature = "layer-shell"))]
fn init_window(settings: LockScreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

// #[cfg(feature = "layer-shell")]
// fn init_window(settings: LockScreenSettings) -> gtk::Window {
//     let window_settings = settings.window;
//     let window = gtk::Window::builder()
//         .title(settings.title)
//         .default_width(window_settings.size.0)
//         .default_height(window_settings.size.1)
//         .css_classes(["window"])
//         .build();

//     gtk4_layer_shell::init_for_window(&window);

//     // Display above normal windows
//     gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

//     // The margins are the gaps around the window's edges
//     // Margins and anchors can be set like this...
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

//     gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::OnDemand);

//     // ... or like this
//     // Anchors are if the window is pinned to each edge of the output
//     let anchors = [
//         (gtk4_layer_shell::Edge::Left, true),
//         (gtk4_layer_shell::Edge::Right, true),
//         (gtk4_layer_shell::Edge::Top, true),
//         (gtk4_layer_shell::Edge::Bottom, true),
//     ];

//     for (anchor, state) in anchors {
//         gtk4_layer_shell::set_anchor(&window, anchor, state);
//     }

//     window
// }
#[async_trait(?Send)]
impl AsyncComponent for LockScreen {
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
            Err(_) => LockScreenSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for Lock Screen: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for Lock Screen: {:?}", custom_theme
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
        let icon_theme = gtk::IconTheme::builder().build();
        info!("icon paths are {:?}", icon_theme.resource_path());
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        let css = settings.css.clone();
        // relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => LockScreenTheme::default(),
        };

        let modules = settings.modules.clone();
        let layout = settings.layout.clone();
        let widget_configs = settings.widget_configs.clone();

        //Stack used to render different screens
        //At a time one screen will be rendered
        let screens_stack = gtk::Stack::builder().build();

        let home_page = HomePage::builder()
            .launch(HomePageSettings {
                lock_icon: modules.lock.icon.default.to_owned(),
                unlock_icon: modules.unlock.icon.default.to_owned(),
                password_icon: modules.home_password.icon.default.to_owned(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    HomePageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen)
                }),
            );

        screens_stack.add_named(
            home_page.widget(),
            Option::from(Screens::Home.to_string().as_str()),
        );

        let settings_page = SettingsPage::builder()
            .launch(SettingsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("settings_page - auth page message to parent - settings page {:?}", msg);
                    match msg {
                        SettingsPageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen)
                     }
                }),
            );

        screens_stack.add_named(
            settings_page.widget(),
            Option::from(Screens::Settings.to_string().as_str()),
        );

        let network_page = NetworkPage::builder()
            .launch(NetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("network_page - auth page message to parent {:?}", msg);
                    println!("network_page - auth page message to parent {:?}", msg);
                    match msg {
                        NetworkPageMessage::BackPressed => Message::GoBack,
                        NetworkPageMessage::EnableNetworkPressed => Message::ChangeScreen(Screens::NetworkDetails),
                        NetworkPageMessage::ManageNetworkPressed => Message::ChangeScreen(Screens::ManageNetworks),
                        NetworkPageMessage::IpSettingsPressed => Message::ChangeScreen(Screens::IPSettings),
                        NetworkPageMessage::EthernetPressed => Message::ChangeScreen(Screens::Ethernet),
                        NetworkPageMessage::DNSPressed => Message::ChangeScreen(Screens::DNSPage),
                        NetworkPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            network_page.widget(),
            Option::from(Screens::Network.to_string().as_str()),
        );


        let connect_network_page: AsyncController<ConnectNetworkPage> = ConnectNetworkPage::builder()
            .launch(ConnectNetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageNetworks 
                        ConnectNetworkPageMessage::BackPressed => Message::GoBack,
                        ConnectNetworkPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            connect_network_page.widget(),
            Option::from(Screens::ConnectNetwork.to_string().as_str()),
        );

        let manage_networks_page: AsyncController<ManageNetworksPage> = ManageNetworksPage::builder()
            .launch(ManageNetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("manage_networks_page - auth page message to parent {:?}", msg);
                    match msg {
                        ManageNetworkPageMessage::BackPressed => Message::GoBack,
                        ManageNetworkPageMessage::KnownNetworkPressed => 
                        Message::ChangeScreen(Screens::NetworkDetails),
                        ManageNetworkPageMessage::AvailableNetworkPressed =>{
                            Message::ChangeScreen(Screens::ConnectNetwork)
                        }
                        ManageNetworkPageMessage::SelectedNetworkChanged(value) => {
                            Message::SelectedNetworkChanged(value)
                        },
                        ManageNetworkPageMessage::AddNetworkPressed => Message::ChangeScreen(Screens::AddNetwork), 
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            manage_networks_page.widget(),
            Option::from(Screens::ManageNetworks.to_string().as_str()),
        );

        let ip_settings_page: Controller<IPSettingsPage> = IPSettingsPage::builder()
            .launch(IPSettingsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("ip_settings_page - auth page message to parent {:?}", msg);
                    match msg {
                        IPSettingsPageMessage::BackPressed => Message::GoBack,
                        IPSettingsPageMessage::ProtocolModes => Message::ChangeScreen(Screens::ProtocolModes),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            ip_settings_page.widget(),
            Option::from(Screens::IPSettings.to_string().as_str()),
        );

        let ethernet_page : Controller<EthernetPage> = EthernetPage::builder()
        .launch(EthernetPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("ethernet_page - auth page message to parent {:?}", msg);
                match msg {
                    EthernetPageMessage::BackPressed => Message::GoBack,
                    EthernetPageMessage::IPSettingsPressed => Message::ChangeScreen(Screens::IPSettings),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            ethernet_page.widget(),
            Option::from(Screens::Ethernet.to_string().as_str()),
        );

        let dns_page : Controller<DNSPage> = DNSPage::builder()
        .launch(DNSPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("dns_page - auth page message to parent {:?}", msg);
                match msg {
                    DNSPageMessage::BackPressed => Message::GoBack,
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            dns_page.widget(),
            Option::from(Screens::DNSPage.to_string().as_str()),
        );

        let protocol_modes_page: Controller<ProtocolModesPage> = ProtocolModesPage::builder()
        .launch(ProtocolModesPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("protocol_modes_page - auth page message to parent {:?}", msg);
                match msg {
                // back -> ProtocolModes
                ProtocolModesPageMessage::BackPressed => Message::GoBack,   // IPSettings
                ProtocolModesPageMessage::StaticModePressed => Message::ChangeScreen(Screens::ProtocolDetails),
                _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            protocol_modes_page.widget(),
            Option::from(Screens::ProtocolModes.to_string().as_str()),
        );


        let protocol_details_page: Controller<ProtocolDetailsPage> = ProtocolDetailsPage::builder()
        .launch(ProtocolDetailsPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("protocol_details_page - auth page message to parent {:?}", msg);
                match msg {
                    // back -> ProtocolModes
                    ProtocolDetailsPageMessage::BackPressed => Message::GoBack,
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            protocol_details_page.widget(),
            Option::from(Screens::ProtocolDetails.to_string().as_str()),
        );

        let network_details_page: Controller<NetworkDetailsPage> = NetworkDetailsPage::builder()
            .launch(NetworkDetailsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("network_details_page - auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageNetworks 
                        NetworkDetailsPageMessage::BackPressed => Message::GoBack,
                        NetworkDetailsPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            network_details_page.widget(),
            Option::from(Screens::NetworkDetails.to_string().as_str()),
        );


        let add_network_page: Controller<AddNetworkPage> = AddNetworkPage::builder()
            .launch(AddNetworkPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageNetworks 
                        AddNetworkPageMessage::BackPressed => Message::GoBack,
                        AddNetworkPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            add_network_page.widget(),
            Option::from(Screens::AddNetwork.to_string().as_str()),
        );

        let manage_bluetooth_page: Controller<ManageBluetoothPage> = ManageBluetoothPage::builder()
            .launch(ManageBluetoothPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg { 
                        ManageBluetoothPageMessage::BackPressed => Message::GoBack,
                        ManageBluetoothPageMessage::AvaiableDevicePressed => Message::ChangeScreen(Screens::BluetoothDetails),
                        ManageBluetoothPageMessage::OtherDevicePressed => Message::ChangeScreen(Screens::ConnectBluetooth),
                        // ManageBluetoothPageMessage::OtherDevicePressed => Message::ChangeScreen(Screens::BluetoothPairRequest),
                        ManageBluetoothPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            manage_bluetooth_page.widget(),
            Option::from(Screens::ManageBluetooth.to_string().as_str()),
        );

        let bluetooth_details_page: Controller<BluetoothDetailsPage> = BluetoothDetailsPage::builder()
            .launch(BluetoothDetailsPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageBluetooth
                        BluetoothDetailsPageMessage::BackPressed => Message::GoBack,
                        BluetoothDetailsPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            bluetooth_details_page.widget(),
            Option::from(Screens::BluetoothDetails.to_string().as_str()),
        );

        let connect_bluetooth_page: Controller<ConnectBluetoothPage> = ConnectBluetoothPage::builder()
            .launch(ConnectBluetoothPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageBluetooth
                        ConnectBluetoothPageMessage::BackPressed => Message::GoBack,
                        ConnectBluetoothPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            connect_bluetooth_page.widget(),
            Option::from(Screens::ConnectBluetooth.to_string().as_str()),
        );

        let bluetooth_pair_request_page: Controller<BluetoothPairRequestPage> = BluetoothPairRequestPage::builder()
            .launch(BluetoothPairRequestPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> ManageBluetooth
                        BluetoothPairRequestPageMessage::BackPressed => Message::GoBack,
                        BluetoothPairRequestPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            bluetooth_pair_request_page.widget(),
            Option::from(Screens::BluetoothPairRequest.to_string().as_str()),
        );

        let display_page: Controller<DisplayPage> = DisplayPage::builder()
            .launch(DisplayPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg { 
                        DisplayPageMessage::BackPressed => Message::GoBack,
                        DisplayPageMessage::ScreenTimeoutOpted => Message::ChangeScreen(Screens::ScreenTimeout),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            display_page.widget(),
            Option::from(Screens::Display.to_string().as_str()),
        );

        let battery_page: AsyncController<BatteryPage> = BatteryPage::builder()
            .launch(BatteryPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("battery_page - auth page message to parent {:?}", msg);
                    match msg { 
                        BatteryPageMessage::BackPressed => Message::GoBack,
                        BatteryPageMessage::ScreenTimeoutOpted => Message::ChangeScreen(Screens::ScreenTimeout),
                        BatteryPageMessage::PerformanceOpted => Message::ChangeScreen(Screens::PerformanceMode),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            battery_page.widget(),
            Option::from(Screens::Battery.to_string().as_str()),
        );

        let screen_timeout_page: AsyncController<ScreenTimeoutPage> = ScreenTimeoutPage::builder()
            .launch(ScreenTimeoutPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg { 
                        // back -> Display or Battery
                        ScreenTimeoutPageMessage::BackPressed => Message::GoBack,
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            screen_timeout_page.widget(),
            Option::from(Screens::ScreenTimeout.to_string().as_str()),
        );

        let sound_page: Controller<SoundPage> = SoundPage::builder()
            .launch(SoundPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg { 
                        SoundPageMessage::BackPressed => Message::GoBack,
                        SoundPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            sound_page.widget(),
            Option::from(Screens::Sound.to_string().as_str()),
        );

        let performance_mode_page: AsyncController<PerformanceModePage> = PerformanceModePage::builder()
            .launch(PerformanceModePageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> Battery
                        PerformanceModePageMessage::BackPressed => Message::GoBack,
                            _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            performance_mode_page.widget(),
            Option::from(Screens::PerformanceMode.to_string().as_str()),
        );

        let security_page: Controller<SecurityPage> = SecurityPage::builder()
        .launch(SecurityPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("security_page - auth page message to parent {:?}", msg);
                match msg {
                    SecurityPageMessage::BackPressed => Message::GoBack,
                    SecurityPageMessage::LockTimeoutOpted => Message::ChangeScreen(Screens::LockTimeout),
                    SecurityPageMessage::ResetPinOpted => Message::ChangeScreen(Screens::ResetPin),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            security_page.widget(),
            Option::from(Screens::Security.to_string().as_str()),
        );

        let lock_timeout_page: Controller<LockTimeoutPage> = LockTimeoutPage::builder()
            .launch(LockTimeoutPageSettings {
                modules: modules.clone(),
                layout: layout.clone(),
                widget_configs: widget_configs.clone()
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| {
                    info!("auth page message to parent {:?}", msg);
                    match msg {
                        // back -> Security
                        LockTimeoutPageMessage::BackPressed => Message::GoBack, 
                        LockTimeoutPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                    }
                }),
            );

        screens_stack.add_named(
            lock_timeout_page.widget(),
            Option::from(Screens::LockTimeout.to_string().as_str()),
        );

        let reset_pin_page:Controller<ResetPinPage> = ResetPinPage::builder()
        .launch(ResetPinPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                    // back -> Security
                    ResetPinPageMessage::BackPressed => Message::GoBack,   
                    ResetPinPageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            reset_pin_page.widget(),
            Option::from(Screens::ResetPin.to_string().as_str()),
        );


        let date_time_page: Controller<DateTimePage> = DateTimePage::builder()
        .launch(DateTimePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("date_time_page - auth page message to parent {:?}", msg);
                match msg {
                    DateTimePageMessage::BackPressed => Message::GoBack,    
                    DateTimePageMessage::SetTimeOpted => Message::ChangeScreen(Screens::SetTime),
                    DateTimePageMessage::SetDateOpted => Message::ChangeScreen(Screens::SetDate),
                    _ => Message::Dummy
                }
            }),
        );

        screens_stack.add_named(
            date_time_page.widget(),
            Option::from(Screens::DateTime.to_string().as_str()),
        );

        
        let set_time_page:Controller<SetTimePage> = SetTimePage::builder()
        .launch(SetTimePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                    // back -> DateTime
                    SetTimePageMessage::BackPressed => Message::GoBack,  
                    SetTimePageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                        _ => Message::Dummy
                }
            }),
        );
        screens_stack.add_named(
            set_time_page.widget(),
            Option::from(Screens::SetTime.to_string().as_str()),
        );


        let set_date_page:Controller<SetDatePage> = SetDatePage::builder()
        .launch(SetDatePageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                    // back -> DateTime
                    SetDatePageMessage::BackPressed => Message::GoBack,  
                    SetDatePageMessage::HomeIconPressed => Message::ChangeScreen(Screens::LockScreen),
                    _ => Message::Dummy
                }
            }),
        );  
        screens_stack.add_named(
            set_date_page.widget(),
            Option::from(Screens::SetDate.to_string().as_str()),
        );


        let about_page:AsyncController<AboutPage> = AboutPage::builder()
        .launch(AboutPageSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            widget_configs: widget_configs.clone()
        })
        .forward(
            sender.input_sender(),
            clone!(@strong modules => move|msg| {
                info!("auth page message to parent {:?}", msg);
                match msg {
                   AboutPageMessage::BackPressed => Message::GoBack,
                    _ => Message::Dummy
                }
            }),
        );
        
        screens_stack.add_named(
            about_page.widget() ,
            Option::from(Screens::About.to_string().as_str()),
        );


        let current_screen = Screens::Settings;

        //Setting current active screen in stack
        screens_stack.set_visible_child_name(&current_screen.to_string());

        //Adding stack to window
        window.set_child(Some(&screens_stack));

        let lock_screen_settings = settings.clone();

        let model = LockScreen {
            settings,
            custom_theme,
            current_screen,
            previous_screen: Vec::new(),
            home_page,
            settings_page,
            network_page,
            manage_networks_page,
            network_details_page,
            connect_network_page,
            add_network_page,
            manage_bluetooth_page,
            bluetooth_details_page,
            connect_bluetooth_page,
            bluetooth_pair_request_page,
            display_page,
            screen_timeout_page,
            sound_page,
            performance_mode_page,
            security_page,
            lock_timeout_page,
            battery_page,
            reset_pin_page,
            date_time_page,
            set_time_page,
            set_date_page,
            about_page,
            ip_settings_page,
            protocol_modes_page,
            protocol_details_page,
            ethernet_page,
            dns_page,

            selected_network: WirelessInfoResponse::default()
        };

        let widgets = AppWidgets { screens_stack };

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>,_root: &Self::Root,) {
        info!("Update message is {:?}", message);
        match message {
            Message::ChangeScreen(screen) => {
                // self.previous_screen = Some(self.current_screen.clone());
                self.previous_screen.push(self.current_screen.clone());
                self.current_screen = screen;
                update_screen_view(&self);
               
            }
            Message::GoBack => {
                if let Some(previous_screen) = self.previous_screen.pop() {
                    self.current_screen = previous_screen;
                    update_screen_view(&self);
                }
                
            },
            Message::SelectedNetworkChanged(value) =>{
                self.selected_network = value.clone()
            },
            _ => (),
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        //updating stack screen when current screen changes
        widgets
            .screens_stack
            .set_visible_child_name(self.current_screen.to_string().as_str());

        // match widgets.screens_stack.last_child() {
        //     Some(value)=>{ value.emit(, args) }
        //     None => {} }

        match self.current_screen {
            Screens::ConnectNetwork => {
                self.connect_network_page.emit(ConnectNetworkPageMessage::ConnectToNetworkChanged(self.selected_network.clone()))
            },
            _ => {

            }
        }

    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("settings=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("apps.settings").with_args(vec![]);

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => LockScreenSettings::default(),
    };

    let css = settings.css.clone();
    app.set_global_css_from_file(css.default);

    app.run_async::<LockScreen>(());

    
}

fn update_screen_view(app: &LockScreen) {
    match app.current_screen {
        Screens::LockScreen => {},
        Screens::PasswordScreen => {},
        Screens::PinScreen => {},
        Screens::Home => {},
        Screens::Settings => {},
        Screens::Network => {
            app.network_page.emit(NetworkPageMessage::UpdateView);
        },
        Screens::ManageNetworks => {},
        Screens::NetworkDetails => {},
        Screens::ConnectNetwork => {
            
        },
        Screens::AddNetwork => {},
        Screens::ManageBluetooth => {},
        Screens::BluetoothDetails => {},
        Screens::ConnectBluetooth => {},
        Screens::BluetoothPairRequest => {},
        Screens::Display => {},
        Screens::ScreenTimeout => {
            app.screen_timeout_page.emit(ScreenTimeoutPageMessage::UpdateView);
        },
        Screens::Sound => {},
        Screens::PerformanceMode => {
            app.performance_mode_page.emit(PerformanceModePageMessage::UpdateView);
        },
        Screens::Security => {},
        Screens::LockTimeout => {},
        Screens::Battery => {
            app.battery_page.emit(BatteryPageMessage::UpdateView);
        },
        Screens::ResetPin => {},
        Screens::DateTime => {},
        Screens::SetTime => {},
        Screens::SetDate => {},
        Screens::About => {},
        Screens::IPSettings => {},
        Screens::Ethernet => {},
        Screens::DNSPage => {},
        Screens::ProtocolModes => {},
        Screens::ProtocolDetails => {},
    }
        
    
}