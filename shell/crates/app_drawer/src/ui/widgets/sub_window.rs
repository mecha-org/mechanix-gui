use commons::widgets::wing;
use dispatcher::Dispatcher;
use gpui::*;
use icons::prelude::Icons;
use mxsearch::prelude::AppInfo;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use theme::prelude::{AlphaExt, Theme};

const POPUP_BASE_TOP: f32 = 10.0;
const CARD_WIDTH: f32 = 508.0;
const CARD_MAX_HEIGHT: f32 = 450.0;
const CARD_PADDING: f32 = 20.0;
const ICON_BOX_SIZE: f32 = 88.0;
const ICON_SIZE: f32 = 60.0;
const GRID_GAP: f32 = 20.0;
const APPS_PER_ROW: u16 = 4;
const DRAG_THRESHOLD: f32 = 2.0;

pub struct SubWindow {
    pub apps: Vec<AppInfo>,
    pub category: String,
    pub scroll_offset: Pixels,
    pub last_scroll_offset: Pixels,
    pub drag_start_y: Pixels,
    pub drag_start_x: Pixels,
    pub is_dragging: bool,
    pub has_moved: bool,
    pub is_vertical_scroll: bool,
}

impl SubWindow {
    pub fn new(category: String, apps: Vec<AppInfo>) -> Self {
        Self {
            apps,
            category,
            scroll_offset: px(0.0),
            last_scroll_offset: px(0.0),
            drag_start_y: px(0.0),
            drag_start_x: px(0.0),
            is_dragging: false,
            has_moved: false,
            is_vertical_scroll: false,
        }
    }

    pub fn calculate_scroll_bounds(&self, content_height: Pixels) -> (Pixels, Pixels) {
        let container_height = px(CARD_MAX_HEIGHT - CARD_PADDING * 2.0 - 36.0);
        let max_scroll = px(0.0);
        let min_scroll = container_height - content_height;

        if content_height <= container_height {
            (px(0.0), px(0.0))
        } else {
            (min_scroll, max_scroll)
        }
    }

    pub fn resolved_icon(app_icon: &Option<String>, cx: &mut gpui::App) -> Img {
        let icons = Icons::global(cx).app_drawer.clone();
        match app_icon {
            Some(path) if !path.trim().is_empty() => img(PathBuf::from(path.clone())).size_full(),
            _ => img(icons.default_app).size_full(),
        }
    }

    pub fn estimate_content_height(&self) -> Pixels {
        let rows = ((self.apps.len() as f32) / (APPS_PER_ROW as f32)).ceil();

        if rows == 0.0 {
            return px(0.0);
        }

        px(ICON_BOX_SIZE * rows + GRID_GAP * (rows - 1.0))
    }

    pub fn calculate_card_height(&self) -> f32 {
        let content_height = self.estimate_content_height();
        let needed_height = (content_height + px(CARD_PADDING * 2.0 + 36.0)).to_f64() as f32;
        needed_height.min(CARD_MAX_HEIGHT)
    }

    pub fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.drag_start_y = event.position.y;
        self.drag_start_x = event.position.x;
        self.last_scroll_offset = self.scroll_offset;
        self.is_dragging = true;
        self.has_moved = false;
        self.is_vertical_scroll = false;
        // Don't call cx.stop_propagation() - let horizontal swipes pass through to homescreen
    }

    pub fn on_mouse_up(&mut self, _event: &MouseUpEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
    }

    pub fn on_app_click(&self, possible_app_id: String, exec: String, cx: &mut Context<Self>) {
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

    pub fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;
            let delta_x = event.position.x - self.drag_start_x;

            let distance = (delta_y.abs().to_f64() * delta_y.abs().to_f64()
                + delta_x.abs().to_f64() * delta_x.abs().to_f64())
                as f32;

            // Determine scroll direction once threshold is exceeded
            if distance > DRAG_THRESHOLD {
                // Determine if this is a vertical scroll or horizontal swipe
                if !self.has_moved {
                    let abs_delta_y = delta_y.abs().to_f64();
                    let abs_delta_x = delta_x.abs().to_f64();

                    // If movement is primarily vertical, treat as scroll
                    // Otherwise, let it pass through to homescreen for page swipe
                    self.is_vertical_scroll = abs_delta_y > abs_delta_x;
                }

                self.has_moved = true;
            }

            // Only scroll vertically if this is a vertical scroll gesture
            if self.is_vertical_scroll {
                let content_height = self.estimate_content_height();
                let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);
                self.scroll_offset =
                    (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);
                cx.notify();

                // Stop propagation only for vertical scrolls
                cx.stop_propagation();
            }
            // For horizontal swipes, don't stop propagation - let homescreen handle it
        }
    }

    /// Render the modal overlay with the SubWindow card
    pub fn render_modal(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.render_card(cx))
    }

    /// Render the SubWindow card content
    pub fn render_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();
        let card_height = self.calculate_card_height();
        let category = self.category.clone();

        div()
            .id("subwindow-modal-card")
            .relative()
            .top(px(POPUP_BASE_TOP))
            .mx_auto()
            .w(px(CARD_WIDTH))
            .h(px(card_height))
            .min_h(px(420.0))
            .bg(colors.background_900)
            .border(px(1.0))
            .border_color(colors.background_700)
            .rounded(px(12.0))
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .id("modal_card_content")
                    .size_full()
                    .p(px(CARD_PADDING))
                    .pb(px(CARD_PADDING + 36.0))
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
                    .child(self.render_grid(cx)),
            )
            .child(self.render_bottom_wing(cx))
            .child(
                div()
                    .absolute()
                    .bottom_5()
                    .left_6()
                    .font_weight(FontWeight(400.0))
                    .text_size(px(16.0))
                    .line_height(px(1.25))
                    .text_color(colors.foreground_300)
                    .child(category)
                    .text_ellipsis()
                    .w(px(120.0)),
            )
    }
    /// Render the grid of appss
    pub fn render_grid(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();
        div()
            .grid()
            .grid_cols(APPS_PER_ROW)
            .gap(px(GRID_GAP))
            .top(self.scroll_offset)
            .children(self.apps.iter().enumerate().map(|(idx, app)| {
                let app_id = app.possible_app_id.clone();
                let id = hash_id(&app_id);
                let exec = app.exec.clone();
                let icon = Self::resolved_icon(&app.icon_path, cx);

                div()
                    .id(id + idx)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .id(idx)
                            .bg(colors.background_800)
                            .size(px(ICON_BOX_SIZE))
                            .rounded(px(15.3))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .on_click(cx.listener(
                                move |this: &mut SubWindow, _event, _window, cx| {
                                    if !this.has_moved {
                                        this.on_app_click(app_id.clone(), exec.clone(), cx);
                                    }
                                    this.has_moved = false;
                                },
                            ))
                            .child(
                                div()
                                    .size(px(ICON_SIZE))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(icon),
                            ),
                    )
            }))
    }

    /// Render the bottom wing decoration
    fn render_bottom_wing(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();

        let mut w = wing()
            .absolute()
            .bottom_0()
            .left_0()
            .h(px(36.0))
            .w(px(CARD_WIDTH))
            // .bg(colors.accent_200.with_alpha(0.1))
            .border_color(colors.background_700);

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
    }
}

impl Render for SubWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_modal(cx)
    }
}

fn hash_id(s: &str) -> usize {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() as usize
}
