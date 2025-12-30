use crate::widgets::HomescreenWidget;
use app_drawer::prelude::*;
use app_drawer::ui::utils::prelude::DesktopApps;
use gpui::*;
use crate::Homescreen;

pub struct AppDrawerWidget {
    bounds: Bounds<Pixels>,
    drawer_handle: Entity<AppDrawer>,
}

impl AppDrawerWidget {
    pub fn new(cx: &mut Context<Homescreen>) -> Self {
        let desktop_apps = DesktopApps::scan();
        let state = AppDrawerState {
            apps: desktop_apps,
        };
        let drawer_handle = cx.new(|cx| AppDrawer::new(state, cx));
        Self {
            bounds: Bounds::default(),
            drawer_handle,
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
        false
    }
}