use bevy::prelude::*;
use bevy_styled_widgets::prelude::StyledText;
use chrono::{Datelike, Timelike};

use crate::{
    styled_card::StyledCard,
    utils::{FontAssets, Icon},
};

#[derive(Component)]
pub struct Clock;

#[derive(Resource)]
struct ClockUpdateTimer(Timer);

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClockUpdateTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, update_clock);
        app.add_systems(Update, update_wireless);
        app.add_systems(Update, update_battery);
        app.add_systems(Update, update_bluetooth);
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
fn update_wireless(mut query: Query<&mut StyledText, With<Wireless>>) {
    for mut styled_text in &mut query {
        styled_text.content = Icon::WirelessHigh.into();
    }
}

#[derive(Component)]
struct Bluetooth;

fn update_bluetooth(mut query: Query<&mut StyledText, With<Bluetooth>>) {
    for mut styled_text in &mut query {
        styled_text.content = Icon::BluetoothConnected.into();
    }
}

#[derive(Component)]
struct Battery;

fn update_battery(mut query: Query<&mut StyledText, With<Battery>>) {
    for mut styled_text in &mut query {
        styled_text.content = Icon::BatteryFull.into();
    }
}

pub fn get_current_datetime() -> String {
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

pub fn status_bar(font_assets: &FontAssets) -> impl Bundle {
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
                            .content(Icon::BluetoothWarning)
                            .font_size(icon_size)
                            .font(font_assets.font_icons.clone())
                            .build(),
                        Bluetooth
                    ),
                    (
                        StyledText::builder()
                            .content(Icon::WirelessWarning)
                            .font_size(icon_size)
                            .font(font_assets.font_icons.clone())
                            .build(),
                        Wireless
                    ),
                    (
                        StyledText::builder()
                            .content(Icon::BatteryWarning)
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
