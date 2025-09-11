use super::AnimationCommands;
use crate::prelude::TweenEventData;
use bevy::prelude::*;
use bevy_time_runner::TimeSpan;
use std::time::Duration;

/// Animations in sequence.
///
/// Each animation output will be passed to the next one.
/// Returns position from the last animation.
pub fn sequence<S>(sequence: S) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    S: Sequence,
{
    move |b, pos| sequence.call(b, pos)
}

/// Animations in parallel.
///
/// Each animation will receive the same starting position.
/// Returns the longest offset from the passed animations.
pub fn parallel<P>(parallel: P) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    P: Parallel,
{
    move |b, pos| parallel.call(b, pos)
}

/// Combinator for creating a basic tween using interpolation and a tween.
///
/// Starts from last position and tween for provided `duration`
///
/// Position is shifted to this tween's end.
pub fn tween<I, T>(
    duration: Duration,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    I: Bundle,
    T: Bundle,
{
    move |a, pos| {
        let start = *pos;
        let end = start + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
        ));
        *pos = end;
    }
}

/// Combinator for creating a basic tween using interpolation and a tween.
///
/// Starts and ends at provided span.
///
/// Position is not mutated because the operation is not relative.
pub fn tween_exact<S, I, T>(
    span: S,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    I: Bundle,
    T: Bundle,
{
    move |a, _pos| {
        a.spawn((span.try_into().unwrap(), interpolation, tween));
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at current position.
///
/// Position is not mutated because the event has no length.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin).
///
/// </div>
pub fn event<Data>(event_data: Data) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        a.spawn((
            TimeSpan::try_from(*pos..=*pos).unwrap(),
            TweenEventData::with_data(event_data),
        ));
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the provided position.
///
/// Position is not mutated because the operation is not relative.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin).
///
/// </div>
pub fn event_at<Data>(
    at: Duration,
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, _pos| {
        a.spawn((
            TimeSpan::try_from(at..=at).unwrap(),
            TweenEventData::with_data(event_data),
        ));
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the current position for provided length every frame.
///
/// Position is at the end of the event.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin).
///
/// </div>
pub fn event_for<Data>(
    length: Duration,
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        let start = *pos;
        let end = start + length;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            TweenEventData::with_data(event_data),
        ));
        *pos = end;
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the provided span every frame.
///
/// Position is not mutated because the operation is not relative.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin).
///
/// </div>
pub fn event_exact<S, Data>(
    span: S,
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
{
    move |a, _pos| {
        a.spawn((
            span.try_into().unwrap(),
            TweenEventData::with_data(event_data),
        ));
    }
}

/// Shift the position forward by provided duration
pub fn forward(by: Duration) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos += by
}

/// Shift the position backward by provided duration
pub fn backward(by: Duration) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos = pos.saturating_sub(by)
}

/// Set the position to the provided duration
pub fn go(to: Duration) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos = to
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Sequence: sealed::SequenceSealed {}
impl<T> Sequence for T where T: sealed::SequenceSealed {}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Parallel: sealed::ParallelSealed {}
impl<T> Parallel for T where T: sealed::ParallelSealed {}

mod sealed {
    use super::*;

    pub(super) trait SequenceSealed {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: FnOnce(&mut AnimationCommands, &mut Duration)> SequenceSealed for T {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self(a, pos)
        }
    }

    pub(super) trait ParallelSealed {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: FnOnce(&mut AnimationCommands, &mut Duration)> ParallelSealed for T {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self(a, pos)
        }
    }

    macro_rules! impl_sequence {
        ($($i:tt $t:ident)+) => {
            impl< $($t: SequenceSealed,)+ > SequenceSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
                    $(
                        self.$i.call(a, pos);
                    )*
                }
            }
        }
    }
    macro_rules! impl_parallel {
        ($($i:tt $t:ident)+) => {
            impl< $($t: ParallelSealed,)+ > ParallelSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationCommands, main_pos: &mut Duration) {
                    let mut furthest = *main_pos;
                    let mut pos = *main_pos;
                    $(
                        self.$i.call(a, &mut pos);
                        if pos > furthest {
                            furthest = pos;
                        }
                        #[allow(unused)]
                        {pos = *main_pos;}
                    )*
                    *main_pos = furthest;
                }
            }
        }
    }

    // It's possible to make a macro that use shorter input but i'm tryna make it simple here
    //
    // Built by using Helix macro:
    //
    // xyp<S-F>=;b;vf<S-T>eyp<A-;>i<space>jk;f=;b_<C-a>f<S-T>ev<A-;>l<C-a>
    //
    // starting from
    //
    // impl_TupleFnOnce! { 0 => T0 }

    impl_sequence! { 0 T0 }
    impl_sequence! { 0 T0 1 T1 }
    impl_sequence! { 0 T0 1 T1 2 T2 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 }

    impl_parallel! { 0 T0 }
    impl_parallel! { 0 T0 1 T1 }
    impl_parallel! { 0 T0 1 T1 2 T2 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 }
}
