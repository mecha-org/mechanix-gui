use crate::ui::utils::prelude::{ DesktopApp, DesktopApps };
use commons::widgets::wing;
use gpui::*;
use theme::prelude::{ AlphaExt, Theme };
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher };

const POPUP_BASE_TOP: f32 = 40.0;
const CARD_WIDTH: f32 = 420.0;
const CARD_MAX_HEIGHT: f32 = 450.0;
const CARD_PADDING: f32 = 20.0;
const ICON_BOX_SIZE: f32 = 88.0;
const ICON_SIZE: f32 = 60.0;
const GRID_GAP: f32 = 20.0;
const APPS_PER_ROW: u16 = 4;

pub struct SubWindow {
    apps: Vec<DesktopApp>,
    pub category: String,

    // scrolling inside card
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
}

impl SubWindow {
    pub fn scan(category: String) -> Self {
        let desktop_apps = DesktopApps::scan();
        let apps = desktop_apps.get_apps_by_category(&category);

        Self {
            apps,
            category,
            scroll_offset: px(0.0),
            last_scroll_offset: px(0.0),
            drag_start_y: px(0.0),
            is_dragging: false,
        }
    }

    fn calculate_scroll_bounds(&self, content_height: Pixels) -> (Pixels, Pixels) {
        // Available space inside card for scrolling
        let container_height = px(CARD_MAX_HEIGHT - CARD_PADDING * 2.0 - 36.0); // subtract padding and wing height
        let max_scroll = px(0.0);
        let min_scroll = container_height - content_height;

        if content_height <= container_height {
            (px(0.0), px(0.0))
        } else {
            (min_scroll, max_scroll)
        }
    }

    fn estimate_content_height(&self) -> Pixels {
        // Calculate number of rows needed (4 apps per row)
        let rows = ((self.apps.len() as f32) / (APPS_PER_ROW as f32)).ceil();

        if rows == 0.0 {
            return px(0.0);
        }

        // Each icon box is 88px, plus gap between rows
        let total = px(ICON_BOX_SIZE * rows + GRID_GAP * (rows - 1.0));
        total
    }

    fn calculate_card_height(&self) -> f32 {
        let content_height = self.estimate_content_height();
        let needed_height = (content_height + px(CARD_PADDING * 2.0 + 36.0)).to_f64() as f32; // padding + wing height
        needed_height.min(CARD_MAX_HEIGHT)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
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
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;
            let content_height = self.estimate_content_height();
            let (min_scroll, max_scroll) = self.calculate_scroll_bounds(content_height);
            self.scroll_offset = (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);
            cx.notify();
        }
    }
}

impl Render for SubWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Theme::global(cx).colors.clone();
        let card_height = self.calculate_card_height();

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
                    .bg(colors.background_900)
                    .opacity(0.6)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _, window, _| window.remove_window())
                    )
            )
            .child(
                // popup card
                div()
                    .id("popup_card")
                    .relative()
                    .top(px(POPUP_BASE_TOP))
                    .mx_auto()
                    .w(px(CARD_WIDTH))
                    .h(px(card_height))
                    .min_h(px(420.))
                    .bg(colors.background_900)
                    .border(px(1.0))
                    .border_color(colors.accent_200.with_alpha(0.6))
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .child(
                        // scrollable content area
                        div()
                            .id("card_content")
                            .size_full()
                            .p(px(CARD_PADDING))
                            .pb(px(CARD_PADDING + 36.0)) // extra padding for wing
                            .overflow_hidden()
                            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                            .on_mouse_move(cx.listener(Self::on_mouse_move))
                            .child(
                                // grid with scroll offset
                                div()
                                    .grid()
                                    .grid_cols(APPS_PER_ROW)
                                    .gap(px(GRID_GAP))
                                    .top(self.scroll_offset)
                                    .children(
                                        self.apps
                                            .iter()
                                            .enumerate()
                                            .map(|(idx, app)| {
                                                let icon = DesktopApp::resolved_icon(
                                                    &app.icon_path
                                                );
                                                let app_id = app.app_id.clone();
                                                let id = hash_id(&app_id);

                                                div()
                                                    .id(id + idx)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .bg(colors.background_800)
                                                            .size(px(ICON_BOX_SIZE))
                                                            .rounded(px(15.3))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .cursor_pointer()
                                                            .child(
                                                                div()
                                                                    .size(px(ICON_SIZE))
                                                                    .flex()
                                                                    .items_center()
                                                                    .justify_center()
                                                                    .child(icon)
                                                            )
                                                    )
                                            })
                                    )
                            )
                    )
                    // Wing element (absolute to card)
                    .child({
                        let mut w = wing()
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .h(px(36.0))
                            .w(px(CARD_WIDTH))
                            .flex()
                            .flex_col()
                            .justify_start()
                            .items_start()
                            .bg(colors.accent_200.with_alpha(0.1))
                            .border_color(colors.accent_200.with_alpha(0.6));
                        w.upper_wing_size(Size::new(px(150.0), px(15.0)));
                        w.border_width(px(1.0));
                        w.border_radius(px(12.0));
                        w
                    })
                    // Category label on wing (absolute to card)
                    .child(
                        div()
                            .absolute()
                            .bottom_5()
                            .left_6()
                            .font_weight(FontWeight(400.0))
                            .text_size(px(16.0))
                            .line_height(px(1.25))
                            .text_color(colors.foreground_300)
                            .child(self.category.clone())
                            .text_ellipsis()
                            .w(px(120.0))
                    )
            )
    }
}

fn hash_id(s: &str) -> usize {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish() as usize
}
