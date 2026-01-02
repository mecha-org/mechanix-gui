mod handle;
mod signal;
mod slider;
mod ui;

use gpui::*;

/// Sets up hardware button handling and opens the slider UI overlay.
pub fn run_app(cx: &mut App) {
    handle::init(cx);
    ui::init(cx);
    signal::init(cx);
}