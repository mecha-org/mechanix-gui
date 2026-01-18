use crate::{
    state::HomescreenState,
    widgets::{WidgetId, WidgetWrapper},
};
use gpui::*;
use theme::prelude::Theme;

pub struct HomescreenUi;
impl HomescreenUi {
    pub fn render(state: &HomescreenState, cx: &mut App) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();

        let mut element = div()
            .size_full()
            .overflow_hidden()
            .bg(colors.background_1000)
            .text_color(colors.foreground_300);

        let mut dragged_widgets = Vec::new();

        let state_width = state.config.window.width;
        let state_height = state.config.window.height;

        for (page_number, widgets) in state.pages.iter().enumerate() {
            let page_location =
                (page_number as f32 - state.active_page as f32) * state_width + state.page_offset;

            let mut page = div().relative().w(px(state_width)).h(px(state_height));

            for widget_id in widgets.iter() {
                let widget_data = state.widgets.get(widget_id).unwrap();
                if !widget_data.is_being_dragged() {
                    page = page.child(WidgetWrapper::render(*widget_id, widget_data, state, cx));
                } else {
                    dragged_widgets.push((*widget_id, widget_data));
                }
            }
            element = element.child(
                div()
                    .absolute()
                    .w(px(state_width))
                    .h(px(state_height))
                    .top(px(0.0))
                    .left(px(page_location))
                    .child(page),
            );
        }

        // Render dragged widgets last so they appear on top
        for (widget_id, widget_data) in dragged_widgets {
            element = element.child(WidgetWrapper::render(widget_id, widget_data, state, cx));
        }

        element
    }
}
