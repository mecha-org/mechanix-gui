use bevy::prelude::*;
use service_plugins::bluetooth::{BluetoothAction, BluetoothActionEvent, BluetoothEnabledStatus};

use crate::{
    icons::SettingsDrawerIcons,
    ui::{Bluetooth, BluetoothCount, BluetoothIcon, ButtonType4},
};

pub fn update_bluetooth_state(
    q_icon: Single<&mut ImageNode, With<BluetoothIcon>>,
    bluetooth_state: Res<BluetoothEnabledStatus>,
    q_button: Single<&mut ButtonType4, With<Bluetooth>>,
    q_text: Single<(&mut Text, &mut TextColor), With<BluetoothCount>>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut icon = q_icon.into_inner();
    let mut on_button = q_button.into_inner();
    let (mut text, mut text_color) = q_text.into_inner();

    if bluetooth_state.0 {
        on_button.0 = true;
        icon.image = icons.bluetooth_on.clone();
        text.0 = "ON".to_string();
        text_color.0 = Color::oklch(0.9672, 0., 0.);
    } else {
        on_button.0 = false;
        icon.image = icons.bluetooth_off.clone();
        text.0 = "OFF".to_string();
        text_color.0 = Color::oklch(0.4202, 0., 0.);
    }
}

pub fn on_bluetooth_click(
    mut enabled: ResMut<BluetoothEnabledStatus>,
    mut event_writer: EventWriter<BluetoothActionEvent>,
) {
    enabled.0 = !enabled.0;
    event_writer.write(BluetoothActionEvent(BluetoothAction::ToggleBluetooth(
        enabled.0,
    )));
}
