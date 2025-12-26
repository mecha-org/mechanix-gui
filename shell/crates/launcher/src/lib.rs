use commons::prelude::*;
use gpui::*;

pub fn run() {
    Application::new().with_assets(Assets {}).run(|cx| {
        //init global settings
        settings::init(cx);

        //init dispatcher
        dispatcher::init(cx);

        //init global theme
        theme::init(cx);

        //init shell state
        shell_state::init(cx);

        status_bar::run_app(cx);

        homescreen::run_app(cx);

        running_apps::run_app(cx);

        // universal_search::run_app(cx);

        settings_drawer::run_app(cx);

        notifications::run_app(cx);

        cx.activate(true);
        cx.refresh_windows();
    });
}