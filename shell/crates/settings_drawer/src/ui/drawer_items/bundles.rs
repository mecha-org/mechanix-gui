use bevy::{color::palettes::css::WHITE, ecs::system::SystemId, prelude::*};
use types::prelude::FontAssets;

use crate::{
    icons::SettingsDrawerIcons,
    setup::WINDOW_SIZE,
    ui::{
        BAR_SIZE, DrawerItemsRoot, airplane_mode, auto_rotation, battery, bluetooth, brightness,
        calculator, camera, cellular, microphone, on_airplane_mode_click, on_bluetooth_click,
        on_brightness_change, on_calculator_click, on_camera_click, on_cellular_click,
        on_microphone_click, on_power_saving_mode_click, on_rotation_click,
        on_screen_recording_click, on_screen_sharing_click, on_terminal_click, on_volume_change,
        on_wireless_click, power, power_saving_mode, screen_recording, screen_sharing, settings,
        terminal, volume, wireless,
    },
};

pub fn drawer_items(
    commands: &mut Commands,
    fonts: &FontAssets,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    let on_rotation_click = commands.register_system(on_rotation_click);
    let on_airplane_mode_click = commands.register_system(on_airplane_mode_click);
    let on_microphone_click = commands.register_system(on_microphone_click);
    let on_screen_recording_click = commands.register_system(on_screen_recording_click);
    let on_screen_sharing_click = commands.register_system(on_screen_sharing_click);
    let on_power_saving_click = commands.register_system(on_power_saving_mode_click);
    let on_calculator_click = commands.register_system(on_calculator_click);
    let on_camera_click = commands.register_system(on_camera_click);
    let on_wireless_click = commands.register_system(on_wireless_click);
    let on_bluetooth_click = commands.register_system(on_bluetooth_click);
    let on_terminal_click = commands.register_system(on_terminal_click);
    let on_cellular_click = commands.register_system(on_cellular_click);
    let on_brightness_change = commands.register_system(on_brightness_change);
    let on_volume_change = commands.register_system(on_volume_change);

    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(WINDOW_SIZE.1 - BAR_SIZE.1),
            padding: UiRect {
                left: Val::Px(32.),
                right: Val::Px(32.),
                top: Val::Px(8.),
                ..default()
            },
            flex_direction: FlexDirection::Column,
            ..default()
        },
        DrawerItemsRoot,
        BackgroundColor(Color::oklcha(0.173, 0., 0., 0.96)),
        children![
            row1(icons, fonts),
            row2(
                on_rotation_click,
                on_airplane_mode_click,
                on_microphone_click,
                on_screen_recording_click,
                on_screen_sharing_click,
                on_power_saving_click,
                on_calculator_click,
                on_camera_click,
                icons
            ),
            row3(on_brightness_change, on_volume_change, icons),
            row4(
                on_wireless_click,
                on_bluetooth_click,
                on_terminal_click,
                on_cellular_click,
                fonts,
                icons
            ),
        ],
    )
}

fn row1(icons: &SettingsDrawerIcons, fonts: &FontAssets) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(32.82),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        children![settings(icons), battery(icons, fonts), power(icons)],
    )
}

fn row2(
    on_rotation_click: SystemId,
    on_airplane_mode_click: SystemId,
    on_microphone_click: SystemId,
    on_screen_recording_click: SystemId,
    on_screen_sharing_click: SystemId,
    on_power_saving_click: SystemId,
    on_terminal_click: SystemId,
    on_camera_click: SystemId,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(236.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
            row_gap: Val::Px(30.0),
            column_gap: Val::Px(33.33),
            padding: UiRect::all(Val::Px(20.)),
            margin: UiRect::top(Val::Px(16.)),
            ..default()
        },
        BorderRadius::all(Val::Px(12.0)),
        BackgroundColor(Color::oklch(0.209, 0., 0.)),
        children![
            auto_rotation(on_rotation_click, icons),
            airplane_mode(on_airplane_mode_click, icons),
            screen_sharing(on_screen_sharing_click, icons),
            power_saving_mode(on_power_saving_click, icons),
            microphone(on_microphone_click, icons),
            screen_recording(on_screen_recording_click, icons),
            calculator(on_terminal_click, icons),
            camera(on_camera_click, icons),
        ],
    )
}

fn row3(
    on_brightness_change: SystemId<In<f32>>,
    on_volume_change: SystemId<In<f32>>,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(72.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            column_gap: Val::Px(16.),
            margin: UiRect::top(Val::Px(16.)),
            ..default()
        },
        children![
            brightness(on_brightness_change, icons),
            volume(on_volume_change, icons),
        ],
    )
}

fn row4(
    on_wireless_click: SystemId,
    on_bluetooth_click: SystemId,
    on_terminal_click: SystemId,
    on_cellular_click: SystemId,
    fonts: &FontAssets,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(104.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            column_gap: Val::Px(20.),
            margin: UiRect::top(Val::Px(16.)),
            ..default()
        },
        children![
            wireless(on_wireless_click, fonts, icons),
            bluetooth(on_bluetooth_click, fonts, icons),
            terminal(on_terminal_click, fonts, icons),
            cellular(on_cellular_click, fonts, icons),
        ],
    )
}
