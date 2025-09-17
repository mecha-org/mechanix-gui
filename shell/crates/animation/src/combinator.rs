//! Combinator framework

use std::time::Duration;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_time_runner::{Repeat, RepeatStyle, SkipTimeRunner, TimeDirection, TimeRunner, TimeSpan};

mod animation_combinators;
mod state;
pub use animation_combinators::*;
pub use state::{TargetState, TransformTargetState, TransformTargetStateExt};

/// Commands to use within an animation combinator
pub struct AnimationCommands<'r, 'a> {
    child_builder: &'r mut ChildSpawnerCommands<'a>,
}

impl<'r, 'a> AnimationCommands<'r, 'a> {
    pub(crate) fn new(
        child_builder: &'r mut ChildSpawnerCommands<'a>,
    ) -> AnimationCommands<'r, 'a> {
        AnimationCommands { child_builder }
    }

    /// Spawn an entity as a child.
    /// Currently always spawn as a child of animation root that should contains [`bevy_time_runner::TimeRunner`].
    pub fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        self.child_builder.spawn(bundle)
    }
}

/// Extension trait for types that can be used to make an animation.
pub trait AnimationBuilderExt {
    /// Construct [`AnimationBuilder`] from [`Self`]
    fn animation(&mut self) -> AnimationBuilder<'_>;
}

impl AnimationBuilderExt for EntityCommands<'_> {
    /// Construct [`AnimationBuilder`] from [`EntityCommands`].
    /// Use this entity as the animator.
    /// Tweens will be spawned as children of this entity.
    fn animation(&mut self) -> AnimationBuilder<'_> {
        AnimationBuilder::new(self.reborrow())
    }
}

impl AnimationBuilderExt for Commands<'_, '_> {
    /// Construct [`AnimationBuilder`] from [`Commands`].
    /// This will automatically spawn an entity as the animator.
    fn animation(&mut self) -> AnimationBuilder<'_> {
        AnimationBuilder::new(self.spawn_empty())
    }
}

impl AnimationBuilderExt for ChildSpawnerCommands<'_> {
    /// Construct [`AnimationBuilder`] from [`ChildSpawnerCommands`].
    /// This will automatically spawn a child entity as the animator.
    fn animation(&mut self) -> AnimationBuilder<'_> {
        AnimationBuilder::new(self.spawn_empty())
    }
}

/// Configure [`TimeRunner`] through a builder API and add animation entities
pub struct AnimationBuilder<'a> {
    entity_commands: EntityCommands<'a>,
    time_runner: Option<TimeRunner>,
    custom_length: Option<Duration>,
    skipped: bool,
}
impl<'a> AnimationBuilder<'a> {
    /// Create new [`AnimationBuilder`]
    pub fn new(entity_commands: EntityCommands<'a>) -> AnimationBuilder<'a> {
        AnimationBuilder {
            entity_commands,
            time_runner: None,
            custom_length: None,
            skipped: false,
        }
    }

    /// Get the inner [`EntityCommands`]
    pub fn entity_commands(&mut self) -> &mut EntityCommands<'a> {
        &mut self.entity_commands
    }

    /// Get the inner building [`TimeRunner`]
    pub fn time_runner(&self) -> &Option<TimeRunner> {
        &self.time_runner
    }

    /// Get the inner building [`TimeRunner`] mutably
    pub fn time_runner_mut(&mut self) -> &mut Option<TimeRunner> {
        &mut self.time_runner
    }

    /// Configure [`TimeRunner`]'s [`Repeat`]
    pub fn repeat(mut self, repeat: Repeat) -> Self {
        let time_runner = self.time_runner_or_default();
        match TimeRunner::repeat(time_runner) {
            Some((_, repeat_style)) => {
                time_runner.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                time_runner.set_repeat(Some((repeat, RepeatStyle::default())));
            }
        }
        self
    }

    /// Configure [`TimeRunner`]'s [`RepeatStyle`]
    pub fn repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        let time_runner = self.time_runner_or_default();
        match TimeRunner::repeat(time_runner) {
            Some((repeat, _)) => {
                time_runner.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                time_runner.set_repeat(Some((Repeat::Infinitely, repeat_style)));
            }
        }
        self
    }

    /// Configure [`TimeRunner`]'s `paused`. Note that pausing only pauses the timer
    /// but not the animation it self.
    pub fn paused(mut self, paused: bool) -> Self {
        self.time_runner_or_default().set_paused(paused);
        self
    }

    /// Skip [`TimeRunner`] from inserting [`TimeSpanProgress`](bevy_time_runner::TimeSpanProgress) which is a signal
    /// for an animation entity to execute animation code.
    pub fn skipped(mut self, skipped: bool) -> Self {
        self.skipped = skipped;
        self
    }

    /// [`Self::paused`] and [`Self::skipped`]
    pub fn disabled(self, disabled: bool) -> Self {
        self.paused(disabled).skipped(disabled)
    }

    /// Use custom duration instead of determined by [`insert`](Self::insert).
    pub fn length(mut self, duration: Duration) -> Self {
        self.custom_length = Some(duration);
        self
    }

    /// Configure [`TimeRunner`]'s time scale to adjust animation speed.
    /// Negative scale cause animation play in the opposite of [`TimeDirection`] and
    /// [`Repeat`] counter will tick backward.
    pub fn time_scale(mut self, scale: f32) -> Self {
        self.time_runner_or_default().set_time_scale(scale);
        self
    }

    /// Configure [`TimeRunner`]'s direction to play animation backward or forward.
    pub fn direction(mut self, direction: TimeDirection) -> Self {
        self.time_runner_or_default().set_direction(direction);
        self
    }

    fn time_runner_or_default(&mut self) -> &mut TimeRunner {
        self.time_runner.get_or_insert_with(TimeRunner::default)
    }

    /// Add animations from a closure. Animation entities will be subjected
    /// as a children of this entity.
    /// [`TimeRunner`]'s length is determined by last `&mut Duration` value unless use
    /// [`Self::length`].
    /// It's also possible to use combinator like [`go`], [`forward`], and [`backward`]
    /// as the last combinator to customize the length.
    pub fn insert<F>(self, animation: F) -> EntityCommands<'a>
    where
        F: FnOnce(&mut AnimationCommands, &mut Duration),
    {
        let AnimationBuilder {
            mut entity_commands,
            time_runner,
            custom_length,
            skipped,
        } = self;
        let mut dur = Duration::ZERO;
        entity_commands.with_children(|c| {
            let mut a = AnimationCommands::new(c);
            animation(&mut a, &mut dur);
        });
        let mut time_runner = time_runner.unwrap_or_default();
        match custom_length {
            Some(length) => {
                time_runner.set_length(length);
            }
            None => {
                time_runner.set_length(dur);
            }
        }
        entity_commands.insert(time_runner);
        if skipped {
            entity_commands.insert(SkipTimeRunner);
        }
        entity_commands
    }

    /// Insert tween components directly to this entity.
    /// Can be used to create a simple animation quickly.
    /// [`TimeRunner`]'s length is determined by provided `duration` unless use
    /// [`Self::length`]
    pub fn insert_tween_here<I, T>(
        self,
        duration: Duration,
        interpolation: I,
        tweens: T,
    ) -> EntityCommands<'a>
    where
        I: Bundle,
        T: Bundle,
    {
        let AnimationBuilder {
            mut entity_commands,
            time_runner,
            custom_length,
            skipped,
        } = self;
        let mut time_runner = time_runner.unwrap_or_default();
        match custom_length {
            Some(length) => {
                time_runner.set_length(length);
            }
            None => {
                time_runner.set_length(duration);
            }
        }

        entity_commands.insert((
            TimeSpan::try_from(Duration::ZERO..duration).unwrap(),
            interpolation,
            tweens,
            time_runner,
        ));
        if skipped {
            entity_commands.insert(SkipTimeRunner);
        }
        entity_commands
    }

    /// Insert tween components directly to this entity.
    /// Can be used to create a simple animation quickly.
    /// [`TimeRunner`]'s length is determined by provided `duration` unless use
    /// [`Self::length`]
    ///
    /// # Note
    ///
    /// If the entity does not exist when this command is executed,
    /// the resulting error will be ignored.
    pub fn try_insert_tween_here<I, T>(
        self,
        duration: Duration,
        interpolation: I,
        tweens: T,
    ) -> EntityCommands<'a>
    where
        I: Bundle,
        T: Bundle,
    {
        let AnimationBuilder {
            mut entity_commands,
            time_runner,
            custom_length,
            skipped,
        } = self;
        let mut time_runner = time_runner.unwrap_or_default();
        match custom_length {
            Some(length) => {
                time_runner.set_length(length);
            }
            None => {
                time_runner.set_length(duration);
            }
        }

        entity_commands.try_insert((
            TimeSpan::try_from(Duration::ZERO..duration).unwrap(),
            interpolation,
            tweens,
            time_runner,
        ));
        if skipped {
            entity_commands.try_insert(SkipTimeRunner);
        }
        entity_commands
    }
}
