use crate::Homescreen;
use crate::widgets::HomescreenWidget;
use app_drawer::prelude::*;
use gpui::*;

pub struct AppDrawerWidget {
    bounds: Bounds<Pixels>,
    drawer_handle: Entity<AppDrawer>,
    has_border: bool,
}

impl AppDrawerWidget {
    pub fn new(cx: &mut Context<Homescreen>, has_border: bool) -> Self {
        let drawer_handle = cx.new(|cx| AppDrawer::new(cx));
        Self {
            bounds: Bounds::default(),
            drawer_handle,
            has_border,
        }
    }
}

impl HomescreenWidget for AppDrawerWidget {
    fn render(&self) -> AnyElement {
        div()
            .size_full()
            .child(self.drawer_handle.clone())
            .into_any_element()
    }

    fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    fn get_bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn background_color(&self) -> Hsla {
        gpui::transparent_black().into()
    }

    fn has_border(&self) -> bool {
        self.has_border
    }
}
