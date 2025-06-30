use crate::{
    styled_card::StyledCard,
    utils::{FontAssets, Icon},
};
use bevy::prelude::*;
use bevy_plugins::bluetooth::{BluetoothDeviceConnectedStatus, BluetoothEnabledStatus};
use bevy_plugins::network_manager::{ActiveNetworkStrength, WirelessEnabled};
use bevy_plugins::upower::{BatteryLevel, DeviceState};
use bevy_plugins::{UPowerBatteryLevel, UPowerBatteryState};
use bevy_styled_widgets::prelude::StyledText;
use chrono::{Datelike, Timelike};

#[derive(Component)]
struct Clock;

#[derive(Resource)]
struct ClockUpdateTimer(Timer);

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClockUpdateTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, update_clock);
        app.add_systems(
            Update,
            update_wireless_state.run_if(resource_changed::<WirelessEnabled>),
        );
        app.add_systems(
            Update,
            update_wireless_network_strength.run_if(resource_changed::<ActiveNetworkStrength>),
        );
        app.add_systems(
            Update,
            update_bluetooth_on_powered.run_if(resource_changed::<BluetoothEnabledStatus>),
        );
        app.add_systems(
            Update,
            update_bluetooth_on_connected.run_if(resource_changed::<BluetoothDeviceConnectedStatus>),
        );
        app.add_systems(
            Update,
            update_power_icon.run_if(resource_changed::<DeviceState>),
        );
        app.add_systems(
            Update,
            update_power_icon.run_if(resource_changed::<BatteryLevel>),
        );
    }
}

fn update_clock(
    time: Res<Time>,
    mut timer: ResMut<ClockUpdateTimer>,
    mut query: Query<&mut StyledText, With<Clock>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut styled_text in &mut query {
            styled_text.content = get_current_datetime();
        }
    }
}

#[derive(Component)]
struct Wireless;
//Query resource and update in this function
fn update_wireless_state(
    mut query: Query<&mut StyledText, With<Wireless>>,
    wifi_state: Res<WirelessEnabled>,
) {
    for mut styled_text in &mut query {
        info!("WirelessEnabled is updated :{:?}", wifi_state);
        if wifi_state.0 {
            styled_text.content = Icon::WirelessNone.into();
        } else {
            styled_text.content = Icon::WirelessOff.into();
        }
    }
}

fn update_wireless_network_strength(
    mut query: Query<&mut StyledText, With<Wireless>>,
    active_network_strength: Res<ActiveNetworkStrength>,
) {
    for mut styled_text in &mut query {
        styled_text.content = match active_network_strength.0 {
            0..=20 => Icon::WirelessLow.into(),
            21..=50 => Icon::WirelessLow.into(),
            51..=75 => Icon::WirelessMedium.into(),
            76..=100 => Icon::WirelessHigh.into(),
            _ => unreachable!(),
        }
    }
}
#[derive(Component)]
struct Bluetooth;

fn update_bluetooth_on_powered(
    mut query: Query<&mut StyledText, With<Bluetooth>>,
    bluetooth_state: Res<BluetoothEnabledStatus>,
) {
    for mut styled_text in &mut query {
        if bluetooth_state.0 {
            styled_text.content = Icon::BluetoothNone.into();
        } else {
            styled_text.content = Icon::BluetoothOff.into();
        }
    }
}
fn update_bluetooth_on_connected(
    mut query: Query<&mut StyledText, With<Bluetooth>>,
    connected_status: Res<BluetoothDeviceConnectedStatus>,
) {
    for mut styled_text in &mut query {
        if connected_status.0 {
            styled_text.content = Icon::BluetoothConnected.into();
        } else {
            styled_text.content = Icon::BluetoothNone.into();
        }
    }
}

#[derive(Component)]
struct Battery;

fn update_power_icon(mut query: Query<&mut StyledText, With<Battery>>, state: Res<DeviceState>, level: Res<BatteryLevel>) {
    for mut styled_text in &mut query {
        if state.0 == UPowerBatteryState::Charging {
            match level.0 {
                UPowerBatteryLevel::Unknown => {
                    styled_text.content = Icon::BatteryWarning.into();
                }
                UPowerBatteryLevel::None => {}
                UPowerBatteryLevel::Low => {
                    styled_text.content = Icon::BatteryLowCharging.into();
                }
                UPowerBatteryLevel::Critical => {}
                UPowerBatteryLevel::Normal => {
                    styled_text.content = Icon::BatteryMediumCharging.into();
                }
                UPowerBatteryLevel::High => {
                    styled_text.content = Icon::BatteryHighCharging.into();
                }
                UPowerBatteryLevel::Full => {
                    styled_text.content = Icon::BatteryFullCharging.into();
                }
            }
        }
    }
}

fn get_current_datetime() -> String {
    let now = chrono::Local::now();
    format!(
        "{} {} {:02}:{:02}:{:02}",
        now.day(),
        now.format("%B"),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub fn status_bar(
    font_assets: &FontAssets,
    wireless_icon: Icon,
    bluetooth_icon: Icon,
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
        StyledCard,
        children![
            //Clock
            (
                StyledText::builder()
                    .content(get_current_datetime())
                    .font_size(16.)
                    .font(font_assets.primary_600.clone())
                    .build(),
                Clock
            ),
            //Icons
            (
                Node {
                    align_items: AlignItems::End,
                    column_gap: Val::Px(12.),
                    ..Default::default()
                },
                children![
                    (
                        StyledText::builder()
                            .content(bluetooth_icon)
                            .font_size(icon_size)
                            .font(font_assets.font_icons.clone())
                            .build(),
                        Bluetooth
                    ),
                    (
                        StyledText::builder()
                            .content(wireless_icon)
                            .font_size(icon_size)
                            .font(font_assets.font_icons.clone())
                            .build(),
                        Wireless
                    ),
                    (
                        StyledText::builder()
                            .content(Icon::BatteryEmpty)
                            .font_size(icon_size)
                            .font(font_assets.font_icons.clone())
                            .build(),
                        Battery
                    ),
                ]
            )
        ],
    )
}
