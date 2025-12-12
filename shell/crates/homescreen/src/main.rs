use commons::prelude::*;
use homescreen::prelude::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        run_app(cx);
        cx.activate(true);
    });
}
