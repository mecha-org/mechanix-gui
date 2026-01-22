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
        let colors = cx.theme().colors.clone();
        let digital = cx.fonts().digital.clone();
        let text_color = colors.background_700.clone();
        let date_time = ShellState::global(cx).current_time_date.clone();
        let mut parts = date_time.split(':');
        let hour = parts.next().unwrap_or("00").to_string();
        let minute = parts.next().unwrap_or("00").to_string();

        // Asset paths for layered rendering
        let clock_background_path = "icons/homescreen/clock_background.svg";
        let dot_grid_path = "icons/homescreen/extension/dot_grid.svg";
        let dashed_lines_path = "icons/homescreen/dashed_lines.svg";

        div()
            .size_full()
            .relative()
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(img(clock_background_path).w(px(156.)).h(px(156.)).text_color(colors.accent_200)),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(img(dot_grid_path).w(px(148.)).h(px(146.))),
            )
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(img(dashed_lines_path).w(px(140.)).h(px(140.))),
            )
            .child(
                div().absolute().inset_0().p(px(4.)).child(
                    div()
                        .size_full()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .items_center()
                        .text_size(px(64.))
                        .font_family(digital)
                        .italic()
                        .line_height(px(60.))
                        .font_weight(FontWeight::NORMAL)
                        .text_color(text_color)
                        .child(hour)
                        .child(minute),
                ),
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
