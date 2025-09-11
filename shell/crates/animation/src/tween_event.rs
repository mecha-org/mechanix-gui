//! Module containing implementations for simple event
//!
//! # Tween event
//!
//! **Plugins**:
//! - [`DefaultTweenEventPlugins`]
//! - [`TweenEventPlugin<Data>`]
//!
//! **Components**:
//! - [`TweenEventData`]
//!
//! **Systems**
//! - [`tween_event_system`]
//!
//! **Events**:
//! - [`TweenEvent<Data>`]
//!
//! Simple event system that fires generic data in a timed manner.
//! Your data is stored in [`TweenEventData`] and the event type is [`TweenEvent`].
//! Tween event is not necessarily related to tweening.
//!
//! Add this plugin for your custom data.
//! - [`TweenEventPlugin<Data>`]
//!
//! See [`DefaultTweenEventPlugins`] for default events which is also added in
//! [`DefaultTweenPlugins`](crate::DefaultTweenPlugins)

use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};

use bevy_time_runner::TimeSpanProgress;

use crate::tween::{SkipTween, TweenInterpolationValue};

/// Plugin for simple generic event that fires at a specific time span.
#[derive(Default)]
pub struct TweenEventPlugin<Data>
where
    Data: Send + Sync + 'static + Clone,
{
    marker: PhantomData<Data>,
}

impl<Data> Plugin for TweenEventPlugin<Data>
where
    Data: Send + Sync + 'static + Clone + std::fmt::Debug,
{
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            (tween_event_system::<Data>).in_set(crate::TweenSystemSet::ApplyTween),
        )
        .add_event::<TweenEvent<Data>>();
    }
}

/// Default tween event plugins:
/// - `TweenEventPlugin::<()>::default()`,
/// - `TweenEventPlugin::<&'static str>::default()`
pub struct DefaultTweenEventPlugins;

impl PluginGroup for DefaultTweenEventPlugins {
    #[allow(unused)]
    #[allow(clippy::let_and_return)]
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<DefaultTweenEventPlugins>()
            .add(TweenEventPlugin::<()>::default())
            .add(TweenEventPlugin::<&'static str>::default())
    }
}

/// Fires [`TweenEvent`] whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct TweenEventData<Data = ()>(pub Data)
where
    Data: Send + Sync + 'static;

impl<Data: Send + Sync + 'static> TweenEventData<Data> {
    /// Create new [`TweenEventData`] with custom user data.
    pub fn with_data(data: Data) -> Self {
        TweenEventData(data)
    }
}

impl TweenEventData<()> {
    /// Create new [`TweenEventData`] with no custom user data, simply `()`.
    pub fn new() -> Self {
        TweenEventData(())
    }
}

/// Fires whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity
/// by [`tween_event_system`].
#[derive(Debug, Clone, PartialEq, Event, Reflect)]
pub struct TweenEvent<Data = ()> {
    /// Custom user data
    pub data: Data,
    /// Progress of the event
    pub progress: TimeSpanProgress,
    /// Sampled value of an interpolation.
    pub interpolation_value: Option<f32>,
    /// The entity that emitted the event
    pub entity: Entity,
}

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data>(
    mut commands: Commands,
    q_tween_event_data: Query<
        (
            Entity,
            &TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        Without<SkipTween>,
    >,
    mut event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static + std::fmt::Debug,
{
    q_tween_event_data
        .iter()
        .for_each(|(entity, event_data, progress, interpolation_value)| {
            // println!("event is {:?}  {:?} {:?}", event_data.0, progress, entity);
            let event: TweenEvent<Data> = TweenEvent {
                data: event_data.0.clone(),
                progress: *progress,
                interpolation_value: interpolation_value.map(|v| v.0),
                entity,
            };
            commands.trigger_targets(event.clone(), entity);
            event_writer.write(event);
        });
}
