use bevy::prelude::*;
use headless_widgets::CoreSlider;

use crate::ui::{Brightness, BrightnessFilledArea, BrightnessValue};

pub fn on_brightness_change(val: In<f32>, mut brightness: ResMut<BrightnessValue>) {
    println!("on_brightness_change() {}", *val);
    brightness.0 = *val;
}

pub fn update_brightness_state(
    q_filled_area: Single<&mut Node, With<BrightnessFilledArea>>,
    q_slider: Single<&mut CoreSlider, With<Brightness>>,
    brightness: Res<BrightnessValue>,
) {
    let mut node = q_filled_area.into_inner();
    let mut slider = q_slider.into_inner();
    slider.set_value(brightness.0);
    node.width = Val::Percent(brightness.0);
}
