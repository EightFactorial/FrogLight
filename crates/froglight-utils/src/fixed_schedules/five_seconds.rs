use std::time::Duration;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use derive_more::{Deref, DerefMut, From, Into};

use super::FixedTimer;

/// A [`Schedule`] that runs every five seconds.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct FiveSecondSchedule;

impl FixedTimer for FiveSecondSchedule {
    type ScheduleTimer = FiveSecondDuration;
    const SECONDS: u64 = 5;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, From, Into, Resource)]
pub struct FiveSecondDuration(Duration);

impl Default for FiveSecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
