use std::time::Duration;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use derive_more::{Deref, DerefMut, From, Into};

use super::FixedTimer;

/// A [`Schedule`] that runs every second.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct OneSecondSchedule;

impl FixedTimer for OneSecondSchedule {
    type ScheduleTimer = OneSecondDuration;
    const SECONDS: u64 = 1;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, From, Into, Resource)]
pub struct OneSecondDuration(Duration);

impl Default for OneSecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
