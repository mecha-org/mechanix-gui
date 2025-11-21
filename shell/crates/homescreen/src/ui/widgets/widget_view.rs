use crate::models::Widget;
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct WidgetView {
    widget: Widget,
    cell_size: Pixels,
    gap: Pixels,
    is_hovered: bool,
    is_selected: bool,
}

impl WidgetView {
    pub fn new(widget: Widget, cell_size: Pixels, gap: Pixels) -> Self {
        Self {
            widget,
            cell_size,
            gap,
            is_hovered: false,
            is_selected: false,
        }
    }

    pub fn hovered(mut self, is_hovered: bool) -> Self {
        self.is_hovered = is_hovered;
        self
    }

    pub fn selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    fn calculate_dimensions(&self) -> (Pixels, Pixels) {
        let width = self.cell_size * self.widget.size.cols + self.gap * (self.widget.size.cols - 1);
        let height =
            self.cell_size * self.widget.size.rows + self.gap * (self.widget.size.rows - 1);
        (width, height)
    }
}

impl RenderOnce for WidgetView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (width, height) = self.calculate_dimensions();

        let base_color = self.widget.background_color;
        let bg_color = if self.is_selected {
            let mut color = base_color;
            color.l = (color.l + 0.1).min(1.0);
            color
        } else if self.is_hovered {
            let mut color = base_color;
            color.l = (color.l + 0.05).min(1.0);
            color
        } else {
            base_color
        };

        let border_color = if self.is_selected {
            rgb(0xFFFFFF)
        } else if self.is_hovered {
            rgb(0xCCCCCC)
        } else {
            rgba(0x00000000)
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w(width)
            .h(height)
            .bg(bg_color)
            .rounded(px(16.0))
            .border_2()
            .border_color(border_color)
            .cursor_pointer()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w(px(48.0))
                            .h(px(48.0))
                            .rounded(px(8.0))
                            .bg(rgba(0xFFFFFF33))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .child(self.widget.icon_name.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::MEDIUM)
                            .child(self.widget.name.to_string()),
                    ),
            )
    }
}
