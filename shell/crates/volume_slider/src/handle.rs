use futures::SinkExt;
use gpui::*;
use hw_buttons::{Key, KeyEvent};
use settings::prelude::{Settings, VolumeSliderSettings};
use shell_state::{ShellState, VolumeMessage};

use crate::slider::{SliderEvent, SliderState};

const DEFAULT_VOLUME_LEVEL: f32 = 35.0;
const VOLUME_STEP: f32 = 3.0;

#[derive(Debug)]
pub struct VolumeState {
    slider: Option<Entity<SliderState>>,
    volume: f32,
    volume_step: f32,
    min_volume_level: f32,
    max_volume_level: f32,
}

impl Global for VolumeState {}

impl Default for VolumeState {
    fn default() -> Self {
        Self::new(VolumeSliderSettings::default())
    }
}

impl VolumeState {
    fn new(volume_slider_settings: VolumeSliderSettings) -> Self {
        let min_volume_level = volume_slider_settings.min_volume_level;
        let max_volume_level = volume_slider_settings.max_volume_level;
        let initial_volume = DEFAULT_VOLUME_LEVEL.clamp(min_volume_level, max_volume_level);

        Self {
            slider: None,
            volume: initial_volume,
            volume_step: VOLUME_STEP,
            min_volume_level,
            max_volume_level,
        }
    }
}

pub fn init(cx: &mut App) {
    if cx.has_global::<VolumeState>() {
        return;
    }

    let volume_slider_settings = Settings::global(cx).volume_slider.clone();
    cx.set_global(VolumeState::new(volume_slider_settings));
}

pub fn handle_volume_event(cx: &mut App, event: KeyEvent) {
    let mut slider_update: Option<(Entity<SliderState>, f32)> = None;
    let mut volume_delta: Option<f32> = None;

    {
        let state = cx.global_mut::<VolumeState>();
        match event {
            KeyEvent::Pressed(Key::VolumeUp)
            | KeyEvent::Pressing(Key::VolumeUp)
            | KeyEvent::Unknown(Key::VolumeUp) => {
                volume_delta = Some(state.volume_step);
            }
            KeyEvent::Pressed(Key::VolumeDown)
            | KeyEvent::Pressing(Key::VolumeDown)
            | KeyEvent::Unknown(Key::VolumeDown) => {
                volume_delta = Some(-state.volume_step);
            }
            _ => {}
        }
    }

    if let Some(delta) = volume_delta {
        slider_update = adjust_volume_by(cx, delta);
    }

    if let Some((entity, value)) = slider_update {
        apply_value_to_slider(&entity, value, cx);
    }
}

pub fn register_slider(
    cx: &mut App,
    slider: &Entity<SliderState>,
) -> Option<(Entity<SliderState>, f32)> {
    let current_value = slider_value(cx);
    let state = cx.global_mut::<VolumeState>();
    state.slider = Some(slider.clone());
    Some((slider.clone(), current_value))
}

pub fn slider_value(cx: &mut App) -> f32 {
    let slider_reading = latest_slider_value(cx);
    let state = cx.global_mut::<VolumeState>();
    if let Some(value) = slider_reading {
        state.volume = value.clamp(state.min_volume_level, state.max_volume_level);
    }
    state.volume
}

pub fn sync_slider_value(slider: &Entity<SliderState>, value: f32, cx: &mut App) {
    slider.update(cx, |state, _| {
        state.value = value.clamp(state.min, state.max);
    });

    let state = cx.global_mut::<VolumeState>();
    state.volume = value.clamp(state.min_volume_level, state.max_volume_level);
}

fn apply_value_to_slider(slider: &Entity<SliderState>, value: f32, cx: &mut App) {
    slider.update(cx, |state, cx| {
        let clamped = value.clamp(state.min, state.max);
        if (state.value - clamped).abs() < f32::EPSILON {
            return;
        }
        state.value = clamped;
        cx.emit(SliderEvent::Change(state.value));
        cx.notify();
    });

    // Sync volume to system via PulseAudio
    sync_volume_to_system(value, cx);
}

/// Sends the volume value to the system via PulseAudio using ShellState's volume channel.
pub fn sync_volume_to_system(value: f32, cx: &mut App) {
    // Check if ShellState is initialized (it may not be when running standalone)

    if !cx.has_global::<ShellState>() {
        return;
    }

    let shell_state = ShellState::global(cx);
    let volume_tx = shell_state.volume_tx.clone();
    let sink_name = shell_state
        .sound_device_info
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

fn adjust_volume_by(cx: &mut App, delta: f32) -> Option<(Entity<SliderState>, f32)> {
    let slider_value = latest_slider_value(cx);

    let state = cx.global_mut::<VolumeState>();
    let base = slider_value.unwrap_or(state.volume);
    state.volume = (base + delta).clamp(state.min_volume_level, state.max_volume_level);
    state.slider.clone().map(|entity| (entity, state.volume))
}

fn latest_slider_value(cx: &App) -> Option<f32> {
    let state = cx.global::<VolumeState>();
    state.slider.as_ref().map(|slider| slider.read(cx).value())
}
