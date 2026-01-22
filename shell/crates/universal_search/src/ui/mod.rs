pub mod models;

use std::path::PathBuf;

use crate::data::data::*;
use crate::ui::models::FileType;
use commons::input::TextInput;
use dispatcher::Dispatcher;
use gpui::prelude::FluentBuilder;
use gpui::*;
use icons::prelude::*;
use models::{DragInfo, SearchResults, UniversalSearch};
use mxsearch::prelude::AppInfo;
use mxsearch::service::MxSearchService;
use settings::prelude::Settings;
use theme::ActiveTheme;
use theme::prelude::{AlphaExt, Fonts, Theme};

const APP_SECTION_HEIGHT: f32 = 76.0;
const ROW_SECTION_HEIGHT: f32 = 52.0;
const FILE_SECTION_DIVIDER_HEIGHT: f32 = 1.0;
const SEARCH_HEADER_HEIGHT: f32 = 44.;
const SEARCH_BAR_HEIGHT: f32 = 56.0;
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
        let files_app_name = Settings::global(cx).system_apps.clone().files;
        cx.spawn(async move |this, cx| {
            if let Ok(service) = MxSearchService::new().await {
                let mut files_app: Option<AppInfo> = None;
                if let Ok(apps) = service.search_applications(&files_app_name).await {
                    files_app = apps.first().cloned();
                }

                this.update(cx, |this, cx| {
                    this.search_service = Some(service);
                    this.files_app = files_app;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
        let UniversalSearchIcons {
            ardour: ardour_icon,
            chromium: chromium_icon,
            firefox: firefox_icon,
            github: github_icon,
            default_folder: folder_icon,
            search: search_icon,
            x: x_icon,
            arrow_up_right: arrow_up_right_icon,
            ..
        } = Icons::global(cx).universal_search.clone();

        let homesceen_size = Settings::global(cx).homescreen.clone().layer_shell.size;
        let status_bar_size = Settings::global(cx).homescreen.clone().status_bar_size;
        let app_size = gpui::size(
            homesceen_size.width,
            homesceen_size.height - status_bar_size.height,
        );

        Self {
            app_count: 0,
            file_count: 0,
            scroll_offset: px(0.0),
            is_dragging: false,
            drag_start_y: px(0.0),
            last_scroll_offset: px(0.0),
            ardour_icon,
            arrow_up_right_icon,
            chromium_icon,
            firefox_icon,
            github_icon,
            folder_icon,
            search_icon,
            x_icon,
            text_input: cx.new(|cx| TextInput::new(cx)),
            last_search_query: String::new(),
            is_searching: false,
            drag_offset: None,
            drag_start_pos: 0.0,
            search_service: None,
            file_search_results: Vec::new(),
            app_search_results: Vec::new(),
            files_app: None,
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

    fn calculate_scroll_bounds(
        &self,
        app_size: Size<Pixels>,
        content_height: Pixels,
    ) -> (Pixels, Pixels) {
        let container_height = app_size.height - px(SEARCH_BAR_HEIGHT) - px(SEARCH_HEADER_HEIGHT);

        if content_height <= container_height {
            return (px(0.0), px(0.0));
        }

        let max_scroll = px(0.0);
        let min_scroll = container_height - content_height;
        (min_scroll, max_scroll)
    }

    fn estimate_content_height(&self) -> Pixels {
        let total_rows_count = self.file_count + self.app_count;
        if total_rows_count == 0 {
            return px(0.0);
        }
        px(total_rows_count as f32 * ROW_SECTION_HEIGHT) + px(ROW_SECTION_HEIGHT)
    }

    fn is_search_result_empty(&self) -> bool {
        self.app_search_results.is_empty() && self.file_search_results.is_empty()
    }

    fn build_search_results(&self) -> Vec<SearchResults> {
        if self.is_search_result_empty() {
            return Vec::new();
        }

        self.app_search_results
            .iter()
            .map(|result| SearchResults {
                name: result.name.clone(),
                file_type: FileType::App,
                path: result.icon_path.clone(),
                extension: String::new(),
                possible_app_id: result.possible_app_id.clone(),
                exec: result.exec.clone(),
            })
            .chain(self.file_search_results.iter().map(|result| {
                let mut exec = String::new();
                let mut possible_app_id = String::new();
                if let Some(files_app) = &self.files_app {
                    exec = format!("{} --open-path={}", files_app.exec, result.path.clone());
                    possible_app_id = files_app.possible_app_id.clone();
                }
                SearchResults {
                    name: result.name.clone(),
                    file_type: FileType::File,
                    path: Some(result.path.clone()),
                    extension: result.file_type.clone(),
                    possible_app_id,
                    exec,
                }
            }))
            .collect()
    }

    fn on_drag_move(
        &mut self,
        event: &DragMoveEvent<DragInfo>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_dragging {
            return;
        }
        if !self.is_search_result_empty() {
            cx.stop_propagation();
        }

        let delta_y = event.event.position.y - self.drag_start_y;
        let new_scroll_offset = self.last_scroll_offset + delta_y;
        let content_height = self.estimate_content_height();
        let homesceen_size = Settings::global(cx).homescreen.clone().layer_shell.size;
        let status_bar_size = Settings::global(cx).homescreen.clone().status_bar_size;
        let app_size = gpui::size(
            homesceen_size.width,
            homesceen_size.height - status_bar_size.height,
        );
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(app_size, content_height);

        self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);
        cx.notify();
    }

    fn on_drop(&mut self, _: &DragMoveEvent<DragInfo>, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
        self.last_scroll_offset = self.scroll_offset;
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
    fn closed_pos(app_size: Size<Pixels>, navbar_size: Size<Pixels>) -> f32 {
        (app_size.height - navbar_size.height).into()
    }

    fn on_app_click(&self, possible_app_id: String, exec: String, cx: &mut Context<Self>) {
        let sender = Dispatcher::global(cx).0.clone();
        cx.background_executor()
            .spawn(async move {
                _ = sender
                    .broadcast(dispatcher::Message::LaunchApp {
                        app_id: possible_app_id,
                        exec,
                    })
                    .await;
            })
            .detach();
    }

    fn build_search_result_row(
        &self,
        index: usize,
        search: &SearchResults,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let arrow_up_right_icon = self.arrow_up_right_icon.clone();
        let colors = Theme::global(cx).colors.clone();
        let icons = Icons::global(cx).universal_search.clone();

        div().h(px(ROW_SECTION_HEIGHT)).w_full().child(
            div().size_full().flex().flex_row().items_center().child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .text_color(colors.foreground_400)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .child(
                                div()
                                    .id(("us-app-icon-", index))
                                    .mr(px(8.0))
                                    .bg(colors.background_700)
                                    .w(px(32.0))
                                    .h(px(32.0))
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .rounded(px(8.0))
                                    .on_click({
                                        let possible_app_id = search.possible_app_id.clone();
                                        let exec = search.exec.clone();
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.on_app_click(
                                                possible_app_id.clone(),
                                                exec.clone(),
                                                cx,
                                            );
                                        })
                                    })
                                    .child(
                                        div()
                                            .when(search.file_type == FileType::App, |this| {
                                                this.when_none(&search.path, |this| {
                                                    this.child(
                                                        svg()
                                                            .external_path(SharedString::from(
                                                                icons
                                                                    .default_app
                                                                    .to_string_lossy()
                                                                    .to_string(),
                                                            ))
                                                            .text_color(colors.foreground_900)
                                                            .size(px(22.00)),
                                                    )
                                                })
                                                .when_some(search.path.clone(), |this, path| {
                                                    if path.is_empty() {
                                                        this.child(
                                                            svg()
                                                                .external_path(SharedString::from(
                                                                    icons
                                                                        .default_app
                                                                        .to_string_lossy()
                                                                        .to_string(),
                                                                ))
                                                                .text_color(colors.foreground_900)
                                                                .size(px(22.00)),
                                                        )
                                                    } else {
                                                        this.child(
                                                            img(PathBuf::from(path))
                                                                .w(px(22.26))
                                                                .h(px(22.26))
                                                                .object_fit(ObjectFit::Cover),
                                                        )
                                                    }
                                                })
                                            })
                                            .when(search.file_type == FileType::File, |this| {
                                                this.child(
                                                    svg()
                                                        .external_path(SharedString::from(
                                                            get_file_extension_icon(
                                                                &search.extension,
                                                                cx,
                                                            )
                                                            .to_string_lossy()
                                                            .to_string(),
                                                        ))
                                                        .size(px(22.26))
                                                        .text_color(colors.foreground_400),
                                                )
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .id(("us-app-name-", index))
                                    .font_weight(FontWeight::MEDIUM)
                                    .line_height(px(1.2))
                                    .text_size(px(16.0))
                                    .text_color(colors.foreground_400)
                                    .child(search.name.to_string())
                                    .on_click({
                                        let app_id = search.possible_app_id.clone();
                                        let exec = search.exec.clone();
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.on_app_click(app_id.clone(), exec.clone(), cx);
                                        })
                                    }),
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
                                FileType::App => svg()
                                    .external_path(SharedString::from(
                                        icons
                                            .arrow_counter_clock_wise
                                            .to_string_lossy()
                                            .to_string(),
                                    ))
                                    .size(px(20.0))
                                    .text_color(colors.foreground_800),
                                FileType::File => svg()
                                    .external_path(SharedString::from(
                                        arrow_up_right_icon.to_string_lossy().to_string(),
                                    ))
                                    .size(px(18.0))
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

        for (index, result) in all_results.iter().enumerate() {
            children.push(
                self.build_search_result_row(index, result, cx)
                    .into_any_element(),
            );
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
        let primary_font = Fonts::global(cx).primary.clone();
        let homesceen_size = Settings::global(cx).homescreen.clone().layer_shell.size;
        let status_bar_size = Settings::global(cx).homescreen.clone().status_bar_size;
        let app_size = gpui::size(
            homesceen_size.width,
            homesceen_size.height - status_bar_size.height,
        );

        // Initialize text input
        self.text_input.update(cx, |input, _| {
            input.placeholder_color = Some(colors.accent_300.with_alpha(0.4));
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
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(app_size, content_height);
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
        .font_family(primary_font.clone())
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
                    div()
                        .w_full()
                        .h(px(SEARCH_HEADER_HEIGHT))
                        .flex()
                        .flex_col()
                        .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .items_center()
                                    .h_full()
                                    .child(
                                        div()
                                            .font_family(primary_font)
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_size(px(24.0))
                                            .line_height(px(1.2))
                                            .text_color(colors.foreground_200)
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
                .child(
                    div()
                        .id("scroll-viewport")
                        .relative()
                        .flex_1()  
                        .overflow_hidden()
                        .on_drop(cx.listener(Self::on_drop))
                        .on_drag_move(cx.listener(Self::on_drag_move))
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
                        .when(content_children.len() == 0, |viewport| {
                            viewport.child(
                                div()
                                    .flex()
                                    .items_start()
                                    .justify_start()
                                    .h(px(16.)).py(px(10.))
                                    .child(
                                        div()
                                            .font_weight(FontWeight::LIGHT)
                                            .line_height(px(1.2))
                                            .text_size(px(16.0))
                                            .text_color(colors.foreground_900)
                                            .child("Search an app, a file, a word or anything literally"),
                                    )
                            )
                        })
                        .when(content_children.len() > 0, |viewport| {
                            viewport.child(
                                div()
                                    .absolute()
                                    .top(self.scroll_offset)
                                    .left(px(0.0))
                                    .right(px(0.0))
                                    .flex()
                                    .flex_col()
                                    .children(content_children),
                            )
                        }),
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
                                        .justify_center()
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
                                                    svg()
                                                        .external_path(SharedString::from(
                                                            search_icon
                                                                .to_string_lossy()
                                                                .to_string(),
                                                        ))
                                                        .size(px(24.0))
                                                        .text_color(colors.accent_300),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .font_weight(FontWeight::MEDIUM)
                                                .line_height(px(1.25))
                                                .text_size(px(18.0))
                                                .text_color(colors.foreground_200)
                                                .items_center()
                                                .justify_center()
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
                                    svg()
                                        .external_path(SharedString::from(
                                            x_icon.to_string_lossy().to_string(),
                                        ))
                                        .size(px(24.0))
                                        .text_color(colors.foreground_400),
                                ),
                        ),
                ),
        )
    }
}
