use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use std::time::Duration;
use settings::prelude::{LayerShellSettings, Settings, VolumeSliderSettings};

use crate::handle;
use crate::slider::{Slider, SliderEvent, SliderPattern, SliderState};

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

pub fn init(cx: &mut App) {
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

    let initial_value = handle::slider_value(cx);
    let min_volume = min_volume_level;
    let max_volume = max_volume_level;
    let slider_state = cx.new(|_| {
        SliderState::new()
            .min(min_volume)
            .max(max_volume)
            .default_value(initial_value)
            .pattern(SliderPattern::Bars)
    });

    if let Some((entity, value)) = handle::register_slider(cx, &slider_state) {
        handle::sync_slider_value(&entity, value, cx);
    }

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
                margin: Some((px(0.0), px(0.0), px(60.0), px(0.0))),
                ..Default::default()
            }),
            ..Default::default()
        },
        move |_window, cx| {
            let slider_state = slider_state_for_overlay.clone();
            let initial_value = handle::slider_value(cx);
            cx.new(move |cx| SliderOverlay::new(cx, slider_state.clone(), initial_value))
        },
    )
    .unwrap();
}

struct SliderOverlay {
    slider_state: Entity<SliderState>,
    slider_value: f32,
    visible: bool,
    dismiss_generation: u64,
    _subscription: Subscription,
}

impl SliderOverlay {
    fn new(cx: &mut Context<Self>, slider_state: Entity<SliderState>, initial_value: f32) -> Self {
        let subscription = cx.subscribe(
            &slider_state,
            |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = *event;
                this.slider_value = value;
                this.show_overlay(cx);
            },
        );

        Self {
            slider_state,
            slider_value: initial_value,
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

        cx.spawn(async move |this: WeakEntity<SliderOverlay>, cx: &mut AsyncApp| {
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
        })
        .detach();
    }
}

impl Render for SliderOverlay {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .opacity(if self.visible { 1.0 } else { 0.0 })
                    .shadow_lg()
                    .child(
                        Slider::new("hardware-buttons-slider", &self.slider_state)
                            .width(SLIDER_WIDTH)
                            .height(SLIDER_HEIGHT),
                    )
                    // Volume mode indicator icon space (20x20)
                    .child(
                        div()
                            .id("volume-mode-icon")
                            .w(px(ICON_SIZE))
                            .h(px(ICON_SIZE))
                            .flex()
                            .items_center()
                            .justify_center(),
                    ),
            )
    }
}
