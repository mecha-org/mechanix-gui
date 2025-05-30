use components::{settings_drawer::run_settings_drawer, status_bar::run_status_bar, home::run_home};
pub use widgets::StyledWidgetsPlugin;

mod components;
mod settings;
mod utils;
mod widgets;
fn main() {
    // run_status_bar();
    // run_settings_drawer();
    run_home();
}
