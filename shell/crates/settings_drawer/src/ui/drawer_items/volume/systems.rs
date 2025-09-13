use bevy::prelude::*;
use headless_widgets::CoreSlider;

use crate::ui::{Volume, VolumeFilledArea, VolumeValue};

pub fn on_volume_change(val: In<f32>, mut volume: ResMut<VolumeValue>) {
    println!("on_volume_change() {}", *val);
    volume.0 = *val;
}

pub fn update_volume_state(
    mut q_filled_area: Query<(&mut BackgroundColor, &VolumeFilledArea)>,
    q_slider: Single<&mut CoreSlider, With<Volume>>,
    volume: Res<VolumeValue>,
) {
    let mut slider = q_slider.into_inner();
    slider.set_value(volume.0);
    let total_bars = q_filled_area.iter().count();
    let filled_bars = (volume.0 / 100. * total_bars as f32) as usize;
    for (mut bg_color, filled_area_index) in q_filled_area.iter_mut() {
        if filled_area_index.0 < filled_bars {
            bg_color.0 = Color::oklch(0.9672, 0., 0.);
        } else {
            bg_color.0 = Color::oklch(0.4202, 0., 0.);
        }
    }
}
