use crate::models::{GridPosition, HomescreenState, Page, Widget, WidgetSize};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub mod widgets;
pub use widgets::*;

pub const WINDOW_WIDTH: f32 = 540.0;
pub const WINDOW_HEIGHT: f32 = 540.0;

const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 4;

const GRID_PADDING_PERCENT: f32 = 0.05;
const GAP_PERCENT: f32 = 0.01;
const EDGE_TRIGGER_THRESHOLD: f32 = 50.0;

pub struct Homescreen {
    state: HomescreenState,
    window_size: Size<Pixels>,
}

impl Homescreen {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        let window_size = size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT));
        let mut state = HomescreenState::new();

        let mut page = Page::new(0, GRID_COLS, GRID_ROWS);

        let demo_widgets = vec![
            Widget::new(
                1,
                "Music",
                "🎵",
                WidgetSize::new(2, 2),
                GridPosition::new(0, 0),
            )
            .with_color(rgb(0xFF6B9D).into()),
            Widget::new(
                2,
                "Photos",
                "📷",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 0),
            )
            .with_color(rgb(0x4A90E2).into()),
            Widget::new(
                3,
                "Calendar",
                "📅",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 0),
            )
            .with_color(rgb(0xE74C3C).into()),
            Widget::new(
                4,
                "Weather",
                "🌤️",
                WidgetSize::new(2, 1),
                GridPosition::new(2, 1),
            )
            .with_color(rgb(0x3498DB).into()),
            Widget::new(
                5,
                "Messages",
                "💬",
                WidgetSize::new(1, 1),
                GridPosition::new(0, 2),
            )
            .with_color(rgb(0x2ECC71).into()),
            Widget::new(
                6,
                "Notes",
                "📝",
                WidgetSize::new(1, 2),
                GridPosition::new(1, 2),
            )
            .with_color(rgb(0xF39C12).into()),
            Widget::new(
                7,
                "Clock",
                "🕐",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 2),
            )
            .with_color(rgb(0x9B59B6).into()),
            Widget::new(
                8,
                "Settings",
                "⚙️",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 2),
            )
            .with_color(rgb(0x34495E).into()),
            Widget::new(
                9,
                "Maps",
                "🗺️",
                WidgetSize::new(1, 1),
                GridPosition::new(0, 3),
            )
            .with_color(rgb(0x16A085).into()),
            Widget::new(
                10,
                "Browser",
                "🌐",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 3),
            )
            .with_color(rgb(0xE67E22).into()),
            Widget::new(
                11,
                "Games",
                "🎮",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 3),
            )
            .with_color(rgb(0xC0392B).into()),
        ];

        for widget in demo_widgets {
            if let Err(e) = page.add_widget(widget) {
                eprintln!("Failed to add widget: {}", e);
            }
        }
        state.add_page(page);

        let mut page2 = Page::new(1, GRID_COLS, GRID_ROWS);
        let page2_widgets = vec![
            Widget::new(
                12,
                "Twitter",
                "🐦",
                WidgetSize::new(1, 1),
                GridPosition::new(0, 0),
            )
            .with_color(rgb(0x1DA1F2).into()),
            Widget::new(
                13,
                "Instagram",
                "📸",
                WidgetSize::new(1, 1),
                GridPosition::new(1, 0),
            )
            .with_color(rgb(0xE4405F).into()),
            Widget::new(
                14,
                "WhatsApp",
                "💬",
                WidgetSize::new(2, 1),
                GridPosition::new(2, 0),
            )
            .with_color(rgb(0x25D366).into()),
            Widget::new(
                15,
                "Email",
                "📧",
                WidgetSize::new(2, 2),
                GridPosition::new(0, 1),
            )
            .with_color(rgb(0xD44638).into()),
            Widget::new(
                16,
                "Slack",
                "💼",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 1),
            )
            .with_color(rgb(0x4A154B).into()),
            Widget::new(
                17,
                "Discord",
                "🎮",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 1),
            )
            .with_color(rgb(0x5865F2).into()),
            Widget::new(
                18,
                "Telegram",
                "✈️",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 2),
            )
            .with_color(rgb(0x0088CC).into()),
            Widget::new(
                19,
                "Signal",
                "🔒",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 2),
            )
            .with_color(rgb(0x3A76F0).into()),
            Widget::new(
                20,
                "Phone",
                "📞",
                WidgetSize::new(2, 1),
                GridPosition::new(0, 3),
            )
            .with_color(rgb(0x34C759).into()),
            Widget::new(
                21,
                "Contacts",
                "👥",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 3),
            )
            .with_color(rgb(0xFF9500).into()),
            Widget::new(
                22,
                "FaceTime",
                "📹",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 3),
            )
            .with_color(rgb(0x00D856).into()),
        ];
        for widget in page2_widgets {
            if let Err(e) = page2.add_widget(widget) {
                eprintln!("Failed to add widget: {}", e);
            }
        }
        state.add_page(page2);

        let mut page3 = Page::new(2, GRID_COLS, GRID_ROWS);
        let page3_widgets = vec![
            Widget::new(
                23,
                "Netflix",
                "🎬",
                WidgetSize::new(2, 2),
                GridPosition::new(0, 0),
            )
            .with_color(rgb(0xE50914).into()),
            Widget::new(
                24,
                "Spotify",
                "🎵",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 0),
            )
            .with_color(rgb(0x1DB954).into()),
            Widget::new(
                25,
                "YouTube",
                "📺",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 0),
            )
            .with_color(rgb(0xFF0000).into()),
            Widget::new(
                26,
                "Twitch",
                "🎮",
                WidgetSize::new(2, 1),
                GridPosition::new(2, 1),
            )
            .with_color(rgb(0x9146FF).into()),
            Widget::new(
                27,
                "Apple TV",
                "📺",
                WidgetSize::new(1, 1),
                GridPosition::new(0, 2),
            )
            .with_color(rgb(0x000000).into()),
            Widget::new(
                28,
                "Disney+",
                "🏰",
                WidgetSize::new(1, 2),
                GridPosition::new(1, 2),
            )
            .with_color(rgb(0x113CCF).into()),
            Widget::new(
                29,
                "Podcasts",
                "🎙️",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 2),
            )
            .with_color(rgb(0x8032DC).into()),
            Widget::new(
                30,
                "Books",
                "📚",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 2),
            )
            .with_color(rgb(0xFF9500).into()),
            Widget::new(
                31,
                "Camera",
                "📷",
                WidgetSize::new(1, 1),
                GridPosition::new(0, 3),
            )
            .with_color(rgb(0x8E8E93).into()),
            Widget::new(
                32,
                "Photos",
                "🖼️",
                WidgetSize::new(1, 1),
                GridPosition::new(2, 3),
            )
            .with_color(rgb(0xFF3B30).into()),
            Widget::new(
                33,
                "Gallery",
                "🎨",
                WidgetSize::new(1, 1),
                GridPosition::new(3, 3),
            )
            .with_color(rgb(0x5856D6).into()),
        ];
        for widget in page3_widgets {
            if let Err(e) = page3.add_widget(widget) {
                eprintln!("Failed to add widget: {}", e);
            }
        }
        state.add_page(page3);

        Self { state, window_size }
    }

    fn calculate_grid_padding(&self) -> Pixels {
        self.window_size.width * GRID_PADDING_PERCENT
    }

    fn calculate_gap(&self) -> Pixels {
        self.window_size.width * GAP_PERCENT
    }

    fn calculate_cell_size(&self) -> Pixels {
        let padding = self.calculate_grid_padding();
        let gap = self.calculate_gap();

        let available_width =
            self.window_size.width - (padding * 2.0) - (gap * (GRID_COLS - 1) as f32);
        available_width / GRID_COLS as f32
    }

    fn calculate_grid_width(&self) -> Pixels {
        let cell_size = self.calculate_cell_size();
        let gap = self.calculate_gap();
        cell_size * GRID_COLS as f32 + gap * (GRID_COLS - 1) as f32
    }

    fn calculate_grid_height(&self) -> Pixels {
        let cell_size = self.calculate_cell_size();
        let gap = self.calculate_gap();
        cell_size * GRID_ROWS as f32 + gap * (GRID_ROWS - 1) as f32
    }

    fn calculate_page_offset_x(&self) -> f32 {
        let grid_width: f32 = self.calculate_grid_width().into();
        let window_width: f32 = self.window_size.width.into();
        (window_width - grid_width) / 2.0
    }

    fn render_all_pages(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let page_gap = px(40.0);
        let cell_size = self.calculate_cell_size();
        let gap = self.calculate_gap();
        let grid_width = self.calculate_grid_width();
        let grid_height = self.calculate_grid_height();
        let hovered_id = self.state.hovered_widget;
        let selected_id = self.state.selected_widget;

        let cell_size_f32: f32 = cell_size.into();
        let gap_f32: f32 = gap.into();

        for page in &mut self.state.pages {
            for widget in &mut page.widgets {
                widget.calculate_target_position(cell_size_f32, gap_f32);
                if widget.current_x == 0.0 && widget.current_y == 0.0 {
                    widget.current_x = widget.target_x;
                    widget.current_y = widget.target_y;
                }
            }
        }

        let dragging_widget_id = self.state.dragging_widget;

        let pages_data: Vec<(usize, Vec<_>)> = self
            .state
            .pages
            .iter()
            .enumerate()
            .map(|(idx, page)| (idx, page.widgets.clone()))
            .collect();

        let mut dragged_widget_data: Option<(Widget, usize)> = None;
        if let Some(dragged_id) = dragging_widget_id {
            for (page_idx, widgets) in &pages_data {
                if let Some(widget) = widgets.iter().find(|w| w.id == dragged_id) {
                    dragged_widget_data = Some((widget.clone(), *page_idx));
                    break;
                }
            }
        }

        div()
            .flex()
            .flex_row()
            .gap(page_gap)
            .children(pages_data.into_iter().map(|(page_index, widgets)| {
                div()
                    .id(("page", page_index))
                    .relative()
                    .w(grid_width)
                    .h(grid_height)
                    .flex_shrink_0()
                    .children(widgets.into_iter().filter_map(|widget| {
                        let widget_id = widget.id;

                        if Some(widget_id) == dragging_widget_id {
                            return None;
                        }

                        let is_hovered = hovered_id == Some(widget_id);
                        let is_selected = selected_id == Some(widget_id);
                        let left = px(widget.current_x);
                        let top = px(widget.current_y);

                        Some(
                            div()
                                .id(("widget", widget_id))
                                .absolute()
                                .left(left)
                                .top(top)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(
                                        move |homescreen, event: &MouseDownEvent, _window, cx| {
                                            let x: f32 = event.position.x.into();
                                            let y: f32 = event.position.y.into();
                                            homescreen.state.start_hold(x, y, Some(widget_id));
                                            cx.notify();
                                        },
                                    ),
                                )
                                .child(
                                    WidgetView::new(widget, cell_size, gap)
                                        .hovered(is_hovered)
                                        .selected(is_selected),
                                ),
                        )
                    }))
            }))
            .when_some(dragged_widget_data, |parent_div, (widget, page_idx)| {
                let widget_id = widget.id;
                let is_hovered = hovered_id == Some(widget_id);
                let is_selected = selected_id == Some(widget_id);

                let page_gap_f32: f32 = page_gap.into();
                let grid_width_f32: f32 = grid_width.into();
                let page_offset = page_idx as f32 * (grid_width_f32 + page_gap_f32);

                let left = px(widget.current_x + page_offset);
                let top = px(widget.current_y);

                parent_div.child(
                    div()
                        .id(("widget_dragged", widget_id))
                        .absolute()
                        .left(left)
                        .top(top)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |homescreen, event: &MouseDownEvent, _window, cx| {
                                let x: f32 = event.position.x.into();
                                let y: f32 = event.position.y.into();
                                homescreen.state.start_hold(x, y, Some(widget_id));
                                cx.notify();
                            }),
                        )
                        .child(
                            WidgetView::new(widget, cell_size, gap)
                                .hovered(is_hovered)
                                .selected(is_selected),
                        ),
                )
            })
    }

    fn render_page_indicator(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let total_pages = self.state.pages.len();
        let current = self.state.current_page;

        div()
            .flex()
            .flex_row()
            .gap_2()
            .children((0..total_pages).map(|i| {
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded(px(4.0))
                    .bg(if i == current {
                        rgb(0xFFFFFF)
                    } else {
                        rgba(0xFFFFFF66)
                    })
            }))
    }
}

impl Render for Homescreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let padding = self.calculate_grid_padding();

        if self.state.update_animation() {
            cx.notify();
        }

        let now = std::time::Instant::now();
        let delta_time = if let Some(last_time) = self.state.last_update_time {
            now.duration_since(last_time).as_secs_f32()
        } else {
            0.016
        };
        self.state.last_update_time = Some(now);

        let mut needs_update = false;
        for page in &mut self.state.pages {
            for widget in &mut page.widgets {
                if widget.update_position(delta_time) {
                    needs_update = true;
                }
            }
        }

        if needs_update {
            cx.notify();
        }

        let visual_offset = self.state.get_visual_offset();

        div()
            .id("homescreen-container")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1a1a1a))
            .items_center()
            .justify_center()
            .gap(padding)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|homescreen, event: &MouseDownEvent, _window, cx| {
                    homescreen.state.start_drag(event.position.x);
                    cx.notify();
                }),
            )
            .on_mouse_move(
                cx.listener(|homescreen, event: &MouseMoveEvent, _window, cx| {
                    let x: f32 = event.position.x.into();
                    let y: f32 = event.position.y.into();

                    if homescreen.state.check_hold(x, y) {
                        cx.notify();
                    }

                    if let Some(dragging_widget_id) = homescreen.state.dragging_widget {
                        let cell_size_f32: f32 = homescreen.calculate_cell_size().into();
                        let gap_f32: f32 = homescreen.calculate_gap().into();

                        for page in &mut homescreen.state.pages {
                            if let Some(widget) =
                                page.widgets.iter_mut().find(|w| w.id == dragging_widget_id)
                            {
                                widget.set_drag_target(x, y, cell_size_f32, gap_f32);
                                break;
                            }
                        }

                        let window_width: f32 = homescreen.window_size.width.into();
                        let grid_width: f32 = homescreen.calculate_grid_width().into();
                        if homescreen.state.check_edge_trigger(
                            x,
                            window_width,
                            EDGE_TRIGGER_THRESHOLD,
                            grid_width,
                            40.0,
                        ) {
                            cx.notify();
                        }

                        cx.notify();
                    } else if event.dragging() {
                        homescreen.state.update_drag(event.position.x);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|homescreen, _event: &MouseUpEvent, _window, cx| {
                    if homescreen.state.dragging_widget.is_some() {
                        let dragging_widget_id = homescreen.state.dragging_widget.unwrap();

                        homescreen.state.end_widget_drag();

                        let cell_size_f32: f32 = homescreen.calculate_cell_size().into();
                        let gap_f32: f32 = homescreen.calculate_gap().into();

                        for page in &mut homescreen.state.pages {
                            if let Some(widget) =
                                page.widgets.iter_mut().find(|w| w.id == dragging_widget_id)
                            {
                                widget.calculate_target_position(cell_size_f32, gap_f32);
                                break;
                            }
                        }
                    } else {
                        let grid_width_f32: f32 = homescreen.calculate_grid_width().into();
                        homescreen.state.end_drag(grid_width_f32, 40.0);
                    }
                    homescreen.state.cancel_hold();
                    cx.notify();
                }),
            )
            .child(
                div().relative().w_full().h_full().overflow_hidden().child(
                    div()
                        .absolute()
                        .left(px(visual_offset + self.calculate_page_offset_x()))
                        .child(self.render_all_pages(cx)),
                ),
            )
            .child(self.render_page_indicator(cx))
            .child(
                div()
                    .absolute()
                    .bottom(px(20.0))
                    .left(px(20.0))
                    .text_size(px(12.0))
                    .text_color(rgba(0xFFFFFF88))
                    .child(format!(
                        "Hovered: {:?} | Selected: {:?} | Page: {}/{}",
                        self.state.hovered_widget,
                        self.state.selected_widget,
                        self.state.current_page + 1,
                        self.state.pages.len()
                    )),
            )
    }
}
