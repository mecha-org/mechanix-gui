pub mod icon;
pub mod models;

use std::path::PathBuf;

use crate::data::data::*;
use crate::ui::icon::Icon;
use crate::ui::models::FileType;
use commons::input::TextInput;
use freedesktop_icons::lookup;
use gpui::prelude::FluentBuilder;
use gpui::*;
use icon::IconName;
use models::{DragInfo, SearchResults, UniversalSearch};
use mxsearch::service::MxSearchService;
use theme::ActiveTheme;
use theme::prelude::{AlphaExt, Theme};

const APP_SECTION_HEIGHT: f32 = 76.0;
const FILE_SECTION_HEIGHT: f32 = 52.0;
const FILE_SECTION_DIVIDER_HEIGHT: f32 = 1.0;
const SEARCH_BAR_HEIGHT: f32 = 56.0;
const NAVBAR_SIZE: (f32, f32) = (199.22, 28.5);
const APP_SIZE: (f32, f32) = (540., 620.);
const MIN_SEARCH_QUERY_LEN: usize = 3;

impl DragInfo {
    fn new() -> Self {
        Self {
            position: Point::default(),
        }
    }

    fn position(mut self, pos: Point<Pixels>) -> Self {
        self.position = pos;
        self
    }
}

impl Render for DragInfo {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        Empty
    }
}

impl UniversalSearch {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let entity = cx.entity();

        cx.spawn(async move |this, cx| {
            if let Ok(service) = MxSearchService::new().await {
                this.update(cx, |this, cx| {
                    this.search_service = Some(service);
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();

        Self {
            app_count: 0,
            file_count: 0,
            scroll_offset: px(0.0),
            is_dragging: false,
            drag_start_y: px(0.0),
            last_scroll_offset: px(0.0),
            ardour_icon: IconName::Ardour,
            arrow_up_right_icon: IconName::ArrowUpRight,
            chromium_icon: IconName::Chromium,
            firefox_icon: IconName::Firefox,
            github_icon: IconName::Github,
            folder_icon: IconName::DefaultFolder,
            search_icon: IconName::Search,
            x_icon: IconName::XIcon,
            text_input: cx.new(|cx| TextInput::new(cx)),
            last_search_query: String::new(),
            is_searching: false,
            position: Self::closed_pos(),
            drag_offset: None,
            drag_start_pos: 0.0,
            search_service: None,
            file_search_results: Vec::new(),
            app_search_results: Vec::new(),
        }
    }

    pub fn perform_search(&mut self, query: SharedString, cx: &mut Context<Self>) {
        if query.is_empty() {
            self.clear_search_results(cx);
            return;
        }

        if query != self.last_search_query {
            self.reset_scroll_state();
            self.last_search_query = query.to_string();
        }

        let query_lowercase = query.to_lowercase();
        let Some(search_service) = self.search_service.clone() else {
            eprintln!("Search service not initialized yet");
            return;
        };

        self.is_searching = true;
        let entity = cx.entity();

        cx.new(|cx| {
            cx.spawn(async move |_, cx| {
                if let Ok(results) = search_service.search_files(&query_lowercase).await {
                    entity
                        .update(cx, |this, cx| {
                            this.file_search_results = results;
                            this.file_count = this.file_search_results.len();
                            cx.notify();
                        })
                        .ok();
                }
                if let Ok(results) = search_service.search_applications(&query_lowercase).await {
                    entity
                        .update(cx, |this, cx| {
                            this.app_search_results = results;
                            this.app_count = this.app_search_results.len();
                            cx.notify();
                        })
                        .ok();
                }
            })
            .detach();
        });

        cx.notify();
    }

    fn clear_search_results(&mut self, cx: &mut Context<Self>) {
        self.file_search_results.clear();
        self.app_search_results.clear();
        self.file_count = 0;
        self.app_count = 0;
        cx.notify();
    }

    fn reset_scroll_state(&mut self) {
        self.scroll_offset = px(0.);
        self.last_scroll_offset = px(0.);
        self.drag_start_y = px(0.);
        self.is_dragging = false;
    }

    fn calculate_scroll_bounds(&self, content_height: Pixels) -> (Pixels, Pixels) {
        let container_height = px(APP_SIZE.1 - SEARCH_BAR_HEIGHT);

        if content_height <= container_height {
            return (px(0.0), px(0.0));
        }

        let max_scroll = px(0.0);
        let min_scroll = container_height - content_height;
        (min_scroll, max_scroll)
    }

    fn estimate_content_height(&self) -> Pixels {
        let columns = 6.0;
        let icon_rows = (self.app_count as f32 / columns).ceil();
        let icon_section_height = px(APP_SECTION_HEIGHT) * icon_rows + px(16.0);
        let file_section_height =
            px(FILE_SECTION_HEIGHT + FILE_SECTION_DIVIDER_HEIGHT) * self.file_count as f32;

        icon_section_height + file_section_height + px(16.0)
    }

    fn build_search_results(&self) -> Vec<SearchResults> {
        if self.app_search_results.is_empty() && self.file_search_results.is_empty() {
            return Vec::new();
        }

        self.app_search_results
            .iter()
            .map(|result| SearchResults {
                name: result.name.clone(),
                file_type: FileType::App,
                path: result.icon.clone(),
                extension: String::new(),
            })
            .chain(self.file_search_results.iter().map(|result| SearchResults {
                name: result.name.clone(),
                file_type: FileType::File,
                path: String::new(),
                extension: result.file_type.clone(),
            }))
            .collect()
    }

    fn on_drag_move(
        &mut self,
        event: &DragMoveEvent<DragInfo>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_dragging {
            return;
        }

        let delta_y = event.event.position.y - self.drag_start_y;
        let new_scroll_offset = self.last_scroll_offset + delta_y;
        let content_height = self.estimate_content_height();
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);

        self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);
        cx.notify();
    }

    fn on_drop(&mut self, _: &DragMoveEvent<DragInfo>, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
        self.last_scroll_offset = self.scroll_offset;
    }

    fn resolved_icon(app_icon: &Option<PathBuf>) -> Icon {
        match app_icon {
            Some(path) => Icon::default()
                .path(path.to_string_lossy().to_string())
                .size((px(22.26), px(22.26))),
            None => Icon::from(IconName::DefaultApp).size((px(22.26), px(22.26))),
        }
    }

    fn handle_text_input_update(&mut self, cx: &mut Context<Self>, _window: &Window) {
        let query = self.text_input.read(cx).content.clone();
        let query_len = query.trim().chars().count();

        if query_len >= MIN_SEARCH_QUERY_LEN && query != self.last_search_query {
            self.perform_search(query, cx);
        } else if query_len < MIN_SEARCH_QUERY_LEN
            && (!self.file_search_results.is_empty() || !self.app_search_results.is_empty())
        {
            self.clear_search_results(cx);
        }
    }

    fn clear_text_input(&mut self, cx: &mut Context<Self>, window: &mut Window) {
        self.text_input.update(cx, |input, cx| {
            input.reset_state(cx);
        });
        window.blur();
    }
}

impl Render for UniversalSearch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.universal_search_items(cx, window))
    }
}

impl UniversalSearch {
    fn closed_pos() -> f32 {
        APP_SIZE.1 - NAVBAR_SIZE.1
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<UniversalSearch>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    // Check if animation is done
                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.position = target;
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position = current;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }

    fn update_input_regions(&self, window: &mut Window, open: bool) {
        let regions = if open {
            vec![Bounds {
                origin: point(px(0.), px(APP_SIZE.1 - NAVBAR_SIZE.1)),
                size: size(px(NAVBAR_SIZE.0), px(APP_SIZE.1)),
            }]
        } else {
            vec![Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(APP_SIZE.0), px(APP_SIZE.1)),
            }]
        };

        window.set_input_regions(Some(regions));
    }

    fn build_search_result_row(
        &self,
        search: &SearchResults,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let arrow_up_right_icon = self.arrow_up_right_icon.clone();
        let colors = Theme::global(cx).colors.clone();

        div().h(px(FILE_SECTION_HEIGHT)).w_full().child(
            div().size_full().flex().flex_row().items_center().child(
                div()
                    .size_full()
                    .text_color(colors.foreground_400)
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .child(
                                div()
                                    .mr(px(8.0))
                                    .bg(colors.background_700)
                                    .w(px(32.0))
                                    .h(px(32.0))
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .rounded(px(8.0))
                                    .child(
                                        div().child(match search.file_type {
                                            FileType::App => {
                                                let icon_path = lookup(&search.path).find();
                                                Self::resolved_icon(&icon_path)
                                            }
                                            FileType::File => Icon::from(get_file_extension_icon(
                                                &search.extension,
                                            ))
                                            .size((px(22.26), px(22.26)))
                                            .text_color(colors.foreground_400),
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .font_weight(FontWeight(500.0))
                                    .text_size(px(16.0))
                                    .text_color(colors.foreground_400)
                                    .child(search.name.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .h(px(18.0))
                            .w(px(18.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(match search.file_type {
                                FileType::App => Icon::from(IconName::ArrowCounterClockWise)
                                    .size((px(20.), px(20.)))
                                    .text_color(colors.foreground_800),
                                FileType::File => Icon::from(arrow_up_right_icon)
                                    .size((px(18.0), px(18.0)))
                                    .text_color(colors.foreground_800),
                            }),
                    ),
            ),
        )
    }

    fn build_divider() -> impl IntoElement {
        div()
            .w(px(508.0))
            .h(px(FILE_SECTION_DIVIDER_HEIGHT))
            .bg(rgb(0x202020))
    }

    fn build_search_content(
        &self,
        all_results: &[SearchResults],
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut children = Vec::with_capacity(all_results.len() * 2);

        for result in all_results {
            children.push(self.build_search_result_row(result, cx).into_any_element());
            // children.push(Self::build_divider().into_any_element());
        }

        children
    }

    fn universal_search_items(
        &mut self,
        cx: &mut Context<Self>,
        window: &mut Window,
    ) -> impl IntoElement {
        let colors = cx.theme().colors.clone();

        // Initialize text input
        self.text_input.update(cx, |input, _| {
            input.placeholder = "Search here".into();
        });

        let text_input = self.text_input.clone();
        let is_active = text_input.read(cx).focus_handle.is_focused(window);

        // Handle search updates
        if is_active {
            self.handle_text_input_update(cx, window);
        }

        // Build search results
        let all_results = self.build_search_results();

        // Calculate scroll bounds
        let content_height = self.estimate_content_height();
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);
        self.scroll_offset = self.scroll_offset.clamp(min_scroll, max_scroll);

        // Build content children
        let content_children = self.build_search_content(&all_results, cx);

        let search_icon = self.search_icon.clone();
        let x_icon = self.x_icon.clone();
        let entity = cx.entity();

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.background_1000)
            .on_drop(cx.listener(Self::on_drop))
            .on_drag_move(cx.listener(Self::on_drag_move))
            .child(
                div()
                    .id("content-container")
                    .flex()
                    .flex_col()
                    .relative()
                    .h_full()
                    .overflow_hidden()
                    .px_4()
                    .child(
                        div().w_full().h(px(44.)).flex().flex_col().child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .h_full()
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .text_color(colors.foreground_600)
                                        .child("Search"),
                                )
                                .child(
                                    div()
                                        .id("clear-result")
                                        .text_size(px(16.0))
                                        .text_color(if all_results.len() == 0 {
                                            colors.background_800
                                        } else {
                                            colors.foreground_100
                                        })
                                        .on_click(cx.listener(
                                            |this: &mut Self, _event, window, cx| {
                                                this.clear_search_results(cx);
                                                this.clear_text_input(cx, window);
                                            },
                                        ))
                                        .child("Clear all"),
                                ),
                        ),
                    )
                    .when(content_children.len() == 0, |content_div| {
                        content_div.child(
                            div().flex().flex_col().h(px(39.)).child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(colors.background_500)
                                    .child("Search an app, a file, a word or anything literally"),
                            ),
                        )
                    })
                    .child(
                        div()
                            .id("scrollable-content")
                            .flex()
                            .flex_col()
                            .relative()
                            .flex_1()
                            .overflow_hidden()
                            .on_drag(DragInfo::new(), move |_: &DragInfo, position, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.drag_start_y = position.y;
                                    this.last_scroll_offset = this.scroll_offset;
                                    this.is_dragging = true;
                                    cx.stop_propagation();
                                    cx.notify();
                                });

                                cx.new(|_| DragInfo::new().position(position))
                            })
                            .child(
                                div()
                                    .absolute()
                                    .top(self.scroll_offset)
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .my_1()
                                    .children(content_children),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(SEARCH_BAR_HEIGHT))
                    .w_full()
                    .bottom(px(0.0))
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .flex_row()
                            .items_center()
                            .bg(colors.accent_300.with_alpha(0.1))
                            .border_color(colors.background_700)
                            .border_1()
                            .py(px(6.))
                            .child(
                                div()
                                    .w(px(488.))
                                    .h(px(44.))
                                    .ml_2()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .items_center()
                                    .bg(colors.background_800)
                                    .border_color(colors.accent_500)
                                    .border_1()
                                    .rounded_sm()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .justify_center()
                                                    .py_2()
                                                    .ml_3()
                                                    .mr_2()
                                                    .w(px(24.0))
                                                    .h(px(24.0))
                                                    .rounded(px(8.0))
                                                    .child(
                                                        Icon::from(search_icon)
                                                            .size((px(24.0), px(24.0)))
                                                            .text_color(colors.accent_300),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(20.0))
                                                    .text_color(colors.foreground_300)
                                                    .child(text_input),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(40.))
                                    .h(px(40.))
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_center()
                                    .id("cancel-button")
                                    .on_click(cx.listener(|this: &mut Self, _event, window, cx| {
                                        this.clear_text_input(cx, window);
                                    }))
                                    .child(
                                        Icon::from(x_icon)
                                            .size((px(24.0), px(24.0)))
                                            .text_color(colors.foreground_400),
                                    ),
                            ),
                    ),
            )
    }
}
