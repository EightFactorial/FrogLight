use bevy_ecs::schedule::ScheduleLabel;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every five seconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct FiveSeconds;

impl ScheduleTrait for FiveSeconds {
    /// Equivalent to 1 time per 5 seconds.
    const MILLISECONDS: u64 = 1000 * 5;
}
