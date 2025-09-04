use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::ecs::system::ScheduleSystem;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod interpolate;
pub mod interpolation;
pub mod tween;
pub mod tween_event;

pub mod combinator;

pub use bevy_time_runner;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};
    pub use crate::interpolation::EaseKind;

    pub use crate::bevy_time_runner::{Repeat, RepeatStyle, TimeDirection};

    pub use crate::combinator::{AnimationBuilderExt, TransformTargetStateExt};

    pub use crate::tween::IntoTarget;
    pub use crate::tween_event::{TweenEvent, TweenEventData};

    pub use crate::tween::ComponentDynTween;
    pub use crate::tween::ComponentTween;

    pub use crate::tween::ResourceDynTween;
    pub use crate::tween::ResourceTween;

    pub use crate::BevyTweenRegisterSystems;
    pub use crate::DefaultTweenPlugins;
}

pub use tween::component_tween_system;

pub use tween::resource_dyn_tween_system;
pub use tween::resource_tween_system;

pub use tween_event::tween_event_system;

pub struct DefaultTweenPlugins;

impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin::default())
            .add(interpolate::DefaultInterpolatorsPlugin)
            .add(interpolate::DefaultDynInterpolatorsPlugin)
            .add(interpolation::EaseKindPlugin)
            .add_group(tween_event::DefaultTweenEventPlugins);
        group
    }
}

/// This resource will be used while initializing tween plugin and systems.
/// [`BevyTweenRegisterSystems`] for example.
#[derive(Resource, Clone)]
pub struct TweenAppResource {
    /// Configured schedule for tween systems.
    pub schedule: InternedScheduleLabel,
}

impl Default for TweenAppResource {
    fn default() -> Self {
        TweenAppResource {
            schedule: PostUpdate.intern(),
        }
    }
}

/// Configure [`TweenSystemSet`] and register types.
///
/// [`TweenSystemSet`] configuration:
/// - In schedule configured by [`TweenAppResource`]:
///   1. [`UpdateInterpolationValue`],
///   2. [`ApplyTween`],
///
///   [`UpdateInterpolationValue`]: [`TweenSystemSet::UpdateInterpolationValue`]
///   [`ApplyTween`]: [`TweenSystemSet::ApplyTween`]
#[derive(Default)]
pub struct TweenCorePlugin {
    /// See [`TweenAppResource`]
    pub app_resource: TweenAppResource,
}

impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_time_runner::TimeRunnerPlugin>() {
            app.add_plugins(bevy_time_runner::TimeRunnerPlugin {
                schedule: self.app_resource.schedule,
            });
        }
        app.configure_sets(
            self.app_resource.schedule,
            (
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain()
                .after(bevy_time_runner::TimeRunnerSet::Progress),
        )
        .insert_resource(self.app_resource.clone())
        .register_type::<tween::AnimationTarget>()
        .register_type::<tween::TweenInterpolationValue>();
    }

    fn cleanup(&self, app: &mut App) {
        app.world_mut().remove_resource::<TweenAppResource>();
    }
}

/// Enum of SystemSet in this crate.
/// See [`TweenCorePlugin`] for default system configuration.
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenInterpolationValue`] such as
    /// [`interpolation::sample_interpolations_system`].
    UpdateInterpolationValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems:
    /// - [`tween::component_tween_system`]
    /// - [`tween::resource_tween_system`]
    /// - [`tween::asset_tween_system`]
    ///
    /// Events is not necessary related to tweening but their code is still working in the same area.
    /// - [`tween::tween_event_system`]
    ApplyTween,
}

/// Helper trait to add systems by this crate to your app and avoid mistake
/// from forgetting to use the intended schedule and set.
pub trait BevyTweenRegisterSystems {
    /// Register tween systems
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl BevyTweenRegisterSystems for App {
    /// Register tween systems in schedule configured in [`TweenAppResource`]
    /// in set [`TweenSystemSet::ApplyTween`]
    ///
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        let app_resource = self
            .world()
            .get_resource::<TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        self.add_systems(
            app_resource.schedule,
            tween_systems.in_set(TweenSystemSet::ApplyTween),
        )
    }
}
