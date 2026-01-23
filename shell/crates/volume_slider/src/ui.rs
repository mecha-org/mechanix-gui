use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use icons::prelude::Icons;
use settings::prelude::{LayerShellSettings, Settings, VolumeSliderSettings};
use std::time::Duration;

use crate::slider::{Slider, SliderEvent, SliderPattern, SliderState};
use crate::{get_volume, set_volume, sync_volume_to_system};

const OVERLAY_PADDING: f32 = 16.0;
const OVERLAY_GAP: f32 = 12.0;
const OVERLAY_RADIUS: f32 = 15.0;
const OVERLAY_BACKGROUND: u32 = 0x101010;
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
        overlay_timeout_ms,
        input_regions,
    } = Settings::global(cx).volume_slider.clone();
    let LayerShellSettings {
        layer,
        anchor,
        namespace,
        exclusive_zone,
        ..
    } = layer_shell;

    let initial_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
    let slider_state = cx.new(|_| {
        SliderState::new()
            .min(min_volume_level)
            .max(max_volume_level)
            .default_value(initial_value)
            .pattern(SliderPattern::Bars)
    });

    let window_bounds = WindowBounds::Windowed(Bounds::new(
        input_regions.maximized.origin,
        input_regions.maximized.size,
    ));
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
                margin: Some((px(0.0), px(0.0), px(0.0), px(16.0))),
                ..Default::default()
            }),
            ..Default::default()
        },
        move |window, cx| {
            let slider_state = slider_state_for_overlay.clone();
            let initial_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
            window.set_input_regions(Some(Vec::new()));
            cx.new(move |cx| {
                SliderOverlay::new(
                    cx,
                    slider_state.clone(),
                    initial_value,
                    min_volume_level,
                    max_volume_level,
                    overlay_timeout_ms,
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
    overlay_timeout_ms: u64,
    visible: bool,
    last_visible: bool,
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
        overlay_timeout_ms: u64,
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
            overlay_timeout_ms,
            visible: false,
            last_visible: false,
            dismiss_generation: 0,
            _subscription: subscription,
        }
    }

    // This is to add and remove input region
    fn update_input_regions(&mut self, window: &mut Window, show: bool, cx: &mut Context<Self>) {
        let regions = if show {
            let input_regions = Settings::global(cx).volume_slider.input_regions.clone();
            vec![Bounds {
                origin: input_regions.maximized.origin,
                size: input_regions.maximized.size,
            }]
        } else {
            Vec::new()
        };

        window.set_input_regions(Some(regions));
        self.last_visible = show;
        cx.notify();
    }

    fn show_overlay(&mut self, cx: &mut Context<Self>) {
        self.visible = true;
        self.dismiss_generation = self.dismiss_generation.wrapping_add(1);
        let generation = self.dismiss_generation;
        let overlay_timeout_ms = self.overlay_timeout_ms;
        cx.notify();

        if overlay_timeout_ms == 0 {
            return;
        }

        cx.spawn(
            async move |this: WeakEntity<SliderOverlay>, cx: &mut AsyncApp| {
                cx.background_executor()
                    .timer(Duration::from_millis(overlay_timeout_ms))
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
}

impl Render for SliderOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let icons = Icons::global(cx).settings_drawer.clone();

        let icon_name = if self.slider_value <= self.min_volume {
            icons.volume_off
        } else {
            let range = self.max_volume - self.min_volume;
            let normalized = (self.slider_value - self.min_volume) / range;
            if normalized <= 0.33 {
                icons.volume_low
            } else if normalized <= 0.66 {
                icons.volume_medium
            } else {
                icons.volume_high
            }
        };

        if self.visible != self.last_visible {
            self.update_input_regions(window, self.visible, cx);
        }
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
                                    svg()
                                        .external_path(SharedString::from(
                                            icon_name.to_string_lossy().to_string(),
                                        ))
                                        .size(px(ICON_SIZE))
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
