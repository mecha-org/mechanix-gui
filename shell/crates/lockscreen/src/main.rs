use commons::prelude::*;
use gpui::*;
use lockscreen::run_app;
fn main() {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        settings::init(cx);
        run_app(cx);
        cx.activate(true);
    });
}
