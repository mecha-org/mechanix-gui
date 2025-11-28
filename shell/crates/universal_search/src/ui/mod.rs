pub mod icon;
pub mod input;
pub mod models;

use std::path::PathBuf;

use crate::data::data::*;
use crate::ui::icon::Icon;
use crate::ui::models::FileType;
use freedesktop_icons::lookup;
use gpui::*;
use icon::IconName;
use models::DragInfo;
use models::SearchResults;
use models::TextInput;
use models::UniversalSearch;
use mxsearch::service::MxSearchService;

const APP_SECTION_HEIGHT: f32 = 76.0;
const FILE_SECTION_HEIGHT: f32 = 56.0;
const FILE_SECTION_DIVIDER_HEIGHT: f32 = 1.0;
const SEARCH_BAR_HEIGHT: f32 = 56.0;
const NAVBAR_SIZE: (f32, f32) = (180., 29.);
const APP_SIZE: (f32, f32) = (540., 620.);

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

        let text_input = cx.new(|cx| {
            TextInput::new(cx).on_change(move |input, cx| {
                let query = input.content.to_string();
                entity.update(cx, |this, cx| {
                    this.perform_search(query, cx);
                });
            })
        });

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
            text_input,
            position: Self::closed_pos(),
            drag_offset: None,
            drag_start_pos: 0.0,
            search_service: None,
            file_search_results: Vec::new(),
            app_search_results: Vec::new(),
        }
    }

    pub fn perform_search(&mut self, query: String, cx: &mut Context<Self>) {
        // If query is empty, clear results
        if query.is_empty() {
            self.file_search_results.clear();
            self.file_count = 0;
            cx.notify();
            return;
        }

        let Some(search_service) = self.search_service.clone() else {
            eprintln!("Search service not initialized yet");
            return;
        };

        let entity = cx.entity();

        cx.new(|cx| {
            cx.spawn(async move |_, cx| {
                match search_service.search_files(&query).await {
                    Ok(results) => {
                        entity
                            .update(cx, |this, cx| {
                                this.file_search_results = results;
                                cx.notify();
                            })
                            .ok();
                    }
                    Err(e) => {
                        eprintln!("Search error: {:?}", e);
                    }
                };
                match search_service.search_applications(&query).await {
                    Ok(results) => {
                        entity
                            .update(cx, |this, cx| {
                                this.app_search_results = results;
                                cx.notify();
                            })
                            .ok();
                    }
                    Err(e) => {
                        eprintln!("Search error: {:?}", e);
                    }
                }
            })
            .detach();
        });
    }

    fn calculate_scroll_bounds(&self, content_height: Pixels) -> (Pixels, Pixels) {
        // Fixed container height - adjust this value as needed
        let container_height = px(620.0 - SEARCH_BAR_HEIGHT); // You can change this to whatever height you want

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
        // Calculate icon grid height
        let columns = 6;
        let icon_rows = ((self.app_count as f32) / (columns as f32)).ceil() as f32;
        let icon_section_height = px(76.0) * icon_rows + px(16.0); // 76px per row + margin

        // Calculate file list height (56px per row + 1px divider)
        let file_section_height = px(57.0) * (self.file_count as f32);

        // Total content height with padding
        icon_section_height + file_section_height + px(16.0)
    }

    // fn on_mouse_down(
    //     &mut self,
    //     event: &MouseDownEvent,
    //     _window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) {
    //     self.drag_start_y = event.position.y;
    //     self.last_scroll_offset = self.scroll_offset;
    //     self.is_dragging = true;
    //     cx.stop_propagation();
    // }

    // fn on_mouse_up(&mut self, _event: &MouseUpEvent, _: &mut Window, _cx: &mut Context<Self>) {
    //     self.is_dragging = false;
    //     self.last_scroll_offset = self.scroll_offset;
    // }

    // fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
    //     if self.is_dragging {
    //         let delta_y = event.position.y - self.drag_start_y;

    //         // Calculate new scroll offset
    //         let new_scroll_offset = self.last_scroll_offset + delta_y;

    //         // Apply bounds based on current content
    //         let content_height = self.estimate_content_height();
    //         let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);

    //         self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);

    //         cx.notify();
    //     }
    // }

    fn on_drag_move(
        &mut self,
        event: &DragMoveEvent<DragInfo>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            let delta_y = event.event.position.y - self.drag_start_y;

            // Calculate new scroll offset
            let new_scroll_offset = self.last_scroll_offset + delta_y;

            // Apply bounds based on current content
            let content_height = self.estimate_content_height();
            let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);

            self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);

            cx.notify();
        }
    }

    fn on_drop(&mut self, _: &DragMoveEvent<DragInfo>, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
        self.last_scroll_offset = self.scroll_offset;
    }

    fn resolved_icon(app_icon: &Option<PathBuf>) -> Icon {
        match app_icon {
            Some(path) => Icon::default()
                .path(path.to_string_lossy().to_string())
                .size((px(21.82), px(21.82)))
                .text_color(rgb(0xFFCC23)),
            None => Icon::from(IconName::DefaultApp).size((px(21.82), px(21.82))),
        }
    }
}

impl Render for UniversalSearch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_y = 0.;
        let closed_y = Self::closed_pos();

        let threshold_px = 40.;

        div()
            .w_full()
            .h_full()
            .on_mouse_move(
                cx.listener(move |this, event: &MouseMoveEvent, _window, cx| {
                    if let Some(offset) = this.drag_offset {
                        let new_y = event.position.y.to_f64() as f32 - offset;
                        this.position = new_y.clamp(open_y, closed_y);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.drag_offset.is_some() {
                        this.drag_offset = None;

                        let target;
                        let started_closed = this.drag_start_pos > (closed_y / 2.0);

                        if started_closed {
                            if this.position < (closed_y - threshold_px) {
                                target = open_y;
                                this.update_input_regions(window, false);
                            } else {
                                target = closed_y;
                                this.update_input_regions(window, true);
                            }
                        } else {
                            if this.position > (open_y + threshold_px) {
                                target = closed_y;
                                this.update_input_regions(window, true);
                            } else {
                                target = open_y;
                                this.update_input_regions(window, false);
                            }
                        }
                        this.snap_to(target, cx);
                        cx.notify();
                    }
                }),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .absolute()
                    .top(px(self.position))
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_start()
                            .h(px(NAVBAR_SIZE.1))
                            .child(
                                img(IconName::Navbar.resolve())
                                    .id("universal-search-navbar")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                            cx.stop_propagation();
                                            this.drag_start_pos = this.position;
                                            this.drag_offset = Some(
                                                event.position.y.to_f64() as f32 - this.position,
                                            );
                                            cx.notify();
                                        }),
                                    ),
                            ),
                    )
                    .child(self.universal_search_items(cx)),
            )
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
        let mut regions = Vec::new();

        if open {
            regions.push(Bounds {
                origin: point(px(0.), px(APP_SIZE.1 - NAVBAR_SIZE.1)),
                size: size(px(NAVBAR_SIZE.0), px(APP_SIZE.1)),
            });
        } else {
            regions.push(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(APP_SIZE.0), px(APP_SIZE.1)),
            });
        }
        window.set_input_regions(Some(regions));
    }

    fn universal_search_items(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let apps = sample_recent_apps();

        self.app_count = apps.len();

        let all_results: Vec<SearchResults> = if self.file_search_results.is_empty() {
            Vec::new()
        } else {
            self.file_search_results
                .iter()
                .map(|result| SearchResults {
                    name: result.name.clone(),
                    file_type: FileType::File,
                    path: String::new(),
                    extension: result.file_type.clone(),
                })
                .chain(self.app_search_results.iter().map(|result| SearchResults {
                    name: result.name.clone(),
                    file_type: FileType::App,
                    path: result.icon.clone(),
                    extension: String::new(),
                }))
                .collect()
        };

        self.file_count = all_results.len();

        // Calculate content height
        let content_height = self.estimate_content_height();
        let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);

        // Update scroll bounds
        self.scroll_offset = self.scroll_offset.clamp(min_scroll, max_scroll);

        let arrow_up_right_icon = self.arrow_up_right_icon.clone();
        let search_icon = self.search_icon.clone();
        let x_icon = self.x_icon.clone();
        let text_input = self.text_input.clone();

        let app = |icon: Icon| {
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
                .id("button")
                .child(icon)
        };

        let row = move |search: &SearchResults| {
            // Add `move` and take reference
            // let folder_small_icon_clone = folder_small_icon.clone(); // Clone the icon
            // let arrow_up_right_icon_clone = arrow_up_right_icon.clone(); // Clone this too

            div().h(px(FILE_SECTION_HEIGHT)).w_full().child(
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
                                            .rounded(px(8.0))
                                            .child(
                                                div().w(px(21.82)).h(px(21.82)).child(match search
                                                    .file_type
                                                {
                                                    FileType::App => {
                                                        let ab: Option<PathBuf> =
                                                            lookup(&search.path).find();
                                                        UniversalSearch::resolved_icon(&ab)
                                                    }
                                                    FileType::File => Icon::from(
                                                        get_file_extension_icon(&search.extension),
                                                    )
                                                    .size((px(21.82), px(21.82)))
                                                    // .text_color(rgb(0xD2D2D2)),
                                                    .text_color(rgb(0xe9e9e9)),
                                                }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .font_weight(FontWeight(500.0))
                                            .text_size(px(16.0))
                                            .text_color(rgb(0xe9e9e9))
                                            .child(search.name.to_string()),
                                    ),
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
                                        .size((px(21.82), px(21.82)))
                                        .text_color(rgb(0xa6a6a6)),
                                    FileType::File => Icon::from(arrow_up_right_icon.clone())
                                        .size((px(11.0), px(11.0)))
                                        .text_color(rgb(0xa6a6a6)),
                                }),
                        ),
                ),
            )
        };

        let divider = || {
            div()
                .w(px(508.0))
                .h(px(FILE_SECTION_DIVIDER_HEIGHT))
                .bg(rgb(0x202020))
        };

        let mut file_children = Vec::new();
        for file in all_results.iter() {
            file_children.push(row(file));
            file_children.push(divider());
        }

        let mut app_children = Vec::new();
        for item in apps.iter() {
            app_children.push(
                app(Icon::from(item.icon_path.clone())
                    .size((px(41.74), px(41.74)))
                    .text_color(rgb(0xa6a6a6)))
                .row_span(1)
                .col_span(1),
            )
        }

        let max_columns: usize = 6;
        let columns: u16 = self.app_count.min(max_columns).try_into().unwrap();
        let rows: u16 = (((self.app_count as f32) / (columns as f32)).ceil() as u32)
            .try_into()
            .unwrap();

        let grid_size = gpui::size(px(508.0), px(APP_SECTION_HEIGHT * (rows as f32) + 16.0));

        let entity = cx.entity();

        div()
            .h_full()
            .w_full()
            .bg(gpui::black())
            .px(px(16.0))
            .flex()
            .flex_col()
            // .on_mouse_up(MouseButton::Left, cx.listener(UniversalSearch::on_mouse_up))
            .on_drop(cx.listener(UniversalSearch::on_drop))
            // .on_mouse_move(cx.listener(UniversalSearch::on_mouse_move))
            .on_drag_move(cx.listener(UniversalSearch::on_drag_move))
            .child(
                div()
                    .id("drag")
                    .flex()
                    .flex_col()
                    .relative()
                    .h(px(620.0 - SEARCH_BAR_HEIGHT - NAVBAR_SIZE.1))
                    .overflow_hidden()
                    .on_drag(DragInfo::new(), move |_: &DragInfo, position, _, cx| {
                        entity.update(cx, |this, cx| {
                            this.drag_start_y = position.y;
                            this.last_scroll_offset = this.scroll_offset;
                            this.is_dragging = true;
                            cx.stop_propagation();
                            cx.notify();
                        });

                        let data = DragInfo::new().position(position);
                        cx.new(|_| data)
                    })
                    // .on_mouse_down(
                    //     MouseButton::Left,
                    //     cx.listener(UniversalSearch::on_mouse_down),
                    // )
                    .child(
                        div().absolute().top(self.scroll_offset).w_full().child(
                            div()
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
                                                .w(grid_size.width)
                                                .h(grid_size.height)
                                                .content_center()
                                                .items_center()
                                                .px(px(9.))
                                                .child(
                                                    div()
                                                        .gap(px(26.0))
                                                        .grid()
                                                        .grid_cols(columns)
                                                        .grid_rows(rows)
                                                        .children(app_children),
                                                ),
                                        ),
                                )
                                .children(file_children),
                        ),
                    ),
            )
            .child(
                div()
                    .h(px(SEARCH_BAR_HEIGHT))
                    .w_full()
                    .bottom(px(0.0))
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
                                                    .justify_center()
                                                    .mr(px(8.0))
                                                    .ml(px(16.0))
                                                    .w(px(24.0))
                                                    .h(px(24.0))
                                                    .rounded(px(8.0))
                                                    .child(
                                                        div().w(px(17.0)).h(px(17.0)).child(
                                                            Icon::from(search_icon.clone())
                                                                .size((px(17.0), px(17.0)))
                                                                .text_color(rgb(0x808080)),
                                                        ),
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
                                        .id("button")
                                        .on_click(cx.listener(
                                            |this: &mut UniversalSearch, _event, _window, cx| {
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
                                            },
                                        ))
                                        .border_color(rgb(0x808080))
                                        .child(
                                            div()
                                                .size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    div()
                                                        .h(px(21.5))
                                                        .w(px(21.5))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(
                                                            Icon::from(x_icon.clone())
                                                                .size((px(15.0), px(15.0)))
                                                                .text_color(rgb(0xe9e9e9)),
                                                        ),
                                                ),
                                        ),
                                ),
                        ),
                    ),
            )
    }
}
