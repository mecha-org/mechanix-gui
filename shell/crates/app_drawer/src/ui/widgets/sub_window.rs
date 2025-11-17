use crate::models::AppInfo;
use crate::ui::widgets::IconButton;
use gpui::{prelude::FluentBuilder, *};

pub struct SubWindow {
    pub apps: Vec<AppInfo>,
    pub category: String,
}

impl Render for SubWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x181818))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|_this, _ev, window, _cx| {
                    window.remove_window(); // close when clicking outside
                }),
            )
            // .size_full()
            .h(px(620.))
            .w(px(540.))
            .gap_2()
            .rounded(px(12.))
            // main grid of apps
            .child(
                div()
                    .grid()
                    .grid_cols(4)
                    .gap(px(14.))
                    .bg(rgb(0x181818))
                    .p(px(20.))
                    .rounded(px(12.))
                    //  .on_mouse_down(
                    //     MouseButton::Left,
                    //     cx.listener(|_this, _ev, _window, cx| {
                    //         cx.stop_propagation(); // <-- prevent close on inside click
                    //     }),
                    
                    // )
                    .children(self.apps.clone().into_iter().map(|app| {
                        let app_name = app.name.clone();
                        let app_icon = app.icon_path.clone();
                        let app_id = app.id.clone();

                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(
                                IconButton::new(("popup_app", app_id))
                                    .icon(app_icon)
                                    .on_click(cx.listener(move |_, _, _, _| {})),
                            )
                            .child(
                                div()
                                    .mt(px(6.0))
                                    .font_weight(FontWeight(400.0))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0xffffff))
                                    .child(app_name),
                            )
                    })),
            )
            .child(
                div()
                    .relative()
                    .flex_col()
                    .bottom(px(-54.))
                    .child(
                        img("icons/app_drawer/category.png")
                            .top(px(-66.))
                            .left(px(-10.))
                            .w(px(528.))
                            .h(px(38.)),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(-50.0))
                            .left(px(12.0))
                            .w(px(100.0))
                            .h(px(16.0))
                            .justify_start()
                            .child(
                                div().flex().child(
                                    div()
                                        .font_weight(FontWeight(400.0))
                                        .text_size(px(16.0))
                                        .text_color(rgb(0xffffff))
                                        .child(self.category.clone()),
                                ),
                            ),
                    ),
            )
            // .child(
            //     div()
            //         .flex()
            //         .justify_center()
            //         .p_8()
            //         .child(button("Close", |window, _| {
            //             window.remove_window();
            //         })),
            // )
    }
}

pub fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_sm()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}
