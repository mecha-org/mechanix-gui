use crate::models::AppInfo;
use crate::prelude::IconName;
use crate::ui::widgets::IconButton;
use gpui::{prelude::FluentBuilder, *};

pub struct SubWindow {
    pub apps: Vec<AppInfo>,
    pub category: String,
}

impl Render for SubWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("overlay_root")
            .size_full()
            .absolute()
            .child(
                // semi-transparent dim background
                div()
                    .id("overlay_dim")
                    .absolute()
                    .size_full()
                    .bg(rgb(0x000000))
                    .opacity(0.6)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _, window, _| window.remove_window()),
                    ),
            )
            .child(
                // popup (not transparent)
                div()
                    .id("popup_panel")
                    .absolute()
                    .top(px(40.))
                    .left(px(20.))
                    .right(px(20.))
                    .bg(rgb(0x181818))
                    .rounded(px(12.))
                    .p(px(20.))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _, _, cx| cx.stop_propagation()), // Prevent closing
                    )
                    .child(div().grid().grid_cols(4).gap(px(14.)).children(
                        self.apps.clone().into_iter().map(|app| {
                            let name = app.name.clone();
                            let icon = app.icon_path.clone();
                            let app_name = app.name.clone();

                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .child(
                                    IconButton::new(("popup_app", app.id.clone()))
                                        .icon(icon)
                                        .on_click(cx.listener(move |_, _, _, _| {
                                            println!("Launching app: {}", app_name);
                                        })),
                                )
                                .child(div().mt(px(4.)).text_color(rgb(0xffffff)).child(name))
                        }),
                    ))
                    .child(
                        div()
                            .relative()
                            .flex_col()
                            .bottom(px(-94.))
                            .child(
                                img(IconName::Category.resolve())
                                    .top(px(-66.))
                                    .right(px(32.))
                                    .w(px(524.))
                                    .h(px(42.)),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .top(px(-50.0))
                                    // .left(px(12.0))
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
                    ),
            )
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
