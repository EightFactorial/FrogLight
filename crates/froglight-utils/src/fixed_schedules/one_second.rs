use std::time::Duration;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::FixedTimer;

/// A [`Schedule`] that runs every second.
///
/// Equivalent to 20 ticks.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct OneSecondSchedule;

impl FixedTimer for OneSecondSchedule {
    type ScheduleTimer = OneSecondDuration;
    const SECONDS: f32 = 1f32;
}

/// A [`Duration`] that tracks the time since the last run.
#[derive(Debug, Clone, Eq, PartialEq, Deref, DerefMut, Resource)]
pub struct OneSecondDuration(pub Duration);

impl Default for OneSecondDuration {
    fn default() -> Self { Self(Duration::ZERO) }
}
