use bevy::prelude::*;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{ButtonType4, Cellular, CellularEnabled, CellularIcon, CellularName},
};

pub fn update_cellular_state(
    q_cellular_icon: Single<&mut ImageNode, With<CellularIcon>>,
    q_button: Single<&mut ButtonType4, With<Cellular>>,
    q_text: Single<(&mut Text, &mut TextColor), With<CellularName>>,
    cellular_enabled: Res<CellularEnabled>,
    icons: Res<SettingsDrawerIcons>,
) {
    let mut icon = q_cellular_icon.into_inner();
    let mut button = q_button.into_inner();
    let (mut text, mut text_color) = q_text.into_inner();
    if cellular_enabled.0 {
        text.0 = "".to_string();
        text_color.0 = Color::oklch(0.9672, 0., 0.);
        button.0 = true;
        icon.image = icons.cell_signal_high.clone();
    } else {
        text.0 = "No Sim".to_string();
        text_color.0 = Color::oklch(0.4202, 0., 0.);
        button.0 = false;
        icon.image = icons.cell_signal_none.clone();
    }
}

pub fn on_cellular_click(mut enabled: ResMut<CellularEnabled>) {
    enabled.0 = !enabled.0;
}
