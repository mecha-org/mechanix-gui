// use components::{settings_drawer::run_settings_drawer, status_bar::run_status_bar};
// pub use widgets::StyledWidgetsPlugin;

use crate::launcher::run_launcher;

mod components;
mod desktop_apps;
mod launcher;
mod settings;
mod settings_panel;

// mod sprites_button;
mod styled_card;
mod utils;
mod widgets;
fn main() {
    // run_status_bar();
    // run_settings_drawer();
    // run_settings_drawer();
    run_launcher();
}
