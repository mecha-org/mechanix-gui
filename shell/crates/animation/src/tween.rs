//! Module containing implementations for tween
//!
//! # Tween
//!
//! **Components**:
//! - [`Tween<T, I>`]
//! - [`SkipTween`]
//! - [`TweenInterpolationValue`]
//!
//! **Systems**
//! - [`component_tween_system`]
//! - [`resource_tween_system`]
//! - [`asset_tween_system`]
//!
//! **Targets**:
//! - [`TargetComponent`]
//! - [`TargetResource`]
//! - [`TargetAsset`]
//!
//! See available interpolators in [`interpolate`].
//!
//! ## Registering systems
//!
//! In order for your custom interpolators to work. You have to register systems
//! to actually have something happening.
//! The [`DefaultTweenPlugins`] will already register some systems for you already to get started.
//! Check [`DefaultInterpolatorsPlugin`] or [`DefaultDynInterpolatorsPlugin`].
//!
//! This crate contains generic systems tweening components, assets,
//! and resources, allowing you to quickly register your custom interpolators.
//!
//! Systems:
//! - [`component_tween_system()`], component tweening system
//! - [`resource_tween_system()`], resource tweening system
//! - [`asset_tween_system()`], asset tweening system
//!
//! Let's say you have some custom components with multiple interpolators.
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! use bevy::prelude::*;
//! use bevy_tween::prelude::*;
//!
//! #[derive(Component)]
//! pub struct Foo {
//!     a: f32,
//!     b: f32,
//!     c: f32,
//! }
//!
//! mod my_interpolate {
//!     use bevy::prelude::*;
//!     use bevy_tween::prelude::*;
//!
//!     pub struct FooA {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooA {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//!
//!     pub struct FooB {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooB {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//!
//!     pub struct FooC {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooC {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//! }
//! # }
//! ```
//!
//! There's 2 kind of system you might want to register.
//!
//! ### Registering system for generic interpolator
//!
//! Generic interpolator means we're not using any dynamic dispatch.
//! We've to register this system for **every individual interpolator**.
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! # use bevy::prelude::*;
//! # use bevy_tween::prelude::*;
//! # #[derive(Component)]
//! # pub struct Foo {
//! #     a: f32,
//! #     b: f32,
//! #     c: f32,
//! # }
//! # mod my_interpolate {
//! #     use bevy::prelude::*;
//! #     use bevy_tween::prelude::*;
//! #     pub struct FooA {}
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooB {}
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooC {}
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! # }
//! fn main() {
//!     use bevy_tween::component_tween_system;
//!     use my_interpolate::*;
//!
//!     App::new().add_tween_systems((
//!         component_tween_system::<FooA>(),
//!         component_tween_system::<FooB>(),
//!         component_tween_system::<FooC>(),
//!     ));
//! }
//! # }
//! ```
//!
//! ### Registering system for dynamic interpolator
//!
//! Dynamic interpolator means we're using dynamic dispatch or trait object.
//! We don't have to register system for every interpolator, we only have to
//! register this system just for **every individual component/asset/resource**.
// ///! <div class="warning">
// ///! <a href="fn.component_dyn_tween_system.html"><code>component_dyn_tween_system</code></a> is type of dynamic
// ///! interpolator for <code>Box&lt;dyn Interpolator&gt;</code>.
// ///! </div>
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! # use bevy::prelude::*;
//! # use bevy_tween::prelude::*;
//! # #[derive(Component)]
//! # pub struct Foo {
//! #     a: f32,
//! #     b: f32,
//! #     c: f32,
//! # }
//! # mod my_interpolate {
//! #     use bevy::prelude::*;
//! #     use bevy_tween::prelude::*;
//! #     pub struct FooA {}
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooB {}
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooC {}
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! # }
//! fn main() {
//!     use my_interpolate::*;
//!     use bevy_tween::component_dyn_tween_system;
//!
//!     // One system to rule them all
//!     // Note that we're only using the `Foo` type, not `FooA`, `FooB`,
//!     // and `FooC`!
//!     App::new().add_tween_systems(component_dyn_tween_system::<Foo>());
//!     // `component_dyn_tween_system` is just an alias for
//!     // `component_tween_system::<Box<dyn Interpolator<Item = ...>>>`
//! }
//! # }
//! ```
//!
//! [`BevyTweenRegisterSystems`]: crate::BevyTweenRegisterSystems
//! [`interpolate`]: crate::interpolate
//! [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
//! [`DefaultInterpolatorsPlugin`]: crate::interpolate::DefaultInterpolatorsPlugin
//! [`DefaultDynInterpolatorsPlugin`]: crate::interpolate::DefaultDynInterpolatorsPlugin

use bevy::prelude::*;

use crate::combinator::TargetState;
use crate::interpolate::Interpolator;

mod systems;

pub use systems::{
    apply_component_tween_system, component_dyn_tween_system, component_tween_system,
};
pub use systems::{apply_resource_tween_system, resource_dyn_tween_system, resource_tween_system};

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;

/// Automatically managed by an [`Interpolation`] such as [`EaseKind`] and
/// [`EaseClosure`] when a tween has the component [`TimeSpanProgress`](bevy_time_runner::TimeSpanProgress).
/// See [`sample_interpolations_system`]
///
/// [`sample_interpolations_system`]: crate::interpolation::sample_interpolations_system
/// [`Interpolation`]: crate::interpolation::Interpolation
/// [`EaseKind`]: crate::interpolation::EaseKind
/// [`EaseClosure`]: crate::interpolation::EaseClosure
#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct TweenInterpolationValue(pub f32);

/// Containing `target` and `interpolator`
#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[require(TweenPreviousValue)]
#[reflect(Component)]
pub struct Tween<T, I> {
    #[allow(missing_docs)]
    pub target: T,
    #[allow(missing_docs)]
    pub interpolator: I,
}

/// Tracks the tween's previous value
#[derive(Debug, Default, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct TweenPreviousValue(pub f32);

impl<T, I> Tween<T, I>
where
    I: Interpolator,
{
    /// Create a new [`Tween`] with a target and an interpolator.
    pub fn new_target<G>(target: G, interpolator: I) -> Self
    where
        G: Into<T>,
    {
        Tween {
            interpolator,
            target: target.into(),
        }
    }
}

impl<T, I> Tween<T, I>
where
    T: Default,
    I: Interpolator,
{
    /// Create a new [`Tween`] with the default target and an interpolator.
    pub fn new(interpolator: I) -> Self {
        Tween::new_target(T::default(), interpolator)
    }
}

impl<T, Item> Tween<T, Box<dyn Interpolator<Item = Item>>>
where
    Item: 'static,
{
    /// Create a new [`Tween`] with a target and an interpolator that will be boxed internally.
    pub fn new_target_boxed<G, I>(target: G, interpolator: I) -> Self
    where
        G: Into<T>,
        I: Interpolator<Item = Item>,
    {
        Self::new_target(target, Box::new(interpolator))
    }
}

impl<T, Item> Tween<T, Box<dyn Interpolator<Item = Item>>>
where
    T: Default,
    Item: 'static,
{
    /// Create a new [`Tween`] with the default target and an interpolator that will be boxed internally.
    pub fn new_boxed<I>(interpolator: I) -> Self
    where
        I: Interpolator<Item = Item>,
    {
        Self::new(Box::new(interpolator))
    }
}

/// Convenient alias for [`Tween`] that [`TargetComponent`] with generic [`Interpolator`].
pub type ComponentTween<I> = Tween<TargetComponent, I>;

/// Convenient alias for [`Tween`] that [`TargetComponent`] with boxed dynamic [`Interpolator`].
pub type ComponentDynTween<C> = Tween<TargetComponent, Box<dyn Interpolator<Item = C>>>;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TargetComponent {
    /// Navigate up the parent chain for entity with [`AnimationTarget`] marker component
    #[non_exhaustive]
    Marker,
    /// Target this entity.
    Entity(Entity),
    /// Target these entities.
    Entities(Vec<Entity>),
}

impl TargetComponent {
    /// Navigate up the parent chain for entity with [`AnimationTarget`] marker component
    pub fn marker() -> TargetComponent {
        TargetComponent::Marker
    }

    /// Target this entity.
    pub fn entity(entity: Entity) -> TargetComponent {
        TargetComponent::Entity(entity)
    }

    /// Target these entities.
    pub fn entities<I>(entities: I) -> TargetComponent
    where
        I: IntoIterator<Item = Entity>,
    {
        TargetComponent::from_iter(entities)
    }

    /// Create a new [`TargetState`] with the initial value out of this target.
    pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
        TargetState::new(self.clone(), value)
    }

    /// Create a new tween with the supplied interpolator out of this target.
    pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
        Tween {
            target: self.clone(),
            interpolator,
        }
    }

    /// Create a new tween with the supplied closure out of this target.
    pub fn with_closure<F, C>(&self, closure: F) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    where
        F: Fn(&mut C, f32, f32) + Send + Sync + 'static,
        C: Component,
    {
        let closure = crate::interpolate::closure(closure);
        let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
        Tween {
            target: self.clone(),
            interpolator,
        }
    }
}

impl Default for TargetComponent {
    fn default() -> Self {
        TargetComponent::marker()
    }
}

impl From<Entity> for TargetComponent {
    fn from(value: Entity) -> Self {
        TargetComponent::entity(value)
    }
}

impl FromIterator<Entity> for TargetComponent {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        TargetComponent::Entities(iter.into_iter().collect())
    }
}

impl<const N: usize> From<[Entity; N]> for TargetComponent {
    fn from(value: [Entity; N]) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<Vec<Entity>> for TargetComponent {
    fn from(value: Vec<Entity>) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<&Vec<Entity>> for TargetComponent {
    fn from(value: &Vec<Entity>) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl From<&[Entity]> for TargetComponent {
    fn from(value: &[Entity]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<const N: usize> From<&[Entity; N]> for TargetComponent {
    fn from(value: &[Entity; N]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

/// [`ComponentTween`]'s system will navigate up the parent chain
/// for this marker component while using [`TargetComponent::Marker`].
#[derive(Default, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct AnimationTarget;

impl<I> ComponentTween<I>
where
    I: Interpolator,
    I::Item: Component,
{
}

impl<C> ComponentDynTween<C> where C: Component {}

/// Convenient alias for [`Tween`] that [`TargetResource`] with generic [`Interpolator`].
pub type ResourceTween<I> = Tween<TargetResource, I>;

/// Convenient alias for [`Tween`] that [`TargetResource`] with dyanmic [`Interpolator`].
pub type ResourceDynTween<R> = Tween<TargetResource, Box<dyn Interpolator<Item = R>>>;

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct TargetResource;

impl TargetResource {
    /// New resource target
    pub fn new() -> TargetResource {
        TargetResource
    }

    /// Create a new [`TargetState`] with the initial value out of this target.
    pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
        TargetState::new(self.clone(), value)
    }

    /// Create a new tween with the supplied interpolator out of this target.
    pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
        Tween {
            target: self.clone(),
            interpolator,
        }
    }

    /// Create a new tween with the supplied closure out of this target.
    pub fn with_closure<F, C>(&self, closure: F) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    where
        F: Fn(&mut C, f32, f32) + Send + Sync + 'static,
        C: Component,
    {
        let closure = crate::interpolate::closure(closure);
        let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
        Tween {
            target: self.clone(),
            interpolator,
        }
    }
}

/// Trait for type to convert into a target type.
pub trait IntoTarget {
    /// The target type
    type Target;

    /// Convert [`Self`] into [`Self::Target`]
    fn into_target(self) -> Self::Target;
}

impl IntoTarget for Entity {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entity(self)
    }
}

impl<const N: usize> IntoTarget for [Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for &[Entity] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl<const N: usize> IntoTarget for &[Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl IntoTarget for &Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl IntoTarget for AnimationTarget {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::marker()
    }
}
