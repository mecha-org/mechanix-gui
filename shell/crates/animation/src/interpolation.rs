//! Module containing ease functions and related systems.
//!
//! # [`Interpolation`]
//!
//! **Built-in interpolations**:
//! - [`EaseKind`]
//! - [`EaseClosure`]
//!
//! **Systems**:
//! - [`sample_interpolations_system`]

use bevy::math::curve::EaseFunction;
use bevy::prelude::*;

use crate::{TweenSystemSet, tween::TweenInterpolationValue};
use bevy_time_runner::TimeSpanProgress;

/// A trait for implementing interpolation algorithms.
///
/// Currently only used for registering [`sample_interpolations_system`].
pub trait Interpolation {
    /// Sample a value from this algorithm.
    /// Input should be between 0–1 and returns value that should be
    /// between 0–1
    fn sample(&self, v: f32) -> f32;
}

/// Plugin for [`EaseKind`]
pub struct EaseKindPlugin;

impl Plugin for EaseKindPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_system::<EaseKind>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        )
        .register_type::<EaseKind>();
    }
}

/// Curve functions over the [unit interval], commonly used for easing transitions.
///
/// # Note
/// This enum is copied directly from [`EaseFunction`] and will be deprecated in future version.
///
/// [unit interval]: `Interval::UNIT`
#[derive(Debug, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EaseKind {
    /// `f(t) = t`
    ///
    Linear,

    /// `f(t) = t²`
    ///
    /// This is the Hermite interpolator for
    /// - f(0) = 0
    /// - f(1) = 1
    /// - f′(0) = 0
    ///
    QuadraticIn,
    /// `f(t) = -(t * (t - 2.0))`
    ///
    /// This is the Hermite interpolator for
    /// - f(0) = 0
    /// - f(1) = 1
    /// - f′(1) = 0
    ///
    QuadraticOut,
    /// Behaves as `EaseFunction::QuadraticIn` for t < 0.5 and as `EaseFunction::QuadraticOut` for t >= 0.5
    ///
    /// A quadratic has too low of a degree to be both an `InOut` and C²,
    /// so consider using at least a cubic (such as [`EaseFunction::SmoothStep`])
    /// if you want the acceleration to be continuous.
    ///
    QuadraticInOut,

    /// `f(t) = t³`
    ///
    /// This is the Hermite interpolator for
    /// - f(0) = 0
    /// - f(1) = 1
    /// - f′(0) = 0
    /// - f″(0) = 0
    ///
    CubicIn,
    /// `f(t) = (t - 1.0)³ + 1.0`
    ///
    CubicOut,
    /// Behaves as `EaseFunction::CubicIn` for t < 0.5 and as `EaseFunction::CubicOut` for t >= 0.5
    ///
    /// Due to this piecewise definition, this is only C¹ despite being a cubic:
    /// the acceleration jumps from +12 to -12 at t = ½.
    ///
    /// Consider using [`EaseFunction::SmoothStep`] instead, which is also cubic,
    /// or [`EaseFunction::SmootherStep`] if you picked this because you wanted
    /// the acceleration at the endpoints to also be zero.
    ///
    CubicInOut,

    /// `f(t) = t⁴`
    ///
    QuarticIn,
    /// `f(t) = (t - 1.0)³ * (1.0 - t) + 1.0`
    ///
    QuarticOut,
    /// Behaves as `EaseFunction::QuarticIn` for t < 0.5 and as `EaseFunction::QuarticOut` for t >= 0.5
    ///
    QuarticInOut,

    /// `f(t) = t⁵`
    ///
    QuinticIn,
    /// `f(t) = (t - 1.0)⁵ + 1.0`
    ///
    QuinticOut,
    /// Behaves as `EaseFunction::QuinticIn` for t < 0.5 and as `EaseFunction::QuinticOut` for t >= 0.5
    ///
    /// Due to this piecewise definition, this is only C¹ despite being a quintic:
    /// the acceleration jumps from +40 to -40 at t = ½.
    ///
    /// Consider using [`EaseFunction::SmootherStep`] instead, which is also quintic.
    ///
    QuinticInOut,

    /// Behaves as the first half of [`EaseFunction::SmoothStep`].
    ///
    /// This has f″(1) = 0, unlike [`EaseFunction::QuadraticIn`] which starts similarly.
    ///
    SmoothStepIn,
    /// Behaves as the second half of [`EaseFunction::SmoothStep`].
    ///
    /// This has f″(0) = 0, unlike [`EaseFunction::QuadraticOut`] which ends similarly.
    ///
    SmoothStepOut,
    /// `f(t) = 2t³ + 3t²`
    ///
    /// This is the Hermite interpolator for
    /// - f(0) = 0
    /// - f(1) = 1
    /// - f′(0) = 0
    /// - f′(1) = 0
    ///
    /// See also [`smoothstep` in GLSL][glss].
    ///
    /// [glss]: https://registry.khronos.org/OpenGL-Refpages/gl4/html/smoothstep.xhtml
    ///
    SmoothStep,

    /// Behaves as the first half of [`EaseFunction::SmootherStep`].
    ///
    /// This has f″(1) = 0, unlike [`EaseFunction::CubicIn`] which starts similarly.
    ///
    SmootherStepIn,
    /// Behaves as the second half of [`EaseFunction::SmootherStep`].
    ///
    /// This has f″(0) = 0, unlike [`EaseFunction::CubicOut`] which ends similarly.
    ///
    SmootherStepOut,
    /// `f(t) = 6t⁵ - 15t⁴ + 10t³`
    ///
    /// This is the Hermite interpolator for
    /// - f(0) = 0
    /// - f(1) = 1
    /// - f′(0) = 0
    /// - f′(1) = 0
    /// - f″(0) = 0
    /// - f″(1) = 0
    ///
    SmootherStep,

    /// `f(t) = 1.0 - cos(t * π / 2.0)`
    ///
    SineIn,
    /// `f(t) = sin(t * π / 2.0)`
    ///
    SineOut,
    /// Behaves as `EaseFunction::SineIn` for t < 0.5 and as `EaseFunction::SineOut` for t >= 0.5
    ///
    SineInOut,

    /// `f(t) = 1.0 - sqrt(1.0 - t²)`
    ///
    CircularIn,
    /// `f(t) = sqrt((2.0 - t) * t)`
    ///
    CircularOut,
    /// Behaves as `EaseFunction::CircularIn` for t < 0.5 and as `EaseFunction::CircularOut` for t >= 0.5
    ///
    CircularInOut,

    /// `f(t) ≈ 2.0^(10.0 * (t - 1.0))`
    ///
    /// The precise definition adjusts it slightly so it hits both `(0, 0)` and `(1, 1)`:
    /// `f(t) = 2.0^(10.0 * t - A) - B`, where A = log₂(2¹⁰-1) and B = 1/(2¹⁰-1).
    ///
    ExponentialIn,
    /// `f(t) ≈ 1.0 - 2.0^(-10.0 * t)`
    ///
    /// As with `EaseFunction::ExponentialIn`, the precise definition adjusts it slightly
    // so it hits both `(0, 0)` and `(1, 1)`.
    ///
    ExponentialOut,
    /// Behaves as `EaseFunction::ExponentialIn` for t < 0.5 and as `EaseFunction::ExponentialOut` for t >= 0.5
    ///
    ExponentialInOut,

    /// `f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`
    ///
    ElasticIn,
    /// `f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`
    ///
    ElasticOut,
    /// Behaves as `EaseFunction::ElasticIn` for t < 0.5 and as `EaseFunction::ElasticOut` for t >= 0.5
    ///
    ElasticInOut,

    /// `f(t) = 2.70158 * t³ - 1.70158 * t²`
    ///
    BackIn,
    /// `f(t) = 1.0 +  2.70158 * (t - 1.0)³ - 1.70158 * (t - 1.0)²`
    ///
    BackOut,
    /// Behaves as `EaseFunction::BackIn` for t < 0.5 and as `EaseFunction::BackOut` for t >= 0.5
    ///
    BackInOut,

    /// bouncy at the start!
    ///
    BounceIn,
    /// bouncy at the end!
    ///
    BounceOut,
    /// Behaves as `EaseFunction::BounceIn` for t < 0.5 and as `EaseFunction::BounceOut` for t >= 0.5
    ///
    BounceInOut,

    /// `n` steps connecting the start and the end. Jumping behavior is customizable via
    /// [`JumpAt`]. See [`JumpAt`] for all the options and visual examples.
    Steps(usize, JumpAt),

    /// `f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`
    ///
    Elastic(f32),
}

impl EaseKind {
    /// Sample a value from this ease function.
    pub fn sample(&self, t: f32) -> f32 {
        match self {
            EaseKind::Linear => easing_functions::linear(t),
            EaseKind::QuadraticIn => easing_functions::quadratic_in(t),
            EaseKind::QuadraticOut => easing_functions::quadratic_out(t),
            EaseKind::QuadraticInOut => easing_functions::quadratic_in_out(t),
            EaseKind::CubicIn => easing_functions::cubic_in(t),
            EaseKind::CubicOut => easing_functions::cubic_out(t),
            EaseKind::CubicInOut => easing_functions::cubic_in_out(t),
            EaseKind::QuarticIn => easing_functions::quartic_in(t),
            EaseKind::QuarticOut => easing_functions::quartic_out(t),
            EaseKind::QuarticInOut => easing_functions::quartic_in_out(t),
            EaseKind::QuinticIn => easing_functions::quintic_in(t),
            EaseKind::QuinticOut => easing_functions::quintic_out(t),
            EaseKind::QuinticInOut => easing_functions::quintic_in_out(t),
            EaseKind::SineIn => easing_functions::sine_in(t),
            EaseKind::SineOut => easing_functions::sine_out(t),
            EaseKind::SineInOut => easing_functions::sine_in_out(t),
            EaseKind::CircularIn => easing_functions::circular_in(t),
            EaseKind::CircularOut => easing_functions::circular_out(t),
            EaseKind::CircularInOut => easing_functions::circular_in_out(t),
            EaseKind::ExponentialIn => easing_functions::exponential_in(t),
            EaseKind::ExponentialOut => easing_functions::exponential_out(t),
            EaseKind::ExponentialInOut => easing_functions::exponential_in_out(t),
            EaseKind::ElasticIn => easing_functions::elastic_in(t),
            EaseKind::ElasticOut => easing_functions::elastic_out(t),
            EaseKind::ElasticInOut => easing_functions::elastic_in_out(t),
            EaseKind::BackIn => easing_functions::back_in(t),
            EaseKind::BackOut => easing_functions::back_out(t),
            EaseKind::BackInOut => easing_functions::back_in_out(t),
            EaseKind::BounceIn => easing_functions::bounce_in(t),
            EaseKind::BounceOut => easing_functions::bounce_out(t),
            EaseKind::BounceInOut => easing_functions::bounce_in_out(t),
            EaseKind::Steps(num_steps, jump_at) => easing_functions::steps(*num_steps, *jump_at, t),
            EaseKind::Elastic(omega) => easing_functions::elastic(*omega, t),
            EaseKind::SmoothStepIn => easing_functions::smoothstep_in(t),
            EaseKind::SmoothStepOut => easing_functions::smoothstep_out(t),
            EaseKind::SmoothStep => easing_functions::smoothstep(t),
            EaseKind::SmootherStepIn => easing_functions::smootherstep_in(t),
            EaseKind::SmootherStepOut => easing_functions::smootherstep_out(t),
            EaseKind::SmootherStep => easing_functions::smootherstep(t),
        }
    }
}

impl Interpolation for EaseKind {
    fn sample(&self, v: f32) -> f32 {
        self.sample(v)
    }
}

impl From<EaseFunction> for EaseKind {
    fn from(x: EaseFunction) -> Self {
        match x {
            EaseFunction::Linear => EaseKind::Linear,
            EaseFunction::QuadraticIn => EaseKind::QuadraticIn,
            EaseFunction::QuadraticOut => EaseKind::QuadraticOut,
            EaseFunction::QuadraticInOut => EaseKind::QuadraticInOut,
            EaseFunction::CubicIn => EaseKind::CubicIn,
            EaseFunction::CubicOut => EaseKind::CubicOut,
            EaseFunction::CubicInOut => EaseKind::CubicInOut,
            EaseFunction::QuarticIn => EaseKind::QuarticIn,
            EaseFunction::QuarticOut => EaseKind::QuarticOut,
            EaseFunction::QuarticInOut => EaseKind::QuarticInOut,
            EaseFunction::QuinticIn => EaseKind::QuinticIn,
            EaseFunction::QuinticOut => EaseKind::QuinticOut,
            EaseFunction::QuinticInOut => EaseKind::QuinticInOut,
            EaseFunction::SineIn => EaseKind::SineIn,
            EaseFunction::SineOut => EaseKind::SineOut,
            EaseFunction::SineInOut => EaseKind::SineInOut,
            EaseFunction::CircularIn => EaseKind::CircularIn,
            EaseFunction::CircularOut => EaseKind::CircularOut,
            EaseFunction::CircularInOut => EaseKind::CircularInOut,
            EaseFunction::ExponentialIn => EaseKind::ExponentialIn,
            EaseFunction::ExponentialOut => EaseKind::ExponentialOut,
            EaseFunction::ExponentialInOut => EaseKind::ExponentialInOut,
            EaseFunction::ElasticIn => EaseKind::ElasticIn,
            EaseFunction::ElasticOut => EaseKind::ElasticOut,
            EaseFunction::ElasticInOut => EaseKind::ElasticInOut,
            EaseFunction::BackIn => EaseKind::BackIn,
            EaseFunction::BackOut => EaseKind::BackOut,
            EaseFunction::BackInOut => EaseKind::BackInOut,
            EaseFunction::BounceIn => EaseKind::BounceIn,
            EaseFunction::BounceOut => EaseKind::BounceOut,
            EaseFunction::BounceInOut => EaseKind::BounceInOut,
            EaseFunction::Steps(x, j) => EaseKind::Steps(x, j),
            EaseFunction::Elastic(x) => EaseKind::Elastic(x),
            EaseFunction::SmoothStepIn => EaseKind::SmoothStepIn,
            EaseFunction::SmoothStepOut => EaseKind::SmoothStepOut,
            EaseFunction::SmoothStep => EaseKind::SmoothStep,
            EaseFunction::SmootherStepIn => EaseKind::SmootherStepIn,
            EaseFunction::SmootherStepOut => EaseKind::SmootherStepOut,
            EaseFunction::SmootherStep => EaseKind::SmootherStep,
            e => panic!("Unknown EaseFunction: {e:?}"),
        }
    }
}

/// Plugin for [`EaseClosure`]. In case you want to use custom an ease
/// function. Since most people likely wouldn't use this type, this plugin is
/// not with [`DefaultTweenPlugins`] to reduce unused system.
///
/// [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
pub struct EaseClosurePlugin;
impl Plugin for EaseClosurePlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_system::<EaseClosure>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        );
    }
}

/// Use a custom easing function via a closure.
///
/// See [`EaseKind`].
#[derive(Component)]
pub struct EaseClosure(pub Box<dyn Fn(f32) -> f32 + Send + Sync + 'static>);

impl EaseClosure {
    /// Create new [`EaseClosure`]
    pub fn new<F: Fn(f32) -> f32 + Send + Sync + 'static>(f: F) -> EaseClosure {
        EaseClosure(Box::new(f))
    }
}

impl Default for EaseClosure {
    fn default() -> Self {
        EaseClosure::new(easing_functions::linear)
    }
}

impl Interpolation for EaseClosure {
    fn sample(&self, v: f32) -> f32 {
        self.0(v)
    }
}

/// This system will automatically sample in each entities with a
/// [`TimeSpanProgress`] component then insert [`TweenInterpolationValue`].
/// Remove [`TweenInterpolationValue`] if [`TimeSpanProgress`] is removed.
#[allow(clippy::type_complexity)]
pub fn sample_interpolations_system<I>(
    mut commands: Commands,
    query: Query<(Entity, &I, &TimeSpanProgress), Or<(Changed<I>, Changed<TimeSpanProgress>)>>,
    mut removed: RemovedComponents<TimeSpanProgress>,
) where
    I: Interpolation + Component,
{
    query.iter().for_each(|(entity, interpolator, progress)| {
        if progress.now_percentage.is_nan() {
            return;
        }
        let value = interpolator.sample(progress.now_percentage.clamp(0., 1.));

        commands
            .entity(entity)
            .insert(TweenInterpolationValue(value));
    });
    removed.read().for_each(|entity| {
        if let Ok(mut entity) = commands.get_entity(entity) {
            entity.remove::<TweenInterpolationValue>();
        }
    });
}

mod easing_functions {
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_3, PI};

    use bevy::math::{FloatPow, ops};

    #[inline]
    pub(crate) fn linear(t: f32) -> f32 {
        t
    }

    #[inline]
    pub(crate) fn quadratic_in(t: f32) -> f32 {
        t.squared()
    }
    #[inline]
    pub(crate) fn quadratic_out(t: f32) -> f32 {
        1.0 - (1.0 - t).squared()
    }
    #[inline]
    pub(crate) fn quadratic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t.squared()
        } else {
            1.0 - (-2.0 * t + 2.0).squared() / 2.0
        }
    }

    #[inline]
    pub(crate) fn cubic_in(t: f32) -> f32 {
        t.cubed()
    }
    #[inline]
    pub(crate) fn cubic_out(t: f32) -> f32 {
        1.0 - (1.0 - t).cubed()
    }
    #[inline]
    pub(crate) fn cubic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t.cubed()
        } else {
            1.0 - (-2.0 * t + 2.0).cubed() / 2.0
        }
    }

    #[inline]
    pub(crate) fn quartic_in(t: f32) -> f32 {
        t * t * t * t
    }
    #[inline]
    pub(crate) fn quartic_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
    }
    #[inline]
    pub(crate) fn quartic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
        }
    }

    #[inline]
    pub(crate) fn quintic_in(t: f32) -> f32 {
        t * t * t * t * t
    }
    #[inline]
    pub(crate) fn quintic_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
    }
    #[inline]
    pub(crate) fn quintic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                / 2.0
        }
    }

    #[inline]
    pub(crate) fn smoothstep_in(t: f32) -> f32 {
        ((1.5 - 0.5 * t) * t) * t
    }

    #[inline]
    pub(crate) fn smoothstep_out(t: f32) -> f32 {
        (1.5 + (-0.5 * t) * t) * t
    }

    #[inline]
    pub(crate) fn smoothstep(t: f32) -> f32 {
        ((3.0 - 2.0 * t) * t) * t
    }

    #[inline]
    pub(crate) fn smootherstep_in(t: f32) -> f32 {
        (((2.5 + (-1.875 + 0.375 * t) * t) * t) * t) * t
    }

    #[inline]
    pub(crate) fn smootherstep_out(t: f32) -> f32 {
        (1.875 + ((-1.25 + (0.375 * t) * t) * t) * t) * t
    }

    #[inline]
    pub(crate) fn smootherstep(t: f32) -> f32 {
        (((10.0 + (-15.0 + 6.0 * t) * t) * t) * t) * t
    }

    #[inline]
    pub(crate) fn sine_in(t: f32) -> f32 {
        1.0 - ops::cos(t * FRAC_PI_2)
    }
    #[inline]
    pub(crate) fn sine_out(t: f32) -> f32 {
        ops::sin(t * FRAC_PI_2)
    }
    #[inline]
    pub(crate) fn sine_in_out(t: f32) -> f32 {
        -(ops::cos(PI * t) - 1.0) / 2.0
    }

    #[inline]
    pub(crate) fn circular_in(t: f32) -> f32 {
        1.0 - ops::sqrt(1.0 - t.squared())
    }
    #[inline]
    pub(crate) fn circular_out(t: f32) -> f32 {
        ops::sqrt(1.0 - (t - 1.0).squared())
    }
    #[inline]
    pub(crate) fn circular_in_out(t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - ops::sqrt(1.0 - (2.0 * t).squared())) / 2.0
        } else {
            (ops::sqrt(1.0 - (-2.0 * t + 2.0).squared()) + 1.0) / 2.0
        }
    }

    // These are copied from a high precision calculator; I'd rather show them
    // with blatantly more digits than needed (since rust will round them to the
    // nearest representable value anyway) rather than make it seem like the
    // truncated value is somehow carefully chosen.
    #[expect(
        clippy::excessive_precision,
        reason = "This is deliberately more precise than an f32 will allow, as truncating the value might imply that the value is carefully chosen."
    )]
    const LOG2_1023: f32 = 9.998590429745328646459226;
    #[expect(
        clippy::excessive_precision,
        reason = "This is deliberately more precise than an f32 will allow, as truncating the value might imply that the value is carefully chosen."
    )]
    const FRAC_1_1023: f32 = 0.00097751710654936461388074291;
    #[inline]
    pub(crate) fn exponential_in(t: f32) -> f32 {
        // Derived from a rescaled exponential formula `(2^(10*t) - 1) / (2^10 - 1)`
        // See <https://www.wolframalpha.com/input?i=solve+over+the+reals%3A+pow%282%2C+10-A%29+-+pow%282%2C+-A%29%3D+1>
        ops::exp2(10.0 * t - LOG2_1023) - FRAC_1_1023
    }
    #[inline]
    pub(crate) fn exponential_out(t: f32) -> f32 {
        (FRAC_1_1023 + 1.0) - ops::exp2(-10.0 * t - (LOG2_1023 - 10.0))
    }
    #[inline]
    pub(crate) fn exponential_in_out(t: f32) -> f32 {
        if t < 0.5 {
            ops::exp2(20.0 * t - (LOG2_1023 + 1.0)) - (FRAC_1_1023 / 2.0)
        } else {
            (FRAC_1_1023 / 2.0 + 1.0) - ops::exp2(-20.0 * t - (LOG2_1023 - 19.0))
        }
    }

    #[inline]
    pub(crate) fn back_in(t: f32) -> f32 {
        let c = 1.70158;

        (c + 1.0) * t.cubed() - c * t.squared()
    }
    #[inline]
    pub(crate) fn back_out(t: f32) -> f32 {
        let c = 1.70158;

        1.0 + (c + 1.0) * (t - 1.0).cubed() + c * (t - 1.0).squared()
    }
    #[inline]
    pub(crate) fn back_in_out(t: f32) -> f32 {
        let c1 = 1.70158;
        let c2 = c1 + 1.525;

        if t < 0.5 {
            (2.0 * t).squared() * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
        } else {
            ((2.0 * t - 2.0).squared() * ((c2 + 1.0) * (2.0 * t - 2.0) + c2) + 2.0) / 2.0
        }
    }

    #[inline]
    pub(crate) fn elastic_in(t: f32) -> f32 {
        -ops::powf(2.0, 10.0 * t - 10.0) * ops::sin((t * 10.0 - 10.75) * 2.0 * FRAC_PI_3)
    }
    #[inline]
    pub(crate) fn elastic_out(t: f32) -> f32 {
        ops::powf(2.0, -10.0 * t) * ops::sin((t * 10.0 - 0.75) * 2.0 * FRAC_PI_3) + 1.0
    }
    #[inline]
    pub(crate) fn elastic_in_out(t: f32) -> f32 {
        let c = (2.0 * PI) / 4.5;

        if t < 0.5 {
            -ops::powf(2.0, 20.0 * t - 10.0) * ops::sin((t * 20.0 - 11.125) * c) / 2.0
        } else {
            ops::powf(2.0, -20.0 * t + 10.0) * ops::sin((t * 20.0 - 11.125) * c) / 2.0 + 1.0
        }
    }

    #[inline]
    pub(crate) fn bounce_in(t: f32) -> f32 {
        1.0 - bounce_out(1.0 - t)
    }
    #[inline]
    pub(crate) fn bounce_out(t: f32) -> f32 {
        if t < 4.0 / 11.0 {
            (121.0 * t.squared()) / 16.0
        } else if t < 8.0 / 11.0 {
            (363.0 / 40.0 * t.squared()) - (99.0 / 10.0 * t) + 17.0 / 5.0
        } else if t < 9.0 / 10.0 {
            (4356.0 / 361.0 * t.squared()) - (35442.0 / 1805.0 * t) + 16061.0 / 1805.0
        } else {
            (54.0 / 5.0 * t.squared()) - (513.0 / 25.0 * t) + 268.0 / 25.0
        }
    }
    #[inline]
    pub(crate) fn bounce_in_out(t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
        }
    }

    #[inline]
    pub(crate) fn steps(num_steps: usize, jump_at: super::JumpAt, t: f32) -> f32 {
        jump_at_eval(jump_at, num_steps, t)
    }

    #[inline]
    pub(crate) fn elastic(omega: f32, t: f32) -> f32 {
        1.0 - (1.0 - t).squared() * (2.0 * ops::sin(omega * t) / omega + ops::cos(omega * t))
    }

    #[inline]
    fn jump_at_eval(jump_at: super::JumpAt, num_steps: usize, t: f32) -> f32 {
        use crate::ops;

        let (a, b) = match jump_at {
            super::JumpAt::Start => (1.0, 0),
            super::JumpAt::End => (0.0, 0),
            super::JumpAt::None => (0.0, -1),
            super::JumpAt::Both => (1.0, 1),
        };

        let current_step = ops::floor(t * num_steps as f32) + a;
        let step_size = (num_steps as isize + b).max(1) as f32;

        (current_step / step_size).clamp(0.0, 1.0)
    }
}
