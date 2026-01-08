mod handle;
mod icon;
mod signal;
mod slider;
mod ui;

use gpui::*;

/// Sets up hardware button handling and opens the slider UI overlay.
pub fn run_app(cx: &mut App) {
    let config = ui::init(cx);
    signal::init(cx, config.slider, config.min_volume, config.max_volume);
}