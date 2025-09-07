mod bar;
mod button_system;
pub mod setup;

use crate::components::styled_card::StyledCard;
use crate::components::Clock;
use crate::systems::button_system::HOVERED_BUTTON;
use crate::utils::{FontAssets, Icon};
use crate::widgets::button::StyledButton;
use crate::widgets::slider::{AccessibleName, StyledSlider};
use crate::{
    divider, list_popup, AirplaneMode, AirplaneModeEnabled, AutoRotation, Bluetooth,
    BluetoothEntry, BluetoothIcon, Brightness, ContainerNode, Microphone, MicrophoneEnabled,
    RotationEnabled, ScreenRecording, ScreenRecordingEnabled, Screens, SettingsDrawerRoot,
    SettingsItem, SettingsItemText, SettingsPanelBackgroud, SettingsPanelBackgroudEvent, Sound,
    StyledPopup, Wireless, WirelessEntry, WirelessIcon,
};
pub use bar::{on_bar_drag, on_bar_drag_end, on_bar_drag_start};
use bevy::ecs::system::SystemId;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;
use bevy_core_widgets::hover::Hovering;
use bevy_core_widgets::CoreSlider;
use bevy_styled_widgets::prelude::{StyledText, ThemeManager};
pub use button_system::{button_system, NORMAL_BUTTON};
use chrono::{Datelike, Timelike};
use headless_widgets::CoreButton;
use service_plugins::bluetooth::{
    BluetoothAction, BluetoothActionEvent, BluetoothEnabledStatus, ListPairedDevices,
};
use service_plugins::network_manager::{
    KnownNetworkList, NetworkAction, NetworkActionEvent, WirelessEnabled,
};
use service_plugins::pulse_audio::{DefaultSink, PulseAudioAction, PulseAudioActionEvent};
use service_plugins::upower::DevicePercentage;
use service_plugins::UPowerBatteryState;
pub use setup::exit_on_esc;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

pub fn get_current_datetime() -> String {
    let now = chrono::Local::now();
    format!(
        "{} {} {:02}:{:02}",
        now.day(),
        now.format("%B"),
        now.hour(),
        now.minute(),
    )
}
pub fn update_bluetooth_state(
    mut query: Query<&mut ImageNode, With<BluetoothIcon>>,
    bluetooth_state: Res<BluetoothEnabledStatus>,
    font_assets: Res<FontAssets>,
) {
    for mut bluetooth_icon in &mut query {
        info!("BluetoothEnabledStatus is updated :{:?}", bluetooth_state);
        if bluetooth_state.0 {
            bluetooth_icon.image = font_assets.bluetooth_on.clone();
        } else {
            bluetooth_icon.image = font_assets.bluetooth_off.clone();
        }
    }
}

pub fn poll_default_sink_volume(mut event_writer: EventWriter<PulseAudioActionEvent>) {
    event_writer.write(PulseAudioActionEvent(PulseAudioAction::GetDefaultSink));
}
pub fn update_wireless_state(
    mut query: Query<&mut ImageNode, With<WirelessIcon>>,
    wifi_state: Res<WirelessEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!(
        "WirelessEnabled is updated Settings drawer :{:?}",
        wifi_state
    );
    for mut wireless_image_node in &mut query {
        if wifi_state.0 {
            wireless_image_node.image = font_assets.wireless_on.clone();
        } else {
            wireless_image_node.image = font_assets.wireless_off.clone();
        }
    }
}

pub fn update_auto_rotation_state(
    mut query: Query<&mut StyledButton, With<AutoRotation>>,
    auto_rotation: Res<RotationEnabled>,
    font_assets: Res<FontAssets>,
) {
    for mut styled_button in &mut query {
        info!("RotationEnabled is updated :{:?}", auto_rotation);

        if auto_rotation.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.rotation_on.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.rotation_off.clone());
        }
    }
}
// pub fn update_wireless_list_state(
//     mut commands: Commands,
//     network_list: Res<KnownNetworkList>,
//     container_query: Query<Entity, With<ContainerNode>>,
//     children_query: Query<&Children>,
//     ui_node_query: Query<Entity, With<Node>>,
//     font_assets: Res<FontAssets>,
// ) {
//     println!("update_wireless_list_state {:?}", network_list.clone());
//     if network_list.is_changed() {
//         if let Ok(container_entity) = container_query.single() {
//             // First, remove existing UI elements inside the container
//             if let Ok(children) = children_query.get(container_entity) {
//                 for child in children.iter() {
//                     if ui_node_query.contains(child) {
//                         commands.entity(child).despawn();
//                     }
//                 }
//             }
//
//             for (i, network) in network_list.0.iter().enumerate() {
//                 let wireless_clone = network.clone();
//
//                 let on_click = commands.register_system(
//                     move |mut commands: Commands,
//                           q_status_text: Query<&mut StyledText, With<WirelessEntry>>,
//                           mut event_writer: EventWriter<NetworkActionEvent>| {
//                         println!("Wireless entry clicked : {:?} ", wireless_clone);
//
//                         if !wireless_clone.is_active {
//                             event_writer.write(NetworkActionEvent(
//                                 NetworkAction::ConnectToSavedNetwork(wireless_clone.ssid.clone()),
//                             ));
//                             // // Todo: change status to connecting
//                             // for mut text in q_status_text.iter() {
//                             //     // text.content = DeviceStatus::Connecting.to_string();
//                             // }
//                         }
//                     },
//                 );
//
//                 commands.entity(container_entity).with_children(|parent| {
//                     parent.spawn(wireless_clickable_row(
//                         &network.ssid,
//                         network.is_active,
//                         network.signal_strength,
//                         &network.security,
//                         &font_assets,
//                         on_click,
//                     ));
//                 });
//             }
//         }
//     }
// }

// pub fn update_bluetooth_list_state(
//     mut commands: Commands,
//     bluetooth_list: Res<ListPairedDevices>,
//     container_query: Query<Entity, With<ContainerNode>>,
//     children_query: Query<&Children>,
//     ui_node_query: Query<Entity, With<Node>>,
//     font_assets: Res<FontAssets>,
// ) {
//     println!("update_bluetooth_list_state {:?}", bluetooth_list.clone());
//     if bluetooth_list.is_changed() {
//         if let Ok(container_entity) = container_query.single() {
//             // First, remove existing UI elements inside the container
//             if let Ok(children) = children_query.get(container_entity) {
//                 for child in children.iter() {
//                     if ui_node_query.contains(child) {
//                         commands.entity(child).despawn();
//                     }
//                 }
//             }
//
//             // Now populate the container with the new wireless entries
//             for (i, bluetooth) in bluetooth_list.0.iter().enumerate() {
//                 let bluetooth_clone = bluetooth.clone();
//
//                 let on_click = commands.register_system(
//                     move |mut commands: Commands,
//                           q_status_text: Query<&mut StyledText, With<BluetoothEntry>>,
//                           mut event_writer: EventWriter<BluetoothActionEvent>| {
//                         println!("Bluetooth entry clicked : {:?} ", bluetooth_clone);
//
//                         if !bluetooth_clone.connected && bluetooth_clone.paired {
//                             event_writer.write(BluetoothActionEvent(
//                                 BluetoothAction::ConnectToDevice(bluetooth_clone.address.clone()),
//                             ));
//                         } else {
//                             event_writer.write(BluetoothActionEvent(
//                                 BluetoothAction::DisconnectDevice(bluetooth_clone.address.clone()),
//                             ));
//                         }
//                     },
//                 );
//
//                 commands.entity(container_entity).with_children(|parent| {
//                     parent.spawn(bluetooth_clickable_row(
//                         &bluetooth.name,
//                         bluetooth.connected,
//                         &font_assets,
//                         on_click,
//                     ));
//
//                     if i != bluetooth_list.0.len() - 1 {
//                         parent.spawn(divider());
//                     }
//                 });
//             }
//         }
//     }
// }

pub fn update_microphone_state(
    mut query: Query<&mut StyledButton, With<Microphone>>,
    state: Res<MicrophoneEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!("MicrophoneEnabled is updated :{:?}", state);
    for mut styled_button in &mut query {
        if state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.mic_on.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.mic_off.clone());
        }
    }
}

pub fn update_screen_recording_state(
    mut query: Query<&mut StyledButton, With<ScreenRecording>>,
    state: Res<ScreenRecordingEnabled>,
    font_assets: Res<FontAssets>,
) {
    info!("ScreenRecordingEnabled is updated :{:?}", state);
    for mut styled_button in &mut query {
        if state.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.screen_recording_on.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.screen_recording_off.clone());
        }
    }
}

pub fn set_initial_airplane_mode_state(
    mut query: Query<&mut StyledButton, With<AirplaneMode>>,
    font_assets: Res<FontAssets>,
    bluetooth_enabled: ResMut<BluetoothEnabledStatus>,
    wireless_enabled: ResMut<WirelessEnabled>,
) {
    for mut styled_button in &mut query {
        if !bluetooth_enabled.0 && !wireless_enabled.0 {
            // Airplane mode is activated
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.airplane_tilt.clone());
        } else {
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.airplane_tilt.clone());
        }
    }
}
pub fn update_airplane_mode_state(
    mut query: Query<&mut StyledButton, With<AirplaneMode>>,
    airplane_enabled: Res<AirplaneModeEnabled>,
    font_assets: Res<FontAssets>,
    mut network_event_writer: EventWriter<NetworkActionEvent>,
    mut bluetooth_event_writer: EventWriter<BluetoothActionEvent>,
    bluetooth_enabled: ResMut<BluetoothEnabledStatus>,
) {
    info!("AirplaneModeEnabled is updated :{:?}", airplane_enabled);
    for mut styled_button in &mut query {
        if airplane_enabled.0 {
            styled_button.active = Some(true);
            styled_button.icon = Some(font_assets.airplane_tilt.clone());
            bluetooth_event_writer.write(BluetoothActionEvent(BluetoothAction::ToggleBluetooth(
                !bluetooth_enabled.0,
            )));
            network_event_writer.write(NetworkActionEvent(NetworkAction::ToggleWifi(false)));
        } else {
            bluetooth_event_writer.write(BluetoothActionEvent(BluetoothAction::ToggleBluetooth(
                !bluetooth_enabled.0,
            )));
            network_event_writer.write(NetworkActionEvent(NetworkAction::ToggleWifi(true)));
            styled_button.active = Some(false);
            styled_button.icon = Some(font_assets.airplane_tilt.clone());
        }
    }
}

pub fn update_volume_state(
    mut query: Query<&mut StyledSlider, With<Sound>>,
    default_sink_info: Res<DefaultSink>,
) {
    for mut styled_slider in &mut query {
        let volume = default_sink_info.0.volume;
        let avg = volume.avg();
        let value = (avg.0 as f64 / 65536.0 * 100.0) as f32;
        info!("Volume is updated :{:?} ", value);
        styled_slider.value = value;
    }
}

pub fn update_popup_background(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut BackgroundColor, With<StyledPopup>>,
) {
    for mut bg_color in query.iter_mut() {
        let theme_styles = theme_manager.styles.clone();
        let color = theme_styles.popup.background_color;
        bg_color.0 = color;
    }
}

pub fn animate_settings_item_text(
    mut text_font_query: Query<(&mut TextFont, &SettingsItemText), With<SettingsItemText>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    for (mut text_font, styled_item_text) in text_font_query.iter_mut() {
        // Target size
        let start_font_size = 0.60 * styled_item_text.font_size;
        let target_font_size = styled_item_text.font_size;

        // Animation duration in seconds
        let animation_duration = 0.80;
        let elapsed = time.elapsed_secs();

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_font_size = start_font_size + (target_font_size - start_font_size) * eased_progress;

        println!("Progress 2: {}", new_font_size);

        // Update the font size
        text_font.font_size = new_font_size;
    }
}

pub fn animate_settings_item(
    mut settings_query: Query<&mut Node, With<SettingsItem>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
    mut anim_state: Local<Option<f32>>, // Option to store animation start time
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    // On first run (in this state), store the start time
    let start_time = anim_state.get_or_insert(time.elapsed_secs());

    for mut node in settings_query.iter_mut() {
        // Target size
        let target_width = 100.0;
        let target_height = 100.0;
        let start_width = 60.0;
        let start_height = 60.0;

        // Animation duration in seconds
        let animation_duration = 0.10;
        let elapsed = time.elapsed_secs() - *start_time;

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_width = start_width + (target_width - start_width) * eased_progress;
        let new_height = start_height + (target_height - start_height) * eased_progress;
        // Update the node dimensions
        node.width = Val::Percent(new_width);
        node.height = Val::Percent(new_height);
    }
}

pub fn toggle_mode(mut theme_manager: ResMut<ThemeManager>) {
    // let current_mode = theme_manager.current_mode;
    // let new_mode = match current_mode {
    //     ThemeMode::Light => ThemeMode::Dark,
    //     ThemeMode::Dark => ThemeMode::Light,
    // };
    // theme_manager.set_theme_mode(new_mode);
}

pub fn toggle_auto_rotation(mut enabled: ResMut<RotationEnabled>) {
    enabled.0 = !enabled.0;
}

pub fn toggle_wireless(
    enabled: ResMut<WirelessEnabled>,
    mut event_writer: EventWriter<NetworkActionEvent>,
) {
    event_writer.write(NetworkActionEvent(NetworkAction::ToggleWifi(!enabled.0)));
}

// pub fn change_sound_volume(
//     mut volume: ResMut<DefaultSinkVolume>,
//     mut event_writer: EventWriter<PulseAudioActionEvent>,
// ) {
//     // TODO:
//     // get sink name and set volume
//     // event_writer.write(PulseAudioActionEvent(PulseAudioAction::SetDefaultSink());
// }

pub fn long_press_wireless(
    mut commands: Commands,
    mut q_settings_drawer: Query<Entity, With<SettingsDrawerRoot>>,
    mut event_writer: EventWriter<NetworkActionEvent>,
    font_assets: Option<Res<FontAssets>>,
) {
    println!("long press wireless");
    event_writer.write(NetworkActionEvent(NetworkAction::ListKnownNetworks));
    for entity in q_settings_drawer.iter_mut() {
        let popup_id = commands.spawn_empty().id();
        let popup = list_popup(&mut commands, font_assets.as_ref().unwrap(), "Wi-Fi");
        commands.entity(popup_id).insert(popup);
        commands.entity(entity).add_child(popup_id);
    }
}

pub fn long_press_bluetooth(
    mut commands: Commands,
    mut q_settings_drawer: Query<Entity, With<SettingsDrawerRoot>>,
    mut event_writer: EventWriter<BluetoothActionEvent>,
    font_assets: Option<Res<FontAssets>>,
) {
    println!("long press bluetooth");
    event_writer.write(BluetoothActionEvent(BluetoothAction::ListPairedDevices));
    for entity in q_settings_drawer.iter_mut() {
        let popup_id = commands.spawn_empty().id();
        let popup = list_popup(&mut commands, font_assets.as_ref().unwrap(), "Bluetooth");
        commands.entity(popup_id).insert(popup);
        commands.entity(entity).add_child(popup_id);
    }
}

pub fn toggle_bluetooth(
    mut enabled: ResMut<BluetoothEnabledStatus>,
    mut event_writer: EventWriter<BluetoothActionEvent>,
) {
    enabled.0 = !enabled.0;
    event_writer.write(BluetoothActionEvent(BluetoothAction::ToggleBluetooth(
        enabled.0,
    )));
}

pub fn toggle_airplane_mode(mut enabled: ResMut<AirplaneModeEnabled>) {
    enabled.0 = !enabled.0;
}

pub fn toggle_screen_recording(mut enabled: ResMut<ScreenRecordingEnabled>) {
    enabled.0 = !enabled.0;
}

pub fn toggle_microphone(mut enabled: ResMut<MicrophoneEnabled>) {
    enabled.0 = !enabled.0;
}

// pub fn change_sound_volume(mut enabled: ResMut<DefaultSink>) {
//     let volume = enabled.0.volume;
//     enabled.0 = !enabled.0;
// }
pub fn on_animation_background_completed(
    mut commands: Commands,
    mut event_reader: EventReader<SettingsPanelBackgroudEvent>,
    mut bg_query: Query<Entity, With<SettingsPanelBackgroud>>,
    font_assets: Option<Res<FontAssets>>,
    mut screens_state: ResMut<NextState<Screens>>,
    battery_percentage: Res<DevicePercentage>,
) {
    let on_toggle_theme_mode = commands.register_system(toggle_mode);
    let on_toggle_auto_rotation = commands.register_system(toggle_auto_rotation);
    let on_toggle_wireless = commands.register_system(toggle_wireless);
    let on_long_press_wireless = commands.register_system(long_press_wireless);
    let on_long_press_bluetooth = commands.register_system(long_press_bluetooth);
    let on_toggle_bluetooth = commands.register_system(toggle_bluetooth);
    let on_toggle_airplane_mode = commands.register_system(toggle_airplane_mode);
    let on_toggle_screen_recording = commands.register_system(toggle_screen_recording);
    let on_toggle_microphone = commands.register_system(toggle_microphone);
    // let on_change_sound_volume = commands.register_system(change_sound_volume);

    if font_assets.is_none() {
        return;
    }
    let battery_10 = font_assets.as_ref().unwrap().battery_10.clone();
    for _event in event_reader.read() {
        // Remove the background component when the animation is completed
        println!("Animation completed");
        if let Ok(entity) = bg_query.single_mut() {
            println!("Entity: {:?}", entity);
            // Add new childs
            commands.entity(entity).with_children(|parent| {
                //spawn settings edit and power

                parent.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    children![(
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![
                            (
                                Text::new(get_current_datetime()),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                TextShadow::default()
                            ),
                            (
                                Node {
                                    align_items: AlignItems::Center, // This aligns text and icon vertically
                                    ..default()
                                },
                                children![
                                    (
                                        Text::new(format!("{}%", battery_percentage.0)),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        TextShadow::default()
                                    ),
                                    (Node {
                                        margin: UiRect::right(Val::Px(1.)),
                                        ..default()
                                    }),
                                    (
                                        ImageNode::new(battery_10.clone()),
                                        Node {
                                            width: Val::Px(28.),
                                            height: Val::Px(28.),
                                            ..Default::default()
                                        }
                                    ),
                                ]
                            ),
                            (
                                Node { ..default() },
                                children![
                                    (
                                        StyledText::builder()
                                            .content(Icon::Settings.to_string())
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .font_size(24.)
                                            .build(),
                                        Node { ..default() }
                                    ),
                                    (
                                        StyledText::builder()
                                            .content(Icon::Power.to_string())
                                            .font(font_assets.as_ref().unwrap().font_icons.clone())
                                            .font_size(24.)
                                            .build(),
                                        Node {
                                            margin: UiRect::right(Val::Px(16.)),
                                            ..default()
                                        }
                                    )
                                ]
                            )
                        ]
                    ),],
                ));

                let FontAssets {
                    airplane_tilt,
                    rotation_on,
                    rotation_off,
                    second_screen_on,
                    second_screen_off,
                    power_mode_none,
                    power_mode_low,
                    power_mode_high,
                    mic_on,
                    mic_off,
                    screen_recording_on,
                    screen_recording_off,
                    calculator,
                    camera,
                    sound_low,
                    brightness_low,
                    wireless_off,
                    bluetooth_off,
                    terminal,
                    cell_signal_none,
                    ..
                } = &**font_assets.as_ref().unwrap();

                //spawn settings items
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(262.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                            row_gap: Val::Px(28.0),
                            column_gap: Val::Px(32.0),
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect {
                                left: Val::Px(21.),
                                right: Val::Px(21.),
                                top: Val::Px(22.),
                                bottom: Val::Px(22.),
                            },
                            margin: UiRect::top(Val::Px(14.5)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(12.0)),
                        BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.8)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            AutoRotation,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![(
                                    ImageNode::new(rotation_on.clone()),
                                    Node {
                                        width: Val::Percent(75.),
                                        height: Val::Percent(75.),
                                        ..Default::default()
                                    }
                                )],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_auto_rotation),
                                },
                            ),
                        ));

                        parent.spawn((
                            AirplaneMode,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![(
                                    ImageNode::new(airplane_tilt.clone()),
                                    Node {
                                        width: Val::Percent(75.),
                                        height: Val::Percent(75.),
                                        ..Default::default()
                                    }
                                )],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_airplane_mode),
                                },
                            ),
                        ));

                        // Screen
                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(second_screen_off.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(HOVERED_BUTTON),
                            Button,
                            CoreButton { on_click: None },
                        ));

                        // Power
                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(power_mode_high.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(NORMAL_BUTTON),
                            Button,
                            CoreButton { on_click: None },
                        ));

                        // Microphone
                        parent.spawn((
                            Microphone,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![(
                                    ImageNode::new(mic_off.clone()),
                                    Node {
                                        width: Val::Percent(75.),
                                        height: Val::Percent(75.),
                                        ..Default::default()
                                    }
                                )],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_microphone),
                                },
                            ),
                        ));

                        // Screen Recording
                        parent.spawn((
                            ScreenRecording,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![(
                                    ImageNode::new(screen_recording_off.clone()),
                                    Node {
                                        width: Val::Percent(75.),
                                        height: Val::Percent(75.),
                                        ..Default::default()
                                    }
                                )],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_screen_recording),
                                },
                            ),
                        ));

                        // Calculator
                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(calculator.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(NORMAL_BUTTON),
                            Button,
                            CoreButton { on_click: None },
                        ));

                        // Camera
                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(camera.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(NORMAL_BUTTON),
                            Button,
                            CoreButton { on_click: None },
                        ));
                    });
                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(74.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(32.0),
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(20.)),
                        ..default()
                    },))
                    .with_children(|parent| {
                        // Spawn the settings items
                        parent.spawn((
                            Sound,
                            StyledSlider::builder()
                                .icon(sound_low.clone())
                                .max(100.)
                                .min(0.)
                                .value(10.)
                                // .on_change(on_change_sound_volume)
                                .build(),
                        ));
                        parent.spawn((
                            Brightness,
                            StyledSlider::builder()
                                .icon(brightness_low.clone())
                                .max(100.)
                                .min(0.)
                                .value(50.)
                                .build()),
                        );
                    });

                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(95.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                            row_gap: Val::Px(0.0),
                            column_gap: Val::Px(40.0),
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::top(Val::Px(20.)),
                            ..default()
                        },
                        // BackgroundColor(Color::linear_rgba(1., 1., 1., 0.8)),
                    ))
                    .with_children(|parent| {
                        // Spawn the settings items
                        parent.spawn((
                            Wireless,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column, // stack vertically
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![
                                    (
                                        WirelessIcon,
                                        ImageNode::new(wireless_off.clone()),
                                        Node {
                                            width: Val::Percent(75.),
                                            height: Val::Percent(75.),
                                            ..Default::default()
                                        }
                                    ),
                                    (
                                        // Text below icon
                                        Text::new("office"),
                                        TextFont {
                                            font_size: 12.0,
                                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        TextShadow::default()
                                    )
                                ],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_wireless),
                                },
                            ),
                        ));

                        parent.spawn((
                            Bluetooth,
                            (
                                Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                children![
                                    (
                                        BluetoothIcon,
                                        ImageNode::new(bluetooth_off.clone()),
                                        Node {
                                            width: Val::Percent(75.),
                                            height: Val::Percent(75.),
                                            ..Default::default()
                                        }
                                    ),
                                    (
                                        Text::new("office"),
                                        TextFont {
                                            font_size: 12.0,
                                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        TextShadow::default()
                                    )
                                ],
                                BorderRadius::all(Val::Px(11.82)),
                                BackgroundColor(NORMAL_BUTTON),
                                Button,
                                CoreButton {
                                    on_click: Some(on_toggle_airplane_mode),
                                },
                            ),
                        ));

                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(terminal.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(NORMAL_BUTTON),
                            Button,
                            CoreButton {
                                on_click: Some(on_toggle_airplane_mode),
                            },
                        ));

                        parent.spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            children![(
                                ImageNode::new(cell_signal_none.clone()),
                                Node {
                                    width: Val::Percent(75.),
                                    height: Val::Percent(75.),
                                    ..Default::default()
                                }
                            )],
                            BorderRadius::all(Val::Px(11.82)),
                            BackgroundColor(NORMAL_BUTTON),
                            Button,
                            CoreButton {
                                on_click: Some(on_toggle_airplane_mode),
                            },
                        ));
                    });
            });
            screens_state.set(Screens::SettingsDrawer);
        }
    }
}

fn get_icon_by_percentage(device_percentage: f64) -> String {
    match device_percentage.round() as u32 {
        p if p >= 95 => Icon::BatteryFull.into(),
        p if p >= 90 => Icon::BatteryFull.into(),
        p if p >= 80 => Icon::BatteryFull.into(),
        p if p >= 70 => Icon::BatteryFull.into(),
        p if p >= 60 => Icon::BatteryFull.into(),
        p if p >= 50 => Icon::BatteryFull.into(),
        p if p >= 40 => Icon::BatteryFull.into(),
        p if p >= 30 => Icon::BatteryFull.into(),
        p if p >= 20 => Icon::BatteryFull.into(),
        p if p >= 10 => Icon::BatteryEmpty.into(),
        _ => {
            // Default fallback icon
            Icon::BatteryEmpty.into()
        }
    }
}

pub fn animate_background(
    mut bg_query: Query<&mut Node, With<SettingsPanelBackgroud>>,
    time: Res<Time>,
    mut is_completed: Local<bool>,
    mut event_writer: EventWriter<SettingsPanelBackgroudEvent>,
) {
    if *is_completed {
        // If the animation is completed, do nothing
        return;
    }

    if let Ok(mut node) = bg_query.single_mut() {
        // Target size
        let target_width = 100.0;
        let target_height = 100.0;
        let start_width = 10.0;
        let start_height = 10.0;

        // Animation duration in seconds
        let animation_duration = 0.6;
        let elapsed = time.elapsed_secs();

        // Calculate progress (0.0 to 1.0)
        let progress = (elapsed / animation_duration).min(1.0);

        println!("Progress: {}", progress);

        if progress >= 1.0 {
            // Animation completed, set the flag to true
            *is_completed = true;
            event_writer.write(SettingsPanelBackgroudEvent);
        }

        // Ease-in cubic function: t^3
        let eased_progress = progress * progress * progress;

        // Interpolate between start and target using eased progress
        let new_width = start_width + (target_width - start_width) * eased_progress;
        let new_height = start_height + (target_height - start_height) * eased_progress;

        // Update the node dimensions
        node.width = Val::Percent(new_width);
        node.height = Val::Percent(new_height);
    }
}
