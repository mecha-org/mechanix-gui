use std::path::PathBuf;

use crate::widgets::HomescreenWidget;
use gpui::*;
use shell_state::ShellState;
use theme::{ActiveFonts, ActiveTheme};

pub struct Time {
    bounds: Bounds<Pixels>,
    color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl Time {
    pub fn new(color: impl Into<Hsla>, border_color: impl Into<Hsla>, has_border: bool) -> Self {
        Self {
            bounds: Bounds::default(),
            color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new(rgb(0x4ecdc4), rgb(0x45b7aa), false)
    }
}

impl HomescreenWidget for Time {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        let background = cx.theme().colors.accent_200.clone();
        let digital = cx.fonts().digital.clone();
        let text_color = cx.theme().colors.background_700.clone();
        let date_time = ShellState::global(cx).current_time_date.clone();
        let mut parts = date_time.split(':');
        let hour = parts.next().unwrap_or("00").to_string();
        let minute = parts.next().unwrap_or("00").to_string();

        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .bg(background)
            .flex()
            .flex_col()
            .rounded(px(4.))
            .p(px(4.))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .border(px(2.))
                    .border_dashed()
                    .border_color(gpui::black())
                    .text_size(px(64.))
                    .font_family(digital)
                    .italic()
                    .line_height(px(60.))
                    .font_weight(FontWeight::NORMAL)
                    .text_color(text_color)
                    .child(hour)
                    .child(minute),
            )
            // .child(img(self.icon.clone()).size_full())
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
