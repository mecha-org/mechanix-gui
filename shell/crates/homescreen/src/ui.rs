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

        for (page_number, widgets) in state.pages.iter().enumerate() {
            let page_location = (page_number as f32 - state.active_page as f32)
                * state.config.window.width
                + state.page_offset;

            let mut page = div()
                .relative()
                .w(px(state.config.window.width))
                .h(px(state.config.window.height));

            for widget_id in widgets.iter() {
                page = page.child(WidgetWrapper::render(
                    state.widgets.get(widget_id).unwrap().widget(),
                ));
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

        element
    }
}
