use std::time::Duration;

use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledButton {
    #[reflect(ignore)]
    pub variant: ButtonVariant,
    #[reflect(ignore)]
    pub size: Option<ButtonSize>,
    pub text: Option<String>,
    pub icon: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub font: Option<Handle<Font>>,
    #[reflect(ignore)]
    pub on_click: Option<SystemId>,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub hover_background_color: Option<Color>,
    pub hover_border_color: Option<Color>,
    pub text_color: Option<Color>,
    pub disabled: bool,
    pub width: Option<Val>,
    pub height: Option<Val>,
    pub border_radius: Option<f32>,
    pub active: Option<bool>,
    pub active_background_color: Option<Color>,
    pub on_press_icon: Option<Handle<Image>>,
    pub on_press_layout: Option<Handle<TextureAtlasLayout>>,
}

impl StyledButton {
    pub fn builder() -> super::builder::ButtonBuilder {
        super::builder::ButtonBuilder::default()
    }
}
#[derive(Component)]
pub struct StyledButtonText;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Primary
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum ButtonSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

#[derive(Component, Default)]
#[component(immutable, on_add = on_set_label, on_replace = on_set_label)]
pub struct AccessibleName(pub String);

fn on_set_label(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let name = entt.get::<AccessibleName>().unwrap().0.clone();
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(name.as_str());
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}
