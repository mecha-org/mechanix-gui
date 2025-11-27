use crate::prelude::IconName;
use crate::ui::utils::prelude::{DesktopApp, DesktopApps};
use crate::ui::widgets::IconButton;
use gpui::*;

const POPUP_BASE_TOP: f32 = 40.0;

pub struct SubWindow {
    apps: Vec<DesktopApp>,
    pub category: String,

    // scrolling
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
    content_height: Pixels,
}

impl SubWindow {
    pub fn scan(category: String) -> Self {
        let desktop_apps = DesktopApps::scan();
        let apps = desktop_apps.get_apps_by_category(&category);

        Self {
            apps,
            category,
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
            content_height: px(0.),
        }
    }

    fn calculate_scroll_bounds(
        &self,
        content_height: Pixels,
        window_height: Pixels,
    ) -> (Pixels, Pixels) {
        let container_height = window_height - px(80.); // subtract header/padding
        let max_scroll = px(0.);
        let min_scroll = container_height - content_height;

        if content_height <= container_height {
            (px(0.), px(0.))
        } else {
            (min_scroll, max_scroll)
        }
    }

    fn estimate_content_height(&self) -> Pixels {
        let row_height = px(128.);
        let spacing = px(20.);
        let rows = ((self.apps.len() as f32) / 4.0).ceil() as usize;
        rows as f32 * row_height + spacing
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag_start_y = event.position.y;
        self.last_scroll_offset = self.scroll_offset;
        self.is_dragging = true;
        cx.stop_propagation();
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;

            let content_height = self.content_height;
            let window_height = window.bounds().size.height;

            let content_height = self.estimate_content_height();

            let (min_scroll, max_scroll) =
                self.calculate_scroll_bounds(content_height, window_height);

            self.scroll_offset = (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);

            cx.notify();
        }
    }
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
                    .left(px(20.))
                    .right(px(20.))
                    .bg(rgb(0x181818))
                    .rounded(px(12.))
                    .p(px(20.))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                    .top(px(POPUP_BASE_TOP) + self.scroll_offset)
                    .child(div().grid().grid_cols(4).gap(px(14.)).children(
                        self.apps.clone().into_iter().enumerate().map(|(idx, app)| {
                            let name = app.name.clone();
                            let icon_path = app.icon_path.clone();
                            let app_name = app.name.clone();
                            let icon = DesktopApp::resolved_icon(&app.icon_path);

                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .child(IconButton::new(("popup_app", idx)).icon(icon).on_click(
                                    cx.listener(move |_, _, window, _| {
                                        // Close the popup window
                                        window.remove_window();

                                        // Launch the app
                                        let _ = DesktopApps::run_app_exec(app.exec.as_str());
                                    }),
                                ))
                                .child(
                                    div()
                                        .mt(px(4.))
                                        .text_color(rgb(0xffffff))
                                        .child(name)
                                        .text_ellipsis(),
                                )
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
                                    .w(px(100.0))
                                    .h(px(16.0))
                                    .justify_start()
                                    .child(
                                        div().flex().child(
                                            div()
                                                .font_weight(FontWeight(400.0))
                                                .text_size(px(16.0))
                                                .text_color(rgb(0xffffff))
                                                .child(self.category.clone())
                                                .text_ellipsis()
                                                .w(px(120.)),
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}
