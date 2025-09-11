use crate::interpolate::Interpolator;
use std::sync::Arc;

impl<I> Interpolator for Box<I>
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        (**self).interpolate(item, value, previous_value)
    }
}

impl<I> Interpolator for &'static I
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        (**self).interpolate(item, value, previous_value)
    }
}

impl<I> Interpolator for Arc<I>
where
    I: Interpolator + ?Sized,
{
    type Item = I::Item;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        (**self).interpolate(item, value, previous_value)
    }
}

impl<I: 'static> Interpolator for dyn Fn(&mut I, f32, f32) + Send + Sync + 'static {
    type Item = I;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        self(item, value, previous_value)
    }
}
