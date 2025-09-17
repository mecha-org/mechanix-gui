use super::*;
use bevy::ecs::{
    component::Mutable, query::QueryEntityError, schedule::ScheduleConfigs, system::ScheduleSystem,
};
use bevy::platform::collections::{HashMap, HashSet};
use std::any::type_name;

/// Alias for [`apply_component_tween_system`] and may contains more systems
/// in the future.
pub fn component_tween_system<I>() -> ScheduleConfigs<ScheduleSystem>
where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component<Mutability = Mutable>,
{
    apply_component_tween_system::<I>.into_configs()
}

/// [`QueryEntityError`] but implemented hash [`Hash`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryEntityErrorWithoutWorld {
    /// The given [`Entity`]'s components do not match the query.
    ///
    /// Either it does not have a requested component, or it has a component which the query filters out.
    QueryDoesNotMatch(Entity, bevy::ecs::archetype::ArchetypeId),
    /// The given [`Entity`] does not exist.
    EntityDoesNotExist(Entity),
    /// The [`Entity`] was requested mutably more than once.
    ///
    /// See [`Query::get_many_mut`](crate::system::Query::get_many_mut) for an example.
    AliasedMutability(Entity),
}

impl From<&QueryEntityError> for QueryEntityErrorWithoutWorld {
    fn from(x: &QueryEntityError) -> Self {
        use QueryEntityError as E;
        use QueryEntityErrorWithoutWorld as EH;
        match x {
            E::QueryDoesNotMatch(entity, archetype_id) => {
                EH::QueryDoesNotMatch(*entity, *archetype_id)
            }
            E::EntityDoesNotExist(entity_does_not_exist_error) => {
                EH::EntityDoesNotExist(entity_does_not_exist_error.entity)
            }
            E::AliasedMutability(entity) => EH::AliasedMutability(*entity),
        }
    }
}

impl core::error::Error for QueryEntityErrorWithoutWorld {}

impl core::fmt::Display for QueryEntityErrorWithoutWorld {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::QueryDoesNotMatch(entity, _) => {
                write!(f, "The query does not match the entity {entity}")
            }
            Self::EntityDoesNotExist(entity) => {
                write!(f, "The entity {entity} does not exist")
            }
            Self::AliasedMutability(entity) => write!(
                f,
                "The entity {entity} was requested mutably more than once"
            ),
        }
    }
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic with the trait [`Interpolator`] for you to quickly
/// make your interpolators work. The trait is only necessary to be used with
/// this built-in system.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Component)]
/// struct Size(f32);
///
/// struct InterpolateSize {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateSize {
///     type Item = Size;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
///
/// fn main() {
///     let mut app = App::new();
///
///     // You might want to use `bevy_tween::component_tween_system` instead.
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_component_tween_system::<InterpolateSize>,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_component_tween_system::<
///             BoxedInterpolator<Size>,
///         >,
///     );
/// }
/// ```
#[allow(clippy::type_complexity)]
pub fn apply_component_tween_system<I>(
    q_animation_target: Query<(Option<&ChildOf>, Has<AnimationTarget>)>,
    mut q_tween: Query<
        (
            Entity,
            &Tween<TargetComponent, I>,
            &TweenInterpolationValue,
            &mut TweenPreviousValue,
        ),
        Without<SkipTween>,
    >,
    mut q_component: Query<&mut I::Item>,
    mut last_entity_error: Local<HashMap<Entity, QueryEntityErrorWithoutWorld>>,
    mut last_search_error: Local<HashSet<Entity>>,
) where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component<Mutability = Mutable>,
{
    let mut entity_error = HashMap::default();
    let mut search_error = HashSet::default();
    q_tween
        .iter_mut()
        .for_each(|(entity, tween, ease_value, mut previous_value)| match &tween.target {
            TargetComponent::Entities(e) => {
                e.iter().for_each(|target| {
                    let mut target_component =
                        match q_component.get_mut(*target) {
                            Ok(target_component) => target_component,
                            Err(e) => {
                                let e_no_world = QueryEntityErrorWithoutWorld::from(&e);
                                if last_entity_error
                                    .get(target)
                                    .map(|old_e| old_e != &e_no_world)
                                    .unwrap_or(true)
                                    && entity_error
                                        .get(target)
                                        .map(|old_e| old_e != &e_no_world)
                                        .unwrap_or(true)
                                {
                                    error!(
                                        "{} attempted to tween {} component but got query error: {e}",
                                        type_name::<I>(),
                                        type_name::<I::Item>()
                                    );
                                }
                                entity_error.insert(*target, e_no_world);
                                return;
                            }
                        };
                    tween
                        .interpolator
                        .interpolate(&mut target_component, ease_value.0, previous_value.0);
                    previous_value.0 = ease_value.0;
                });
            }
            _ => {
                let target = match &tween.target {
                    TargetComponent::Marker => {
                        let mut curr = entity;
                        let found = 'l: loop {
                            match q_animation_target.get(curr) {
                                Ok((parent, has_marker)) => {
                                    if has_marker {
                                        break 'l Some(curr);
                                    } else {
                                        match parent {
                                            Some(parent) => curr = parent.parent(),
                                            None => break 'l None,
                                        }
                                    }
                                }
                                _ => break 'l None,
                            }
                        };
                        match found {
                            Some(found) => found,
                            None => {
                                if !last_search_error.contains(&entity) && !search_error.contains(&entity) {
                                    error!(
                                        "Tween {:?} {} cannot find AnimationTarget marker",
                                        entity,
                                        type_name::<I>(),
                                    );
                                }
                                search_error.insert(entity);
                                return;
                            },
                        }
                    }
                    TargetComponent::Entity(e) => *e,
                    _ => unreachable!(),
                };

                let mut target_component = match q_component.get_mut(target) {
                    Ok(target_component) => target_component,
                    Err(e) => {
                        let e_no_world = QueryEntityErrorWithoutWorld::from(&e);
                        if last_entity_error
                            .get(&target)
                            .map(|old_e| old_e != &e_no_world)
                            .unwrap_or(true)
                            && entity_error
                                .get(&target)
                                .map(|old_e| old_e != &e_no_world)
                                .unwrap_or(true)
                        {
                            error!(
                                "{} attempted to tween {} component but got query error: {e}",
                                type_name::<I>(),
                                type_name::<I::Item>()
                            );
                        }
                        entity_error.insert(target, e_no_world);
                        return;
                    }
                };
                tween
                    .interpolator
                    .interpolate(&mut target_component, ease_value.0, previous_value.0);
                previous_value.0 = ease_value.0;
            }
        });
    *last_entity_error = entity_error;
    *last_search_error = search_error;
}

/// System alias for [`component_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
///
/// This currently exists for backward compatibility and there's not really any big reason to deprecate it just yet.
/// You might want to use `component_tween_system::<BoxedInterpolator<...>>()` for consistency
pub fn component_dyn_tween_system<C>() -> ScheduleConfigs<ScheduleSystem>
where
    C: Component<Mutability = Mutable>,
{
    apply_component_tween_system::<Box<dyn Interpolator<Item = C>>>.into_configs()
}

/// Alias for [`apply_resource_tween_system`] and may contains more systems
/// in the future.
pub fn resource_tween_system<I>() -> ScheduleConfigs<ScheduleSystem>
where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Resource,
{
    apply_resource_tween_system::<I>.into_configs()
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetResource`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic with the trait [`Interpolator`] for you to quickly
/// make your interpolators work. The trait is only necessary to be used with
/// this built-in system.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Resource)]
/// struct ScreenFade(f32);
///
/// struct InterpolateScreenFade {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateScreenFade {
///     type Item = ScreenFade;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
///
/// fn main() {
///     let mut app = App::new();
///
///     // You might want to use `bevy_tween::resource_tween_system` instead.
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_resource_tween_system::<
///             InterpolateScreenFade,
///         >,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_resource_tween_system::<
///             BoxedInterpolator<ScreenFade>,
///         >,
///     );
/// }
/// ```
#[allow(clippy::type_complexity)]
pub fn apply_resource_tween_system<I>(
    mut q_tween: Query<
        (
            &Tween<TargetResource, I>,
            &TweenInterpolationValue,
            &mut TweenPreviousValue,
        ),
        Without<SkipTween>,
    >,
    resource: Option<ResMut<I::Item>>,
    mut last_error: Local<bool>,
) where
    I: Interpolator,
    I::Item: Resource,
{
    let Some(mut resource) = resource else {
        if !*last_error {
            error!(
                "{} resource tween system cannot find the resource",
                type_name::<I>()
            );
            *last_error = true;
        }
        return;
    };
    *last_error = false;
    q_tween
        .iter_mut()
        .for_each(|(tween, ease_value, mut previous_value)| {
            tween
                .interpolator
                .interpolate(&mut resource, ease_value.0, previous_value.0);
            previous_value.0 = ease_value.0;
        })
}

/// System alias for [`apply_resource_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
///
/// This currently exists for backward compatibility and there's not really any big reason to deprecate it just yet.
/// You might want to use `resource_tween_system::<BoxedInterpolator<...>>()` for consistency
pub fn resource_dyn_tween_system<R>() -> ScheduleConfigs<ScheduleSystem>
where
    R: Resource,
{
    apply_resource_tween_system::<Box<dyn Interpolator<Item = R>>>.into_configs()
}
