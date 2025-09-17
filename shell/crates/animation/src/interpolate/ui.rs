use crate::prelude::Interpolator;
use bevy::prelude::*;

/// [`Interpolator`] for Bevy's [`BackgroundColor`](bevy::prelude::BackgroundColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}

impl Interpolator for BackgroundColor {
    type Item = bevy::prelude::BackgroundColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.0.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.0 = updated_color.into();
        } else {
            item.0 = self.start.mix(&self.end, value)
        }
    }
}

/// Constructor for [`BackgroundColor`](crate::interpolate::BackgroundColor)
pub fn background_color(start: Color, end: Color) -> BackgroundColor {
    BackgroundColor {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`BackgroundColor`](crate::interpolate::BackgroundColor) that's relative to previous value using currying.
pub fn background_color_to(to: Color) -> impl Fn(&mut Color) -> BackgroundColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        background_color(start, end)
    }
}

/// Constructor for delta [`BackgroundColor`](crate::interpolate::BackgroundColor)
pub fn background_color_delta_to(to: Color) -> impl Fn(&mut Color) -> BackgroundColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        BackgroundColor {
            start,
            end,
            delta: true,
        }
    }
}

/// [`Interpolator`] for Bevy's [`BorderColor`](bevy::prelude::BorderColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}

impl Interpolator for BorderColor {
    type Item = bevy::prelude::BorderColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.0.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.0 = updated_color.into();
        } else {
            item.0 = self.start.mix(&self.end, value)
        }
    }
}

/// Constructor for [`BorderColor`](crate::interpolate::BorderColor)
pub fn border_color(start: Color, end: Color) -> BorderColor {
    BorderColor {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`BorderColor`](crate::interpolate::BorderColor) that's relative to previous value using currying.
pub fn border_color_to(to: Color) -> impl Fn(&mut Color) -> BorderColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        border_color(start, end)
    }
}

/// Constructor for [`BorderColor`](crate::interpolate::BorderColor) that's relative to previous value using currying.
pub fn border_color_delta_to(to: Color) -> impl Fn(&mut Color) -> BorderColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        BorderColor {
            start,
            end,
            delta: true,
        }
    }
}

/// [`Interpolator`] for Bevy's [`BorderColor`](bevy::prelude::BorderColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct NodeInterpolator {
    #[allow(missing_docs)]
    pub start: Node,
    #[allow(missing_docs)]
    pub end: Node,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}

// impl Interpolator for NodeInterpolator {
//     type Item = bevy::prelude::Node;

//     fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
//         item.top = match item.top {
//             Val::Px(p) => {
//                 if let Val::Px(start) = self.start.top {
//                     if let Val::Px(end) = self.end.top {
//                         Val::Px(start + (end - start) * value)
//                     } else {
//                         item.top
//                     }
//                 } else {
//                     item.top
//                 }
//             }
//             _ => item.top,
//         };
//         println!("node top is {:?}", item.top);
//     }
// }

impl Interpolator for NodeInterpolator {
    type Item = bevy::prelude::Node;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        // Helper to interpolate Val::Px
        fn interpolate_val(start: &Val, end: &Val, t: f32) -> Val {
            match (start, end) {
                (Val::Px(s), Val::Px(e)) => Val::Px(s + (e - s) * t),
                (Val::Percent(s), Val::Percent(e)) => Val::Percent(s + (e - s) * t),
                // Add additional cases (Percent, Auto, etc.) if needed for your use-case
                _ => end.clone(),
            }
        }

        if self.delta {
            // For delta: only apply incremental changes based on previous_value and value
            let prev_top = interpolate_val(&self.start.top, &self.end.top, previous_value);
            let next_top = interpolate_val(&self.start.top, &self.end.top, value);
            match (prev_top, next_top, &item.top) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.top = Val::Px(current + (next - prev));
                }
                _ => {
                    // fallback: replace with next_top if unable to delta
                    item.top = next_top;
                }
            }
            // Repeat similarly for other fields:
            let prev_bottom = interpolate_val(&self.start.bottom, &self.end.bottom, previous_value);
            let next_bottom = interpolate_val(&self.start.bottom, &self.end.bottom, value);
            match (prev_bottom, next_bottom, &item.bottom) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.bottom = Val::Px(current + (next - prev));
                }
                _ => {
                    item.bottom = next_bottom;
                }
            }
            let prev_left = interpolate_val(&self.start.left, &self.end.left, previous_value);
            let next_left = interpolate_val(&self.start.left, &self.end.left, value);
            match (prev_left, next_left, &item.left) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.left = Val::Px(current + (next - prev));
                }
                _ => {
                    item.left = next_left;
                }
            }
            let prev_right = interpolate_val(&self.start.right, &self.end.right, previous_value);
            let next_right = interpolate_val(&self.start.right, &self.end.right, value);
            match (prev_right, next_right, &item.right) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.right = Val::Px(current + (next - prev));
                }
                _ => {
                    item.right = next_right;
                }
            }
            let prev_width = interpolate_val(&self.start.width, &self.end.width, previous_value);
            let next_width = interpolate_val(&self.start.width, &self.end.width, value);
            match (prev_width, next_width, &item.width) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.width = Val::Px(current + (next - prev));
                }
                _ => {
                    item.width = next_width;
                }
            }
            let prev_height = interpolate_val(&self.start.height, &self.end.height, previous_value);
            let next_height = interpolate_val(&self.start.height, &self.end.height, value);
            match (prev_height, next_height, &item.height) {
                (Val::Px(prev), Val::Px(next), Val::Px(current)) => {
                    item.height = Val::Px(current + (next - prev));
                }
                _ => {
                    item.height = next_height;
                }
            }
        } else {
            // Absolute: just interpolate and set fields
            item.top = interpolate_val(&self.start.top, &self.end.top, value);
            item.bottom = interpolate_val(&self.start.bottom, &self.end.bottom, value);
            item.left = interpolate_val(&self.start.left, &self.end.left, value);
            item.right = interpolate_val(&self.start.right, &self.end.right, value);
            item.width = interpolate_val(&self.start.width, &self.end.width, value);
            item.height = interpolate_val(&self.start.height, &self.end.height, value);
        }
    }
}

pub fn node_to(to: Node) -> impl Fn(&mut Node) -> NodeInterpolator {
    move |state| {
        let start = state.clone();
        let end = to.clone();
        *state = to.clone();
        NodeInterpolator {
            start,
            end,
            delta: true,
        }
    }
}

pub fn node(start: Node, end: Node) -> NodeInterpolator {
    NodeInterpolator {
        start,
        end,
        delta: true,
    }
}
