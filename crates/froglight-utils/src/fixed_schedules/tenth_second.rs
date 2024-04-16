use std::time::Duration;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::FixedTimer;

/// A [`Schedule`] that runs every 1/10 second.
///
/// Equivalent to 2 ticks.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct TenthSecondSchedule;

impl FixedTimer for TenthSecondSchedule {
    type ScheduleTimer = TenthSecondDuration;
    const SECONDS: f32 = 0.1f32;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, Resource)]
pub struct TenthSecondDuration(pub Duration);

impl Default for TenthSecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
