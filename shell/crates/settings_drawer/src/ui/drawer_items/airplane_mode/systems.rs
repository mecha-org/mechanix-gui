use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        AirplaneModeIcon, ButtonType2,
        drawer_items::airplane_mode::{AirplaneMode, AirplaneModeEnabled},
    },
};

pub fn update_airplane_mode_state(
    q_airplane_mode_icon: Single<&mut ImageNode, With<AirplaneModeIcon>>,
    q_button: Single<&mut ButtonType2, With<AirplaneMode>>,
    airplane_enabled: ResMut<AirplaneModeEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut airplane_mode_icon = q_airplane_mode_icon.into_inner();
    let mut button = q_button.into_inner();
    if airplane_enabled.0 {
        button.0 = true;
        airplane_mode_icon.image = icons.airplane_on.clone();
    } else {
        button.0 = false;
        airplane_mode_icon.image = icons.airplane_off.clone();
    }
}

pub fn on_airplane_mode_click(mut airplane_enabled: ResMut<AirplaneModeEnabled>) {
    println!("on_airplane_mode_click()");
    airplane_enabled.0 = !airplane_enabled.0;
}
