// type ReflectInterpolatorTransform = ReflectInterpolator<Transform>;

use crate::interpolate::Interpolator;
use bevy::prelude::*;

/// [`Interpolator`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Translation {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}
impl Interpolator for Translation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_translation = self.start.lerp(self.end, previous_value);
            let next_translation = self.start.lerp(self.end, value);
            let translation_delta = next_translation - previous_translation;
            item.translation += translation_delta;
        } else {
            item.translation = self.start.lerp(self.end, value);
        }
    }
}

/// Constructor for [`Translation`]
pub fn translation(start: Vec3, end: Vec3) -> Translation {
    Translation {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_to(to: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        translation(start, end)
    }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_by(by: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        translation(start, end)
    }
}

/// Constructor for [`Translation`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn translation_delta_by(by: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        Translation {
            start,
            end,
            delta: true,
        }
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Rotation {
    #[allow(missing_docs)]
    pub start: Quat,
    #[allow(missing_docs)]
    pub end: Quat,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}
impl Interpolator for Rotation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_rotation = self.start.slerp(self.end, previous_value);
            let next_rotation = self.start.slerp(self.end, value);
            let rotation_delta = next_rotation - previous_rotation;
            item.rotation = item.rotation.mul_quat(rotation_delta);
        } else {
            item.rotation = self.start.slerp(self.end, value);
        }
    }
}

/// Constructor for [`Rotation`]
pub fn rotation(start: Quat, end: Quat) -> Rotation {
    Rotation {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_to(to: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        rotation(start, end)
    }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_by(by: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state = state.mul_quat(by);
        rotation(start, end)
    }
}

/// Constructor for [`Rotation`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn rotation_delta_by(by: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state = state.mul_quat(by);
        Rotation {
            start,
            end,
            delta: true,
        }
    }
}

/// [`Interpolator`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Scale {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}
impl Interpolator for Scale {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_scale = self.start.lerp(self.end, previous_value);
            let next_scale = self.start.lerp(self.end, value);
            let scale_delta = next_scale - previous_scale;
            item.scale += scale_delta;
        } else {
            item.scale = self.start.lerp(self.end, value);
        }
    }
}

/// Constructor for [`Scale`]
pub fn scale(start: Vec3, end: Vec3) -> Scale {
    Scale {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_to(to: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        scale(start, end)
    }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_by(by: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        scale(start, end)
    }
}

/// Constructor for [`Scale`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn scale_delta_by(by: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        Scale {
            start,
            end,
            delta: true,
        }
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation at Z axis.
/// Usually used for 2D rotation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct AngleZ {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}
impl Interpolator for AngleZ {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_angle = (self.end - self.start).mul_add(previous_value, self.start);
            let update_angle = (self.end - self.start).mul_add(value, self.start);
            let angle_delta_as_quat = Quat::from_rotation_z(update_angle - previous_angle);
            item.rotation = item.rotation.mul_quat(angle_delta_as_quat);
        } else {
            let angle = (self.end - self.start).mul_add(value, self.start);
            item.rotation = Quat::from_rotation_z(angle);
        }
    }
}

/// Constructor for [`AngleZ`]
pub fn angle_z(start: f32, end: f32) -> AngleZ {
    AngleZ {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_to(to: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        angle_z(start, end)
    }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_by(by: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        angle_z(start, end)
    }
}

/// Constructor for [`AngleZ`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn angle_z_delta_by(by: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        AngleZ {
            start,
            end,
            delta: true,
        }
    }
}
