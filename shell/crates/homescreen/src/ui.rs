use gpui::*;

use crate::{state::HomescreenState, widgets::WidgetWrapper};

pub struct HomescreenUi;
impl HomescreenUi {
    pub fn render(state: &HomescreenState) -> impl IntoElement {
        let mut element = div()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0x1a1a1a))
            .text_color(rgb(0xe0e0e0));

        let mut dragged_widgets = Vec::new();

        for (page_number, widgets) in state.pages.iter().enumerate() {
            let page_location = (page_number as f32 - state.active_page as f32)
                * state.config.window.width
                + state.page_offset;

            let mut page = div()
                .relative()
                .w(px(state.config.window.width))
                .h(px(state.config.window.height));

            for widget_id in widgets.iter() {
                let widget_data = state.widgets.get(widget_id).unwrap();
                if !widget_data.is_being_dragged() {
                    page = page.child(WidgetWrapper::render(widget_data));
                } else {
                    dragged_widgets.push(widget_data);
                }
            }
            element = element.child(
                div()
                    .absolute()
                    .w(px(state.config.window.width))
                    .h(px(state.config.window.height))
                    .top(px(0.0))
                    .left(px(page_location))
                    .child(page),
            );
        }

        // Render dragged widgets last so they appear on top
        for widget_data in dragged_widgets {
            element = element.child(WidgetWrapper::render(widget_data));
        }

        element
    }
}
