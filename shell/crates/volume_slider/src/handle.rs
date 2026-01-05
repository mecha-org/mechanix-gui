use gpui::*;
use hw_buttons::{Key, KeyEvent};
use power_options::run_app as run_power_overlay;
use settings::prelude::{Settings, VolumeSliderSettings};

use crate::slider::{SliderEvent, SliderState};

const DEFAULT_VOLUME_LEVEL: f32 = 35.0;
const VOLUME_STEP: f32 = 3.0;

#[derive(Debug)]
pub struct HardwareState {
    slider: Option<Entity<SliderState>>,
    volume: f32,
    last_home: Option<KeyEvent>,
    last_power: Option<KeyEvent>,
    power_overlay_visible: bool,
    volume_step: f32,
    min_volume_level: f32,
    max_volume_level: f32,
}

impl Global for HardwareState {}

impl Default for HardwareState {
    fn default() -> Self {
        Self::new(VolumeSliderSettings::default())
    }
}

impl HardwareState {
    fn new(volume_slider_settings: VolumeSliderSettings) -> Self {
        let min_volume_level = volume_slider_settings.min_volume_level;
        let max_volume_level = volume_slider_settings.max_volume_level;
        let initial_volume = DEFAULT_VOLUME_LEVEL.clamp(min_volume_level, max_volume_level);

        Self {
            slider: None,
            volume: initial_volume,
            last_home: None,
            last_power: None,
            power_overlay_visible: false,
            volume_step: VOLUME_STEP,
            min_volume_level,
            max_volume_level,
        }
    }
}

pub fn init(cx: &mut App) {
    if cx.has_global::<HardwareState>() {
        return;
    }

    let volume_slider_settings = Settings::global(cx).volume_slider.clone();
    cx.set_global(HardwareState::new(volume_slider_settings));
}

pub fn handle_event(cx: &mut App, event: KeyEvent) {
    let mut launch_power_overlay = false;
    let mut slider_update: Option<(Entity<SliderState>, f32)> = None;
    let mut volume_delta: Option<f32> = None;

    {
        let state = cx.global_mut::<HardwareState>();
        match event {
            KeyEvent::Pressed(Key::Home) => {
                state.last_home = Some(event);
                println!("[hardware-buttons] Home button short press: {:?}", event);
            }
            KeyEvent::Pressing(Key::Home) => {
                println!(
                    "[hardware-buttons] Home button long press detected: {:?}",
                    event
                );
            }
            KeyEvent::Released(Key::Home) => {
                state.last_home = Some(event);
                println!("[hardware-buttons] Home button released: {:?}", event);
            }
            KeyEvent::Pressed(Key::Power) => {
                state.last_power = Some(event);
                println!("[hardware-buttons] Power button short press: {:?}", event);
            }
            KeyEvent::Pressing(Key::Power) => {
                if !state.power_overlay_visible {
                    println!(
                        "[hardware-buttons] Power button long press detected, showing power overlay"
                    );
                    state.power_overlay_visible = true;
                    launch_power_overlay = true;
                }
            }
            KeyEvent::Released(Key::Power) => {
                state.last_power = Some(event);
                println!("[hardware-buttons] Power button released: {:?}", event);
            }
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

    if launch_power_overlay {
        run_power_overlay(cx);
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
    let state = cx.global_mut::<HardwareState>();
    state.slider = Some(slider.clone());
    Some((slider.clone(), current_value))
}

pub fn slider_value(cx: &mut App) -> f32 {
    let slider_reading = latest_slider_value(cx);
    let state = cx.global_mut::<HardwareState>();
    if let Some(value) = slider_reading {
        state.volume = value.clamp(state.min_volume_level, state.max_volume_level);
    }
    state.volume
}

pub fn sync_slider_value(slider: &Entity<SliderState>, value: f32, cx: &mut App) {
    slider.update(cx, |state, _| {
        state.value = value.clamp(state.min, state.max);
    });

    let state = cx.global_mut::<HardwareState>();
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
}

fn adjust_volume_by(cx: &mut App, delta: f32) -> Option<(Entity<SliderState>, f32)> {
    let slider_value = latest_slider_value(cx);

    let state = cx.global_mut::<HardwareState>();
    let base = slider_value.unwrap_or(state.volume);
    state.volume = (base + delta).clamp(state.min_volume_level, state.max_volume_level);
    state.slider.clone().map(|entity| (entity, state.volume))
}

fn latest_slider_value(cx: &App) -> Option<f32> {
    let state = cx.global::<HardwareState>();
    state
        .slider
        .as_ref()
        .map(|slider| slider.read(cx).value())
}
