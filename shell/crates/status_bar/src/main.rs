use commons::prelude::*;
use gpui::*;
use status_bar::prelude::*;

fn main() {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        run_app(cx);
        cx.activate(true);
    });
}
