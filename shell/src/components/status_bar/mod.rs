use bevy::{
    color::palettes::css,
    ecs::{
        relationship::RelatedSpawner,
        spawn::{self, SpawnWith},
    },
    platform::collections::HashMap,
    prelude::*,
    winit::WinitSettings,
};
use bevy_asset_loader::prelude::*;
use bevy_styled_widgets::prelude::*;

use crate::{
    settings::status_bar::{
        BatteryState, BluetoothState, DateSettings, MobileNetworkState, RightTrayStatus,
        StatusBarSettings, SystemStatus, WifiState,
    },
    utils::{FontAssets, Icon},
};
use chrono::Local;

/// Loads image assets
#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(key = "images.parachute")]
    parachute: Handle<Image>,
}

/// Defines app asset loading states
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

/// Global UI settings container
#[derive(Default, Debug, Clone)]
pub struct GuiSettings {
    pub status_bar: StatusBarSettings,
}

/// Entry point: initializes app with plugin, asset loading, systems
pub fn run_status_bar() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup_view_root)
        .run();
}

/// Initializes the status bar UI layout and data
fn setup_view_root(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    commands.spawn(Camera2d);

    // Example system data injected manually
    let right_tray_status = RightTrayStatus {
        headphones_connected: true,
        monitor_connected: true,
        terminal_active: true,
        usb_connected: false,
    };

    let system_status = SystemStatus {
        bluetooth_state: BluetoothState::Connected,
        wifi_state: WifiState::OnButNotConnected,
        mobile_network_state: MobileNetworkState::Full,
        battery_state: BatteryState::Charging,
    };

    commands.insert_resource(right_tray_status.clone());
    commands.insert_resource(system_status.clone());

    // Load settings
    let GuiSettings { status_bar } = GuiSettings::default();
    let StatusBarSettings {
        width,
        height,
        menus,
        date,
    } = status_bar;
    let date = DateSettings {
        time: date.time,
        date: date.date,
        day: date.day,
    };

    // Select menu layout
    let current_menu = "sm";
    let list_menus = menus.get(current_menu).unwrap_or(&vec![]).clone();

    // Root status bar node
    commands
        .spawn((
            Node {
                width: Val::Vw(width),
                height: Val::Vh(height),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Percent(1.5)),
                ..Default::default()
            },
            Name::new("status_bar"),
            // RootWindow,
        ))
        .with_children(|parent| {
            // LEFT SLOT
            parent
                .spawn((
                    Node {
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    Name::new("left_slot"),
                ))
                .with_children(|left| match current_menu {
                    "sm" => spawn_date(left, date.clone(), current_menu, &font_assets),
                    "lg" => {
                        spawn_menu(left, &font_assets);
                        // spawn_logo_img(left, &image_assets);
                    }
                    _ => {}
                });

            // RIGHT SLOT
            parent
                .spawn((
                    Node {
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    Name::new("right_slot"),
                ))
                .with_children(|right| {
                    spawn_right_system(right, &font_assets, &right_tray_status);
                    spawn_system(right, &font_assets, &system_status);
                    if current_menu == "lg" {
                        spawn_date(right, date.clone(), current_menu, &font_assets);
                    }
                });
        });
}

/// Spawns the logo image in the left slot of the status bar
/// This function is called when the current menu is "lg"
fn spawn_logo_img(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    image_assets: &ImageAssets,
) {
    parent.spawn((
        ImageNode {
            image: image_assets.parachute.clone(),
            ..Default::default()
        },
        Node {
            width: Val::Vw(4.),
            height: Val::Vh(4.),
            ..Default::default()
        },
    ));
}

/// Spawns a basic menu button (placeholder for future dropdown)
fn spawn_menu(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
) {
    parent.spawn((StyledButton::builder()
        .text("Menu")
        .font(font_assets.secondary_700.clone())
        .variant(ButtonVariant::Secondary)
        .build(),));
}

/// Spawns the right system tray icons based on the current status
fn spawn_right_system(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
    right_tray_status: &RightTrayStatus,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|row| {
            if right_tray_status.headphones_connected {
                spawn_icon(row, Icon::Headphones, font_assets);
            }

            if right_tray_status.monitor_connected {
                spawn_icon(row, Icon::Monitor, font_assets);
            }

            if right_tray_status.terminal_active {
                spawn_icon(row, Icon::Terminal, font_assets);
            }

            if right_tray_status.usb_connected {
                spawn_icon(row, Icon::Usb, font_assets);
            }
        });
}

/// Spawns system status icons (Bluetooth, Wi-Fi, Battery, etc.)
fn spawn_system(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
    system_status: &SystemStatus,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|row| {
            if system_status.bluetooth_state != BluetoothState::Off {
                spawn_icon(
                    row,
                    bluetooth_icon_for_state(&system_status.bluetooth_state),
                    font_assets,
                );
            }

            spawn_icon(
                row,
                wifi_icon_for_state(&system_status.wifi_state),
                font_assets,
            );
            spawn_icon(
                row,
                mobile_network_icon_for_state(&system_status.mobile_network_state),
                font_assets,
            );
            spawn_icon(
                row,
                battery_icon_for_state(&system_status.battery_state),
                font_assets,
            );
        });
}

/// Displays current date and time based on formatting settings
fn spawn_date(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    date_settings: DateSettings,
    current_menu: &str,
    font_assets: &FontAssets,
) {
    let now = Local::now();

    let formatted_time = now.format(&date_settings.time).to_string();
    let formatted_date = now.format(&date_settings.date).to_string();
    let formatted_day = now.format(&date_settings.day).to_string();

    if current_menu == "sm" {
        parent
            .spawn((Node {
                padding: UiRect::vertical(Val::Px(8.0)),
                align_items: AlignItems::FlexStart,
                ..default()
            },))
            .with_children(|right| {
                right.spawn((
                    Text::new(format!("{} {}", formatted_date, formatted_time)),
                    TextFont {
                        font: font_assets.secondary_700.clone(),
                        font_size: 15.,
                        ..Default::default()
                    },
                ));
            });
    }

    if current_menu == "lg" {
        parent
            .spawn((Node {
                padding: UiRect::vertical(Val::Px(8.0)),
                align_items: AlignItems::FlexStart,
                ..default()
            },))
            .with_children(|right| {
                right.spawn((
                    Text::new(format!(
                        "{}, {} {}",
                        formatted_day, formatted_date, formatted_time
                    )),
                    TextFont {
                        font: font_assets.secondary_700.clone(),
                        font_size: 15.,
                        ..Default::default()
                    },
                ));
            });
    }
}

/// Spawns a generic icon inside a ghost-styled button
fn spawn_icon(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    icon: Icon,
    font_assets: &FontAssets,
) {
    let font_icons = font_assets.font_icons.clone();
    parent.spawn((StyledButton::builder()
        .icon(icon)
        .font(font_icons.clone())
        .variant(ButtonVariant::Ghost)
        .build(),));
}

/// Maps Wi-Fi state to appropriate icon
fn wifi_icon_for_state(state: &WifiState) -> Icon {
    match state {
        WifiState::ConnectedStrong => Icon::WifiConnectedStrong, // Placeholder for strong connection
        WifiState::Off => Icon::WifiOff,                         // Placeholder for off state
        WifiState::ConnectedMedium => Icon::WifiMedium,
        WifiState::ConnectedWeak => Icon::WifiLow,
        // WifiState::Connecting => Icon::WifiAnimated,             // Placeholder for animation
        WifiState::ConnectedNoInternet => Icon::WifiNoInternet, // Placeholder for warning
        WifiState::OnButNotConnected => Icon::WifiOnNotConnected, // Empty bars
    }
}

/// Maps bluetooth state to appropriate icon
fn bluetooth_icon_for_state(state: &BluetoothState) -> Icon {
    match state {
        BluetoothState::Connected => Icon::BluetoothConnected,
        BluetoothState::On => Icon::Bluetooth,
        _ => todo!(),
    }
}

/// Maps battery state to appropriate icon
fn battery_icon_for_state(state: &BatteryState) -> Icon {
    match state {
        BatteryState::Charging => Icon::BatteryCharging,
        BatteryState::LowBattery => Icon::BatteryLow,
        BatteryState::CriticallyLow => Icon::BatteryCriticallyLow,
        BatteryState::ChargedComplete => Icon::BatteryChargedComplete,
        BatteryState::NoBattery => Icon::BatteryNoBattery,
    }
}

/// Maps mobile network state to appropriate icon
fn mobile_network_icon_for_state(state: &MobileNetworkState) -> Icon {
    match state {
        MobileNetworkState::Full => Icon::SignalBarsFull,
        // MobileNetworkState::Medium => Icon::SignalBarsMedium,
        // MobileNetworkState::Low => Icon::SignalBarsLow,
        // MobileNetworkState::NoSimInserted => Icon::NoSim,
        // MobileNetworkState::NoSignal => Icon::NoSignal,
        _ => todo!(),
    }
}
