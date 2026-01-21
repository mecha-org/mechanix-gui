use std::path::PathBuf;
use gpui::{prelude::FluentBuilder, *};
use shell_state::ShellState;
mod wallpaper;
mod wedges;
use theme::ActiveTheme;
use wallpaper::wallpaper;
use wedges::{left_wedge, right_wedge, LockState};
use theme::prelude::{AlphaExt, Fonts};


// Threshold: if user swipes up more than this many pixels, hide the lockscreen
const UNLOCK_THRESHOLD: f32 = 100.0;

// The wedges area height
const WEDGES_AREA_HEIGHT: f32 = 67.0;

const WEDGE_TARGET_GAP: f32 = 110.0;

// Slider travel after which lock + bell icons fully fade
const LOCK_ICONS_FADE_THRESHOLD: f32 = 310.0;

// Unlock prompt styling
const UNLOCK_PROMPT_RADIUS: f32 = 12.0;
const UNLOCK_PROMPT_ARROW_SIZE: f32 = 40.0;
const UNLOCK_PROMPT_ICON_PATH: &str = "icons/lockscreen/arrow.svg";
const UNLOCK_PROMPT_SIZE_FACTOR: f32 = 0.9;
const UNLOCK_PROMPT_BOTTOM_OFFSET: f32 = 45.0;

// Gap between the slider panel and the wedges (adjust to control spacing)
const PANEL_WEDGE_GAP: f32 = 8.0;

// Fractions of the slider's vertical travel applied in the opposite direction to each wedge
const LEFT_WEDGE_PARALLAX_FRACTION: f32 = 0.20;
const RIGHT_WEDGE_PARALLAX_FRACTION: f32 = 0.25;

// The current wedge SVGs extend beyond their viewBox heights (left path to ~118px, right to ~74px),
const LEFT_WEDGE_BOTTOM_LIMIT: f32 = 73.0;
const RIGHT_WEDGE_BOTTOM_LIMIT: f32 = 38.0;

// How quickly icons fade relative to slider movement (left fades slower than right)
const LEFT_WEDGE_ICON_FADE_STRENGTH: f32 = 0.1;
const RIGHT_WEDGE_ICON_FADE_STRENGTH: f32 = 0.8;

pub struct Lockscreen {
    drag_offset: Option<f32>,
    drag_start_mouse_y: f32,
    // position_y: 0 means panel at bottom (fully visible), negative means moved up (hidden)
    position_y: f32,
    window_height: f32,
    pub show: bool,
    show_arrow_prompt: bool,
    pub wallpaper_path: Option<PathBuf>,
}

impl Lockscreen {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<ShellState>(|_this, cx| {
            cx.notify();
        })
        .detach();

        Self {
            drag_offset: None,
            drag_start_mouse_y: 0.0,
            position_y: 0.0,
            window_height: 0.0,
            show: false,
            show_arrow_prompt: false,
            wallpaper_path: None,
        }
    }

    fn update_input_regions(&self, window: &mut Window, show: bool, cx: &mut Context<Self>) {
        let size = window.bounds().size;
        let regions = if show {
            vec![Bounds {
                origin: point(px(0.0), px(0.0)),
                size,
            }]
        } else {
            vec![]
        };

        window.set_input_regions(Some(regions));
        cx.notify();
    }

    fn handle_unlock(&mut self, cx: &mut Context<Self>) {
        self.position_y = -self.window_height;
        self.show = false;
        cx.notify();
    }

    fn snap_back(&mut self, cx: &mut Context<Self>) {
        self.position_y = 0.0;
        cx.notify();
    }

    /// Reset the lockscreen to initial locked state
    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.drag_offset = None;
        self.drag_start_mouse_y = 0.0;
        self.position_y = 0.0;
        cx.notify();
    }
}

impl Render for Lockscreen {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();
        
        let size = window.bounds().size;
        let window_height = f32::from(size.height);
        let show = self.show;

        if self.window_height == 0.0 {
            self.window_height = window_height;
        }

        self.update_input_regions(window, show, cx);

        let overlay_color = hsla(0.0, 0.0, 0.0, 0.75);
        let text_color = colors.accent_200;
        let unlock_prompt_bg_color = colors.background_1000;
        let show_arrow_prompt = self.show_arrow_prompt;

        // Panel top position: starts at 0 (top of screen), moves up (negative) when swiped
        let panel_top = self.position_y.min(0.0);
        // Panel height excludes the wedges area and gap at the bottom
        let panel_height = window_height - WEDGES_AREA_HEIGHT - PANEL_WEDGE_GAP;
        // Move each wedge downward proportionally to the upward panel motion, clamped to limit
        let left_wedge_offset =
            ((-panel_top) * LEFT_WEDGE_PARALLAX_FRACTION).min(LEFT_WEDGE_BOTTOM_LIMIT);
        let right_wedge_offset =
            ((-panel_top) * RIGHT_WEDGE_PARALLAX_FRACTION).min(RIGHT_WEDGE_BOTTOM_LIMIT);
        // Move wedges horizontally outward using same fractions, clamped to half the target gap
        let left_wedge_gap =
            ((-panel_top) * LEFT_WEDGE_PARALLAX_FRACTION).min(WEDGE_TARGET_GAP * 0.5);
        let right_wedge_gap =
            ((-panel_top) * RIGHT_WEDGE_PARALLAX_FRACTION).min(WEDGE_TARGET_GAP * 0.5);
        // Fade icons as slider moves; clamp to [0, 1]
        let fade_progress = ((-panel_top) / UNLOCK_THRESHOLD).max(0.0).min(1.0);
        let left_icon_opacity = 1.0 - fade_progress * LEFT_WEDGE_ICON_FADE_STRENGTH;
        let right_icon_opacity = 1.0 - fade_progress * RIGHT_WEDGE_ICON_FADE_STRENGTH;
        // Additional fade for lock + bell icons so they vanish at the fade threshold
        let lock_icon_fade = 1.0 - ((-panel_top) / LOCK_ICONS_FADE_THRESHOLD).max(0.0).min(1.0);
        let wallpaper_path = self.wallpaper_path.clone();

        div().size_full().when(show, |this| {
            this.bg(overlay_color)
                // Slider panel - stops above the wedges
                .child(
                    div()
                        .absolute()
                        .top(px(panel_top))
                        .left_0()
                        .w(size.width)
                        .h(px(panel_height))
                        .overflow_hidden()
                        // Wallpaper background
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .child(wallpaper(size.width, px(panel_height), wallpaper_path)),
                        )
                        // Content overlay
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_end()
                                .pb(px(UNLOCK_PROMPT_BOTTOM_OFFSET))
                                .gap_4()
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .bg(unlock_prompt_bg_color)
                                        .rounded(px(UNLOCK_PROMPT_RADIUS * UNLOCK_PROMPT_SIZE_FACTOR))
                                        .px(px(14.0 * UNLOCK_PROMPT_SIZE_FACTOR))
                                        .py(px(10.0 * UNLOCK_PROMPT_SIZE_FACTOR))
                                        .flex()
                                        .opacity(0.8)
                                        .items_center()
                                        .justify_center()
                                        .gap(px(8.0 * UNLOCK_PROMPT_SIZE_FACTOR))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|component, _event: &MouseDownEvent, _, cx| {
                                                component.show_arrow_prompt = !component.show_arrow_prompt;
                                                cx.notify();
                                            }),
                                        )
                                        .child({
                                            let mut inner = div();
                                            if show_arrow_prompt {
                                                inner = inner.child(
                                                    svg()
                                                        .path(UNLOCK_PROMPT_ICON_PATH)
                                                        .w(px(UNLOCK_PROMPT_ARROW_SIZE * UNLOCK_PROMPT_SIZE_FACTOR))
                                                        .h(px(UNLOCK_PROMPT_ARROW_SIZE * UNLOCK_PROMPT_SIZE_FACTOR))
                                                        .text_color(text_color),
                                                );
                                            } else {
                                                inner = inner
                                                    .text_size(px(18.0 * UNLOCK_PROMPT_SIZE_FACTOR))
                                                    .text_color(text_color)
                                                       .font_family(primary_font)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .opacity(1.0)
                                                        .child("Swipe up to unlock");
                                            }
                                            inner
                                        }),
                                ),
                        )
                        .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                            if let Some(offset) = this.drag_offset {
                                let new_y = event.position.y.to_f64() as f32 - offset;
                                // Only allow dragging upward (negative values)
                                this.position_y = new_y.min(0.0);
                                cx.notify();
                            }
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                cx.stop_propagation();
                                this.drag_start_mouse_y = event.position.y.to_f64() as f32;
                                this.drag_offset =
                                    Some(event.position.y.to_f64() as f32 - this.position_y);
                                cx.notify();
                            }),
                        )
                        .on_mouse_up(
                            MouseButton::Left,
                            cx.listener(move |this, event: &MouseUpEvent, _, cx| {
                                if this.drag_offset.is_some() {
                                    this.drag_offset = None;

                                    let current_mouse_y = event.position.y.to_f64() as f32;
                                    let mouse_delta = current_mouse_y - this.drag_start_mouse_y;

                                    // If swiped up enough (negative delta), unlock
                                    if mouse_delta <= -UNLOCK_THRESHOLD {
                                        this.handle_unlock(cx);
                                    } else {
                                        this.snap_back(cx);
                                    }
                                }
                            }),
                        ),
                )
                // Wedges container - fixed at bottom, both wedges overlap
                .child({
                    let lock_state = LockState::from_position(self.position_y, UNLOCK_THRESHOLD);
                    // Left wedge icons should remain visible even when unlocked; keep clamped only.
                    let left_icon_opacity = (left_icon_opacity.max(0.0) * lock_icon_fade).max(0.0);
                    let right_icon_opacity = if lock_state == LockState::FullyOpen {
                        0.0
                    } else {
                        right_icon_opacity.max(0.0)
                    };
                    div()
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .w(size.width)
                        .h(px(WEDGES_AREA_HEIGHT))
                        // Left wedge (below right wedge in z-order, with bell and lock icons)
                        .child(
                            div()
                                .absolute()
                                .bottom(px(-left_wedge_offset))
                                .left(px(-left_wedge_gap))
                                .child(left_wedge(&colors, lock_state, left_icon_opacity)),
                        )
                        // Right wedge (overlaps left wedge, rendered on top, with status icons)
                        .child(
                            div()
                                .absolute()
                                .bottom(px(-right_wedge_offset))
                                .right(px(-right_wedge_gap))
                                .child(right_wedge(cx, &colors, right_icon_opacity)),
                        )
                })
        })
    }
}
