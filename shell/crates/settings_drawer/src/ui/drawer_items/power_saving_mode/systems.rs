use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1, PowerSavingModeIcon,
        drawer_items::power_saving_mode::{PowerSavingMode, PowerSavingModeEnabled},
    },
};

pub fn update_power_saving_mode_state(
    // q_power_saving_mode_icon: Single<&mut ImageNode, With<PowerSavingModeIcon>>,
    q_button: Single<&mut ButtonType1, With<PowerSavingMode>>,
    power_saving_mode_enabled: ResMut<PowerSavingModeEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    // let mut power_saving_mode_icon = q_power_saving_mode_icon.into_inner();
    let mut button = q_button.into_inner();
    if power_saving_mode_enabled.0 {
        button.0 = true;
        // power_saving_mode_icon.image = icons.power_saving_mode_on.clone();
    } else {
        button.0 = false;
        // power_saving_mode_icon.image = icons.power_saving_mode_off.clone();
    }
}

pub fn on_power_saving_mode_click(mut power_saving_mode_enabled: ResMut<PowerSavingModeEnabled>) {
    power_saving_mode_enabled.0 = !power_saving_mode_enabled.0;
}
