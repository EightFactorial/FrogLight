use bevy_ecs::schedule::ScheduleLabel;

/// A [`ScheduleLabel`] that runs before the [`Tick`] schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PreTick;

/// A [`ScheduleLabel`] for the main tick schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct Tick;

/// A [`ScheduleLabel`] that runs after the [`Tick`] schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PostTick;
