use commons::prelude::*;
use running_apps::run_app;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // reads RUST_LOG
        .init();

    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        run_app(cx);
        cx.activate(true);
    });
}
