use std::path::PathBuf;

use crate::widgets::HomescreenWidget;
use gpui::*;
use theme::{ActiveFonts, ActiveTheme};

pub struct PinnedApps {
    bounds: Bounds<Pixels>,
    apps: Vec<PathBuf>,
    background_color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl PinnedApps {
    pub fn new(
        apps: impl Into<Vec<PathBuf>>,
        color: impl Into<Hsla>,
        border_color: impl Into<Hsla>,
        has_border: bool,
    ) -> Self {
        Self {
            bounds: Bounds::default(),
            apps: apps.into(),
            background_color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl Default for PinnedApps {
    fn default() -> Self {
        Self::new(vec![], rgb(0x4ecdc4), rgb(0x45b7aa), true)
    }
}

impl HomescreenWidget for PinnedApps {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        let primary = cx.fonts().primary.clone();
        let text_color = cx.theme().colors.accent_200.clone();

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .absolute()
                    .top(px(8.))
                    .left(px(12.))
                    .text_size(px(20.))
                    .font_family(primary)
                    .text_color(text_color)
                    .child("Apps"),
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
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
    }
}
