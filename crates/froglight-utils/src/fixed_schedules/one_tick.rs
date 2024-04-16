use std::time::Duration;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::FixedTimer;

/// A [`Schedule`] that runs every tick.
///
/// In real-time, this is equivalent to 20 times per second.
///
/// This runs in virtual time, which may be faster or slower than real time.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct OneTickSchedule;

impl FixedTimer for OneTickSchedule {
    type ScheduleTimer = OneTickDuration;
    const SECONDS: f32 = 0.05f32;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, Resource)]
pub struct OneTickDuration(pub Duration);

impl Default for OneTickDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
