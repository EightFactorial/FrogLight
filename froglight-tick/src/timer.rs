//! TODO

use core::{ops::Deref, time::Duration};

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_time::{Timer, TimerMode};

/// The per-instance timer for the [`Tick`] schedule.
///
/// If not ticking, disables the entity and all children
/// recursively for the duration of the [`Tick`] schedule.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Debug, Clone, PartialEq, Component)]
pub struct TickTimer {
    timer: Timer,
}

impl TickTimer {
    /// Create the default [`TickTimer`] with a duration of 50 milliseconds,
    /// or 20 ticks per second (TPS).
    #[inline]
    #[must_use]
    pub fn default_20tps() -> Self { Self::new_millis(50) }

    /// Create the default [`TickTimer`] with a duration in milliseconds.
    #[must_use]
    pub fn new_millis(millis: u64) -> Self {
        Self { timer: Timer::new(Duration::from_millis(millis), TimerMode::Repeating) }
    }

    /// Create the default [`TickTimer`] with a duration in seconds.
    #[must_use]
    pub fn new_from_secs_f32(secs: f32) -> Self {
        Self { timer: Timer::new(Duration::from_secs_f32(secs), TimerMode::Repeating) }
    }

    /// Create the default [`TickTimer`] with a duration in seconds.
    #[must_use]
    pub fn new_from_secs_f64(secs: f64) -> Self {
        Self { timer: Timer::new(Duration::from_secs_f64(secs), TimerMode::Repeating) }
    }

    /// Create the default [`TickTimer`] with a duration in ticks per second
    /// (TPS).
    #[must_use]
    pub fn new_tps(tps: f32) -> Self {
        let duration = Duration::from_secs_f32(1.0 / tps);
        Self { timer: Timer::new(duration, TimerMode::Repeating) }
    }

    /// Set the [`Duration`] of the tick timer.
    #[inline]
    pub fn set_duration(&mut self, duration: Duration) { self.timer.set_duration(duration) }

    /// Advance the timer by the given [`Duration`].
    #[inline]
    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.timer.tick(delta);
        self
    }
}

impl Deref for TickTimer {
    type Target = Timer;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.timer }
}
