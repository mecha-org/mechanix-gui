use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledAppBundle {
    #[reflect(ignore)]
    pub variant: ButtonVariant,
    #[reflect(ignore)]
    pub size: Option<ButtonSize>,
    pub text: Option<String>,
    pub icon: Option<String>,
    pub font: Option<Handle<Font>>,
    pub image: Option<Handle<Image>>,
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
}

impl StyledAppBundle {
    pub fn builder() -> super::builder::ButtonBuilder {
        super::builder::ButtonBuilder::default()
    }
}
#[derive(Component)]
pub struct StyledAppBundleText;

#[derive(Component)]
pub struct StyledAppBundleIcon;

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
