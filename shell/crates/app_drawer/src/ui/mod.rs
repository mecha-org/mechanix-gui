use gpui::prelude::*;
use gpui::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use crate::models::AppDrawerState;
use crate::prelude::Icon;
use crate::prelude::IconName;
use crate::ui::utils::prelude::{DesktopApp, DesktopApps};
use crate::ui::widgets::{IconButton, SubWindow};
use input::TextInput;

pub mod icon;
pub mod input;
pub mod utils;
mod widgets;

const SEARCH_BAR_HEIGHT: f32 = 56.0;
const SEARCH_BAR_WIDTH: f32 = 508.0;

const GRID_ROW_HEIGHT: f32 = 142.0;
const GRID_ROW_WIDTH: f32 = 508.0;

const SECTION_SPACING: f32 = 32.0;
const APP_SIZE: (f32, f32) = (540., 620.);
const APP_SECTION_DIVIDER_HEIGHT: f32 = 1.0;
const APP_SECTION_DIVIDER_WIDTH: f32 = 508.0;

const APP_SECTION_HEIGHT: f32 = 76.0;
const APP_ROW_HEIGHT: f32 = 56.0;

pub struct AppDrawer {
    pub state: AppDrawerState,
    pub grouped: HashMap<String, Vec<DesktopApp>>,
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
    content_height: Pixels,
    pub text_input: Entity<TextInput>,
    show_popup: bool,
    popup_category: String,
    filtered: Vec<DesktopApp>,
    is_searching: bool,
    last_search_query: String,
}

impl AppDrawer {
    pub fn new(state: AppDrawerState, cx: &mut Context<Self>) -> Self {
        let grouped = state.apps.clone().get_apps_by_categories();
        let filteredApps = state.apps.apps.clone();

        Self {
            state,
            grouped,
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
            content_height: px(0.),
            text_input: cx.new(|cx| TextInput::new(cx)),
            show_popup: false,
            popup_category: "".to_string(),
            filtered: filteredApps.clone(),
            is_searching: false,
            last_search_query: "".to_string(),
        }
    }

    fn calculate_scroll_bounds(&self, content_height: Pixels) -> (Pixels, Pixels) {
        // Fixed container height - adjust this value as needed
        let container_height = px(APP_SIZE.1 - SEARCH_BAR_HEIGHT); // You can change this to whatever height you want

        // Max scroll: when content is at the top (no empty space)
        let max_scroll = px(0.0);

        // Min scroll: when bottom of content reaches container bottom
        let min_scroll = container_height - content_height;

        // If content is smaller than container, don't allow scrolling
        if content_height <= container_height {
            (px(0.0), px(0.0))
        } else {
            (min_scroll, max_scroll)
        }
    }

    fn estimate_content_height(&self) -> Pixels {
        if self.is_searching {
            let apps_section_height =
                px(APP_ROW_HEIGHT + APP_SECTION_DIVIDER_HEIGHT) * (self.filtered.len() as f32);

            // Total content height with padding
            apps_section_height + px(16.0)
        } else {
            // Count total apps per category
            let grouped: HashMap<String, Vec<DesktopApp>> =
                self.state.apps.clone().get_apps_by_categories();

            let mut total = px(0.0);

            for (_, apps) in grouped.iter() {
                let rows = ((apps.len() as f32) / 4.0).ceil() as usize;

                // Add height for this category
                total += px(GRID_ROW_HEIGHT) * rows + px(SECTION_SPACING);
            }

            // Adjust for search bar since drawer height = 620px - SEARCH_BAR_HEIGHT
            total - px(SEARCH_BAR_HEIGHT)
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
        self.last_scroll_offset = self.scroll_offset;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;

            // Calculate new scroll offset
            let new_scroll_offset = self.last_scroll_offset + delta_y;

            // Apply bounds based on current content
            let content_height = self.estimate_content_height();
            let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);

            self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);

            cx.notify();
        }
    }

    fn filter(&mut self, cx: &mut Context<Self>) {
        let query = self.text_input.read(cx).content.clone();

        // Reset scroll when the text actually changes
        if query != self.last_search_query {
            self.scroll_offset = px(0.);
            self.last_scroll_offset = px(0.);
            self.drag_start_y = px(0.);
            self.is_dragging = false;

            self.last_search_query = query.to_string();
        }

        let query_lower = query.to_lowercase();

        self.filtered = if query_lower.is_empty() {
            self.state.apps.clone().apps
        } else {
            self.state
                .apps
                .apps
                .iter()
                .filter(|a| a.name.to_lowercase().contains(&query_lower))
                .cloned()
                .collect()
        };

        cx.notify();
    }
}

impl Render for AppDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_bounds = window.bounds();

        // Group apps by category
        let apps = self.filtered.clone();

        // Search
        let mut search_apps_children = Vec::new();

        let text_input = self.text_input.clone();
        text_input.update(cx, |input, _| {
            input.placeholder = "Search here".into();
        });

        let is_active = text_input.read(cx).focus_handle.is_focused(window);

        if is_active && !self.is_searching {
            self.scroll_offset = px(0.);
            self.last_scroll_offset = px(0.);
            self.drag_start_y = px(0.);
        }

        if is_active {
            self.is_searching = true;
            Self::filter(self, cx);

            let row = |searched_app: &DesktopApp, cx: &mut Context<Self>| {
                let exec = searched_app.exec.clone();
                let app_id = searched_app.app_id.clone();
                let id = hash_id(&app_id);

                div()
                    .id(id)
                    .h(px(APP_ROW_HEIGHT))
                    .w_full()
                    .child(
                        div().size_full().flex().flex_row().items_center().child(
                            div()
                                .size_full()
                                .text_color(rgb(0xe9e9e9))
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
                                                    .mr(px(8.0))
                                                    .bg(rgb(0x202020))
                                                    .w(px(36.0))
                                                    .h(px(36.0))
                                                    .flex()
                                                    .justify_center()
                                                    .items_center()
                                                    .rounded(px(6.0))
                                                    .child({
                                                        let icon = DesktopApp::resolved_icon(
                                                            &searched_app.icon_path,
                                                        );

                                                        IconButton::new(id)
                                                            .icon(icon)
                                                            .width(px(30.))
                                                            .height(px(30.))
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .font_weight(FontWeight(500.0))
                                                    .text_size(px(16.0))
                                                    .text_color(rgb(0xe9e9e9))
                                                    .child(searched_app.name.to_string()),
                                            ),
                                    ),
                                ),
                        ),
                    )
                    .on_click(cx.listener(move |_, _, _, _| {
                        let _ = DesktopApps::run_app_exec(exec.as_str());
                    }))
            };

            let divider = || {
                div()
                    .w(px(APP_SECTION_DIVIDER_WIDTH))
                    .h(px(APP_SECTION_DIVIDER_HEIGHT))
                    .bg(rgb(0x202020))
                    .id("divider")
            };

            let searched_apps = self.filtered.clone();

            for searched_app in searched_apps.iter() {
                search_apps_children.push(row(searched_app, cx));
                search_apps_children.push(divider());
            }
        }

        let grouped = &self.grouped;

        div()
            .bg(rgb(0x000000))
            .size_full()
            // normal mode (categories grid)
            .when(!self.is_searching, |main_page_div| {
                    main_page_div
                    .bg(rgb(0x000000))
                    .pt_16()
                    .pl_4()
                    .pr_4()
                    .size_full()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                    .children(
                        grouped
                            .into_iter()
                            .enumerate()
                            .map(|(idx, (category, apps))| {
                                let total = apps.len();
                                let show_popup = total > 4;
                                let shown_apps = if show_popup {
                                    apps.iter().take(4).cloned().collect::<Vec<_>>()
                                } else {
                                    apps.clone()
                                };

                                let category_for_popup = category.clone();

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
                                            .w(px(GRID_ROW_WIDTH))
                                            .h(px(GRID_ROW_HEIGHT))
                                            .justify_center()
                                            .top(self.scroll_offset)
                                            .children(shown_apps.into_iter().map(|app| {
                                                let app_name = app.name.clone();
                                                let app_icon = app.icon_path.clone();
                                                let app_id = app.app_id.clone();
                                                let id = hash_id(&app_id);

                                                let icon =
                                                    DesktopApp::resolved_icon(&app.icon_path);
                                                IconButton::new(id + idx).icon(icon).on_click(
                                                    cx.listener(move |_, _, _, _| {
                                                        let _ = DesktopApps::run_app_exec(
                                                            app.exec.as_str(),
                                                        );
                                                    }),
                                                )
                                            })),
                                    )
                                    .child({
                                        let category_for_header = category.clone();

                                        div()
                                            .relative()
                                            .flex_col()
                                            .top(self.scroll_offset)
                                            .child(
                                                img(IconName::Category.resolve())
                                                    .id(idx)
                                                    .top(px(-46.))
                                                    .left(px(-10.))
                                                    .w(px(528.))
                                                    .h(px(38.))
                                                    .bottom(px(-50.))
                                                    // .when(show_popup, |img| {
                                                    //     img.on_click(cx.listener(
                                                    //         move |this, _event, _window, cx| {
                                                    //             this.show_popup = true;
                                                    //             this.popup_category =
                                                    //                 category_for_popup.clone();

                                                    //             // request re-render
                                                    //             cx.notify();
                                                    //         },
                                                    //     ))
                                                    // }),

                                                .when(show_popup, |img| {
                                                        img.on_click(cx.listener(
                                                            move |_, _event, _window, cx| {
                                                                let popup_category =
                                                                    category_for_popup.clone();
                                                                let popup_origin = point(
                                                                    window_bounds.origin.x,
                                                                    window_bounds.origin.y,
                                                                );

                                                                let popup_bounds = Bounds {
                                                                    origin: popup_origin,
                                                                    size: window_bounds.size,
                                                                };

                                                                cx.open_window(
                                                                    WindowOptions {
                                                                        window_bounds: Some(
                                                                            WindowBounds::Windowed(
                                                                                popup_bounds,
                                                                            ),
                                                                        ),
                                                                        window_background: WindowBackgroundAppearance::Transparent,
                                                                        kind: WindowKind::PopUp,
                                                                        show: true,
                                                                        is_movable: false,
                                                                        ..Default::default()
                                                                    },
                                                                    move |_, cx| {
                                                                        cx.new(|_| {
                                                                            SubWindow::scan(
                                                                                popup_category
                                                                                    .clone(),
                                                                            )
                                                                        })
                                                                    },
                                                                )
                                                                .unwrap();
                                                            },
                                                        ))
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top(px(-28.0))
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
                                                                .child(category_for_header.clone())
                                                                .text_ellipsis()
                                                                .w(px(120.)),
                                                        ),
                                                    ),
                                            )
                                    })
                            }),
                    )
            })
                // searching mode (search results list)
            .when(self.is_searching, |search_div| {
                         search_div
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .w(px(APP_SIZE.0))
                        .h(px(APP_SIZE.1))
                        .bg(rgb(0x000000))
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                        .on_mouse_move(cx.listener(Self::on_mouse_move))
                        .child(
                            div()
                                .absolute()
                                .top(px(10.) + self.scroll_offset)
                                .pl_4()
                                .pr_4()
                                .children(search_apps_children),
                        )
                }
            )
            // .child(if self.show_popup {
            //     div()
            //         .absolute()
            //         .top(px(0.))
            //         .left(px(0.))
            //         .w(px(APP_SIZE.0))
            //         .h(px(APP_SIZE.1))
            //         // Stop all mouse events from reaching AppDrawer
            //         .on_mouse_down(MouseButton::Left, |_, _event, cx| cx.stop_propagation())
            //         .on_mouse_up(MouseButton::Left, |_, _event, cx| cx.stop_propagation())
            //         .on_mouse_move(|_, _event, cx| cx.stop_propagation())
            //         .child(cx.new(|_| SubWindow::scan(self.popup_category.clone())))
            //         .into_any()
            // } else {
            //     Empty.into_any()
            // })
            .child(
                div()
                    .h(px(SEARCH_BAR_HEIGHT))
                    .absolute()
                    .bottom(px(6.))
                    .when(self.is_searching, |d| d.ml(px(16.0)))
                    .w(px(SEARCH_BAR_WIDTH))
                    .on_mouse_down(MouseButton::Left, |_, _event, cx| cx.stop_propagation())
                    .on_mouse_up(MouseButton::Left, |_, _event, cx| cx.stop_propagation())
                    .on_mouse_move(|_, _event, cx| cx.stop_propagation())
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
                                        .id("close-button")
                                        .on_click(cx.listener(
                                                            |this: &mut AppDrawer, _event, _window, cx| {

                                                                this.text_input.update(cx, |input, cx| {
                                                                    input.content = "".into();
                                                                    input.selected_range = 0..0;
                                                                    input.selection_reversed = false;
                                                                    input.marked_range = None;
                                                                    input.last_layout = None;
                                                                    input.last_bounds = None;
                                                                    input.is_selecting = false;
                                                                    cx.notify();
                                                                });
                                                                // Remove focus
                                                                this.text_input.read(cx).blur(_window);

                                                                // Stop searching
                                                                this.is_searching = false;

                                                                this.scroll_offset = px(0.);
                                                                this.last_scroll_offset = px(0.);
                                                                this.drag_start_y = px(0.);
                                                                this.is_dragging = false;
                                                            },
                                                        ))
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
                                                        .child(Icon::build(IconName::Close))
                                                        .id("button")
                                                ),
                                        ),
                                ),
                        ),
                    )
            )
    }
}

fn hash_id(s: &str) -> usize {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() as usize
}
