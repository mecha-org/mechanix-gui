use commons::prelude::*;
use gpui::*;
use power_options::run_app;
fn main() {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        run_app(cx);
        cx.activate(true);
    });
}
