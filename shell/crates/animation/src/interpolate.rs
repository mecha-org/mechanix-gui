//! Module containing some basic built-in interpolator
//!
//! **Plugins**:
//! - [`DefaultDynInterpolatorsPlugin`]
//! - [`DefaultInterpolatorsPlugin`]
//!
//! **Built-in interpolators**:
//! - [`Translation`]
//! - [`Rotation`]
//! - [`Scale`]
//! - [`AngleZ`]
//! - [`SpriteColor`]
//! - [`ColorMaterial`]
//!
//! # Your own [`Interpolator`]
//!
//! There are a few amount of built-in interpolator because this crate only
//! implemented the most common ones such as [`Translation`] or
//! [`SpriteColor`] and some more.
//! For others, you must implement your own!
//!
//! Let's say you've created some custom component and you want to interpolate it:
//! ```no_run
//! use bevy::prelude::*;
//!
//! #[derive(Component)]
//! struct Foo(f32);
//! ```
//!
//! You'll need to create a specific interpolator for this component by:
//! ```no_run
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct Foo(f32);
//! use bevy_tween::prelude::*;
//!
//! // First we define an interpolator type for `Foo`.
//! struct InterpolateFoo {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFoo {
//!     // We define the asscioate type `Item` as the `Foo` component
//!     type Item = Foo;
//!
//!     // Then we define how we want to interpolate `Foo`
//!     fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
//!         // Usually if the type already have the `.lerp` function provided
//!         // by the `FloatExt` trait then we can use just that
//!         item.0 = self.start.lerp(self.end, value);
//!     }
//! }
//! ```
//!
//! If you've created a custom interpolator or a custom component/asset/resource,
//! you may want to [register some systems](crate::tween#registering-systems).
//!
//! While it's recommended to use the [`Interpolator`] trait, it's not required
//! to make your interpolators work in this crate. the [`Interpolator`] as of
//! currently is only used for registering built-in simple interpolator systems
//! such as [`component_tween_system`], [`resource_tween_system`], and
//! [`asset_tween_system`]. Its next use is being object-safe for dynamic interpolator.
//!
//! If you need interpolators with more specific or complex system param, you
//! have to define your own system!
//!
//! [`component_tween_system`]: crate::tween::component_tween_system
//! [`resource_tween_system`]: crate::tween::resource_tween_system
//! [`asset_tween_system`]: crate::tween::asset_tween_system

mod blanket_impl;
mod transform;
mod ui;

pub use transform::*;

pub use ui::*;

use crate::{BevyTweenRegisterSystems, tween};
use bevy::prelude::*;

/// Alias for an `Interpolator` as a boxed trait object.
pub type BoxedInterpolator<Item> = Box<dyn Interpolator<Item = Item>>;

type InterpolatorClosure<I> = Box<dyn Fn(&mut I, f32, f32) + Send + Sync + 'static>;

/// Create boxed closure in order to be used with dynamic [`Interpolator`]
pub fn closure<I, F>(f: F) -> InterpolatorClosure<I>
where
    I: 'static,
    F: Fn(&mut I, f32, f32) + Send + Sync + 'static,
{
    Box::new(f)
}

/// [`Interpolator`] is used to specify how to interpolate an [`Self::Item`] by the
/// implementor.
///
/// Currently only used for registering systems
/// and being object-safe for dynamic interpolator.
///
/// See [module-level documentation](self) for more info.
pub trait Interpolator: Send + Sync + 'static {
    /// Type to be interpolated.
    type Item;
    /// Interpolate an item using `value` which is typically between 0–1.
    /// The value should be already sampled from an [`Interpolation`]
    ///
    /// [`Interpolation`]: crate::interpolation::Interpolation
    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32);
}

// /// Reflect [`Interpolator`] trait
// #[allow(clippy::type_complexity)]
// pub struct ReflectInterpolator<Item> {
//     get_func: fn(&dyn Reflect) -> Option<&dyn Interpolator<Item = Item>>,
//     get_mut_func:
//         fn(&mut dyn Reflect) -> Option<&mut dyn Interpolator<Item = Item>>,
//     get_boxed_func:
//         fn(
//             Box<dyn Reflect>,
//         )
//             -> Result<Box<dyn Interpolator<Item = Item>>, Box<dyn Reflect>>,
// }

// impl<Item> Clone for ReflectInterpolator<Item> {
//     #[inline]
//     fn clone(&self) -> ReflectInterpolator<Item> {
//         ReflectInterpolator {
//             get_func: Clone::clone(&self.get_func),
//             get_mut_func: Clone::clone(&self.get_mut_func),
//             get_boxed_func: Clone::clone(&self.get_boxed_func),
//         }
//     }
// }
// impl<Item> ReflectInterpolator<Item> {
//     /** Downcast a `&dyn Reflect` type to `&dyn Interpolator`.

//     If the type cannot be downcast, `None` is returned.*/
//     pub fn get<'a>(
//         &self,
//         reflect_value: &'a dyn Reflect,
//     ) -> Option<&'a dyn Interpolator<Item = Item>> {
//         (self.get_func)(reflect_value)
//     }

//     // /** Downcast a `&mut dyn Reflect` type to `&mut dyn Interpolator`.

//     // If the type cannot be downcast, `None` is returned.*/
//     // pub fn get_mut<'a>(
//     //     &self,
//     //     reflect_value: &'a mut dyn Reflect,
//     // ) -> Option<&'a mut dyn Interpolator<Item = Item>> {
//     //     (self.get_mut_func)(reflect_value)
//     // }

//     /** Downcast a `Box<dyn Reflect>` type to `Box<dyn Interpolator>`.

//     If the type cannot be downcast, this will return `Err(Box<dyn Reflect>)`.*/
//     pub fn get_boxed(
//         &self,
//         reflect_value: Box<dyn Reflect>,
//     ) -> Result<Box<dyn Interpolator<Item = Item>>, Box<dyn Reflect>> {
//         (self.get_boxed_func)(reflect_value)
//     }
// }

// impl<Item, T> bevy::reflect::FromType<T> for ReflectInterpolator<Item>
// where
//     T: Interpolator<Item = Item> + Reflect,
// {
//     fn from_type() -> Self {
//         Self {
//             get_func: |reflect_value| {
//                 <dyn Reflect>::downcast_ref::<T>(reflect_value)
//                     .map(|value| value as &dyn Interpolator<Item = Item>)
//             },
//             get_mut_func: |reflect_value| {
//                 <dyn Reflect>::downcast_mut::<T>(reflect_value)
//                     .map(|value| value as &mut dyn Interpolator<Item = Item>)
//             },
//             get_boxed_func: |reflect_value| {
//                 <dyn Reflect>::downcast::<T>(reflect_value)
//                     .map(|value| value as Box<dyn Interpolator<Item = Item>>)
//             },
//         }
//     }
// }

/// Default interpolators
///
/// Register type and systems for the following interpolators and their delta interpolators:
/// - [`Translation`]
/// - [`Rotation`]
/// - [`Scale`]
/// - [`AngleZ`]
/// - [`SpriteColor`] and [`ColorMaterial`] if `"bevy_sprite"` feature is enabled.
/// - [`BackgroundColor`] and [`BorderColor`] if `"bevy_ui"` feature is enabled.
pub struct DefaultInterpolatorsPlugin;
impl Plugin for DefaultInterpolatorsPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        app.add_tween_systems((
            tween::component_tween_system::<Translation>(),
            tween::component_tween_system::<Rotation>(),
            tween::component_tween_system::<Scale>(),
            tween::component_tween_system::<AngleZ>(),
        ))
        .register_type::<tween::ComponentTween<Translation>>()
        .register_type::<tween::ComponentTween<Rotation>>()
        .register_type::<tween::ComponentTween<Scale>>()
        .register_type::<tween::ComponentTween<AngleZ>>();

        app.add_tween_systems((
            tween::component_tween_system::<ui::BackgroundColor>(),
            tween::component_tween_system::<ui::BorderColor>(),
            tween::component_tween_system::<ui::NodeInterpolator>(),
        ))
        .register_type::<tween::ComponentTween<ui::BackgroundColor>>()
        .register_type::<tween::ComponentTween<ui::BorderColor>>()
        .register_type::<tween::ComponentTween<ui::NodeInterpolator>>();
    }
}

/// Default dynamic interpolators
///
/// Register systems for the following:
/// - [`Transform`] component.
/// - [`Sprite`] component if `"bevy_sprite"` feature is enabled.
/// - [`ColorMaterial`] asset if `"bevy_sprite"` feature is enabled.
/// - [`BackgroundColor`] and [`BorderColor`] components if `"bevy_ui"` feature is enabled.
///
/// [`ColorMaterial`]: bevy::sprite::ColorMaterial
pub struct DefaultDynInterpolatorsPlugin;
impl Plugin for DefaultDynInterpolatorsPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        app.add_tween_systems(tween::component_tween_system::<BoxedInterpolator<Transform>>());

        app.add_tween_systems((
            tween::component_tween_system::<BoxedInterpolator<bevy::prelude::BackgroundColor>>(),
            tween::component_tween_system::<BoxedInterpolator<bevy::prelude::BorderColor>>(),
            tween::component_tween_system::<BoxedInterpolator<bevy::prelude::Node>>(),
        ));
    }
}
