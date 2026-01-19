use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use std::time::Duration;
use settings::prelude::{LayerShellSettings, Settings, VolumeSliderSettings};

use crate::icon::{VolumeIcon, VolumeIconName};
use crate::slider::{Slider, SliderEvent, SliderPattern, SliderState};
use crate::{get_volume, set_volume, sync_volume_to_system};

const OVERLAY_PADDING: f32 = 16.0;
const OVERLAY_GAP: f32 = 12.0;
const OVERLAY_RADIUS: f32 = 15.0;
const OVERLAY_BACKGROUND: u32 = 0x101010;
const OVERLAY_TIMEOUT_MS: u64 = 2_000;
// Vertical slider dimensions (bars area)
const SLIDER_WIDTH: f32 = 20.0;
const SLIDER_HEIGHT: f32 = 233.0;
// Volume mode indicator icon space
const ICON_SIZE: f32 = 20.0;

/// Slider configuration returned from init for use by signal handlers
pub struct SliderConfig {
    pub slider: Entity<SliderState>,
    pub min_volume: f32,
    pub max_volume: f32,
}

pub fn init(cx: &mut App) -> SliderConfig {
    let VolumeSliderSettings {
        layer_shell,
        min_volume_level,
        max_volume_level,
    } = Settings::global(cx).volume_slider.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = layer_shell;

    let initial_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
    let slider_state = cx.new(|_| {
        SliderState::new()
            .min(min_volume_level)
            .max(max_volume_level)
            .default_value(initial_value)
            .pattern(SliderPattern::Bars)
    });

    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));
    let slider_state_for_overlay = slider_state.clone();

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: Some(exclusive_zone),
                margin: Some((px(0.0), px(0.0), px(60.0), px(16.0))),
                ..Default::default()
            }),
            ..Default::default()
        },
        move |_window, cx| {
            let slider_state = slider_state_for_overlay.clone();
            let initial_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
            cx.new(move |cx| {
                SliderOverlay::new(
                    cx,
                    slider_state.clone(),
                    initial_value,
                    min_volume_level,
                    max_volume_level,
                )
            })
        },
    )
    .unwrap();

    SliderConfig {
        slider: slider_state,
        min_volume: min_volume_level,
        max_volume: max_volume_level,
    }
}

struct SliderOverlay {
    slider_state: Entity<SliderState>,
    slider_value: f32,
    min_volume: f32,
    max_volume: f32,
    visible: bool,
    dismiss_generation: u64,
    _subscription: Subscription,
}

impl SliderOverlay {
    fn new(
        cx: &mut Context<Self>,
        slider_state: Entity<SliderState>,
        initial_value: f32,
        min_volume: f32,
        max_volume: f32,
    ) -> Self {
        let subscription = cx.subscribe(&slider_state, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = *event;
            this.slider_value = value;
            this.show_overlay(cx);
            // Sync volume to ShellState and system when slider is changed via touch/drag
            set_volume(cx, value);
            sync_volume_to_system(value, cx);
        });

        Self {
            slider_state,
            slider_value: initial_value,
            min_volume,
            max_volume,
            visible: false,
            dismiss_generation: 0,
            _subscription: subscription,
        }
    }

    fn show_overlay(&mut self, cx: &mut Context<Self>) {
        self.visible = true;
        self.dismiss_generation = self.dismiss_generation.wrapping_add(1);
        let generation = self.dismiss_generation;
        cx.notify();

        if OVERLAY_TIMEOUT_MS == 0 {
            return;
        }

        cx.spawn(
            async move |this: WeakEntity<SliderOverlay>, cx: &mut AsyncApp| {
                cx.background_executor()
                    .timer(Duration::from_millis(OVERLAY_TIMEOUT_MS))
                    .await;

                this.update(cx, |this, cx| {
                    if this.dismiss_generation == generation {
                        this.visible = false;
                        cx.notify();
                    }
                })
                .ok();
            },
        )
        .detach();
    }

    fn overlay_bounds(&self, window: &Window) -> Bounds<Pixels> {
        let window_size = window.bounds().size;
        let overlay_width = SLIDER_WIDTH.max(ICON_SIZE) + (OVERLAY_PADDING * 2.0);
        let overlay_height = SLIDER_HEIGHT + ICON_SIZE + OVERLAY_GAP + (OVERLAY_PADDING * 2.0);
        let overlay_size = size(px(overlay_width), px(overlay_height));
        let origin = point(
            (window_size.width - overlay_size.width) / 2.0,
            (window_size.height - overlay_size.height) / 2.0,
        );

        Bounds {
            origin,
            size: overlay_size,
        }
    }

    fn update_input_regions(&self, window: &mut Window, show: bool) {
        let regions = if show {
            vec![self.overlay_bounds(window)]
        } else {
            vec![]
        };

        window.set_input_regions(Some(regions));
    }
}

impl Render for SliderOverlay {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon_name =
            VolumeIconName::from_volume(self.slider_value, self.min_volume, self.max_volume);
        self.update_input_regions(window, self.visible);
        if self.visible {
            div()
                .flex()
                .items_center()
                .justify_center()
                .w_full()
                .h_full()
                .child(
                    div()
                        .id("slider-overlay")
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(OVERLAY_GAP))
                        .p(px(OVERLAY_PADDING))
                        .rounded(px(OVERLAY_RADIUS))
                        .bg(rgb(OVERLAY_BACKGROUND))
                        .shadow_lg()
                        .child(
                            Slider::new("hardware-buttons-slider", &self.slider_state)
                                .width(SLIDER_WIDTH)
                                .height(SLIDER_HEIGHT),
                        )
                        // Volume mode indicator icon
                        .child(
                            div()
                                .id("volume-mode-icon")
                                .w(px(ICON_SIZE))
                                .h(px(ICON_SIZE))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    VolumeIcon::new(icon_name)
                                        .size((px(ICON_SIZE), px(ICON_SIZE)))
                                        .text_color(rgb(0xFFFFFF)),
                                ),
                        ),
                )
                .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }
}
