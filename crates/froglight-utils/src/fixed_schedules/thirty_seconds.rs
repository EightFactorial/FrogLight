use std::time::Duration;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::FixedTimer;

/// A [`Schedule`] that runs every thirty seconds.
///
/// Equivalent to 600 real-time ticks.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct ThirtySecondSchedule;

impl FixedTimer for ThirtySecondSchedule {
    type ScheduleTimer = ThirtySecondDuration;
    const SECONDS: f32 = 30f32;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, Resource)]
pub struct ThirtySecondDuration(pub Duration);

impl Default for ThirtySecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
