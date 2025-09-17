use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1, MicrophoneIcon,
        drawer_items::microphone::{Microphone, MicrophoneEnabled},
    },
};

pub fn update_microphone_state(
    q_microphone_icon: Single<&mut ImageNode, With<MicrophoneIcon>>,
    q_button: Single<&mut ButtonType1, With<Microphone>>,
    microphone_enabled: ResMut<MicrophoneEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut microphone_icon = q_microphone_icon.into_inner();
    let mut button = q_button.into_inner();
    if microphone_enabled.0 {
        button.0 = true;
        microphone_icon.image = icons.mic_on.clone();
    } else {
        button.0 = false;
        microphone_icon.image = icons.mic_off.clone();
    }
}

pub fn on_microphone_click(mut microphone_enabled: ResMut<MicrophoneEnabled>) {
    microphone_enabled.0 = !microphone_enabled.0;
}
