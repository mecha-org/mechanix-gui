mod components;
mod systems;
mod utils;
mod widgets;

use crate::components::styled_card::{StyledCard, StyledCardPlugin};
use crate::systems::{AssetsLoadingState, exit_on_esc, animate_background, on_animation_background_completed, animate_settings_item, animate_settings_item_text, poll_default_sink_volume, update_wireless_state, set_initial_airplane_mode_state, update_bluetooth_state, update_volume_state, update_airplane_mode_state};
use crate::{
    utils::{FontAssets, Icon},
    widgets::{
        LauncherStyledWidgetsPlugin,
        button::{StyledButton},
    },
};
use bevy::{
    asset::meta::Settings,
    ecs::system::SystemId,
    prelude::*,
    reflect::List,
};
use bevy_asset_loader::prelude::*;
use bevy_core_widgets::{CoreButton, CoreScrollArea};
use service_plugins::upower::UPowerPlugin;
use service_plugins::{
    BluetoothPlugin, NetworkManagerPlugin, PulseAudioPlugin,
    bluetooth::{BluetoothAction, BluetoothActionEvent, BluetoothEnabledStatus, ListPairedDevices},
    network_manager::{
        ActiveNetworkStrength, KnownNetworkList, NetworkAction, NetworkActionEvent, WirelessEnabled,
    },
    pulse_audio::{DefaultSink, PulseAudioAction, PulseAudioActionEvent},
};
use bevy_styled_widgets::{
    StyledWidgetsPlugin,
    prelude::{StyledText, StyledTextPlugin, ThemeManager},
};
use core::fmt;
use std::time::Duration;
use bevy::time::common_conditions::on_timer;
use crate::systems::setup::{pre_setup, SettingsDrawerCamera};
use crate::widgets::slider::StyledSliderPlugin;

#[derive(Debug, States, Hash, Clone, Eq, PartialEq)]
pub enum Screens {
    Loading,
    Homescreen,
    SettingsDrawerAnimating,
    SettingsDrawer,
}

#[derive(Component)]
pub struct SettingsDrawerRoot;

#[derive(Component)]
pub struct SettingsDrawerPopup;

#[derive(Component)]
struct HeaderNode;

#[derive(Component)]
struct ContainerNode;

#[derive(Component)]
struct WirelessEntry;

#[derive(Component)]
struct BluetoothEntry;

#[derive(Component)]
pub struct Wireless;

#[derive(Component)]
pub struct Sound;

#[derive(Component)]
pub struct Brightness;

#[derive(Component)]
pub struct AutoRotation;

#[derive(Component)]
pub struct AirplaneMode;

#[derive(Component)]
pub struct Bluetooth;

#[derive(Component)]
pub struct Microphone;

#[derive(Component)]
pub struct ScreenRecording;

#[derive(Resource, Default, Debug, Clone)]
pub struct RotationEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct AirplaneModeEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct PowerSavingEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct MicrophoneEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct ScreenRecordingEnabled(pub bool);

#[derive(Event)]
pub struct SettingsPanelBackgroudEvent;

#[derive(Component)]
pub struct SettingsPanelBackgroud;

#[derive(Component)]
pub struct SettingsItem;

#[derive(Component)]
pub struct SettingsItemText {
    pub font_size: f32,
}

#[derive(Component)]
pub struct StyledPopup;

// network or bluetooth device state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceStatus {
    Connecting,
    Disconnecting,
    Connected,
    Unknown,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceStatus::Connecting => write!(f, "Connecting"),
            DeviceStatus::Disconnecting => write!(f, "Disconnecting"),
            DeviceStatus::Connected => write!(f, "Connected"),
            DeviceStatus::Unknown => write!(f, ""),
        }
    }
}

pub struct SettingsDrawerPlugin;

pub mod prelude {
    pub use crate::SettingsDrawerPlugin;
}

impl Plugin for SettingsDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, pre_setup);
        app.add_systems(OnEnter(AssetsLoadingState::Loaded), spawn_settings_drawer);
        app.add_systems(Update, exit_on_esc);
        app.add_event::<SettingsPanelBackgroudEvent>();
        app.add_plugins(NetworkManagerPlugin)
            .add_plugins(BluetoothPlugin)
            .add_plugins(UPowerPlugin)
            .add_plugins(PulseAudioPlugin)
            .add_plugins(StyledWidgetsPlugin)
            .add_plugins(StyledCardPlugin)
            .add_plugins(LauncherStyledWidgetsPlugin);
        app.insert_state(Screens::SettingsDrawer);
        app.insert_resource(ThemeManager::default());
        app.insert_resource(RotationEnabled(true));
        app.insert_resource(AirplaneModeEnabled(false));
        app.init_resource::<PowerSavingEnabled>();
        app.init_resource::<MicrophoneEnabled>();
        app.init_resource::<ScreenRecordingEnabled>();

        app.init_state::<AssetsLoadingState>().add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<FontAssets>(),
        );
        app.add_systems(
            Update,
            (
                // update_popup_background,
                animate_background,
                on_animation_background_completed,
            ),
        );

        app.add_systems(
            Update,
            (
                animate_settings_item.run_if(in_state(Screens::SettingsDrawer)),
                animate_settings_item_text.run_if(in_state(Screens::SettingsDrawer)),
            ),
        );

        app.add_systems(Update, poll_default_sink_volume.run_if(on_timer(Duration::from_secs(5))));
        app.add_systems(
            OnEnter(Screens::SettingsDrawer),
            (
                update_wireless_state,
                set_initial_airplane_mode_state,
                update_bluetooth_state,
                update_volume_state,
                // update_microphone_state,
                // update_screen_recording_state,
                // update_auto_rotation_state,
            )
                .run_if(resource_exists::<FontAssets>),
        );

        app.add_systems(
            Update,
            (
                update_wireless_state
                    .run_if(resource_changed::<WirelessEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_airplane_mode_state
                    .run_if(resource_changed::<AirplaneModeEnabled>)
                    .run_if(resource_exists::<FontAssets>),
                update_bluetooth_state
                    .run_if(resource_changed::<BluetoothEnabledStatus>)
                    .run_if(resource_exists::<FontAssets>),
                update_volume_state
                    .run_if(resource_changed::<DefaultSink>)
                    .run_if(resource_exists::<FontAssets>),
                // update_bluetooth_list_state
                //     .run_if(resource_changed::<ListPairedDevices>)
                //     .run_if(resource_exists::<FontAssets>),
                // update_wireless_list_state
                //     .run_if(resource_changed::<KnownNetworkList>)
                //     .run_if(resource_exists::<FontAssets>),
                // update_microphone_state
                //     .run_if(resource_changed::<MicrophoneEnabled>)
                //     .run_if(resource_exists::<FontAssets>),
                // update_screen_recording_state
                //     .run_if(resource_changed::<ScreenRecordingEnabled>)
                //     .run_if(resource_exists::<FontAssets>),
                // update_auto_rotation_state
                //     .run_if(resource_changed::<RotationEnabled>)
                //     .run_if(resource_exists::<FontAssets>),
            ),
        );
    }
}

pub fn spawn_settings_drawer(
    mut commands: Commands,
    camera: Res<SettingsDrawerCamera>,
    theme_manager: Res<ThemeManager>,
) {
    let settings_drawer = settings_drawer(&theme_manager);
    commands.spawn((
        UiTargetCamera(camera.0),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        children![settings_drawer],
    ));
}

fn setup2(mut commands: Commands, theme_manager: Res<ThemeManager>) {
    commands.spawn((
        Camera2d,
        Camera {
            ..Default::default()
        },
    ));
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        children![
            (Text::new("Bevy"), TextColor(Color::linear_rgb(0., 0., 0.))),
            (
                Node {
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(40.),
                    left: Val::Percent(40.),
                    ..Default::default()
                },
                BackgroundColor(Color::linear_rgba(0., 0., 0., 0.8))
            ),
        ],
    ));
}

pub fn settings_drawer( theme_manager: &ThemeManager) -> impl Bundle {
    (
        //make this transparent
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            ..default()
        },
        SettingsDrawerRoot,
        children![
            (
                Node {
                    width: Val::Percent(10.0),
                    height: Val::Percent(10.0),
                    padding: UiRect {
                        left: Val::Px(32.),
                        right: Val::Px(32.),
                        top: Val::Px(30.),
                        bottom: Val::Px(30.)
                    },
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme_manager.styles.panel.background_color),
                StyledCard,
                SettingsPanelBackgroud,
            ),
            // (
            //     Node {
            //         width: Val::Percent(50.0),
            //         height: Val::Percent(50.0),
            //         padding: UiRect {
            //             left: Val::Px(32.),
            //             right: Val::Px(32.),
            //             top: Val::Px(30.),
            //             bottom: Val::Px(30.)
            //         },
            //         display: Display::Flex,
            //         flex_direction: FlexDirection::Column,
            //         align_items: AlignItems::Center,
            //         align_self: AlignSelf::Center,
            //         justify_self: JustifySelf::Center,
            //         position_type: PositionType::Absolute,
            //         left: Val::Percent(50.),
            //         ..default()
            //     },
            //     BackgroundColor(theme_manager.styles.popup.background_color),
            //     StyledPopup,
            //     ZIndex(999)
            // )
        ],
    )
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
    control_name: &str,
    on_click: SystemId,
) {
    let mut grid_column = GridPlacement::span(1);

    if control_name == "brightness" || control_name == "volume" {
        grid_column = GridPlacement::span(4);
    };

    let FontAssets { font_icons, .. } = font_assets;

    // let click_system_id = parent
    //     .commands()
    //     .register_system(control_click_system(control_name.to_string()));

    let (icon, layout, on_press_icon, on_press_layout) = match control_name {
        "airplane_mode" => (
            font_assets.airplane_off.clone(),
            font_assets.layout_airplane.clone(),
            font_assets.airplane_on.clone(),
            font_assets.layout_airplane.clone(),
        ),

        "auto_rotation" => (
            font_assets.rotation_off.clone(),
            font_assets.layout_rotation.clone(),
            font_assets.rotation_on.clone(),
            font_assets.layout_rotation.clone(),
        ),
        "screen_record" => (
            font_assets.screen_recording_off.clone(),
            font_assets.layout_screen_recording.clone(),
            font_assets.screen_recording_on.clone(),
            font_assets.layout_screen_recording.clone(),
        ),
        "wifi" => (
            font_assets.gray_wireless_medium.clone(),
            font_assets.layout_wireless.clone(),
            font_assets.gray_wireless_off.clone(),
            font_assets.layout_wireless.clone(),
        ),
        "bluetooth" => (
            font_assets.bluetooth_off.clone(),
            font_assets.layout_bluetooth.clone(),
            font_assets.bluetooth_on.clone(),
            font_assets.layout_bluetooth.clone(),
        ),
        "camera" => (
            font_assets.camera.clone(),
            font_assets.layout_camera.clone(),
            font_assets.camera_pressed.clone(),
            font_assets.layout_camera.clone(),
        ),
        "battery" => (
            font_assets.power_saving_off.clone(),
            font_assets.layout_power_saving.clone(),
            font_assets.power_saving_on.clone(),
            font_assets.layout_power_saving.clone(),
        ),
        "terminal" => (
            font_assets.terminal.clone(),
            font_assets.layout_terminal.clone(),
            font_assets.terminal.clone(),
            font_assets.layout_terminal.clone(),
        ),
        "voice_record" => (
            font_assets.microphone_off.clone(),
            font_assets.layout_microphone.clone(),
            font_assets.microphone_on.clone(),
            font_assets.layout_microphone.clone(),
        ),
        "brightness" => (
            font_assets.brightness_low.clone(),
            font_assets.layout_brightness.clone(),
            font_assets.brightness_low.clone(),
            font_assets.layout_brightness.clone(),
        ),
        "volume" => (
            font_assets.sound_low.clone(),
            font_assets.layout_sound.clone(),
            font_assets.sound_low.clone(),
            font_assets.layout_sound.clone(),
        ),
        // "screen_mirror" => (
        //     font_assets.screen_mirroring_off.clone(),
        //     font_assets.screen_mirroring_off_layout.clone(),
        // ),
        "calculator" => (
            font_assets.calculator.clone(),
            font_assets.layout_calculator.clone(),
            font_assets.calculator.clone(),
            font_assets.layout_calculator.clone(),
        ),
        "cellular" => (
            font_assets.cell_signal_high.clone(),
            font_assets.layout_cell_signal.clone(),
            font_assets.cell_signal_high.clone(),
            font_assets.layout_cell_signal.clone(),
        ),
        _ => (
            font_assets.gray_wireless_medium.clone(),
            font_assets.layout_wireless.clone(),
            font_assets.gray_wireless_off.clone(),
            font_assets.layout_wireless.clone(),
        ),
    };

    parent.spawn((StyledButton::builder()
        .icon(icon)
        .layout(layout)
        .font(font_icons.clone())
        .active(false)
        .on_press_icon(on_press_icon)
        .on_press_layout(on_press_layout)
        .active_background_color(Color::oklcha(0.7878, 0.1643, 75.13, 0.90))
        .on_click(on_click)
        .build(),));

    // parent.spawn((
    //     Node {
    //         width: Val::Percent(60.0),
    //         height: Val::Percent(60.0),
    //         display: Display::Flex,
    //         flex_direction: FlexDirection::Column,
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         grid_column,
    //         grid_row: GridPlacement::span(1),
    //         ..default()
    //     },
    //     BackgroundColor(Color::linear_rgb(0.85, 0.85, 0.85)),
    //     BorderRadius::all(Val::Px(12.0)),
    //     SettingsItem,
    //     Children::spawn(Spawn((
    //         Text::new(icon),
    //         TextFont {
    //             font: font_assets.font_icons.clone(),
    //             font_size: 16.0,
    //             ..Default::default()
    //         },
    //         TextColor(Color::linear_rgba(0.24, 0.24, 0.24, 1.)),
    //         SettingsItemText,
    //     ))),
    // ));
}

fn popup_click(
    mut commands: Commands,
    mut q_settings_drawer: Query<Entity, With<SettingsDrawerPopup>>,
) {
    for entity in q_settings_drawer.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn list_popup(
    commands: &mut Commands,
    font_assets: &FontAssets,
    header_text: &str,
) -> impl Bundle {
    let on_popup_click = commands.register_system(popup_click);
    let FontAssets {
        settings_icon,
        layout_settings,
        ..
    } = font_assets.clone();

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            right: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        SettingsDrawerPopup,
        CoreButton {
            on_click: Some(on_popup_click),
            on_long_press: None,
        },
        BackgroundColor(Color::oklch(0.173, 0., 0.)),
        children![(
            Node {
                width: Val::Px(432.),
                height: Val::Px(333.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0),
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(24.)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius {
                        top_left: Val::Px(12.0),
                        top_right: Val::Px(12.0),
                        bottom_left: Val::Px(0.0),
                        bottom_right: Val::Px(0.0),
                    },
                    BackgroundColor(Color::oklch(0.4313, 0., 0.)),
                    children![
                        HeaderNode,
                        StyledText::new(header_text),
                        (
                            Node {
                                align_self: AlignSelf::End,
                                ..default()
                            },
                            children![
                                StyledButton::builder()
                                    .icon(settings_icon.clone())
                                    .layout(layout_settings.clone())
                                    .build(),
                            ]
                        )
                    ]
                ),
                (
                    Node {
                        width: Val::Percent(100.),
                        // height: Val::Percent(80.),
                        height: Val::Px(266.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.),
                        overflow: Overflow::scroll_y(),
                        padding: UiRect {
                            left: Val::Px(24.),
                            right: Val::Px(24.),
                            top: Val::Px(1.),
                            bottom: Val::Px(1.),
                        },
                        ..default()
                    },
                    CoreScrollArea,
                    ScrollPosition {
                        offset_x: 0.0,
                        offset_y: 0.0,
                    },
                    ContainerNode,
                    BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.90)),
                    BorderRadius {
                        top_left: Val::Px(0.0),
                        top_right: Val::Px(0.0),
                        bottom_left: Val::Px(12.0),
                        bottom_right: Val::Px(12.0),
                    },
                    // children![
                    //     Text::new("Loading data"),
                    //     TextFont {
                    //         font_size: 16.,
                    //         ..Default::default()
                    //     },
                    // ]
                )
            ]
        )],
    )
}

fn wireless_clickable_row(
    name: &str,
    is_active: bool,
    active_network_strength: u8,
    security: &str,
    font_assets: &FontAssets,
    on_click: SystemId,
) -> impl Bundle {
    let status = get_device_status(is_active);

    let wireless_icon =
        get_wireless_icon(is_active, security, active_network_strength, font_assets);

    let icon_size = 24.;

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(26.0),
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::vertical(Val::Px(20.)),
            ..default()
        },
        CoreButton {
            on_click: Some(on_click),
            on_long_press: None,
        },
        WirelessEntry,
        children![
            ImageNode::from_atlas_image(
                wireless_icon.clone(),
                TextureAtlas::from(font_assets.layout_wireless.clone()),
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(0.),
                        top: Val::Px(4.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                children![(StyledText::new(name),)]
            ),
            StyledText::new(status),
        ],
    )
}

fn get_wireless_icon(
    is_active: bool,
    security: &str,
    strength: u8,
    font_assets: &FontAssets,
) -> Handle<Image> {
    let FontAssets {
        gray_wireless_low,
        gray_wireless_medium,
        gray_wireless_high,

        gray_secured_wireless_low,
        gray_secured_wireless_medium,
        gray_secured_wireless_high,

        blue_wireless_low,
        blue_wireless_medium,
        blue_wireless_high,

        blue_secured_wireless_low,
        blue_secured_wireless_medium,
        blue_secured_wireless_high,
        layout_wireless,
        ..
    } = font_assets.clone();

    match (is_active, security) {
        (true, "Protected") => match strength {
            (0..=20) => blue_secured_wireless_low,
            (21..=50) => blue_secured_wireless_low,
            (51..=75) => blue_secured_wireless_medium,
            (76..=100) => blue_secured_wireless_high,
            _ => unreachable!(),
        },
        (true, "Open") => match strength {
            (0..=20) => blue_wireless_low,
            (21..=50) => blue_wireless_low,
            (51..=75) => blue_wireless_medium,
            (76..=100) => blue_wireless_high,
            _ => unreachable!(),
        },
        (false, "Protected") => match strength {
            (0..=20) => gray_secured_wireless_low,
            (21..=50) => gray_secured_wireless_low,
            (51..=75) => gray_secured_wireless_medium,
            (76..=100) => gray_secured_wireless_high,
            _ => unreachable!(),
        },
        (false, "Open") => match strength {
            (0..=20) => gray_wireless_low,
            (21..=50) => gray_wireless_low,
            (51..=75) => gray_wireless_medium,
            (76..=100) => gray_wireless_high,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

// Function to create the status string based on is_active
fn get_device_status(is_active: bool) -> String {
    if is_active {
        DeviceStatus::Connected.to_string()
    } else {
        DeviceStatus::Unknown.to_string()
    }
}

fn bluetooth_clickable_row(
    name: &str,
    is_active: bool,
    font_assets: &FontAssets,
    on_click: SystemId,
) -> impl Bundle {
    let status: String = if is_active {
        DeviceStatus::Connected
    } else {
        DeviceStatus::Unknown
    }
    .to_string();

    let FontAssets {
        bluetooth_on,
        bluetooth_off,
        settings_icon,
        layout_bluetooth,
        layout_settings,
        ..
    } = font_assets.clone();

    let bluetooth_icon = if is_active {
        bluetooth_on
    } else {
        bluetooth_off
    };
    let icon_size = 24.;

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(26.0),
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::vertical(Val::Px(20.)),
            ..default()
        },
        CoreButton {
            on_click: Some(on_click),
            on_long_press: None,
        },
        BluetoothEntry,
        children![
            ImageNode::from_atlas_image(
                bluetooth_icon.clone(),
                TextureAtlas::from(layout_bluetooth.clone()),
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(8.),
                        right: Val::Px(0.),
                        top: Val::Px(4.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                children![(StyledText::new(name),)]
            ),
            StyledText::new(status)
        ],
    )
}

fn divider() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(Color::oklcha(0.4054, 0., 0., 0.80)),
    )
}
