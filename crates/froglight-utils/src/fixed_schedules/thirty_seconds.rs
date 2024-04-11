use std::time::Duration;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use derive_more::{Deref, DerefMut, From, Into};

use super::FixedTimer;

/// A [`Schedule`] that runs every thirty seconds.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct ThirtySecondSchedule;

impl FixedTimer for ThirtySecondSchedule {
    type ScheduleTimer = ThirtySecondDuration;
    const SECONDS: u64 = 30;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, From, Into, Resource)]
pub struct ThirtySecondDuration(Duration);

impl Default for ThirtySecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
