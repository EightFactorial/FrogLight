use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct OneTick;

impl ScheduleTrait<Virtual> for OneTick {
    /// Equivalent to 20 times per second.
    const MILLISECONDS: u64 = 1000 / 20;
}
