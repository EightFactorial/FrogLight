use bevy_ecs::schedule::ScheduleLabel;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every two ticks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct TwoTicks;

impl ScheduleTrait<Virtual> for TwoTicks {
    /// Equivalent to 10 times per second.
    const MILLISECONDS: u64 = 1000 / 10;
}
