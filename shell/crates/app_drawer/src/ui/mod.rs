use crate::ui::widgets::{ BottomSheetKind, SubWindow };
use commons::prelude::TextInput;
use commons::widgets::{ WingSide, wing };
use dispatcher::Dispatcher;
use gpui::prelude::*;
use gpui::*;
use icons::prelude::Icons;
use mxsearch::prelude::AppInfo;
use mxsearch::service::MxSearchService;
use settings::prelude::Settings;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };
use std::path::PathBuf;
use std::time::{ Duration, Instant };
use theme::prelude::{ AlphaExt, Fonts, Theme };

pub mod utils;
mod widgets;
const MIN_MODAL_SIZE: (f32, f32) = (10.0, 10.0);
const MODAL_ANIMATION_DURATION_MS: f32 = 300.0;
const MODAL_ANIMATION_FRAME_MS: u64 = 16;

const SEARCH_BAR_HEIGHT: f32 = 56.0;
const GRID_ROW_HEIGHT: f32 = 126.0;
const SECTION_SPACING: f32 = 15.0;
const APP_ROW_HEIGHT: f32 = 60.0;
const FLOATING_BTN_SIZE: f32 = 56.0;
const FLOATING_BTN_BOTTOM: f32 = 16.0;
const FLOATING_BTN_RIGHT: f32 = 16.0;
const SEARCH_BAR_BOTTOM: f32 = 20.0;
const DRAG_THRESHOLD: f32 = 2.0;
const LONG_PRESS_DURATION: Duration = Duration::from_millis(500);

pub struct AppDrawer {
    pub grouped: HashMap<String, Vec<AppInfo>>,
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    all_apps: Vec<AppInfo>,

    drag_start_y: Pixels,
    drag_start_x: Pixels,
    is_dragging: bool,
    has_moved: bool,
    is_vertical_scroll: bool,
    pub text_input: Entity<TextInput>,
    filtered: Vec<AppInfo>,
    is_searching: bool,
    last_search_query: String,
    pub show_bottom_sheet: bool,
    pub sheet_kind: BottomSheetKind,
    pub sheet_app: Option<AppInfo>,
    pub show_subwindow_modal: bool,
    pub subwindow_category: String,
    pub subwindow: Option<Entity<SubWindow>>,
    pub search_service: Option<MxSearchService>,
    is_loading: bool,
    press_start_time: Option<Instant>,
    press_app_info: Option<AppInfo>,
    is_long_press: bool,
    pub modal_animation_progress: f32,
    pub is_modal_animating: bool,
    pub modal_origin_point: Point<Pixels>,
    pub modal_origin_center: (f32, f32),
    pub modal_current_center: (f32, f32),
    pub modal_target_center: (f32, f32),
    pub modal_current_size: (f32, f32),
}

impl AppDrawer {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let grouped = HashMap::new();
        let filtered_apps = Vec::new();

        let drawer = Self {
            grouped,
            scroll_offset: px(0.0),
            last_scroll_offset: px(0.0),
            drag_start_y: px(0.0),
            drag_start_x: px(0.0),
            is_dragging: false,
            has_moved: false,
            is_vertical_scroll: false,
            text_input: cx.new(|cx| TextInput::new(cx)),
            all_apps: Vec::new(),
            filtered: filtered_apps,
            is_searching: false,
            last_search_query: "".to_string(),
            show_bottom_sheet: false,
            sheet_kind: BottomSheetKind::MainOptions,
            sheet_app: None,
            show_subwindow_modal: false,
            subwindow_category: String::new(),
            subwindow: None,
            search_service: None,
            is_loading: true,
            press_start_time: None,
            press_app_info: None,
            is_long_press: false,
            modal_animation_progress: 0.0,
            is_modal_animating: false,
            modal_origin_point: Point::new(px(0.0), px(0.0)),
            modal_origin_center: (0.0, 0.0),
            modal_current_center: (0.0, 0.0),
            modal_target_center: (0.0, 0.0),
            modal_current_size: MIN_MODAL_SIZE,
        };

        cx.spawn(async move |this, cx| {
            match MxSearchService::new().await {
                Ok(service) => {
                    if let Ok(app_infos) = service.list_applications().await {
                        this.update(cx, |this, cx| {
                            this.search_service = Some(service);
                            this.all_apps = app_infos.clone();
                            this.grouped = Self::group_apps_by_category(app_infos.clone());
                            this.filtered = app_infos;
                            this.is_loading = false;
                            cx.notify();
                        }).ok();
                    } else {
                        this.update(cx, |this, cx| {
                            this.is_loading = false;
                            cx.notify();
                        }).ok();
                    }
                }
                Err(_) => {
                    this.update(cx, |this, cx| {
                        this.is_loading = false;
                        cx.notify();
                    }).ok();
                }
            }
        }).detach();

        drawer
    }

    fn group_apps_by_category(apps: Vec<AppInfo>) -> HashMap<String, Vec<AppInfo>> {
        let mut grouped: HashMap<String, Vec<AppInfo>> = HashMap::new();

        for app in apps {
            let mut has_valid_category = false;

            for category in &app.categories {
                let category = category.trim();

                if category.is_empty() {
                    continue;
                }

                grouped.entry(category.to_string()).or_insert_with(Vec::new).push(app.clone());

                has_valid_category = true;
            }

            if !has_valid_category {
                grouped.entry("Other".to_string()).or_insert_with(Vec::new).push(app);
            }
        }

        grouped
    }

    pub fn on_app_click(&self, possible_app_id: String, exec: String, cx: &mut Context<Self>) {
        let sender = Dispatcher::global(cx).0.clone();
        cx.background_executor()
            .spawn(async move {
                _ = sender.broadcast(dispatcher::Message::LaunchApp {
                    app_id: possible_app_id,
                    exec,
                }).await;
            })
            .detach();
    }

    fn calculate_scroll_bounds(
        &self,
        content_height: Pixels,
        app_drawer_size: Size<Pixels>,
        _: &mut Context<Self>
    ) -> (Pixels, Pixels) {
        let container_height = if self.is_searching {
            app_drawer_size.height - px(SEARCH_BAR_HEIGHT + SEARCH_BAR_BOTTOM + 32.0)
        } else {
            app_drawer_size.height - px(16.0) // Fixed: was adding, should subtract
        };

        if content_height <= container_height {
            return (px(0.0), px(0.0));
        }

        // Fixed: max_scroll should be 0 (top), min_scroll should be negative (bottom)
        let max_scroll = px(0.0);
        let min_scroll = -(content_height - container_height);

        (min_scroll, max_scroll)
    }

    // 2. Fix estimate_content_height - more accurate calculations
    fn estimate_content_height(&self) -> Pixels {
        if self.is_searching {
            let num_apps = self.filtered.len() as f32;
            if num_apps == 0.0 {
                return px(100.0); // Return minimum height for empty state
            }
            // Each app row is APP_ROW_HEIGHT (60px) + 1px gap
            let height = px(APP_ROW_HEIGHT * num_apps + (num_apps - 1.0) + 32.0);
            height
        } else {
            let count = self.grouped.len() as f32;
            if count == 0.0 {
                return px(100.0); // Return minimum height for empty state
            }
            // Each category is GRID_ROW_HEIGHT + SECTION_SPACING between them
            let rows_height = GRID_ROW_HEIGHT * count;
            let gaps_height = if count > 1.0 { SECTION_SPACING * (count - 1.0) } else { 0.0 };
            let height = px(rows_height + gaps_height + 32.0);
            height
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>
    ) {
        self.drag_start_y = event.position.y;
        self.drag_start_x = event.position.x;
        self.last_scroll_offset = self.scroll_offset;
        self.is_dragging = true;
        self.has_moved = false;
        self.is_vertical_scroll = false;
        self.press_start_time = None;
        self.is_long_press = false;
        // Don't call cx.stop_propagation() - let horizontal swipes pass through to homescreen
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
        self.last_scroll_offset = self.scroll_offset;
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;
            let delta_x = event.position.x - self.drag_start_x;

            let distance_squared =
                delta_y.abs().to_f64() * delta_y.abs().to_f64() +
                delta_x.abs().to_f64() * delta_x.abs().to_f64();

            if distance_squared > ((DRAG_THRESHOLD * DRAG_THRESHOLD) as f64) {
                if !self.has_moved {
                    let abs_delta_y = delta_y.abs().to_f64();
                    let abs_delta_x = delta_x.abs().to_f64();
                    self.is_vertical_scroll = abs_delta_y > abs_delta_x;
                }

                self.has_moved = true;
                self.press_start_time = None;
            }

            if self.is_vertical_scroll {
                let new_scroll_offset = self.last_scroll_offset + delta_y;
                let content_height = self.estimate_content_height();
                let (min_scroll, max_scroll) = self.calculate_scroll_bounds(
                    content_height,
                    gpui::size(
                        window.bounds().size.width,
                        window.bounds().size.height -
                            Settings::global(cx).homescreen.status_bar_size.height
                    ),
                    cx
                );

                // Clamp: min_scroll is negative (bottom limit), max_scroll is 0 (top limit)
                self.scroll_offset = new_scroll_offset.clamp(min_scroll, max_scroll);
                cx.notify();
                cx.stop_propagation();
            }
        }
    }

    fn filter(&mut self, cx: &mut Context<Self>) {
        let query = self.text_input.read(cx).content.clone();
        let query = query.trim().to_lowercase();

        if query != self.last_search_query {
            self.scroll_offset = px(0.0);
            self.last_scroll_offset = px(0.0);
            self.is_dragging = false;
            self.has_moved = false;
            self.is_vertical_scroll = false;
            self.last_search_query = query.clone();
        }

        if query.is_empty() {
            // Reset to full list
            self.filtered = self.all_apps.clone();
            cx.notify();
            return;
        }

        self.filtered = self.all_apps
            .iter()
            .filter(|app| app.name.to_lowercase().contains(&query))
            .cloned()
            .collect();

        cx.notify();
    }

    fn start_modal_animation(&mut self, origin: Point<Pixels>, cx: &mut Context<Self>) {
        self.modal_animation_progress = 0.0;
        self.is_modal_animating = true;
        self.modal_origin_point = origin;

        cx.spawn(async move |this, cx| {
            let start = std::time::Instant::now();
            let duration = std::time::Duration::from_millis(MODAL_ANIMATION_DURATION_MS as u64);

            loop {
                let elapsed = start.elapsed();
                if elapsed >= duration {
                    this.update(cx, |this, cx| {
                        this.modal_animation_progress = 1.0;
                        this.is_modal_animating = false;
                        cx.notify();
                    }).ok();
                    break;
                }

                let progress = elapsed.as_secs_f32() / duration.as_secs_f32();
                let eased = 1.0 - (1.0 - progress).powi(3); // ease out cubic

                this.update(cx, |this, cx| {
                    this.modal_animation_progress = eased;
                    cx.notify();
                }).ok();

                cx
                    .background_executor()
                    .timer(std::time::Duration::from_millis(MODAL_ANIMATION_FRAME_MS)).await;
            }
        }).detach();
    }

    fn render_floating_search_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();
        let icon = Icons::global(cx).app_drawer.search.clone();

        div()
            .id("floating-search-btn")
            .absolute()
            .bottom(px(FLOATING_BTN_BOTTOM))
            .right(px(FLOATING_BTN_RIGHT))
            .w(px(FLOATING_BTN_SIZE))
            .h(px(FLOATING_BTN_SIZE))
            .rounded(px(12.0))
            .bg(colors.accent_400)
            .flex()
            .items_center()
            .justify_center()
            .shadow_lg()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_floating_search_mouse_down))
            .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(Self::on_floating_search_click))
            .child(
                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .size_full()
                            .external_path(SharedString::from(icon.to_string_lossy().to_string()))
                            .text_color(colors.foreground_300)
                    )
            )
    }

    fn on_floating_search_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        cx.stop_propagation();
    }

    fn on_floating_search_click(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>
    ) {
        self.show_bottom_sheet = false;
        self.show_subwindow_modal = false;
        self.subwindow = None;
        self.sheet_app = None;

        self.has_moved = false;
        self.is_dragging = false;
        self.is_vertical_scroll = false;

        self.is_searching = true;
        self.text_input.update(cx, |input, _| {
            input.focus_handle.focus(window);
        });

        // Reset scroll when entering search mode
        self.scroll_offset = px(0.0);
        self.last_scroll_offset = px(0.0);

        cx.stop_propagation();
        cx.notify();
    }

    fn render_search_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_input = self.text_input.clone();
        let colors = Theme::global(cx).colors.clone();
        let icons = Icons::global(cx).app_drawer.clone();

        div()
            .id("main-search")
            .h(px(SEARCH_BAR_HEIGHT))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .bg(colors.accent_300.with_alpha(0.2))
            .child(
                div()
                    .id("search-bar")
                    .flex_1()
                    .h(px(44.0))
                    .bg(colors.background_900)
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(colors.accent_300.with_alpha(0.4))
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(16.0))
                    .gap(px(12.0))
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_mouse_move(|_, _, cx| cx.stop_propagation())
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .size_full()
                                    .external_path(
                                        SharedString::from(
                                            icons.search.to_string_lossy().to_string()
                                        )
                                    )
                                    .text_color(colors.accent_200)
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_start()
                            .child(
                                div()
                                    .w_full()
                                    .text_size(px(16.0))
                                    .text_color(colors.foreground_200)
                                    .child(text_input.clone())
                            )
                    )
            )
            .child(
                div()
                    .cursor_pointer()
                    .id("close-search-btn")
                    .ml_1()
                    .w(px(40.0))
                    .h(px(40.0))
                    .rounded(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_click(
                        cx.listener(|this: &mut AppDrawer, _event, window, cx| {
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

                            this.text_input.read(cx).blur(window);
                            this.is_searching = false;

                            // Reset scroll when exiting search mode
                            this.scroll_offset = px(0.0);
                            this.last_scroll_offset = px(0.0);
                            this.drag_start_y = px(0.0);
                            this.drag_start_x = px(0.0);
                            this.is_dragging = false;
                            this.has_moved = false;
                            this.is_vertical_scroll = false;

                            cx.notify();
                        })
                    )
                    .child(
                        div()
                            .w(px(16.0))
                            .h(px(16.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(img(icons.x).size_full().text_color(colors.foreground_100))
                    )
            )
    }

    fn render_category_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let grouped = &self.grouped;
        let colors = Theme::global(cx).colors.clone();

        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .px_4()
                    .py_2() // Add top padding
                    .gap(px(SECTION_SPACING))
                    .relative()
                    .top(self.scroll_offset)
                    .children(
                        grouped
                            .into_iter()
                            .enumerate()
                            .map(|(idx, (category, apps))| {
                                let total = apps.len();
                                let show_popup = total > 1;
                                let shown_apps = if show_popup {
                                    apps.iter().take(6).cloned().collect::<Vec<_>>()
                                } else {
                                    apps.clone()
                                };

                                let category_for_popup = category.clone();

                                div()
                                    .id(idx)
                                    .relative()
                                    .w_full()
                                    .h(px(GRID_ROW_HEIGHT))
                                    .cursor_pointer()
                                    .on_click(
                                        cx.listener(
                                            move |this: &mut AppDrawer, _event, window, cx| {
                                                if !this.has_moved && !this.is_long_press {
                                                    let app_drawer_size = gpui::size(
                                                        window.bounds().size.width,
                                                        window.bounds().size.height -
                                                            Settings::global(
                                                                cx
                                                            ).homescreen.status_bar_size.height
                                                    );

                                                    // Calculate card position
                                                    let card_x = px(16.0);
                                                    let card_y =
                                                        px(8.0) +
                                                        this.scroll_offset +
                                                        px(
                                                            (idx as f32) *
                                                                (GRID_ROW_HEIGHT + SECTION_SPACING)
                                                        );
                                                    let card_bottom = card_y + px(GRID_ROW_HEIGHT);

                                                    // Clamp Y position to viewport
                                                    let viewport_top = px(8.0);
                                                    let viewport_bottom = app_drawer_size.height;

                                                    let clamped_y = if card_bottom < viewport_top {
                                                        viewport_top
                                                    } else if card_y > viewport_bottom {
                                                        viewport_bottom
                                                    } else {
                                                        card_bottom
                                                    };

                                                    // Origin point: bottom-left of the card
                                                    let origin = Point::new(card_x, clamped_y);

                                                    this.show_subwindow_modal = true;
                                                    this.subwindow_category =
                                                        category_for_popup.clone();
                                                    this.subwindow = None;

                                                    this.start_modal_animation(origin, cx);
                                                    cx.notify();
                                                }

                                                this.has_moved = false;
                                                this.is_long_press = false;
                                            }
                                        )
                                    )
                                    .child(
                                        div()
                                            .grid()
                                            .grid_cols(6)
                                            .gap(px(28.0))
                                            .bg(colors.background_900)
                                            .p(px(20.0))
                                            .rounded(px(8.0))
                                            .border(px(1.0))
                                            .border_color(colors.background_700)
                                            .w_full()
                                            .h(px(GRID_ROW_HEIGHT))
                                            .justify_center()
                                            .children(
                                                shown_apps
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(idx, app)| {
                                                        let app_id = app.possible_app_id.clone();
                                                        let exec = app.exec.clone();
                                                        let id = hash_id(&app_id);
                                                        let icon = Self::resolved_icon(
                                                            &app.icon_path,
                                                            cx
                                                        );
                                                        let app_for_sheet = app.clone();

                                                        div()
                                                            .id(id + idx)
                                                            .bg(colors.background_800)
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .rounded(px(5.6))
                                                            .size(px(56.0))
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                cx.listener(
                                                                    move |
                                                                        this: &mut AppDrawer,
                                                                        _event,
                                                                        _window,
                                                                        cx
                                                                    | {
                                                                        this.press_start_time =
                                                                            Some(Instant::now());
                                                                        this.press_app_info = Some(
                                                                            app_for_sheet.clone()
                                                                        );
                                                                        this.is_long_press = false;

                                                                        cx.spawn(
                                                                            async move |this, cx| {
                                                                                cx
                                                                                    .background_executor()
                                                                                    .timer(
                                                                                        LONG_PRESS_DURATION
                                                                                    ).await;

                                                                                this.update(
                                                                                    cx,
                                                                                    |this, cx| {
                                                                                        if
                                                                                            this.press_start_time.is_some() &&
                                                                                            !this.has_moved &&
                                                                                            !this.is_long_press
                                                                                        {
                                                                                            this.is_long_press = true;
                                                                                            this.show_bottom_sheet = true;
                                                                                            this.sheet_app =
                                                                                                this.press_app_info.clone();
                                                                                            this.sheet_kind =
                                                                                                BottomSheetKind::MainOptions;
                                                                                            this.press_start_time =
                                                                                                None;
                                                                                            cx.notify();
                                                                                        }
                                                                                    }
                                                                                ).ok();
                                                                            }
                                                                        ).detach();

                                                                        cx.stop_propagation();
                                                                    }
                                                                )
                                                            )
                                                            .on_mouse_up(
                                                                MouseButton::Left,
                                                                cx.listener(
                                                                    move |
                                                                        this: &mut AppDrawer,
                                                                        _event,
                                                                        _window,
                                                                        cx
                                                                    | {
                                                                        if
                                                                            let Some(start_time) =
                                                                                this.press_start_time
                                                                        {
                                                                            let duration =
                                                                                start_time.elapsed();

                                                                            if
                                                                                duration >=
                                                                                    LONG_PRESS_DURATION &&
                                                                                !this.has_moved
                                                                            {
                                                                                this.is_long_press = true;
                                                                                this.show_bottom_sheet = true;
                                                                                this.sheet_app =
                                                                                    this.press_app_info.clone();
                                                                                this.sheet_kind =
                                                                                    BottomSheetKind::MainOptions;
                                                                                cx.notify();
                                                                            } else if
                                                                                !this.has_moved &&
                                                                                !this.is_long_press
                                                                            {
                                                                                this.on_app_click(
                                                                                    app_id.clone(),
                                                                                    exec.clone(),
                                                                                    cx
                                                                                );
                                                                            }
                                                                        }

                                                                        this.press_start_time =
                                                                            None;
                                                                        this.press_app_info = None;
                                                                        this.has_moved = false;
                                                                        cx.stop_propagation();
                                                                    }
                                                                )
                                                            )
                                                            .child(
                                                                div()
                                                                    .size(px(41.0))
                                                                    .border(px(1.0))
                                                                    .items_center()
                                                                    .justify_center()
                                                                    .child(icon)
                                                            )
                                                    })
                                            )
                                    )
                                    .child({
                                        let mut w = wing()
                                            .absolute()
                                            .bottom_0()
                                            .left_0()
                                            .h(px(36.0))
                                            .w_full()
                                            .flex()
                                            .flex_col()
                                            .justify_start()
                                            .items_start()
                                            .bg(colors.accent_200.with_alpha(0.1))
                                            .border_color(colors.accent_200.with_alpha(0.6));
                                        w.upper_wing_size(Size::new(px(150.0), px(15.0)));
                                        w.border_width(px(1.0));
                                        w.border_radius(px(8.0));
                                        w.corner_radii(commons::widgets::CornerRadii {
                                            top_left: px(0.0),
                                            top_right: px(0.0),
                                            bottom_right: px(8.0),
                                            bottom_left: px(8.0),
                                        });
                                        w
                                    })
                                    .child(
                                        div()
                                            .absolute()
                                            .bottom_5()
                                            .left_6()
                                            .font_weight(FontWeight(400.0))
                                            .text_size(px(16.0))
                                            .line_height(px(1.25))
                                            .text_color(colors.foreground_300)
                                            .child(category.clone())
                                            .text_ellipsis()
                                            .w(px(120.0))
                                    )
                            })
                    )
            )
    }
    fn render_search_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        self.is_searching = true;
        Self::filter(self, cx);
        let colors = Theme::global(cx).colors.clone();

        let searched_apps = self.filtered.clone();

        div()
            .id("search-results-list")
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_4()
                    .pt_2() // Add top padding
                    .gap(px(1.0))
                    .relative()
                    .top(self.scroll_offset) // Apply scroll offset here
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                    .children(
                        searched_apps.iter().map(|app| {
                            let exec = app.exec.clone();
                            let app_id = app.possible_app_id.clone();
                            let name = app.name.clone();
                            let id = hash_id(&app_id);
                            let icon = Self::resolved_icon(&app.icon_path, cx);

                            div()
                                .id(id)
                                .h(px(APP_ROW_HEIGHT))
                                .w_full()
                                .flex()
                                .flex_row()
                                .items_center()
                                .cursor_pointer()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .bg(colors.background_800)
                                                .size(px(44.0))
                                                .rounded(px(7.65))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    div()
                                                        .size(px(32.52))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(icon)
                                                )
                                        )
                                        .child(
                                            div()
                                                .font_weight(FontWeight(500.0))
                                                .line_height(px(1.2))
                                                .text_size(px(16.0))
                                                .text_color(colors.foreground_600)
                                                .child(name)
                                        )
                                )
                                .on_click(
                                    cx.listener(move |this: &mut AppDrawer, _event, _window, cx| {
                                        if !this.has_moved {
                                            this.on_app_click(app_id.clone(), exec.clone(), cx);
                                        }
                                        this.has_moved = false;
                                    })
                                )
                        })
                    )
            )
    }

    pub fn resolved_icon(app_icon: &Option<String>, cx: &mut gpui::App) -> Img {
        let icons = Icons::global(cx).app_drawer.clone();
        match app_icon {
            Some(path) if !path.trim().is_empty() => img(PathBuf::from(path.clone())).size_full(),
            _ => img(icons.default_app).size_full(),
        }
    }

    fn render_subwindow_modal(
        &mut self,
        app_drawer_size: Size<Pixels>,
        cx: &mut Context<Self>
    ) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();

        if self.subwindow.is_none() {
            let category = self.subwindow_category.clone();
            if let Some(apps) = self.grouped.get(&category) {
                let apps = apps.clone();
                self.subwindow = Some(cx.new(|_cx| SubWindow::new(category.clone(), apps)));
            }
        }

        let subwindow_entity = self.subwindow.clone().unwrap();
        let progress = self.modal_animation_progress;
        let origin = self.modal_origin_point;

        let final_width = app_drawer_size.width;
        let final_height_f32 = 500.0;

        let target_x = (app_drawer_size.width - final_width) / 2.0;
        let target_y = px(8.0);

        let current_width = px(
            MIN_MODAL_SIZE.0 + ((final_width.to_f64() as f32) - MIN_MODAL_SIZE.0) * progress
        );
        let current_height = px(
            MIN_MODAL_SIZE.1 + (final_height_f32 - MIN_MODAL_SIZE.1) * progress
        );

        let current_x = origin.x + (target_x - origin.x) * progress;
        let current_y =
            origin.y - current_height + (target_y - origin.y + current_height) * progress;

        let opacity = progress;

        div()
            .id("subwindow-modal-overlay")
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .size_full()
            .child(
                div()
                    .id("subwindow-modal-bg")
                    .absolute()
                    .size_full()
                    .bg(colors.background_900)
                    .opacity(0.6 * opacity)
                    .on_click(
                        cx.listener(|this: &mut AppDrawer, _, _, cx| {
                            this.show_subwindow_modal = false;
                            this.subwindow = None;
                            this.modal_animation_progress = 0.0;
                            this.is_modal_animating = false;
                            cx.notify();
                        })
                    )
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            )
            .child(
                div()
                    .absolute()
                    .left(current_x)
                    .top(current_y)
                    .w(current_width)
                    .h(current_height)
                    .overflow_hidden()
                    .opacity(opacity)
                    .child(subwindow_entity)
            )
    }

    fn render_loading(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(colors.foreground_400)
                    .child("Loading applications...")
            )
    }
}

impl Render for AppDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_drawer_size = gpui::size(
            window.bounds().size.width,
            window.bounds().size.height - Settings::global(cx).homescreen.status_bar_size.height
        );

        let colors = Theme::global(cx).colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();

        let text_input = self.text_input.clone();
        text_input.update(cx, |input, _| {
            input.placeholder = "Search".into();
        });

        let is_active = text_input.read(cx).focus_handle.is_focused(window);

        if is_active && !self.is_searching {
            self.scroll_offset = px(0.0);
            self.last_scroll_offset = px(0.0);
            self.drag_start_y = px(0.0);
            self.drag_start_x = px(0.0);
        }

        // Show loading state
        if self.is_loading {
            return div()
                .pt_2()
                .h(app_drawer_size.height)
                .flex()
                .justify_start()
                .relative()
                .items_center()
                .flex_col()
                .overflow_hidden()
                .child(self.render_loading(cx));
        }

        div()
            .size_full()
            .child(
                div()
                    .flex()
                    .relative()
                    .flex_col()
                    .size_full()
                    .font_family(primary_font)
                    // GRID MODE
                    .when(!self.is_searching, |main_page_div| {
                        main_page_div
                            .flex()
                            .flex_col()
                            .size_full()
                            .overflow_hidden()
                            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                            .on_mouse_move(cx.listener(Self::on_mouse_move))
                            .child(self.render_category_list(cx))
                    })
                    // SEARCH MODE
                    .when(self.is_searching, |search_div| {
                        search_div
                            .flex()
                            .flex_col()
                            .size_full()
                            .child(
                                div()
                                    .flex_1()
                                    // .w_full()
                                    .size_full()
                                    .overflow_hidden()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(Self::on_mouse_down)
                                    )
                                    // .pb(px(SEARCH_BAR_HEIGHT))
                                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                                    .child(self.render_search_list(cx))
                            )
                            // SEARCH BAR
                            .child(self.render_search_bar(cx))
                    })
                    // SUBWINDOW MODAL
                    .when(self.show_subwindow_modal, |modal_div| {
                        modal_div.child(
                            self.render_subwindow_modal(
                                gpui::size(
                                    window.bounds().size.width - px(8.0),
                                    window.bounds().size.height -
                                        Settings::global(cx).homescreen.status_bar_size.height
                                ),
                                cx
                            )
                        )
                    })
                    // BOTTOM SHEET
                    .when(self.show_bottom_sheet, |menu_div| {
                        let bottom_sheet_height = match self.sheet_kind {
                            BottomSheetKind::MainOptions => px(280.0),
                            BottomSheetKind::ConfirmDelete => px(239.0),
                            BottomSheetKind::Properties => px(320.0),
                            BottomSheetKind::None => px(0.0),
                        };

                        menu_div
                            .child(
                                div()
                                    .id("bottom-sheet-bg")
                                    .absolute()
                                    .top(px(0.0))
                                    .left(px(0.0))
                                    .size_full()
                                    .bg(colors.background_1000)
                                    .opacity(0.6)
                                    .on_click(
                                        cx.listener(|this: &mut AppDrawer, _, _, cx| {
                                            this.show_bottom_sheet = false;
                                            cx.notify();
                                        })
                                    )
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .on_mouse_up(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                            )
                            .child({
                                let mut w = wing()
                                    .absolute()
                                    .bottom(px(-10.0))
                                    .bg(colors.background_800)
                                    .h(bottom_sheet_height)
                                    .w_full();
                                w.upper_wing_size(Size::new(px(80.0), px(15.0)));
                                w.upper_wing_side(WingSide::Right);
                                w.border_width(px(1.0));
                                w.border_radius(px(8.0));
                                w
                            })
                            .child(
                                div()
                                    .id("bottom-sheet-panel")
                                    .absolute()
                                    .bottom(px(0.0))
                                    .left(px(0.0))
                                    .w(app_drawer_size.width)
                                    .rounded_t(px(24.0))
                                    .pl(px(20.0))
                                    .mt(px(20.0))
                                    .pb(px(20.0))
                                    .pr(px(20.0))
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .on_mouse_up(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .on_click(|_, _, cx| cx.stop_propagation())
                                    .child(match self.sheet_kind {
                                        BottomSheetKind::MainOptions => self.render_main_sheet(cx),
                                        BottomSheetKind::ConfirmDelete => {
                                            self.render_delete_sheet(cx)
                                        }
                                        BottomSheetKind::Properties => {
                                            self.render_properties_sheet(cx)
                                        }
                                        BottomSheetKind::None => Empty.into_any(),
                                    })
                            )
                    })
                    // FLOATING SEARCH BUTTON
                    .when(
                        !self.is_searching && !self.show_bottom_sheet && !self.show_subwindow_modal,
                        |div| div.child(self.render_floating_search_button(cx))
                    )
            )
    }
}

fn hash_id(s: &str) -> usize {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() as usize
}
