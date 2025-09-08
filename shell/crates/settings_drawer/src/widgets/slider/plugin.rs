use bevy::prelude::*;

use super::{on_slide, update_slider};

pub struct StyledSliderPlugin;
impl Plugin for StyledSliderPlugin {
    fn build(&self, app: &mut App) {
        println!("StyledSliderPlugin: Initializing");
        app.add_observer(on_slide);
        app.add_systems(Update, update_slider);
    }
}
