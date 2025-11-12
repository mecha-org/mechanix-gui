mod icon;
pub mod input;

use crate::ui::icon::Icon;
use gpui::*;
use icon::IconName;
use input::TextInput;

pub struct UniversalSearch {
    pub ardour_icon: IconName,
    pub arrow_up_right_icon: IconName,
    pub chromium_icon: IconName,
    pub firefox_icon: IconName,
    pub github_icon: IconName,
    pub folder_medium_icon: IconName,
    pub search_icon: IconName,
    pub folder_small_icon: IconName,
    pub x_icon: IconName,
    pub text_input: Entity<TextInput>,
}

impl UniversalSearch {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            ardour_icon: IconName::Ardour,
            arrow_up_right_icon: IconName::ArrowUpRight,
            chromium_icon: IconName::Chromium,
            firefox_icon: IconName::Firefox,
            github_icon: IconName::Github,
            folder_medium_icon: IconName::FolderMedium,
            search_icon: IconName::Search,
            folder_small_icon: IconName::FolderSmall,
            x_icon: IconName::XIcon,
            text_input: cx.new(|cx| TextInput::new(cx)),
        }
    }
}

impl Render for UniversalSearch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app = |image: Img| {
            let size = gpui::size(px(60.0), px(60.0));

            div()
                .size_full()
                .bg(rgb(0x2b2b2b))
                .w(size.width)
                .h(size.height)
                .rounded(px(10.43))
                .flex()
                .justify_center()
                .items_center()
                .child(div().child(image.h(px(41.74)).w(px(41.74))))
        };

        let size = gpui::size(px(508.0), px(76.0));

        let row = || {
            div()
                .h(px(56.0))
                .w_full()
                .child(
                    div()
                        .size_full()
                        .flex()
                        .flex_row()
                        .items_center()
                        .child(
                            div()
                                .size_full()
                                .text_color(rgb(0xe9e9e9))
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .mr(px(8.0))
                                                        .bg(rgb(0x202020))
                                                        .w(px(36.0))
                                                        .h(px(36.0))
                                                        .flex()
                                                        .justify_center()
                                                        .items_center()
                                                        .rounded(px(8.0))
                                                        .child(
                                                            div()
                                                                .w(px(21.82))
                                                                .h(px(21.82))
                                                                .child(
                                                                    img(
                                                                        "icons/universal-search/folder_small_icon.svg"
                                                                    ).h(px(21.0))
                                                                )
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight(500.0))
                                                        .text_size(px(16.0))
                                                        .text_color(rgb(0xe9e9e9))
                                                        .child("Files")
                                                )
                                        )
                                )
                                .child(
                                    div()
                                        .h(px(18.0))
                                        .w(px(18.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            Icon::new(self.x_icon.clone())
                                                .size((px(18.0), px(18.0)))
                                                .text_color(rgb(0xe9e9e9))
                                        )
                                )
                        )
                )
        };

        let divider = || div().w(px(508.0)).h(px(1.0)).bg(rgb(0x202020));

        div()
            .size_full()
            .bg(gpui::black())
            .px(px(16.0))
            .flex()
            .flex_col()
            // .justify_between()
            .child(
                div()
                    // .id("vertical")
                    // .overflow_scroll()
                    // .h(px(620.))
                    .grid()
                    .child(
                        div()
                            .my(px(8.0))
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
                                    .rounded(px(16.0))
                                    .w(size.width)
                                    .h(size.height)
                                    .content_center()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .gap(px(26.0))
                                            .grid()
                                            .grid_cols(6)
                                            .grid_rows(1)
                                            .child(
                                                app(img(
                                                    "icons/universal-search/chromium_icon.png",
                                                ))
                                                .row_span(1)
                                                .col_span(1),
                                            )
                                            .child(
                                                app(img("icons/universal-search/kitty_icon.png"))
                                                    .row_span(1)
                                                    .col_span(1),
                                            )
                                            .child(
                                                app(img("icons/universal-search/firefox_icon.png"))
                                                    .row_span(1)
                                                    .col_span(1),
                                            )
                                            .child(
                                                app(img("icons/universal-search/github_icon.png"))
                                                    .row_span(1)
                                                    .col_span(1),
                                            )
                                            .child(
                                                app(img(
                                                    "icons/universal-search/folder_medium_icon.svg",
                                                ))
                                                .row_span(1)
                                                .col_span(1),
                                            )
                                            .child(
                                                app(img("icons/universal-search/ardour_icon.png"))
                                                    .row_span(1)
                                                    .col_span(1),
                                            ),
                                    ),
                            ),
                    )
                    .child(row())
                    .child(divider()), // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider())
                                       // .child(row())
                                       // .child(divider()),
            )
            .child(
                div().h(px(100.0)).w_full().child(
                    div().size_full().flex().flex_row().items_center().child(
                        div()
                            .size_full()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .rounded(px(28.0))
                            .bg(rgb(0x363636))
                            // .bg(gpui::blue())
                            .border_color(rgb(0x575757))
                            .child(
                                div()
                                    // .bg(gpui::red())
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .justify_center()
                                                    .mr(px(8.0))
                                                    .ml(px(16.0))
                                                    .w(px(24.0))
                                                    .h(px(24.0))
                                                    .rounded(px(8.0))
                                                    .child(
                                                        div().w(px(17.0)).h(px(17.0)).child(
                                                            Icon::new(self.search_icon.clone())
                                                                .size((px(17.0), px(17.0)))
                                                                .text_color(rgb(0x808080)),
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    // .border_1()
                                                    // .border_color(gpui::white())
                                                    .child(self.text_input.clone()),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .size_full()
                                    .h(px(40.0))
                                    .w(px(48.0))
                                    .rounded(px(21.54))
                                    .mr(px(8.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div().h(px(21.5)).w(px(21.5)).child(
                                            Icon::new(self.x_icon.clone())
                                                .size((px(16.0), px(16.0)))
                                                .text_color(rgb(0xe9e9e9)),
                                        ),
                                    ),
                            ),
                    ),
                ),
            )
    }
}
