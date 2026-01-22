use std::path::PathBuf;

use crate::widgets::HomescreenWidget;
use gpui::*;

pub struct ExtensionWidget {
    bounds: Bounds<Pixels>,
    icon: PathBuf,
    color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl ExtensionWidget {
    pub fn new(
        icon: impl Into<PathBuf>,
        color: impl Into<Hsla>,
        border_color: impl Into<Hsla>,
        has_border: bool,
    ) -> Self {
        Self {
            bounds: Bounds::default(),
            icon: icon.into(),
            color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl Default for ExtensionWidget {
    fn default() -> Self {
        Self::new("Demo", rgb(0x4ecdc4), rgb(0x45b7aa), true)
    }
}

impl HomescreenWidget for ExtensionWidget {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        //println!("image path is {:?}", self.icon);
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(img(self.icon.clone()).size_full())
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
