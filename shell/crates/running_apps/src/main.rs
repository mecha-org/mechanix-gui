use commons::prelude::*;
use gpui::*;
use running_apps::run_app;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // reads RUST_LOG
        .init();

    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let installed_apps = cx.new(|cx| InstalledApps::new(cx));
        run_app(installed_apps, cx);
        cx.activate(true);
    });
}
