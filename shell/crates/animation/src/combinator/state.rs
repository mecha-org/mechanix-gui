use tween::ComponentTween;

use crate::interpolate::*;
use crate::tween::{self, TargetComponent, Tween};
use bevy::prelude::*;

/// Generic target and state
pub struct TargetState<T, V> {
    /// The target type
    pub target: T,
    /// The target's value or property
    pub value: V,
}

impl<T, V> TargetState<T, V> {
    /// Create new [`TargetState`] from target and initial value
    /// Recommended to use other methods like:
    /// - [`TargetComponent::state`]
    /// - [`TargetAsset::state`](crate::tween::TargetAsset::state)
    /// - [`TargetResource::state`](crate::tween::TargetAsset::state)
    pub fn new(target: T, value: V) -> Self {
        TargetState { target, value }
    }

    /// Change the value
    pub fn set_value(&mut self, new_value: V) -> &mut Self {
        self.value = new_value;
        self
    }

    /// Change the target
    pub fn set_target(&mut self, new_target: T) -> &mut Self {
        self.target = new_target;
        self
    }
}

impl<T, V> TargetState<T, V>
where
    T: Clone,
{
    /// Create [`ComponentTween`] of a value from this state and relative interpolator constructor
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_tween::prelude::*;
    /// use bevy_tween::interpolate::translation_to;
    ///
    /// # let sprite = Entity::PLACEHOLDER;
    /// let my_target = sprite.into_target();
    /// let mut my_target_translation = my_target.state(Vec3::ZERO);
    ///
    /// // Creating a ComponentTween that's tweening from previous value to Vec3::ONE
    /// let tween = my_target_translation.with(translation_to(Vec3::ONE));
    /// ```
    pub fn with<I>(&mut self, f: impl FnOnce(&mut V) -> I) -> Tween<T, I> {
        let interpolator = f(&mut self.value);
        Tween {
            target: self.target.clone(),
            interpolator,
        }
    }
}

/// Extension trait to create [`TransformTargetState`]
pub trait TransformTargetStateExt {
    /// Create [`TransformTargetState`] from [`Self`] and initial value
    fn transform_state(&self, value: Transform) -> TransformTargetState;
}

impl TransformTargetStateExt for TargetComponent {
    /// Create [`TransformTargetState`] from [`TargetComponent`] and initial value
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_tween::prelude::*;
    /// # use bevy::prelude::*;
    /// # let sprite = Entity::PLACEHOLDER;
    /// let my_target = sprite.into_target();
    /// let mut my_target_translation = my_target.transform_state(Transform::IDENTITY);
    ///
    /// // Creating a ComponentTween that's tweening from previous translation to Vec3::ONE
    /// let tween = my_target_translation.translation_to(Vec3::ONE);
    /// ```
    fn transform_state(&self, value: Transform) -> TransformTargetState {
        TransformTargetState::new(self.clone(), value)
    }
}

/// Transform state for animating entity
pub struct TransformTargetState {
    target: TargetComponent,
    value: Transform,
}

impl TransformTargetState {
    /// Create new [`TransformTargetState`]
    pub fn new(target: TargetComponent, value: Transform) -> TransformTargetState {
        TransformTargetState { target, value }
    }

    /// Create [`ComponentTween`] of transform from this state and relative interpolator constructor
    pub fn transform_with<I>(
        &mut self,
        f: impl FnOnce(&mut Transform) -> I,
    ) -> Tween<TargetComponent, I> {
        let interpolator = f(&mut self.value);
        Tween {
            target: self.target.clone(),
            interpolator,
        }
    }

    /// Create [`ComponentTween`] of transform's translation from this state and relative interpolator constructor
    pub fn translation_with<I>(
        &mut self,
        f: impl FnOnce(&mut Vec3) -> I,
    ) -> Tween<TargetComponent, I> {
        self.transform_with(|v| f(&mut v.translation))
    }

    /// Create [`ComponentTween`] of transform's rotation from this state and relative interpolator constructor
    pub fn rotation_with<I>(
        &mut self,
        f: impl FnOnce(&mut Quat) -> I,
    ) -> Tween<TargetComponent, I> {
        self.transform_with(|v| f(&mut v.rotation))
    }

    /// Create [`ComponentTween`] of transform's scale from this state and relative interpolator constructor
    pub fn scale_with<I>(&mut self, f: impl FnOnce(&mut Vec3) -> I) -> Tween<TargetComponent, I> {
        self.transform_with(|v| f(&mut v.scale))
    }

    /// Create [`ComponentTween`] of transform's translation tweening to provided input
    pub fn translation_to(&mut self, to: Vec3) -> ComponentTween<Translation> {
        self.translation_with(translation_to(to))
    }

    /// Create [`ComponentTween`] of transform's rotation tweening to provided input
    pub fn rotation_to(&mut self, to: Quat) -> ComponentTween<Rotation> {
        self.rotation_with(rotation_to(to))
    }

    /// Create [`ComponentTween`] of transform's scale tweening to provided input
    pub fn scale_to(&mut self, to: Vec3) -> ComponentTween<Scale> {
        self.scale_with(scale_to(to))
    }

    /// Create [`ComponentTween`] of transform's translation tweening by provided input
    pub fn translation_by(&mut self, by: Vec3) -> ComponentTween<Translation> {
        self.translation_with(translation_by(by))
    }

    /// Create [`ComponentTween`] of transform's rotation tweening by provided input
    pub fn rotation_by(&mut self, by: Quat) -> ComponentTween<Rotation> {
        self.rotation_with(rotation_by(by))
    }

    /// Create [`ComponentTween`] of transform's scale tweening by provided input
    pub fn scale_by(&mut self, by: Vec3) -> ComponentTween<Scale> {
        self.scale_with(scale_by(by))
    }

    /// Create delta [`ComponentTween`] of transform's translation tweening by provided input
    pub fn translation_delta_by(&mut self, by: Vec3) -> ComponentTween<Translation> {
        self.translation_with(translation_delta_by(by))
    }

    /// Create delta [`ComponentTween`] of transform's rotation tweening by provided input
    pub fn rotation_delta_by(&mut self, by: Quat) -> ComponentTween<Rotation> {
        self.rotation_with(rotation_delta_by(by))
    }

    /// Create delta [`ComponentTween`] of scale's rotation tweening by provided input
    pub fn scale_delta_by(&mut self, by: Vec3) -> ComponentTween<Scale> {
        self.scale_with(scale_delta_by(by))
    }
}
