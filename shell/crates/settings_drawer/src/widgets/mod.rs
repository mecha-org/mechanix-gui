pub mod button;
pub mod slider;

use bevy::app::{App, Plugin};
use button::StyledButtonPlugin;

use crate::widgets::slider::StyledSliderPlugin;

pub struct LauncherStyledWidgetsPlugin;

impl Plugin for LauncherStyledWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StyledButtonPlugin, StyledSliderPlugin));
    }
}
