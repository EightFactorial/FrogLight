use bevy_ecs::schedule::ScheduleLabel;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every second.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct OneSecond;

impl ScheduleTrait for OneSecond {
    /// Equivalent to 1 time per second.
    const MILLISECONDS: u64 = 1000;
}
