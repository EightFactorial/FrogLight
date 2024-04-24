use bevy_ecs::schedule::ScheduleLabel;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every ten ticks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct TenTicks;

impl ScheduleTrait<Virtual> for TenTicks {
    /// Equivalent to 2 times per second.
    const MILLISECONDS: u64 = 1000 / 2;
}
