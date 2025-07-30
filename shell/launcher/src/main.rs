// use components::{settings_drawer::run_settings_drawer, status_bar::run_status_bar};
// pub use widgets::StyledWidgetsPlugin;

use crate::{
    launcher::run_launcher,
    // settings_panel::run_settings_panel
};

mod components;
mod desktop_apps;
mod launcher;
mod settings;
mod styled_card;
mod notification;
// mod settings_panel;
mod utils;
// mod widgets;
fn main() {
    // run_status_bar();
    // run_settings_drawer();
    // run_settings_panel();
    run_launcher();
}
