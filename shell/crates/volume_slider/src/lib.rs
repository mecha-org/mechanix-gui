mod slider;
mod ui;

use dispatcher::Dispatcher;
use futures::SinkExt;
use gpui::*;
use shell_state::{ShellState, VolumeMessage};

use crate::slider::{SliderEvent, SliderState};

const DEFAULT_VOLUME_LEVEL: f32 = 0.0;
const VOLUME_STEP: f32 = 3.0;

/// Sets up dispatcher-driven volume handling and opens the slider UI overlay.
pub fn run_app(cx: &mut App) {
    let config = ui::init(cx);
    listen_dispatcher(cx, config.slider, config.min_volume, config.max_volume);
}

fn listen_dispatcher(cx: &mut App, slider: Entity<SliderState>, min_volume: f32, max_volume: f32) {
    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();
    let slider_entity = slider.clone();

    cx.spawn(async move |app| {
        while let Ok(message) = dispatcher_rx.recv().await {
            println!("volume listen_dispatcher() message: {:#?}", message);
            match message {
                dispatcher::Message::VolumeUp => {
                    let _ = app.update(|cx| {
                        handle_volume_delta(
                            cx,
                            &slider_entity,
                            VOLUME_STEP,
                            min_volume,
                            max_volume,
                        );
                    });
                }
                dispatcher::Message::VolumeDown => {
                    let _ = app.update(|cx| {
                        handle_volume_delta(
                            cx,
                            &slider_entity,
                            -VOLUME_STEP,
                            min_volume,
                            max_volume,
                        );
                    });
                }
                _ => {}
            }
        }
    })
    .detach();
}

/// Get current volume from ShellState, or default if not available
pub fn get_volume(cx: &App) -> f32 {
    if cx.has_global::<ShellState>() {
        let vol = ShellState::global(cx).volume;
        if vol > 0.0 {
            vol
        } else {
            DEFAULT_VOLUME_LEVEL
        }
    } else {
        DEFAULT_VOLUME_LEVEL
    }
}

/// Set volume in ShellState
pub fn set_volume(cx: &mut App, value: f32) {
    if cx.has_global::<ShellState>() {
        ShellState::global_mut(cx).volume = value;
    }
}

/// Adjust volume by delta, update slider, and sync to system.
fn handle_volume_delta(
    cx: &mut App,
    slider: &Entity<SliderState>,
    delta: f32,
    min_volume: f32,
    max_volume: f32,
) {
    let current = get_volume(cx);
    let new_value = (current + delta).clamp(min_volume, max_volume);

    slider.update(cx, |state, cx| {
        if (state.value - new_value).abs() < f32::EPSILON {
            return;
        }
        state.value = new_value;
        cx.emit(SliderEvent::Change(state.value));
        cx.notify();
    });

    set_volume(cx, new_value);
    sync_volume_to_system(new_value, cx);
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
