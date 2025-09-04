use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::Component,
};

/// A marker component to indicate that a widget is disabled and should be "grayed out".
/// This is used to prevent user interaction with the widget. It should not, however, prevent
/// the widget from being updated or rendered, or from acquiring keyboard focus.
///
/// For apps which support a11y: if a widget (such as a slider) contains multiple entities,
/// the `InteractionDisabled` component should be added to the root entity of the widget - the
/// same entity that contains the `AccessibilityNode` component. This will ensure that
/// the a11y tree is updated correctly.
#[derive(Component, Debug, Clone, Copy)]
#[component(on_add = on_add_disabled, on_remove = on_remove_disabled)]
pub struct InteractionDisabled;

// Hook to set the a11y "disabled" state when the widget is disabled.
fn on_add_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_disabled();
    }
}

// Hook to remove the a11y "disabled" state when the widget is enabled.
fn on_remove_disabled(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.clear_disabled();
    }
}

/// Component that indicates whether a button is currently pressed. This will be true while
/// a drag action is in progress.
#[derive(Component, Default, Debug)]
pub struct ButtonPressed(pub bool);

/// Component that indicates whether a checkbox or radio button is in a checked state.
#[derive(Component, Default, Debug)]
#[component(immutable, on_add = on_add_checked, on_replace = on_add_checked)]
pub struct Checked(pub bool);

// Hook to set the a11y "checked" state when the checkbox is added.
fn on_add_checked(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let checked = entt.get::<Checked>().unwrap().0;
    let mut accessibility = entt.get_mut::<AccessibilityNode>().unwrap();
    accessibility.set_toggled(match checked {
        true => accesskit::Toggled::True,
        false => accesskit::Toggled::False,
    });
}
