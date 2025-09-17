use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1, ScreenSharingIcon,
        drawer_items::screen_sharing::{ScreenSharing, ScreenSharingEnabled},
    },
};

pub fn update_screen_sharing_state(
    q_screen_sharing_icon: Single<&mut ImageNode, With<ScreenSharingIcon>>,
    q_button: Single<&mut ButtonType1, With<ScreenSharing>>,
    screen_sharing_enabled: ResMut<ScreenSharingEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut screen_sharing_icon = q_screen_sharing_icon.into_inner();
    let mut button = q_button.into_inner();
    if screen_sharing_enabled.0 {
        button.0 = true;
        screen_sharing_icon.image = icons.second_screen_on.clone();
    } else {
        button.0 = false;
        screen_sharing_icon.image = icons.second_screen_off.clone();
    }
}

pub fn on_screen_sharing_click(mut screen_sharing_enabled: ResMut<ScreenSharingEnabled>) {
    screen_sharing_enabled.0 = !screen_sharing_enabled.0;
}
