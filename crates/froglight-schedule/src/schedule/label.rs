use bevy_ecs::schedule::ScheduleLabel;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) for early-frame networking.
///
/// Typically used for receiving network messages.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PreNetwork;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) for late-frame networking.
///
/// Typically used for sending network messages.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PostNetwork;

// -------------------------------------------------------------------------------------------------

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs before [`Tick`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PreTick;

/// The main `Tick` schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct Tick;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs after [`Tick`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PostTick;
