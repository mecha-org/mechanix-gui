use commons::prelude::*;
use gpui::*;
use mechanix_keyboard::prelude::*;

fn main() {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        run_app(cx);
        cx.activate(true);
    });
}
