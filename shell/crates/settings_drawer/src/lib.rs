pub mod helper;
mod ui;

use dispatcher::Dispatcher;
use futures::SinkExt;
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use settings::prelude::*;
use shell_state::{BrightnessMessage, DEFAULT_MIN_BRIGHTNESS, ShellState, VolumeMessage};
use ui::*;
const DEFAULT_VOLUME_LEVEL: f32 = 0.0;
use crate::ui::widgets::SliderState;

pub mod prelude {
    pub use crate::run_app;
    pub use crate::ui::SettingsDrawer;
}

pub fn run_app(cx: &mut App) {
    let SettingsDrawerSettings {
        layer_shell,
        input_regions,
        ..
    } = Settings::global(cx).settings_drawer.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = layer_shell;

    let VolumeSliderSettings {
        min_volume_level,
        max_volume_level,
        ..
    } = Settings::global(cx).volume_slider.clone();

    // slider initialization
    let initial_volume_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
    let initial_mute_volume_check = get_is_volume_mute(cx);

    let volume_slider_state = cx.new(|_| {
        SliderState::new("volume-slider-state")
            .min(min_volume_level)
            .max(max_volume_level)
            .default_value(initial_volume_value)
            .pattern(widgets::SliderPattern::Bars)
    });

    let initial_brightness_value = get_brightness(cx);
    let brightness_slider_state = cx.new(|_| {
        SliderState::new("brightness-slider-state")
            .min(min_volume_level)
            .max(max_volume_level)
            .default_value(initial_brightness_value)
            .pattern(widgets::SliderPattern::Dots)
    });

    //

    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));
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
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let mut regions = Vec::new();
            regions.push(Bounds {
                origin: input_regions.minimized.origin,
                size: input_regions.minimized.size,
            });
            window.set_input_regions(Some(regions));

            let volume_slider_state_clone = volume_slider_state.clone();
            let initial_volume_value = get_volume(cx).clamp(min_volume_level, max_volume_level);
            let initial_is_volume_mute = initial_mute_volume_check.clone();

            let initial_brightness_value = get_brightness(cx);
            let brightness_slider_state_clone = brightness_slider_state.clone();

            cx.new(|cx| {
                listen_dispatcher(cx);

                SettingsDrawer::new(
                    cx,
                    volume_slider_state_clone,
                    initial_volume_value,
                    initial_is_volume_mute,
                    brightness_slider_state_clone,
                    initial_brightness_value,
                )
            })
        },
    )
    .unwrap();
}

pub fn listen_dispatcher(cx: &mut Context<SettingsDrawer>) {
    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    let settings = Settings::global(cx).settings_drawer.clone();
    let closed_pos = SettingsDrawer::calculate_closed_position(&settings);

    cx.spawn(async move |this, cx| {
        while let Ok(message) = dispatcher_rx.recv().await {
            match message {
                dispatcher::Message::ShowPowerOptions(show) => {
                    if show {
                        let _ = this.update(cx, |this, cx| {
                            this.is_visible = false;
                            this.position = closed_pos;
                            cx.notify();
                        });
                    }
                }
                _ => {}
            }
        }
    })
    .detach();
}

/// Get current brightness from ShellState, or default
pub fn get_brightness(cx: &App) -> f32 {
    if cx.has_global::<ShellState>() {
        let value = ShellState::global(cx).brightness_value;
        if value > 0.0 {
            value
        } else {
            DEFAULT_MIN_BRIGHTNESS
        }
    } else {
        DEFAULT_MIN_BRIGHTNESS
    }
}

/// Set brightness in ShellState
pub fn set_brightness(cx: &mut App, value: f32) {
    if cx.has_global::<ShellState>() {
        ShellState::global_mut(cx).brightness_value = value;
    }
}

/// Sends the brightness value to the system
pub fn sync_brightness_to_system(value: f32, cx: &mut App) {
    if !cx.has_global::<ShellState>() {
        return;
    }

    let shell_state = ShellState::global(cx);
    let brightness_tx = shell_state.brightness_tx.clone();

    if let Some(mut tx) = brightness_tx {
        cx.background_executor()
            .spawn(async move {
                let _ = tx
                    .send(BrightnessMessage::BrightnessChanged { value: value })
                    .await;
            })
            .detach();
    }
}

/// Get current volume from ShellState, or default if not available
pub fn get_volume(cx: &App) -> f32 {
    if cx.has_global::<ShellState>() {
        let vol = ShellState::global(cx).volume;
        if vol > 0.0 { vol } else { DEFAULT_VOLUME_LEVEL }
    } else {
        DEFAULT_VOLUME_LEVEL
    }
}

pub fn get_is_volume_mute(cx: &App) -> bool {
    if cx.has_global::<ShellState>() {
        ShellState::global(cx).default_sound_device.mute
    } else {
        false
    }
}

/// Set volume in ShellState
pub fn set_volume(cx: &mut App, value: f32) {
    if cx.has_global::<ShellState>() {
        ShellState::global_mut(cx).volume = value;
    }
}

/// Sends the volume value to the system via PulseAudio using ShellState's volume channel.
pub fn sync_volume_to_system(value: f32, cx: &mut App) {
    if !cx.has_global::<ShellState>() {
        return;
    }

    let shell_state = ShellState::global(cx);
    let volume_tx = shell_state.volume_tx.clone();
    let sink_name = shell_state
        .default_sound_device
        .name
        .clone()
        .unwrap_or_else(|| "@DEFAULT_SINK@".to_string());

    if let Some(mut tx) = volume_tx {
        cx.background_executor()
            .spawn(async move {
                let _ = tx
                    .send(VolumeMessage::VolumeChanged {
                        name: sink_name,
                        value,
                    })
                    .await;
            })
            .detach();
    }
}

pub fn mute_volume_to_system(cx: &mut App) {
    if !cx.has_global::<ShellState>() {
        return;
    }

    let shell_state = ShellState::global(cx);
    let volume_tx = shell_state.volume_tx.clone();
    let sink_name = shell_state
        .default_sound_device
        .name
        .clone()
        .unwrap_or_else(|| "@DEFAULT_SINK@".to_string());

    if let Some(mut tx) = volume_tx {
        cx.background_executor()
            .spawn(async move {
                let _ = tx
                    .send(VolumeMessage::MuteSink {
                        name: sink_name.clone(),
                    })
                    .await;
            })
            .detach();
    }
}

pub fn unmute_volume_to_system(cx: &mut App) {
    if !cx.has_global::<ShellState>() {
        return;
    }

    let shell_state = ShellState::global(cx);
    let volume_tx = shell_state.volume_tx.clone();
    let sink_name = shell_state
        .default_sound_device
        .name
        .clone()
        .unwrap_or_else(|| "@DEFAULT_SINK@".to_string());

    if let Some(mut tx) = volume_tx {
        cx.background_executor()
            .spawn(async move {
                let _ = tx
                    .send(VolumeMessage::UnmuteSink {
                        name: sink_name.clone(),
                    })
                    .await;
            })
            .detach();
    }
}
