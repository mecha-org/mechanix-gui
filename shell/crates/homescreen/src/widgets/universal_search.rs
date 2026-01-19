use crate::Homescreen;
use crate::widgets::HomescreenWidget;
use gpui::*;
use universal_search::prelude::UniversalSearch;

pub struct UniversalSearchWidget {
    bounds: Bounds<Pixels>,
    search_handle: Entity<UniversalSearch>,
}

impl UniversalSearchWidget {
    pub fn new(cx: &mut Context<Homescreen>) -> Self {
        let search_handle = cx.new(|cx| UniversalSearch::new(cx));
        Self {
            bounds: Bounds::default(),
            search_handle,
        }
    }
}

impl HomescreenWidget for UniversalSearchWidget {
    fn render(&self) -> AnyElement {
        div()
            .size_full()
            .child(self.search_handle.clone())
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
