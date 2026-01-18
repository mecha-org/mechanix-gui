mod components;
use std::time::Duration;

use crate::prelude::models::*;
use commons::prelude::*;
use dispatcher::Dispatcher;
use gpui::foreign_toplevel_management::ForeignToplevelHandle;
use gpui::prelude::*;
use gpui::*;
use settings::prelude::{InputRegions, Settings};
use theme::prelude::*;

const BAR_SIZE: (f32, f32) = (80.0, 29.0);
const APP_SIZE: (f32, f32) = (540.0, 620.0);

impl Render for RunningApps {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let top_levels = window.foreign_toplevels();
        if self.apps.len() != top_levels.len() && !self.is_animating() {
            self.update_running_apps(top_levels, window, cx);
            cx.notify();
        }

        let input_regions = Settings::global(cx).running_apps.input_regions.clone();
        let bar_fixed_pos = APP_SIZE.1 - BAR_SIZE.1;
        let current_bar_y = bar_fixed_pos + self.bar_drag_offset;
        let colors = cx.theme().colors.clone();

        div()
            .w_full()
            .h_full()
            .when(self.show_apps, |this| {
                this.child(
                    div()
                        .w_full()
                        .h_full()
                        .absolute()
                        .top(px(0.))
                        .child(self.running_apps(window, cx)),
                )
            })
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if let Some(start_y) = this.bar_drag_start_y {
                    let current_y = event.position.y.to_f64() as f32;
                    let offset = current_y - start_y;
                    this.bar_drag_offset = offset.min(0.0);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.bar_drag_start_y.is_some() {
                        if this.bar_drag_offset < -80.0 {
                            //long swipe
                            //Show running apps
                            this.update_input_regions(window, !this.show_apps, cx);
                            this.show_apps = !this.show_apps;
                            let initial_offset = -50.0 * this.apps.len() as f32;
                            this.animation_state = AppCardAnimation::Initial {
                                start_time: std::time::Instant::now(),
                                start_offset: initial_offset,
                            };
                            this.animation_start_time = Some(std::time::Instant::now());
                            this.schedule_animation_frame(cx);
                        } else {
                            //short swipe
                            //Mimize all apps
                            this.show_apps = false;
                            this.update_input_regions(window, false, cx);
                            // this.send_minimize_all_apps(cx);
                            let d_sender = Dispatcher::global(cx).clone().0;
                            cx.background_executor()
                                .spawn(async move {
                                    _ = d_sender
                                        .broadcast(dispatcher::Message::MinimizeToHome)
                                        .await;
                                })
                                .detach();
                        }
                        this.bar_drag_start_y = None;
                        this.snap_bar_to(0.0, cx);
                        cx.notify();
                    }
                }),
            )
            .when(true, |this| {
                this.child(
                    deferred(
                        div()
                            .id("input-region")
                            .absolute()
                            .left(input_regions.minimized.origin.x)
                            .top(input_regions.minimized.origin.y)
                            .w(input_regions.minimized.size.width)
                            .h(input_regions.minimized.size.height)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                    cx.stop_propagation();
                                    this.bar_drag_start_y = Some(event.position.y.to_f64() as f32);
                                    cx.notify();
                                }),
                            ),
                    )
                    .priority(1000),
                )
            })
            .child(
                deferred(
                    div()
                        .w_full()
                        .flex()
                        .flex_row()
                        .justify_center()
                        .items_center()
                        .absolute()
                        .top(px(current_bar_y))
                        .h(px(BAR_SIZE.1))
                        .child(div().bg(colors.accent_400).w(px(BAR_SIZE.0)).h(px(4.0))),
                )
                .priority(1000),
            )
    }
}

impl RunningApps {
    pub fn new(installed_apps: Entity<InstalledApps>, cx: &mut Context<Self>) -> Self {
        let apps = Vec::new();
        let _poll_task = cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            loop {
                let executor = cx.background_executor().clone();
                cx.background_spawn(async move {
                    executor.timer(std::time::Duration::from_millis(100)).await;
                })
                .await;

                let _ = this.update(cx, |_this, cx| {
                    cx.notify();
                });
            }
        });

        Self {
            scroll_offset: 0.0,
            drag_start: None,
            horizontal_offset: 0.0,
            animation_state: AppCardAnimation::None,
            animation_start_time: None,
            dragged_card_index: None,
            dragged_parent: false,
            gesture_locked: None,
            apps,
            has_dragged: false,
            bar_drag_offset: 0.0,
            bar_drag_start_y: None,
            show_apps: false,
            installed_apps,
            _poll_task,
        }
    }

    fn is_animating(&self) -> bool {
        !matches!(self.animation_state, AppCardAnimation::None)
    }
}

impl RunningApps {
    fn update_running_apps(
        &mut self,
        top_levels: Vec<ForeignToplevelHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut apps: Vec<(ForeignToplevelHandle, commons::prelude::App)> = Vec::new();
        for top_level in top_levels {
            let Some(app_id) = top_level.app_id() else {
                continue;
            };

            let Some(app) = self.installed_apps.read(cx).find(&app_id.to_string()) else {
                continue;
            };

            apps.push((top_level, app));
        }
        self.apps = apps;
    }

    fn send_minimize_all_apps(&self, cx: &mut Context<Self>) {
        for (top_level, _) in self.apps.clone() {
            top_level.unset_maximized();
            cx.notify();
        }
    }

    fn snap_bar_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.bar_drag_offset;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    // Check if animation is done
                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.bar_drag_offset = target;
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + change * ease;

                    this.update(cx, |this, cx| {
                        this.bar_drag_offset = current;
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

    fn update_input_regions(&self, window: &mut Window, open: bool, cx: &mut Context<Self>) {
        let InputRegions {
            minimized,
            maximized,
        } = Settings::global(cx).running_apps.input_regions.clone();
        let mut regions = Vec::new();

        if open {
            regions.push(Bounds {
                origin: maximized.origin,
                size: maximized.size,
            });
        } else {
            regions.push(Bounds {
                origin: minimized.origin,
                size: minimized.size,
            });
        }
        window.set_input_regions(Some(regions));
        cx.notify();
    }

    fn running_apps(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_apps = !self.apps.is_empty();
        let colors = Theme::global(cx).colors.clone();
        let primary_font = Fonts::global(cx).primary.clone();

        div()
            .flex()
            .w_full()
            .h_full()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(rgb(0x000000))
            .when(has_apps, |this| this.child(self.scroller_container(cx)))
            .when(!has_apps, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_16()
                        .child(
                            div()
                                .text_color(colors.foreground_800)
                                .text_size(px(16.0))
                                .line_height(px(24.0))
                                .text_center()
                                .font_weight(FontWeight(500.0))
                                .font_family(primary_font)
                                .max_w(px(300.0))
                                .child("There are no apps or droids")
                                .child(div().child("you are looking for.")),
                        )
                        .with_animation(
                            "no-apps-animation",
                            Animation::new(Duration::from_secs_f64(0.25))
                                .with_easing(cubic_bezier(0.4, 0., 0.2, 1.)),
                            move |this, delta| this.opacity(delta),
                        ),
                )
            })
    }
}
