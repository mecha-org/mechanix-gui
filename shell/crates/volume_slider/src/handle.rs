use futures::SinkExt;
use gpui::*;
use hw_buttons::{Key, KeyEvent};
use shell_state::{ShellState, VolumeMessage};

use crate::slider::{SliderEvent, SliderState};

const DEFAULT_VOLUME_LEVEL: f32 = 35.0;
const VOLUME_STEP: f32 = 3.0;

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

/// Handle hardware volume button events - updates slider entity and syncs to system
pub fn handle_volume_event(
    cx: &mut App,
    event: KeyEvent,
    slider: &Entity<SliderState>,
    min_volume: f32,
    max_volume: f32,
) {
    let delta = match event {
        KeyEvent::Pressed(Key::VolumeUp)
        | KeyEvent::Pressing(Key::VolumeUp)
        | KeyEvent::Unknown(Key::VolumeUp) => VOLUME_STEP,
        KeyEvent::Pressed(Key::VolumeDown)
        | KeyEvent::Pressing(Key::VolumeDown)
        | KeyEvent::Unknown(Key::VolumeDown) => -VOLUME_STEP,
        _ => return,
    };

    let current = slider.read(cx).value();
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
