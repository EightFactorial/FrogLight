use std::time::Duration;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::FixedTimer;

/// A [`Schedule`] that runs every five seconds.
///
/// Equivalent to 100 real-time ticks.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct FiveSecondSchedule;

impl FixedTimer for FiveSecondSchedule {
    type ScheduleTimer = FiveSecondDuration;
    const SECONDS: f32 = 5f32;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, Resource)]
pub struct FiveSecondDuration(pub Duration);

impl Default for FiveSecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
