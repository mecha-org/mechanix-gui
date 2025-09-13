use bevy::prelude::*;
use service_plugins::network_manager::{ActiveNetworkStrength, WirelessEnabled};

use crate::{
    icons::SettingsDrawerIcons,
    ui::{ButtonType4, Wireless, WirelessIcon, WirelessName},
};

pub fn update_wireless_state(
    q_icon: Single<&mut ImageNode, With<WirelessIcon>>,
    wifi_state: Res<WirelessEnabled>,
    q_button: Single<&mut ButtonType4, With<Wireless>>,
    q_name: Single<&mut Text, With<WirelessName>>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut icon = q_icon.into_inner();
    let mut button = q_button.into_inner();
    let mut name = q_name.into_inner();

    if wifi_state.0 {
        button.0 = true;
        icon.image = icons.wireless_on.clone();
        name.0 = "Mecha1".to_string();
    } else {
        button.0 = false;
        icon.image = icons.wireless_off.clone();
        name.0 = "".to_string();
    }
}

pub fn update_active_network_strength(
    icons: Res<SettingsDrawerIcons>,
    mut query: Query<&mut ImageNode, With<WirelessIcon>>,
    active_network_strength: Res<ActiveNetworkStrength>,
) {
    for mut wireless_icon in &mut query {
        wireless_icon.image = match active_network_strength.0 {
            0..=20 => icons.wireless_low.clone(),
            21..=50 => icons.wireless_medium.clone(), // Ask for icon
            51..=75 => icons.wireless_high.clone(),
            76..=100 => icons.wireless_full.clone(),
            _ => unreachable!(),
        }
    }
}

pub fn on_wireless_click(mut enabled: ResMut<WirelessEnabled>) {
    enabled.0 = !enabled.0;
}
