use gpui::*;
use volume_slider::run_app;
fn main() {
    let application = Application::new();
    application.run(|cx| {
        settings::init(cx);
        run_app(cx);
        // cx.activate(true);
    });
}
