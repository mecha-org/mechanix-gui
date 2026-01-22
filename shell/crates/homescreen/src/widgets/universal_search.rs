use crate::widgets::HomescreenWidget;
use crate::Homescreen;
use gpui::*;
use universal_search::prelude::UniversalSearch;

pub struct UniversalSearchWidget {
    bounds: Bounds<Pixels>,
    search_handle: Entity<UniversalSearch>,
    background_color: Hsla,
    has_border: bool,
}

impl UniversalSearchWidget {
    pub fn new(cx: &mut Context<Homescreen>, color: impl Into<Hsla>, has_border: bool) -> Self {
        let search_handle = cx.new(|cx| UniversalSearch::new(cx));
        Self {
            bounds: Bounds::default(),
            search_handle,
            background_color: color.into(),
            has_border,
        }
    }
}

impl HomescreenWidget for UniversalSearchWidget {
    fn render(&self, cx: &mut App) -> AnyElement {
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
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }
}
