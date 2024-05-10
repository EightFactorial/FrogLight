use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every ten ticks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct TenTicks;

impl ScheduleTrait<Virtual> for TenTicks {
    /// Equivalent to 2 times per second.
    const MILLISECONDS: u64 = 1000 / 2;
}
