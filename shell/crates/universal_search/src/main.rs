use commons::prelude::*;
use gpui::*;
use universal_search::prelude::*;

fn main() {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        cx.activate(true);
    });
}
