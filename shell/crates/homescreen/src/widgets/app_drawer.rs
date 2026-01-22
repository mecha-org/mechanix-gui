use crate::widgets::HomescreenWidget;
use crate::Homescreen;
use app_drawer::prelude::*;
use gpui::*;

pub struct AppDrawerWidget {
    bounds: Bounds<Pixels>,
    drawer_handle: Entity<AppDrawer>,
    background_color: Hsla,
    has_border: bool,
}

impl AppDrawerWidget {
    pub fn new(cx: &mut Context<Homescreen>, color: impl Into<Hsla>, has_border: bool) -> Self {
        let drawer_handle = cx.new(|cx| AppDrawer::new(cx));
        Self {
            bounds: Bounds::default(),
            drawer_handle,
            background_color: color.into(),
            has_border,
        }
    }
}

impl HomescreenWidget for AppDrawerWidget {
    fn render(&self, cx: &mut gpui::App) -> AnyElement {
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
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }
}
