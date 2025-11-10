use gpui::*;

pub struct UniversalSearch {}

impl UniversalSearch {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for UniversalSearch {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon = || {
            let size = gpui::size(px(44.35), px(44.35));

            div().size_full().w(size.width).h(size.height)
        };

        let app = || {
            let size = gpui::size(px(60.), px(60.));

            div()
                .size_full()
                // .bg(gpui::blue())
                .bg(rgb(0x2B2B2B))
                .w(size.width)
                .h(size.height)
                .rounded(px(10.43))
                .flex()
                .justify_center()
                .items_center()
                .child(div().child(icon()))
        };

        let size = gpui::size(px(508.), px(76.));

        let row = || {
            div().h(px(56.)).w_full().child(
                div()
                    .size_full()
                    .text_color(rgb(0xE9E9E9))
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div().flex().flex_row().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .child(
                                    div()
                                        .mr(px(8.))
                                        .bg(rgb(0x202020))
                                        .w(px(36.))
                                        .h(px(36.))
                                        .rounded(px(8.0))
                                        .child(div().w(px(21.82)).h(px(21.82))),
                                )
                                .child(
                                    div()
                                        .font_weight(FontWeight(500.))
                                        .text_size(px(16.))
                                        .text_color(rgb(0xE9E9E9))
                                        .child("Github"),
                                ),
                        ),
                    )
                    .child(
                        div()
                            // .bg(gpui::green())
                            .h(px(18.))
                            .w(px(18.)),
                    ),
            )
        };

        let divider = || div().w(px(508.)).h(px(1.)).bg(rgb(0x202020));

        let search = || {
            div().h(px(56.)).w_full().child(
                div()
                    .size_full()
                    // .text_color(rgb(0xE9E9E9))
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .rounded(px(28.))
                    .bg(rgb(0x363636))
                    .border_color(rgb(0x575757))
                    .child(
                        div().flex().flex_row().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .child(
                                    div()
                                        .mr(px(8.))
                                        .ml(px(16.))
                                        // .bg(gpui::green())
                                        .w(px(24.))
                                        .h(px(24.))
                                        .rounded(px(8.0))
                                        .child(
                                            // ICON FRAME FOR SEARCH ICON
                                            div().w(px(17.)).h(px(17.)),
                                        ),
                                )
                                .child(
                                    div()
                                        .font_weight(FontWeight(500.))
                                        .text_size(px(16.))
                                        .text_color(rgb(0xE9E9E9)), // .child(
                                                                    //     div()
                                                                    //         .size_full()
                                                                    //         .bg(gpui::black())
                                                                    //         .flex()
                                                                    //         .items_center()
                                                                    //         .justify_center()
                                                                    //         .child(input),
                                                                    // ),
                                ),
                        ),
                    )
                    .child(
                        div()
                            .bg(gpui::green())
                            .h(px(40.))
                            .w(px(48.))
                            .rounded(px(21.54))
                            .mr(px(8.))
                            .child(
                                // CLOSE FRAME FOR SEARCH ICON
                                div().h(px(21.5)).w(px(21.5)),
                            ),
                    ),
            )
        };

        div()
            .size_full()
            .bg(gpui::black())
            .px(px(16.0))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .grid()
                    .child(
                        div()
                            .my(px(8.))
                            .flex()
                            .justify_center()
                            .col_span_full()
                            .row_span_full()
                            .child(
                                div()
                                    .bg(rgb(0x181818))
                                    .flex()
                                    .border_1()
                                    .border_color(rgb(0x363636))
                                    .rounded(px(16.))
                                    .w(size.width)
                                    .h(size.height)
                                    .content_center()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .gap(px(26.))
                                            .grid()
                                            .grid_cols(6)
                                            .grid_rows(1)
                                            .child(app().row_span(1).col_span(1))
                                            .child(app().row_span(1).col_span(1))
                                            .child(app().row_span(1).col_span(1))
                                            .child(app().row_span(1).col_span(1))
                                            .child(app().row_span(1).col_span(1))
                                            .child(app().row_span(1).col_span(1)),
                                    ),
                            ),
                    )
                    .child(row())
                    .child(divider()),
            )
            .child(search())
    }
}
