use crate::{widgets::HomescreenWidget, SystemUsageState};
use gpui::*;
use theme::{ActiveFonts, ActiveTheme};

pub struct SystemUsage {
    bounds: Bounds<Pixels>,
    background_color: Hsla,
    border_color: Hsla,
    has_border: bool,
}

impl SystemUsage {
    pub fn new(color: impl Into<Hsla>, border_color: impl Into<Hsla>, has_border: bool) -> Self {
        Self {
            bounds: Bounds::default(),
            background_color: color.into(),
            border_color: border_color.into(),
            has_border,
        }
    }
}

impl HomescreenWidget for SystemUsage {
    fn render(&self, cx: &mut gpui::App) -> gpui::AnyElement {
        let colors = cx.theme().colors.clone();
        let primary_font = cx.fonts().primary.clone();
        let state = cx.global::<SystemUsageState>();
        let cpu_usage = state.cpu_usage.clone();
        let memory_usage = state.memory_usage.clone();
        let uptime = state.uptime.clone();

        // Colors from theme
        let accent_color = colors.accent_200;
        let value_color = colors.foreground_200;
        let label_color = colors.foreground_600;

        div()
            .size_full()
            .flex()
            .flex_col()
            .rounded(px(5.))
            .p(px(12.))
            .font_family(primary_font)
            // Header: "System"
            .child(
                div()
                    .text_size(px(20.))
                    .font_weight(FontWeight::NORMAL)
                    .text_color(accent_color)
                    .line_height(relative(1.25))
                    .mb(px(10.))
                    .child("System"),
            )
            // Middle Row: CPU and Memory
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_between()
                    // Left Group: CPU
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.))
                            .child(
                                div()
                                    .text_size(px(24.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(value_color)
                                    .line_height(relative(1.25))
                                    .child(cpu_usage),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .font_weight(FontWeight::NORMAL)
                                    .text_color(label_color)
                                    .line_height(relative(1.25))
                                    .child("CPU"),
                            ),
                    )
                    // Right Group: Memory
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.))
                            .child(
                                div()
                                    .text_size(px(24.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(value_color)
                                    .line_height(relative(1.25))
                                    .child(format!("{} GB", memory_usage)),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .font_weight(FontWeight::NORMAL)
                                    .text_color(label_color)
                                    .line_height(relative(1.25))
                                    .child("Memory"),
                            ),
                    ),
            )
            // Bottom Section: Uptime
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .mt(px(10.))
                    .child(
                        div()
                            .text_size(px(24.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(value_color)
                            .line_height(relative(1.25))
                            .child(uptime),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .font_weight(FontWeight::NORMAL)
                            .text_color(label_color)
                            .line_height(relative(1.25))
                            .child("Up Time"),
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
        self.background_color
    }

    fn has_border(&self) -> bool {
        self.has_border
    }

    fn border_color(&self) -> Hsla {
        self.border_color
    }
}
