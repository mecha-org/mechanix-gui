
use crate::{systems::get_current_datetime, StatusBarCamera};
pub use bevy::prelude::*;
use bevy_plugins::{bluetooth::BluetoothEnabledStatus, network_manager::WirelessEnabled};
use types::prelude::IconAssets;


#[derive(Component)]
pub struct Clock;

#[derive(Component)]
pub struct Bluetooth;

#[derive(Component)]
pub struct Wireless;

#[derive(Component)]
pub struct Battery;

pub fn spawn_status_bar_ui(
    mut commands: Commands,
    icons: Res<IconAssets>,
    wireless_enabled: Res<WirelessEnabled>,
    bluetooth_enabled: Res<BluetoothEnabledStatus>,
    camera: Res<StatusBarCamera>,
) {
    //Spawn status bar
    let wireless_default_icon = if wireless_enabled.0 {
        &icons.wifi_on
    } else {
        &icons.wifi_off
    };
    let bluetooth_default_icon = if bluetooth_enabled.0 {
        &icons.bluetooth_on
    } else {
        &icons.bluetooth_off
    };
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        UiTargetCamera(camera.0.clone()),
        children![status_bar(
            &icons,
            wireless_default_icon,
            bluetooth_default_icon,
        )],
    ));
}

pub fn status_bar(
    icon_assets: &IconAssets,
    wireless_default_icon: &Handle<Image>,
    bluetooth_default_icon: &Handle<Image>
) -> impl Bundle {
    let icon_size = 24.;

    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            border: UiRect::all(Val::Px(1.)),
            padding: UiRect {
                left: Val::Px(28.),
                right: Val::Px(20.),
                ..Default::default()
            },
            ..Default::default()
        },
        BorderColor(Color::linear_rgba(0., 0., 0., 0.2)),
        // StyledCard,
        children![
            //Clock
            (
                Text::new(get_current_datetime()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                Clock
            ),
            // Icons
            (
                Node {
                    align_items: AlignItems::End,
                    column_gap: Val::Px(12.),
                    ..Default::default()
                },
                children![
                    (
                       ImageNode::new(wireless_default_icon.clone()),
                       Node {
                            width: Val::Px(icon_size),
                            height: Val::Px(icon_size),
                            ..Default::default()
                        },
                        Wireless
                    ),
                    (
                        ImageNode::new(bluetooth_default_icon.clone()),
                        Node {
                            width: Val::Px(icon_size),
                            height: Val::Px(icon_size),
                            ..Default::default()
                        },
                        Bluetooth
                    ),
                    (
                        ImageNode::new(icon_assets.battery_empty.clone()),
                        Node {
                            width: Val::Px(icon_size),
                            height: Val::Px(icon_size),
                            ..Default::default()
                        },
                        Battery
                    ),
                ]
            )
        ],
    )
}
