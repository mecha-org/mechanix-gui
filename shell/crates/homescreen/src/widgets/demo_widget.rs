use crate::widgets::HomescreenWidget;
use gpui::*;

pub struct DemoWidget {
    bounds: Bounds<Pixels>,
    name: String,
    color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl DemoWidget {
    pub fn new(
        name: impl Into<String>,
        color: impl Into<Hsla>,
        border_color: impl Into<Hsla>,
        has_border: bool,
    ) -> Self {
        Self {
            bounds: Bounds::default(),
            name: name.into(),
            color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl Default for DemoWidget {
    fn default() -> Self {
        Self::new("Demo", rgb(0x4ecdc4), rgb(0x45b7aa), true)
    }
}

impl HomescreenWidget for DemoWidget {
    fn render(&self) -> gpui::AnyElement {
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xffffff))
                    .child(self.name.clone()),
            )
            .into_any_element()
    }

    fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }

    fn get_bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn background_color(&self) -> Hsla {
        self.color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
    }
}
