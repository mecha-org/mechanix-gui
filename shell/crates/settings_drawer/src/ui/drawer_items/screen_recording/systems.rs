use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1, ScreenRecordingIcon,
        drawer_items::screen_recording::{ScreenRecording, ScreenRecordingEnabled},
    },
};

pub fn update_screen_recording_state(
    q_screen_recording_icon: Single<&mut ImageNode, With<ScreenRecordingIcon>>,
    q_button: Single<&mut ButtonType1, With<ScreenRecording>>,
    screen_recording_enabled: ResMut<ScreenRecordingEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut screen_recording_icon = q_screen_recording_icon.into_inner();
    let mut button = q_button.into_inner();
    if screen_recording_enabled.0 {
        button.0 = true;
        screen_recording_icon.image = icons.screen_recording_on.clone();
    } else {
        button.0 = false;
        screen_recording_icon.image = icons.screen_recording_off.clone();
    }
}

pub fn on_screen_recording_click(mut screen_recording_enabled: ResMut<ScreenRecordingEnabled>) {
    screen_recording_enabled.0 = !screen_recording_enabled.0;
}
