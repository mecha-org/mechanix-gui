use gpui::prelude::*;
use gpui::*;
use std::collections::BTreeMap;

use crate::models::{AppDrawerState, AppInfo};
use crate::prelude::Icon;
use crate::prelude::IconName;
use crate::ui::widgets::{IconButton, SubWindow, button};
use input::TextInput;

pub mod icon;
pub mod input;
mod widgets;

const SEARCH_BAR_HEIGHT: f32 = 56.0;

pub struct AppDrawer {
    pub state: AppDrawerState,
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
    content_height: Pixels,
    pub text_input: Entity<TextInput>,
}

impl AppDrawer {
    pub fn new(state: AppDrawerState, cx: &mut Context<Self>) -> Self {
        Self {
            state,
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
            content_height: px(0.),
            text_input: cx.new(|cx| TextInput::new(cx)),
        }
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

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_dragging {
            let delta = event.position.y - self.drag_start_y;

            let new_offset = self.last_scroll_offset + delta;

            // Scroll bounds
            let container_h = px(620.);

            // prevent min > max
            let min_scroll = (container_h - self.content_height).min(px(0.));
            let max_scroll = px(0.);

            self.scroll_offset = new_offset.clamp(min_scroll, max_scroll);

            cx.notify();
        }
    }

    fn calculate_popup_size(apps_len: usize) -> (Pixels, Pixels) {
        let grid_row_height = px(142.0); // your grid height
        let header_height = px(46.0); // your category header image height
        let padding = px(40.0); // top/bottom padding

        let rows = ((apps_len as f32) / 4.0).ceil();
        println!("Calculated rows for popup: {}", rows);

        // let popup_height = header_height + grid_row_height * rows + padding;
        let popup_height = grid_row_height * rows + padding;

        let popup_width = px(508.0); // same as grid width

        (popup_width, popup_height)
    }
}

impl Render for AppDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_bounds_popup =
            WindowBounds::Windowed(Bounds::centered(None, size(px(420.0), px(420.0)), cx));
        let window_bounds = window.bounds();

        // Group apps by category
        let mut grouped: BTreeMap<String, Vec<AppInfo>> = BTreeMap::new();
        for app in self.state.apps.clone() {
            grouped.entry(app.category.clone()).or_default().push(app);
        }

        // Estimate your actual content height
        let mut total_height = px(0.0);
        let grid_row_h = px(142.0); // height of one grid box
        let spacing_between_sections = px(32.0); // spacing between sections

        for (_, apps) in grouped.iter() {
            // one row per 4 apps
            let rows = ((apps.len() as f32) / 4.0).ceil() as usize;
            let section_height = grid_row_h * rows;

            total_height += section_height + spacing_between_sections;
        }

        self.content_height = total_height - SEARCH_BAR_HEIGHT.into();

        let text_input = self.text_input.clone();
        text_input.update(cx, |input, _| {
            input.placeholder = "Search here".into();
        });

        div()
            .bg(rgb(0x101010))
            .size_full()
            .child(
                div()
                    .bg(rgb(0x101010))
                    .pt_16()
                    .pl_4()
                    .pr_4()
                    .size_full()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                    .children(grouped.into_iter().map(|(category, apps)| {
                        let total = apps.len();
                        let show_popup = total > 4;
                        let shown_apps = if show_popup {
                            apps.iter().take(4).cloned().collect::<Vec<_>>()
                        } else {
                            apps.clone()
                        };

                        let popup_apps = apps.clone();
                        let popup_apps_for_header = popup_apps.clone();
                        let cat_for_button = category.clone();
                        let cat_for_button_cloned = cat_for_button.clone(); // FIX

                        div()
                            .pt(px(-38.))
                            .child(
                                div()
                                    .grid()
                                    .grid_cols(4)
                                    .gap(px(24.))
                                    .bg(rgb(0x181818))
                                    .p(px(20.))
                                    .rounded(px(12.))
                                    .w(px(508.))
                                    .h(px(142.))
                                    .justify_center()
                                    .top(self.scroll_offset)
                                    .children(shown_apps.into_iter().map(|app| {
                                        let app_name = app.name.clone();
                                        let app_icon = app.icon_path.clone();
                                        let app_id = app.id.clone();

                                        IconButton::new(("app", app_id)).icon(app_icon).on_click(
                                            cx.listener(move |_, _, _, _| {
                                                println!("Launching app: {}", app_name);
                                            }),
                                        )
                                    }))
                                    .when(show_popup, |grid| {
                                        grid.child(
                                            div()
                                                .col_span(4)
                                                .flex()
                                                .justify_center()
                                                .pt(px(8.))
                                                .child(button("Show more", move |window, cx| {
                                                    let popup_apps = popup_apps.clone();
                                                    let cat = cat_for_button_cloned.clone();

                                                    let (popup_w, popup_h) =
                                                        AppDrawer::calculate_popup_size(
                                                            popup_apps.len(),
                                                        );

                                                    let popup_size = size(popup_w, popup_h);

                                                    let popup_origin = point(
                                                        window_bounds.origin.x
                                                            + (window_bounds.size.width
                                                                - popup_size.width)
                                                                / 2.0,
                                                        window_bounds.origin.y + px(20.0), // top position
                                                    );

                                                    let popup_bounds = Bounds {
                                                        origin: popup_origin,
                                                        size: popup_size,
                                                    };

                                                    cx.open_window(
                                                        WindowOptions {
                                                            window_bounds: Some(
                                                                WindowBounds::Windowed(
                                                                    popup_bounds,
                                                                ),
                                                            ),
                                                            kind: WindowKind::PopUp,
                                                            show: true,
                                                            is_movable: true,
                                                            ..Default::default()
                                                        },
                                                        move |_, cx| {
                                                            cx.new(|_| SubWindow {
                                                                apps: popup_apps.clone(),
                                                                category: cat.clone(),
                                                            })
                                                        },
                                                    )
                                                    .unwrap();
                                                })),
                                        )
                                    }),
                            )
                            .child({
                                let popup_data = popup_apps_for_header.clone();
                                let popup_data_cloned = popup_data.clone();
                                let cat_for_popup = category.clone();
                                let cat_for_popup_cloned = cat_for_popup.clone();
                                let cat_for_header = category.clone();

                                div()
                                    .id("category-header")
                                    .relative()
                                    .flex_col()
                                    .top(self.scroll_offset)
                                    .on_click(cx.listener(move |_, _event, _window, cx| {
                                        let popup_data2 = popup_data_cloned.clone();
                                        let cat = cat_for_popup_cloned.clone();

                                        cx.open_window(
                                            WindowOptions {
                                                window_bounds: Some(window_bounds_popup),
                                                kind: WindowKind::PopUp,
                                                ..Default::default()
                                            },
                                            move |_, cx| {
                                                cx.new(|_| SubWindow {
                                                    apps: popup_data2,
                                                    category: cat,
                                                })
                                            },
                                        )
                                        .unwrap();
                                    }))
                                    .child(
                                        img("icons/app_drawer/category.png")
                                            .top(px(-46.))
                                            .left(px(-10.))
                                            .w(px(528.))
                                            .h(px(38.))
                                            .bottom(px(-50.)),
                                    )
                                    .child(
                                        div()
                                            .absolute()
                                            .top(px(-24.0))
                                            .left(px(12.0))
                                            .w(px(100.0))
                                            .h(px(16.0))
                                            .justify_start()
                                            .child(
                                                div().flex().child(
                                                    div()
                                                        .font_weight(FontWeight(400.0))
                                                        .text_size(px(16.0))
                                                        .text_color(rgb(0x888888))
                                                        .child(cat_for_header.clone()),
                                                ),
                                            ),
                                    )
                            })
                    })),
            )
            .child(
                div()
                    .h(px(SEARCH_BAR_HEIGHT))
                    .absolute()
                    .bottom(px(6.))
                    .left(px(6.))
                    .right(px(6.))
                    .w(px(508.))
                    .child(
                        div().size_full().flex().flex_row().items_center().child(
                            div()
                                .size_full()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .rounded(px(28.0))
                                .bg(rgb(0x363636))
                                .border_color(rgb(0x575757))
                                .child(
                                    div().flex().flex_row().items_center().child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    // .w(px(24.0))
                                                    // .h(px(24.0))
                                                    .rounded(px(8.0))
                                                    .child(
                                                        div()
                                                            .w(px(22.0))
                                                            .h(px(22.0))
                                                            .items_center()
                                                            .justify_center()
                                                            .mr(px(8.0))
                                                            .ml(px(16.0))
                                                            .flex()
                                                            .child(Icon::build(IconName::Search)),
                                                    ),
                                            )
                                            .child(
                                                div().text_size(px(20.0)).child(text_input.clone()),
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
                                        .border_1()
                                        .border_color(rgb(0x808080))
                                        .child(
                                            div()
                                                .size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    div()
                                                        .h(px(15.))
                                                        .w(px(15.))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(Icon::build(IconName::Close)),
                                                ),
                                        ),
                                ),
                        ),
                    ),
            )
        // .child(
        //     div()
        //         .flex()
        //         .absolute()
        //         .bottom(px(19.))
        //         .left(px(210.))
        //         .w(px(120.))
        //         .h(px(4.))
        //         .rounded(px(4.))
        //         .bg(rgb(0x797979)),
        // )
    }
}
