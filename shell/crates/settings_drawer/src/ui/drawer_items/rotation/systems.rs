use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        AutoRotationIcon, ButtonType1,
        drawer_items::rotation::{AutoRotation, RotationEnabled},
    },
};

pub fn update_rotation_state(
    q_rotation_icon: Single<&mut ImageNode, With<AutoRotationIcon>>,
    q_button: Single<&mut ButtonType1, With<AutoRotation>>,
    rotation_enabled: ResMut<RotationEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut rotation_icon = q_rotation_icon.into_inner();
    let mut button = q_button.into_inner();
    if rotation_enabled.0 {
        button.0 = true;
        rotation_icon.image = icons.rotation_on.clone();
    } else {
        button.0 = false;
        rotation_icon.image = icons.rotation_off.clone();
    }
}

pub fn on_rotation_click(mut rotation_enabled: ResMut<RotationEnabled>) {
    rotation_enabled.0 = !rotation_enabled.0;
}
